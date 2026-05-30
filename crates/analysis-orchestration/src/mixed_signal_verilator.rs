//! Verilator adapter for the Mixed-Signal Scheduler (tasks.md item
//! #48).
//!
//! # Scope
//!
//! Per the originating spec scenario
//! `mixed-signal-cosim#optimistic-advance-with-correct-prediction`,
//! the scheduler issues a `run_until` to the analog solver for a
//! predicted next-event time, then asks the digital simulator to
//! `confirm_event` at that boundary. This adapter implements the
//! [`crate::mixed_signal::DigitalSimulator`] trait (the same
//! interface the Icarus adapter, tasks.md item #47, implements) so
//! the scheduler can be wired to a Verilator-built digital model
//! with no scheduler-side change.
//!
//! ```gherkin
//! Given SimulationEngineer has constructed a mixed-signal Circuit
//!   with an analog front-end and a digital Verilog block
//! And the digital simulator predicts a next event at time 50 ns
//! When the Scheduler issues a run-until command to the analog
//!   solver for 50 ns
//! And the digital simulator confirms an event at 50 ns
//! Then the Scheduler commits the analog state at 50 ns
//! And the Result contains analog Waveforms and digital event
//!   traces synchronized at 50 ns
//! And no rollback occurs
//! ```
//!
//! # Same interface, different runtime binding
//!
//! tasks.md item #48 explicitly says *"same interface as Icarus
//! adapter, different runtime binding"*. The interface contract is
//! the [`crate::mixed_signal::DigitalSimulator`] trait. The two
//! adapters differ in **how they reach the digital kernel**:
//!
//! - The Icarus adapter (item #47) drives `vvp`, Icarus Verilog's
//!   bytecode runtime, as a *separate process*. Events flow over a
//!   socket or pipe; `next_event_time` translates to a VPI callback
//!   query against the running VVP simulation.
//! - This Verilator adapter calls a *verilated model*: Verilator
//!   compiles the Verilog into a C++ class (`V<Top>`) which is
//!   linked as a shared object into the same address space as the
//!   scheduler. Time advance is a method call (`eval()` after
//!   incrementing `contextp()->time(...)`), and `next_event_time`
//!   is read from the model's evaluation queue rather than queried
//!   over IPC.
//!
//! The seam between the adapter and the verilated binding is the
//! [`VerilatorBackend`] trait below. Production builds plug a real
//! C++ FFI implementation in behind it (linking against
//! `libverilated.so` and the user-supplied `V<Top>.so`); for the
//! correct-prediction scenario currently in scope, the witness
//! implementation is driven by a scripted backend
//! ([`ScriptedVerilatorBackend`]). Both behave identically through
//! the scheduler-facing `DigitalSimulator` trait — that is the
//! point of the two-layer split.
//!
//! # ADRs honored
//!
//! - **ADR-0004** (Optimistic Mixed-Signal Synchronization): the
//!   adapter respects the four commitments by exposing *only* the
//!   trait surface defined in `mixed_signal.rs`. No back-channel
//!   from this adapter to the analog solver is permitted.
//! - **ADR-0007** (Zero-Order Hold at the Analog-Digital Boundary):
//!   the boundary signal exchange is not yet exercised by the
//!   current scenario (sibling task #45 owns it), so the adapter
//!   simply *records* the configured boundary signal map. When #45
//!   lands it can read the map from
//!   [`VerilatorAdapter::boundary_signals`] without an interface
//!   change.
//! - **ADR-0010** (Unstable Public Rust API Surface for v1): the
//!   types in this module are `pub` but the workspace as a whole
//!   makes no stability guarantee yet. The `VerilatorBackend` trait
//!   is `pub` precisely because real-world Verilator builds will
//!   plug their own implementation in.

use crate::mixed_signal::{
    BoundarySignals, DigitalAdapterKind, DigitalSimulator, DigitalStepReport, NextEventReport,
    SchedulerError,
};
use circuit_solver_types::{DigitalEventTrace, SignalName, SimulationTime};

// ---------------------------------------------------------------------------
// Verilator backend trait
// ---------------------------------------------------------------------------

/// The narrow surface a verilated model must expose for the
/// [`VerilatorAdapter`] to drive it.
///
/// Real implementations of this trait wrap the C++ verilated model
/// (`V<Top>`) and call its `eval()` / `contextp()->time(...)` methods
/// through FFI. This crate intentionally does not depend on a C++
/// toolchain or on `libverilated.so`; concrete bindings live in
/// downstream crates (or are pulled in behind a feature flag) and
/// implement this trait.
///
/// The trait is deliberately *event-oriented* rather than
/// *cycle-oriented*: the scheduler's optimistic-advancement contract
/// (ADR-0004) does not care about per-cycle simulation, only about
/// the next time the digital model schedules a state change. Inside
/// a real Verilator backend, [`Self::next_event_time`] would walk
/// the verilated event queue (`contextp()->timeUnit()`-aware) and
/// return the soonest scheduled non-quiescent edge.
pub trait VerilatorBackend {
    /// Return the next time at which the verilated model has a
    /// scheduled event, or `None` if the model is quiescent.
    ///
    /// # Errors
    ///
    /// Returns an error message string if the underlying model has
    /// crashed, become unreachable, or otherwise lost the ability to
    /// answer (`libverilated` aborts surface here).
    fn next_event_time(&mut self) -> Result<Option<SimulationTime>, String>;

    /// Advance the verilated model up to `boundary` time and report
    /// whether the previously predicted event materialised there.
    ///
    /// Implementations should call the verilated model's
    /// `contextp()->time(boundary)` followed by `eval()` and inspect
    /// the resulting state. See [`VerilatorStepOutcome`] for the
    /// three legal verdicts.
    ///
    /// # Errors
    ///
    /// Same error semantics as [`Self::next_event_time`]:
    /// transport/runtime failures only. Mis-predictions are
    /// signalled via [`VerilatorStepOutcome::Mispredicted`].
    fn step_to(&mut self, boundary: SimulationTime) -> Result<VerilatorStepOutcome, String>;

    /// Drain the per-signal event trace accumulated by the backend
    /// since construction (or the last call). The adapter calls this
    /// after every successful `step_to` and aggregates the result
    /// for [`DigitalSimulator::take_trace`].
    ///
    /// Real verilated backends would flush
    /// `Verilated::traceEverOn(true)`'s VCD writer at this point.
    fn take_events(&mut self) -> Vec<VerilatorEvent>;
}

/// Outcome of a single [`VerilatorBackend::step_to`] call.
///
/// Maps 1-to-1 onto the [`DigitalStepReport`] variants the scheduler
/// expects, but kept distinct so the FFI seam never leaks scheduler
/// types into a hypothetical C ABI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerilatorStepOutcome {
    /// The verilated model evaluated to a non-quiescent state at
    /// `boundary` (an event was confirmed there).
    Confirmed,
    /// The verilated model reported no event at `boundary` but
    /// discovered an event earlier at `actual_time`. The adapter
    /// translates this into [`DigitalStepReport::Mispredicted`] and
    /// the scheduler initiates rollback.
    Mispredicted {
        /// The earlier time the verilated event queue actually
        /// scheduled a transition.
        actual_time: SimulationTime,
    },
    /// The verilated model evaluated through `boundary` without any
    /// scheduled event and posted a *new* prediction further out.
    Postponed {
        /// The newly predicted next-event time.
        new_prediction: SimulationTime,
    },
}

/// A single signal-transition event captured from the verilated
/// model. The adapter aggregates these into the
/// [`DigitalEventTrace`] returned to the scheduler.
///
/// The granularity is intentionally per-edge: real verilated builds
/// drive this from `VerilatedVcd::dump`, where each `#<time>` block
/// contains one or more signal transitions.
#[derive(Debug, Clone, PartialEq)]
pub struct VerilatorEvent {
    /// Scheduler-time of the transition.
    pub time: SimulationTime,
    /// Signal that transitioned, in the same naming domain the
    /// boundary signal map uses.
    pub signal: SignalName,
    /// The new value the signal took after the transition. Stored
    /// as a string so it can carry single-bit, multi-bit, and `x`/`z`
    /// values uniformly; real backends populate this from VPI/VCD.
    pub new_value: String,
}

// ---------------------------------------------------------------------------
// Adapter
// ---------------------------------------------------------------------------

/// The Verilator adapter (tasks.md #48).
///
/// Owns a [`VerilatorBackend`] handle and the configured
/// [`BoundarySignals`] map; implements [`DigitalSimulator`] so it
/// drops in wherever the scheduler expects a digital kernel.
///
/// The adapter is generic over `B: VerilatorBackend` so production
/// code can plug a real FFI backend in (`VerilatorAdapter<FfiBackend>`)
/// and tests can use [`ScriptedVerilatorBackend`] without paying for
/// the C++ toolchain.
pub struct VerilatorAdapter<B: VerilatorBackend> {
    backend: B,
    boundary: BoundarySignals,
    signals: Vec<SignalName>,
    /// The most recent prediction returned to the scheduler via a
    /// `Postponed` step. Re-emitted from `next_event_time` so the
    /// scheduler's two-phase query/confirm loop converges on the
    /// updated prediction without re-consulting the backend.
    pending_prediction: Option<SimulationTime>,
    /// Captured events, drained on `take_trace`.
    captured: Vec<VerilatorEvent>,
}

impl<B: VerilatorBackend> VerilatorAdapter<B> {
    /// Construct a new adapter around `backend`. The `boundary`
    /// argument is the configured analog/digital signal exchange map
    /// (see [`BoundarySignals`]); for the current scenario it may be
    /// empty.
    ///
    /// `signals` is the set of digital signals whose VCD output the
    /// scheduler is expected to surface in the Result. The order is
    /// significant — it determines the VCD identifier assignment.
    pub fn new(backend: B, boundary: BoundarySignals, signals: Vec<SignalName>) -> Self {
        Self {
            backend,
            boundary,
            signals,
            pending_prediction: None,
            captured: Vec::new(),
        }
    }

    /// Borrow the configured boundary signal map. Sibling task #45
    /// reads this when it implements zero-order-hold exchange.
    #[must_use]
    pub fn boundary_signals(&self) -> &BoundarySignals {
        &self.boundary
    }

    /// Borrow the configured trace-signal list.
    #[must_use]
    pub fn signals(&self) -> &[SignalName] {
        &self.signals
    }

    /// Borrow the underlying backend; useful for diagnostics and for
    /// tests that script and then inspect the same backend.
    pub fn backend(&self) -> &B {
        &self.backend
    }
}

impl<B: VerilatorBackend> DigitalSimulator for VerilatorAdapter<B> {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::Verilator
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        // If the last `confirm_event` postponed, re-emit the cached
        // prediction; otherwise ask the backend.
        if let Some(t) = self.pending_prediction.take() {
            return Ok(NextEventReport { predicted_time: t });
        }
        match self.backend.next_event_time() {
            Ok(Some(t)) => Ok(NextEventReport { predicted_time: t }),
            Ok(None) => Err(SchedulerError::DigitalAdapterFailed(
                "verilator backend reports quiescent model with no further events".into(),
            )),
            Err(msg) => Err(SchedulerError::DigitalAdapterFailed(format!(
                "verilator backend next_event_time failed: {msg}"
            ))),
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        match self.backend.step_to(boundary) {
            Ok(VerilatorStepOutcome::Confirmed) => {
                // Drain any events the backend captured during this
                // step; we keep them in `self.captured` for the final
                // trace emission.
                self.captured.extend(self.backend.take_events());
                Ok(DigitalStepReport::Confirmed { time: boundary })
            }
            Ok(VerilatorStepOutcome::Mispredicted { actual_time }) => {
                self.captured.extend(self.backend.take_events());
                Ok(DigitalStepReport::Mispredicted { actual_time })
            }
            Ok(VerilatorStepOutcome::Postponed { new_prediction }) => {
                self.pending_prediction = Some(new_prediction);
                self.captured.extend(self.backend.take_events());
                Ok(DigitalStepReport::Postponed { new_prediction })
            }
            Err(msg) => Err(SchedulerError::DigitalAdapterFailed(format!(
                "verilator backend step_to({boundary}) failed: {msg}"
            ))),
        }
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        use std::fmt::Write as _;

        // Build the per-signal index. Preserve the registered signal
        // order so the VCD identifier codes are stable.
        let mut events_by_signal: Vec<(SignalName, Vec<SimulationTime>)> = self
            .signals
            .iter()
            .map(|s| (s.clone(), Vec::new()))
            .collect();
        for event in &self.captured {
            if let Some(slot) = events_by_signal
                .iter_mut()
                .find(|(name, _)| name == &event.signal)
            {
                slot.1.push(event.time);
            }
            // Events on un-registered signals are dropped at the
            // trace level (they still influenced `confirm_event`
            // through `step_to`), matching how a real VCD writer
            // ignores undeclared signals.
        }

        // Emit a minimal but standards-compliant VCD. The shape
        // mirrors the existing test-double output so downstream
        // parsers (and the scenario's "parseable by standard VCD
        // readers" acceptance criterion) treat both adapters
        // identically.
        let mut vcd = String::new();
        vcd.push_str("$timescale 1ps $end\n");
        vcd.push_str("$scope module verilator_top $end\n");
        for (i, sig) in self.signals.iter().enumerate() {
            // Single printable-ASCII VCD identifier code. Scenario
            // bound: signal count is small; the truncation cannot
            // happen in practice.
            #[allow(clippy::cast_possible_truncation)]
            let id_byte = b'!' + (i as u8);
            let id = char::from(id_byte);
            let _ = writeln!(vcd, "$var wire 1 {id} {sig} $end");
        }
        vcd.push_str("$upscope $end\n$enddefinitions $end\n");

        // Emit events in chronological order. Verilator's own
        // `dump` does the same; we re-sort here in case `take_events`
        // returned them out of order across multiple `step_to`
        // calls.
        let mut sorted = self.captured.clone();
        sorted.sort_by_key(|e| (e.time, e.signal.clone()));
        let mut current_time: Option<SimulationTime> = None;
        for event in &sorted {
            if current_time != Some(event.time) {
                let _ = writeln!(vcd, "#{}", event.time.as_picoseconds());
                current_time = Some(event.time);
            }
            if let Some(i) = self.signals.iter().position(|s| s == &event.signal) {
                #[allow(clippy::cast_possible_truncation)]
                let id_byte = b'!' + (i as u8);
                let id = char::from(id_byte);
                // For single-bit values use the compact VCD form
                // (`1!`, `0!`); otherwise fall back to the `b...`
                // multi-bit form.
                if event.new_value.len() == 1 {
                    let _ = writeln!(vcd, "{}{}", event.new_value, id);
                } else {
                    let _ = writeln!(vcd, "b{} {}", event.new_value, id);
                }
            }
        }

        // Reset captured buffer so a subsequent take_trace returns
        // only newly captured events (matches the trait contract:
        // "drain").
        self.captured.clear();

        DigitalEventTrace {
            vcd,
            events_by_signal,
        }
    }
}

// ---------------------------------------------------------------------------
// Scripted backend (witness implementation usable from tests and from
// non-Verilator-toolchain builds)
// ---------------------------------------------------------------------------

/// A programmable [`VerilatorBackend`] used both by this module's
/// tests and by any caller that wants to exercise the adapter
/// without the Verilator toolchain present.
///
/// The backend is configured with a *script* of expected boundaries
/// and the verdict it should return for each, plus the events to
/// emit on each step. This is the same design the
/// `DigitalSimulatorDouble` in `mixed_signal.rs` uses, scoped to the
/// Verilator FFI shape.
#[derive(Debug, Clone)]
pub struct ScriptedVerilatorBackend {
    /// Predicted next-event times the backend will hand back from
    /// `next_event_time`, FIFO.
    predictions: std::collections::VecDeque<SimulationTime>,
    /// Verdicts the backend will return from `step_to`, paired with
    /// the boundary they apply to. FIFO; the boundary acts as a
    /// scripted-contract assertion.
    steps: std::collections::VecDeque<(SimulationTime, VerilatorStepOutcome)>,
    /// Events the backend will surface at each `step_to`; the i-th
    /// entry corresponds to the i-th step. FIFO; an empty inner
    /// `Vec` is valid and means "no signal transitions on this
    /// step."
    events_per_step: std::collections::VecDeque<Vec<VerilatorEvent>>,
    /// Buffer of events accumulated by the most recent `step_to`,
    /// drained by `take_events`.
    pending_events: Vec<VerilatorEvent>,
}

impl ScriptedVerilatorBackend {
    /// Construct a scripted backend.
    ///
    /// `predictions` is the sequence of times `next_event_time` will
    /// hand back. `steps` is the parallel sequence of step outcomes
    /// at expected boundaries. `events_per_step` is the events the
    /// backend will surface at each step (parallel-indexed to
    /// `steps`); pass an empty `Vec` for a step that emits no
    /// signal transitions.
    #[must_use]
    pub fn new(
        predictions: impl IntoIterator<Item = SimulationTime>,
        steps: impl IntoIterator<Item = (SimulationTime, VerilatorStepOutcome)>,
        events_per_step: impl IntoIterator<Item = Vec<VerilatorEvent>>,
    ) -> Self {
        Self {
            predictions: predictions.into_iter().collect(),
            steps: steps.into_iter().collect(),
            events_per_step: events_per_step.into_iter().collect(),
            pending_events: Vec::new(),
        }
    }
}

impl VerilatorBackend for ScriptedVerilatorBackend {
    fn next_event_time(&mut self) -> Result<Option<SimulationTime>, String> {
        Ok(self.predictions.pop_front())
    }

    fn step_to(&mut self, boundary: SimulationTime) -> Result<VerilatorStepOutcome, String> {
        let head = self.steps.pop_front();
        // Move the events for this step into the drain buffer
        // *before* returning, so `take_events` (called by the
        // adapter immediately after) finds them.
        if let Some(events) = self.events_per_step.pop_front() {
            self.pending_events.extend(events);
        }
        match head {
            Some((expected, outcome)) if expected == boundary => Ok(outcome),
            Some((expected, _)) => Err(format!(
                "scripted backend: expected step_to({expected}), got step_to({boundary})"
            )),
            None => Err("scripted backend: step_to called after script exhausted".into()),
        }
    }

    fn take_events(&mut self) -> Vec<VerilatorEvent> {
        std::mem::take(&mut self.pending_events)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mixed_signal::{test_doubles::AnalogSolverDouble, MixedSignalScheduler};
    use circuit_solver_types::{NodeId, SimulationTime};

    /// Test-only constant 1 V analog profile. Kept as a `fn` (not a
    /// closure) because [`AnalogSolverDouble`] takes a `fn` pointer.
    fn const_one_volt(_t: SimulationTime) -> f64 {
        1.0
    }

    // ----- adapter-kind identity --------------------------------------------

    #[test]
    fn adapter_kind_is_verilator() {
        let backend = ScriptedVerilatorBackend::new(
            std::iter::empty(),
            std::iter::empty(),
            std::iter::empty(),
        );
        let signals = vec![SignalName::new("clk"), SignalName::new("dout")];
        let adapter = VerilatorAdapter::new(backend, BoundarySignals::default(), signals);
        assert_eq!(adapter.adapter_kind(), DigitalAdapterKind::Verilator);
    }

    // ----- correct-prediction Gherkin: 50 ns, single event ------------------

    #[test]
    fn correct_prediction_50ns_single_event_confirms_and_records() {
        let t50 = SimulationTime::from_nanoseconds(50);
        let clk = SignalName::new("clk");

        let backend = ScriptedVerilatorBackend::new(
            [t50],
            [(t50, VerilatorStepOutcome::Confirmed)],
            [vec![VerilatorEvent {
                time: t50,
                signal: clk.clone(),
                new_value: "1".into(),
            }]],
        );
        let mut adapter =
            VerilatorAdapter::new(backend, BoundarySignals::default(), vec![clk.clone()]);

        // Scheduler-like usage: query, confirm, drain.
        let predicted = adapter.next_event_time().expect("prediction");
        assert_eq!(predicted.predicted_time, t50);

        let report = adapter.confirm_event(t50).expect("confirm");
        assert_eq!(report, DigitalStepReport::Confirmed { time: t50 });

        let trace = adapter.take_trace();
        // Per-signal index records the event at 50 ns.
        let events = trace.events_for(&clk).expect("clk events");
        assert_eq!(events, &[t50]);
        // VCD text contains the `#<picoseconds>` timestamp and a
        // signal-change line.
        let ps = t50.as_picoseconds();
        assert!(trace.vcd.contains(&format!("#{ps}")));
        assert!(trace.vcd.contains("$timescale 1ps $end"));
        assert!(trace.vcd.contains("$var wire 1 ! clk $end"));
        assert!(trace.vcd.contains("1!"));
    }

    // ----- end-to-end against the real MixedSignalScheduler -----------------

    #[test]
    fn scheduler_drives_verilator_adapter_to_completion() {
        // The Gherkin acceptance: scheduler issues run-until(50 ns)
        // to the analog solver, confirms event at 50 ns, commits,
        // produces unified Result with no rollback.
        let t50 = SimulationTime::from_nanoseconds(50);
        let horizon = SimulationTime::from_nanoseconds(100);
        let observed = NodeId::new(1);
        let analog = AnalogSolverDouble::new(observed, const_one_volt);

        let clk = SignalName::new("clk");
        let backend = ScriptedVerilatorBackend::new(
            [t50],
            [(t50, VerilatorStepOutcome::Confirmed)],
            [vec![VerilatorEvent {
                time: t50,
                signal: clk.clone(),
                new_value: "1".into(),
            }]],
        );
        let digital = VerilatorAdapter::new(backend, BoundarySignals::default(), vec![clk.clone()]);

        let scheduler =
            MixedSignalScheduler::new(analog, digital, BoundarySignals::default(), horizon);
        let result = scheduler.run().expect("scheduler.run must succeed");

        // Scheduler committed exactly once, at 50 ns; no rollbacks.
        assert_eq!(result.scheduler.commits, vec![t50]);
        assert!(result.rollback_free(), "no rollback should occur");
        // Analog trace was committed through the boundary.
        assert_eq!(result.analog.committed_through, t50);
        assert_eq!(result.final_commit(), Some(t50));
        // Digital trace records the clk transition at 50 ns.
        assert_eq!(result.digital.events_for(&clk), Some([t50].as_slice()));
        // VCD text is non-empty and well-formed enough to contain
        // the boundary timestamp.
        let ps = t50.as_picoseconds();
        assert!(result.digital.vcd.contains(&format!("#{ps}")));
    }

    // ----- mispredicted step is forwarded as Mispredicted -------------------

    #[test]
    fn mispredicted_step_maps_to_mispredicted_report() {
        let predicted = SimulationTime::from_nanoseconds(100);
        let actual = SimulationTime::from_nanoseconds(80);
        let backend = ScriptedVerilatorBackend::new(
            [predicted],
            [(
                predicted,
                VerilatorStepOutcome::Mispredicted {
                    actual_time: actual,
                },
            )],
            [Vec::<VerilatorEvent>::new()],
        );
        let mut adapter = VerilatorAdapter::new(backend, BoundarySignals::default(), Vec::new());

        let p = adapter.next_event_time().expect("predict");
        assert_eq!(p.predicted_time, predicted);
        let r = adapter.confirm_event(predicted).expect("confirm");
        assert_eq!(
            r,
            DigitalStepReport::Mispredicted {
                actual_time: actual
            }
        );
    }

    // ----- postponed step caches the new prediction -------------------------

    #[test]
    fn postponed_step_caches_new_prediction_for_next_query() {
        let initial = SimulationTime::from_nanoseconds(40);
        let revised = SimulationTime::from_nanoseconds(70);
        let backend = ScriptedVerilatorBackend::new(
            [initial],
            [(
                initial,
                VerilatorStepOutcome::Postponed {
                    new_prediction: revised,
                },
            )],
            [Vec::<VerilatorEvent>::new()],
        );
        let mut adapter = VerilatorAdapter::new(backend, BoundarySignals::default(), Vec::new());

        let p1 = adapter.next_event_time().expect("first predict");
        assert_eq!(p1.predicted_time, initial);

        let r = adapter.confirm_event(initial).expect("confirm");
        assert_eq!(
            r,
            DigitalStepReport::Postponed {
                new_prediction: revised
            }
        );

        // Next query should hand back the revised prediction without
        // consulting the backend again.
        let p2 = adapter.next_event_time().expect("revised predict");
        assert_eq!(p2.predicted_time, revised);
    }

    // ----- backend transport failures surface as SchedulerError -------------

    #[test]
    fn backend_query_failure_propagates_as_digital_adapter_failed() {
        // Empty prediction script + empty steps → next_event_time
        // returns Ok(None), which the adapter must translate into
        // DigitalAdapterFailed.
        let backend = ScriptedVerilatorBackend::new(
            std::iter::empty(),
            std::iter::empty(),
            std::iter::empty(),
        );
        let mut adapter = VerilatorAdapter::new(backend, BoundarySignals::default(), Vec::new());
        let err = adapter.next_event_time().expect_err("expected failure");
        match err {
            SchedulerError::DigitalAdapterFailed(_) => {}
            other => panic!("expected DigitalAdapterFailed, got {other:?}"),
        }
    }

    #[test]
    fn backend_step_contract_drift_propagates_as_digital_adapter_failed() {
        let t = SimulationTime::from_nanoseconds(50);
        // Backend scripted to expect a step to 40 ns, but the
        // adapter will call step_to(50 ns); the scripted backend
        // surfaces this as an Err which must propagate as
        // DigitalAdapterFailed.
        let backend = ScriptedVerilatorBackend::new(
            [t],
            [(
                SimulationTime::from_nanoseconds(40),
                VerilatorStepOutcome::Confirmed,
            )],
            [Vec::<VerilatorEvent>::new()],
        );
        let mut adapter = VerilatorAdapter::new(backend, BoundarySignals::default(), Vec::new());
        let _ = adapter.next_event_time().expect("predict");
        let err = adapter.confirm_event(t).expect_err("expected failure");
        match err {
            SchedulerError::DigitalAdapterFailed(msg) => {
                assert!(msg.contains("verilator backend step_to"), "got: {msg}");
            }
            other => panic!("expected DigitalAdapterFailed, got {other:?}"),
        }
    }

    // ----- boundary-signal map is preserved verbatim ------------------------

    #[test]
    fn boundary_signal_map_is_retained_unchanged_for_sibling_task_45() {
        let boundary = BoundarySignals {
            analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
            digital_to_analog: vec![(SignalName::new("dout"), SignalName::new("vin"))],
        };
        let backend = ScriptedVerilatorBackend::new(
            std::iter::empty(),
            std::iter::empty(),
            std::iter::empty(),
        );
        let adapter = VerilatorAdapter::new(backend, boundary.clone(), Vec::new());
        assert_eq!(adapter.boundary_signals(), &boundary);
    }

    // ----- multi-event VCD shape: timestamps are chronological --------------

    #[test]
    fn trace_emits_events_in_chronological_order_across_multiple_steps() {
        let t1 = SimulationTime::from_nanoseconds(25);
        let t2 = SimulationTime::from_nanoseconds(50);
        let clk = SignalName::new("clk");
        let dout = SignalName::new("dout");

        let backend = ScriptedVerilatorBackend::new(
            [t1, t2],
            [
                (t1, VerilatorStepOutcome::Confirmed),
                (t2, VerilatorStepOutcome::Confirmed),
            ],
            [
                vec![VerilatorEvent {
                    time: t1,
                    signal: clk.clone(),
                    new_value: "1".into(),
                }],
                vec![
                    VerilatorEvent {
                        time: t2,
                        signal: clk.clone(),
                        new_value: "0".into(),
                    },
                    VerilatorEvent {
                        time: t2,
                        signal: dout.clone(),
                        new_value: "1".into(),
                    },
                ],
            ],
        );
        let mut adapter = VerilatorAdapter::new(
            backend,
            BoundarySignals::default(),
            vec![clk.clone(), dout.clone()],
        );

        // Drive two boundaries in order.
        for boundary in [t1, t2] {
            let _ = adapter.next_event_time().expect("predict");
            let _ = adapter.confirm_event(boundary).expect("confirm");
        }

        let trace = adapter.take_trace();
        // Per-signal indices.
        assert_eq!(trace.events_for(&clk), Some([t1, t2].as_slice()));
        assert_eq!(trace.events_for(&dout), Some([t2].as_slice()));

        // Chronological timestamp ordering in the VCD body.
        let pos1 = trace
            .vcd
            .find(&format!("#{}", t1.as_picoseconds()))
            .expect("t1 timestamp present");
        let pos2 = trace
            .vcd
            .find(&format!("#{}", t2.as_picoseconds()))
            .expect("t2 timestamp present");
        assert!(
            pos1 < pos2,
            "expected t1 (#{}) to appear before t2 (#{}) in VCD",
            t1.as_picoseconds(),
            t2.as_picoseconds()
        );
    }
}
