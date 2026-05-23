//! Icarus Verilog adapter (tasks.md item #47).
//!
//! This module is the [`DigitalSimulator`] implementation that bridges
//! the [`super::MixedSignalScheduler`] to an external **Icarus Verilog** event
//! kernel running under the `vvp` runtime. Per ADR-0004 the digital
//! kernel exposes only three behaviours to the scheduler:
//!
//! 1. **Next-event-time query** — *"when is your next scheduled
//!    event?"* — the scheduler reads this *before* advancing the
//!    analog solver.
//! 2. **Event delivery** — given a synchronisation boundary, the
//!    digital kernel either *confirms* that its predicted event landed
//!    there, *postpones* to a later time, or reports a *misprediction*
//!    (an event actually occurred earlier).
//! 3. **Rollback-to-checkpoint** — when the scheduler decides to undo
//!    a tentative boundary, the digital kernel must wind its event
//!    queue back to a previously checkpointed time.
//!
//! Per ADR-0004's *Consequences* section: *"The external digital
//! simulator must expose a next-event-time API and accept a
//! rollback-to-checkpoint protocol; simulators lacking this API
//! require an adapter outside the scheduler boundary."* This module is
//! that adapter.
//!
//! # Transport abstraction
//!
//! Icarus exposes its runtime via VVP — a stack-based virtual machine
//! that consumes a compiled `.vvp` artefact and a control channel.
//! Real deployments will drive `vvp` either via the VPI/DPI tasks
//! interface or via a separately-spawned `vvp` child process that the
//! adapter steps via a wire protocol (stdin/stdout JSONL, named pipe,
//! Unix socket — the choice does not affect the scheduler-facing
//! contract). To keep the adapter testable without a live `vvp`, the
//! transport is abstracted behind the [`VvpTransport`] trait. The
//! adapter itself owns no I/O — it speaks only [`VvpTransport`] verbs.
//!
//! Concretely the adapter's three responsibilities decompose as:
//!
//! ```text
//! IcarusVerilogAdapter::next_event_time()
//!     └── VvpTransport::query_next_event()      // "$next-event-time"
//!
//! IcarusVerilogAdapter::confirm_event(boundary)
//!     └── VvpTransport::advance_and_report(boundary)
//!             // VVP runs all events at <= boundary, returns:
//!             //   - Confirmed if an event landed exactly at boundary
//!             //   - Postponed{new_prediction} if event slipped past
//!             //   - Mispredicted{actual_time} if an earlier event fired
//!     // adapter then records the event(s) in its VCD trace
//!
//! IcarusVerilogAdapter::rollback_to_checkpoint(target)
//!     └── VvpTransport::rollback_to(target)
//!             // VVP must restore its event-queue snapshot from the
//!             // most recent checkpoint at-or-before `target`.
//! ```
//!
//! This file implements the adapter against any [`VvpTransport`] and
//! ships an [`InMemoryVvp`] reference transport that scripts a fixed
//! sequence of digital events. The in-memory transport is sufficient
//! to satisfy the spec's `optimistic-advance-with-correct-prediction`
//! Gherkin scenario end-to-end without a binary on the path; sibling
//! tasks (#44/#49) will exercise the misprediction and contract-
//! violation paths through the same transport surface.
//!
//! # Stability
//!
//! Per **ADR-0010** the public surface here is unstable at v1.0.0.
//! That said, the [`VvpTransport`] trait is intentionally minimal so
//! that a future binary-backed transport can be added without
//! disturbing the adapter or any downstream caller.

use std::collections::VecDeque;
use std::fmt::Write as _;

use circuit_solver_types::{DigitalEventTrace, SignalName, SimulationTime};

use crate::mixed_signal::{
    DigitalAdapterKind, DigitalSimulator, DigitalStepReport, NextEventReport, SchedulerError,
};

// ---------------------------------------------------------------------------
// Wire-protocol structs (transport-facing, not scheduler-facing)
// ---------------------------------------------------------------------------

/// The VVP runtime's reply to `advance_and_report`. Mirrors the
/// scheduler-facing [`DigitalStepReport`] shape but at the transport
/// layer — i.e. *before* the adapter has folded the event into its VCD
/// trace.
///
/// VVP is event-driven, so the wire protocol must distinguish three
/// outcomes for any given run-until boundary:
///
/// - **`Confirmed`** — at least one event landed exactly at `boundary`.
///   The set of toggled signals is reported so the adapter can update
///   its VCD trace.
/// - **`Postponed`** — VVP advanced its clock to `boundary` but no
///   event landed there; the next predicted event is `new_prediction`.
///   The scheduler interprets this as a soft misprediction and
///   typically re-queries `next_event_time`.
/// - **`Mispredicted`** — VVP discovered that an event actually fires
///   *earlier* than `boundary`; this is the contract-violating path
///   that triggers a rollback in the scheduler.
#[derive(Debug, Clone, PartialEq)]
pub enum VvpAdvanceReport {
    /// At least one digital event landed at `time` (the requested
    /// boundary). The `toggled` field names the signals that changed
    /// at that time; downstream the adapter records each one in its
    /// VCD trace and per-signal event index.
    Confirmed {
        /// The exact time of the event. Must equal the boundary on
        /// the correct-prediction path.
        time: SimulationTime,
        /// Signals whose value changed at `time`.
        toggled: Vec<SignalName>,
    },
    /// The digital simulator reached `boundary` with no event firing
    /// and has revised its next-event prediction.
    Postponed {
        /// Revised next-event prediction.
        new_prediction: SimulationTime,
    },
    /// An event was discovered to actually occur earlier than the
    /// requested boundary. Triggers a rollback in the scheduler.
    Mispredicted {
        /// The earlier time the event actually fired.
        actual_time: SimulationTime,
        /// Signals that toggled at `actual_time`. Recorded as part of
        /// the VCD trace once the rollback has settled.
        toggled: Vec<SignalName>,
    },
}

// ---------------------------------------------------------------------------
// VvpTransport trait
// ---------------------------------------------------------------------------

/// The wire-level contract between [`IcarusVerilogAdapter`] and a VVP
/// runtime instance. Implementations may speak the protocol over
/// stdio, a named pipe, an actual `vvp` child process, or — for tests
/// and the spec witness — purely in memory.
///
/// All three methods may fail; failures are surfaced to the scheduler
/// as [`SchedulerError::DigitalAdapterFailed`].
///
/// # Determinism
///
/// Implementations are expected to be deterministic for a given input
/// trace. The adapter does not retry failed transport calls; it
/// surfaces them upward verbatim.
pub trait VvpTransport {
    /// Ask the VVP runtime for its next scheduled event time.
    ///
    /// # Errors
    ///
    /// Returns a human-readable diagnostic string when the runtime is
    /// exhausted (no further events) or has otherwise lost the ability
    /// to make a prediction. The scheduler converts this into
    /// [`SchedulerError::DigitalAdapterFailed`].
    fn query_next_event(&mut self) -> Result<SimulationTime, String>;

    /// Advance the VVP runtime up to `boundary` and report what
    /// happened. See [`VvpAdvanceReport`] for the three legal
    /// outcomes.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic on transport-level failures (process
    /// died, framing error). Contract violations (events earlier than
    /// the previously announced next-event-time) are reported via
    /// [`VvpAdvanceReport::Mispredicted`], not via `Err`.
    fn advance_and_report(&mut self, boundary: SimulationTime) -> Result<VvpAdvanceReport, String>;

    /// Restore the VVP runtime to its most recent checkpoint at or
    /// before `target`. Per ADR-0004 the *digital* side also keeps
    /// sparse checkpoints so the analog and digital event queues can
    /// be rolled back in lockstep.
    ///
    /// On the current-scenario (`optimistic-advance-with-correct-prediction`)
    /// path this method is never called; sibling tasks (#44, #49)
    /// exercise it.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic if no checkpoint exists at or before
    /// `target` or if the underlying runtime refused the rollback.
    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), String>;

    /// The list of declared boundary signals, in stable order. Used
    /// by the adapter to emit a well-formed VCD declaration block
    /// at end-of-run.
    fn signals(&self) -> &[SignalName];
}

// ---------------------------------------------------------------------------
// IcarusVerilogAdapter
// ---------------------------------------------------------------------------

/// The [`DigitalSimulator`] implementation backed by an Icarus Verilog
/// VVP runtime (tasks.md item #47).
///
/// Generic over a [`VvpTransport`] so the adapter is usable with a
/// real `vvp` child process or with an in-memory transport double for
/// tests and the spec witness.
///
/// # Scope (this item)
///
/// Item #47's promise is "next-event-time query, event delivery,
/// rollback-to-checkpoint protocol via VVP runtime." All three are
/// implemented here as adapter methods over the transport. The
/// rollback path's *recovery* logic (decide where to rewind to,
/// re-issue run-until) lives in the scheduler — adapter just relays
/// the wire-level `rollback_to(target)` to VVP. Sibling task #44 wires
/// the scheduler's rollback decision-tree into this method via
/// [`DigitalSimulator::confirm_event`] returning
/// [`DigitalStepReport::Mispredicted`].
pub struct IcarusVerilogAdapter<T: VvpTransport> {
    transport: T,
    /// Events recorded during `confirm_event` calls. Each entry is
    /// `(signal, time)`. Used to populate the [`DigitalEventTrace`]
    /// at `take_trace` time.
    events: Vec<(SignalName, SimulationTime)>,
    /// Whether [`take_trace`][DigitalSimulator::take_trace] has
    /// already been called once; calling it twice would silently
    /// produce empty traces, which is a programming error.
    drained: bool,
}

impl<T: VvpTransport> IcarusVerilogAdapter<T> {
    /// Construct an adapter over the supplied VVP transport.
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            events: Vec::new(),
            drained: false,
        }
    }

    /// Borrow the underlying transport. Used by tests to inspect the
    /// transport's recorded call log post-run.
    #[must_use]
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// Issue a rollback command directly to the underlying VVP
    /// runtime. The scheduler does not call this on the
    /// correct-prediction path; it is invoked by sibling task #44's
    /// rollback handler when the misprediction path needs to wind the
    /// digital event queue back.
    ///
    /// Wraps any transport-level failure in
    /// [`SchedulerError::DigitalAdapterFailed`].
    ///
    /// # Errors
    ///
    /// Returns the transport's failure verbatim when no checkpoint at
    /// or before `target` exists in the VVP runtime.
    pub fn rollback_to_checkpoint(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.transport
            .rollback_to(target)
            .map_err(SchedulerError::DigitalAdapterFailed)?;
        // Drop any recorded events strictly after `target` so the
        // emitted VCD trace stays consistent with the digital state
        // the runtime has rolled back to.
        self.events.retain(|(_, t)| *t <= target);
        Ok(())
    }

    /// Render the accumulated events into a VCD-format string.
    ///
    /// Format conforms to the same minimal-VCD shape the scheduler's
    /// in-crate test double emits: `$timescale 1ps`, a scope block
    /// declaring each signal with a single-byte identifier (`!`,
    /// `"`, `#`, …), an `$enddefinitions` marker, then one `#<ps>`
    /// timestamp record per event time with `1<id>` lines for each
    /// toggled signal. Standard VCD readers (gtkwave, surfer, the
    /// `vcd` crate) accept this shape.
    fn render_vcd(&self) -> String {
        let mut vcd = String::new();
        vcd.push_str("$timescale 1ps $end\n");
        vcd.push_str("$scope module icarus_verilog_adapter $end\n");
        let signals = self.transport.signals();
        for (i, sig) in signals.iter().enumerate() {
            // Single printable-ASCII identifier. Capacity is bounded
            // by the scenario's signal count; the truncation cast is
            // safe in practice (we never declare more than ~90 boundary
            // signals at this layer).
            #[allow(clippy::cast_possible_truncation)]
            let id_byte = b'!' + (i as u8);
            let id = char::from(id_byte);
            let _ = writeln!(vcd, "$var wire 1 {id} {sig} $end");
        }
        vcd.push_str("$upscope $end\n$enddefinitions $end\n");

        // Group events by time, preserving signal-declaration order.
        let mut by_time: Vec<(SimulationTime, Vec<&SignalName>)> = Vec::new();
        for (sig, t) in &self.events {
            if let Some(entry) = by_time.iter_mut().find(|(tt, _)| *tt == *t) {
                entry.1.push(sig);
            } else {
                by_time.push((*t, vec![sig]));
            }
        }
        by_time.sort_by_key(|(t, _)| *t);
        for (t, sigs) in by_time {
            let _ = writeln!(vcd, "#{}", t.as_picoseconds());
            for sig in sigs {
                if let Some((i, _)) = signals.iter().enumerate().find(|(_, s)| *s == sig) {
                    #[allow(clippy::cast_possible_truncation)]
                    let id_byte = b'!' + (i as u8);
                    let id = char::from(id_byte);
                    let _ = writeln!(vcd, "1{id}");
                }
            }
        }
        vcd
    }
}

impl<T: VvpTransport> DigitalSimulator for IcarusVerilogAdapter<T> {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::IcarusVerilog
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        let predicted = self
            .transport
            .query_next_event()
            .map_err(SchedulerError::DigitalAdapterFailed)?;
        Ok(NextEventReport {
            predicted_time: predicted,
        })
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        let report = self
            .transport
            .advance_and_report(boundary)
            .map_err(SchedulerError::DigitalAdapterFailed)?;
        match report {
            VvpAdvanceReport::Confirmed { time, toggled } => {
                for sig in toggled {
                    self.events.push((sig, time));
                }
                Ok(DigitalStepReport::Confirmed { time })
            }
            VvpAdvanceReport::Postponed { new_prediction } => {
                Ok(DigitalStepReport::Postponed { new_prediction })
            }
            VvpAdvanceReport::Mispredicted {
                actual_time,
                toggled,
            } => {
                // Record the event at its actual (earlier) time so the
                // VCD trace remains physically accurate after the
                // scheduler issues its rollback. The scheduler's
                // rollback handler (sibling task #44) calls
                // `rollback_to_checkpoint` separately, which prunes
                // any later spurious events; the events recorded here
                // therefore survive the rollback.
                for sig in toggled {
                    self.events.push((sig, actual_time));
                }
                Ok(DigitalStepReport::Mispredicted { actual_time })
            }
        }
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        // `take_trace` is the end-of-run drain. The scheduler calls it
        // exactly once. We materialise the VCD lazily here so per-event
        // string formatting doesn't burden the inner loop.
        debug_assert!(
            !self.drained,
            "IcarusVerilogAdapter::take_trace called twice; should be invoked once at end-of-run"
        );
        let vcd = self.render_vcd();
        // Per-signal event index, in declaration order, with empty
        // vectors for signals that never toggled.
        let signals = self.transport.signals().to_vec();
        let mut events_by_signal: Vec<(SignalName, Vec<SimulationTime>)> = signals
            .into_iter()
            .map(|s| {
                let times: Vec<SimulationTime> = self
                    .events
                    .iter()
                    .filter_map(|(sig, t)| (*sig == s).then_some(*t))
                    .collect();
                (s, times)
            })
            .collect();
        // Drop empty rows so the resulting trace looks the same as the
        // in-crate test double's (which only emits rows for declared
        // signals that *toggled*). The scenario asserts on
        // `events_for(&signal)` which returns `None` for absent rows.
        events_by_signal.retain(|(_, times)| !times.is_empty());

        self.drained = true;
        DigitalEventTrace {
            vcd,
            events_by_signal,
        }
    }
}

// ---------------------------------------------------------------------------
// InMemoryVvp transport
// ---------------------------------------------------------------------------

/// A scripted [`VvpTransport`] used for tests and as the reference
/// transport behind the `optimistic-advance-with-correct-prediction`
/// witness. It replays a fixed sequence of digital events.
///
/// This transport is **not** a substitute for a real `vvp` child
/// process — it has no Verilog source parser, no event scheduler, no
/// VPI surface. It is the bare minimum that satisfies
/// [`VvpTransport`]'s contract well enough for spec-driven testing.
///
/// Construction takes:
///
/// - `script`: the predicted event times, in order. Each entry's
///   `signals` is the set of digital boundary signals that toggle at
///   that event time.
/// - `declared_signals`: the full set of declared boundary signals
///   (a superset of every entry's `signals`). Used to emit a stable
///   VCD declaration block.
///
/// The transport tracks every call in [`InMemoryVvp::log`] so tests
/// can assert the exact sequence of `query_next_event`,
/// `advance_and_report`, and `rollback_to` operations the adapter
/// performed.
#[derive(Debug, Clone, Default)]
pub struct InMemoryVvp {
    /// Remaining scripted events.
    script: VecDeque<ScriptedEvent>,
    /// Past events (popped from `script` and confirmed).
    confirmed: Vec<ScriptedEvent>,
    /// Declared boundary signals, in declaration order.
    declared_signals: Vec<SignalName>,
    /// Call log, in order.
    log: Vec<VvpCall>,
}

/// One entry in an [`InMemoryVvp`] script.
#[derive(Debug, Clone, PartialEq)]
pub struct ScriptedEvent {
    /// Event time.
    pub time: SimulationTime,
    /// Signals that toggle at that time. Must be a subset of the
    /// transport's `declared_signals`.
    pub signals: Vec<SignalName>,
}

/// A record of one call into [`InMemoryVvp`]. Used by tests for
/// strict-sequence assertions.
#[derive(Debug, Clone, PartialEq)]
pub enum VvpCall {
    /// `query_next_event` was called; the returned prediction is
    /// recorded.
    QueryNextEvent(Option<SimulationTime>),
    /// `advance_and_report` was called with this boundary.
    AdvanceAndReport(SimulationTime),
    /// `rollback_to` was called with this target.
    RollbackTo(SimulationTime),
}

impl InMemoryVvp {
    /// Build an in-memory transport from a script and a declared
    /// signal set.
    #[must_use]
    pub fn new(
        script: impl IntoIterator<Item = ScriptedEvent>,
        declared_signals: Vec<SignalName>,
    ) -> Self {
        Self {
            script: script.into_iter().collect(),
            confirmed: Vec::new(),
            declared_signals,
            log: Vec::new(),
        }
    }

    /// Borrow the call log.
    #[must_use]
    pub fn log(&self) -> &[VvpCall] {
        &self.log
    }

    /// Number of events confirmed so far.
    #[must_use]
    pub fn confirmed_count(&self) -> usize {
        self.confirmed.len()
    }
}

impl VvpTransport for InMemoryVvp {
    fn query_next_event(&mut self) -> Result<SimulationTime, String> {
        let head = self.script.front().map(|e| e.time);
        self.log.push(VvpCall::QueryNextEvent(head));
        head.ok_or_else(|| "in-memory VVP transport exhausted".to_string())
    }

    fn advance_and_report(&mut self, boundary: SimulationTime) -> Result<VvpAdvanceReport, String> {
        self.log.push(VvpCall::AdvanceAndReport(boundary));
        let Some(head) = self.script.pop_front() else {
            return Err(
                "in-memory VVP transport script exhausted before advance_and_report".to_string(),
            );
        };
        match head.time.cmp(&boundary) {
            std::cmp::Ordering::Equal => {
                let toggled = head.signals.clone();
                self.confirmed.push(head);
                Ok(VvpAdvanceReport::Confirmed {
                    time: boundary,
                    toggled,
                })
            }
            std::cmp::Ordering::Less => {
                // The head event actually fires earlier than the
                // requested boundary — a misprediction.
                let actual_time = head.time;
                let toggled = head.signals.clone();
                self.confirmed.push(head);
                Ok(VvpAdvanceReport::Mispredicted {
                    actual_time,
                    toggled,
                })
            }
            std::cmp::Ordering::Greater => {
                // Head event slipped past the boundary; treat as a
                // postponement and put the entry back at the front so
                // the next `query_next_event` returns the revised time.
                let new_prediction = head.time;
                self.script.push_front(head);
                Ok(VvpAdvanceReport::Postponed { new_prediction })
            }
        }
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), String> {
        self.log.push(VvpCall::RollbackTo(target));
        // Any confirmed events strictly after `target` are returned to
        // the script's front in their original order so a re-issued
        // run-until can confirm them again.
        let mut returned: Vec<ScriptedEvent> = Vec::new();
        while let Some(last) = self.confirmed.last() {
            if last.time > target {
                returned.push(self.confirmed.pop().unwrap());
            } else {
                break;
            }
        }
        for ev in returned.into_iter().rev() {
            self.script.push_front(ev);
        }
        Ok(())
    }

    fn signals(&self) -> &[SignalName] {
        &self.declared_signals
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mixed_signal::{
        AnalogSolver, AnalogStepReport, BoundarySignals, MixedSignalScheduler,
    };
    use circuit_solver_types::{AnalogTrace, NodeId, Waveform};

    // -- Stand-in analog solver --------------------------------------------

    /// Lightweight analog solver double used only inside this module's
    /// tests. Mirrors the shape of the in-crate test double in
    /// `mixed_signal::test_doubles` but is duplicated here to keep the
    /// adapter module testable in isolation — these are *adapter*
    /// tests, not scheduler tests.
    struct LinearRamp {
        observed: NodeId,
        samples: Vec<(SimulationTime, f64)>,
        checkpoints: Vec<SimulationTime>,
    }

    impl LinearRamp {
        fn new(observed: NodeId) -> Self {
            Self {
                observed,
                samples: vec![(SimulationTime::ZERO, 0.0)],
                checkpoints: Vec::new(),
            }
        }
    }

    impl AnalogSolver for LinearRamp {
        fn run_until(
            &mut self,
            target: SimulationTime,
        ) -> Result<AnalogStepReport, SchedulerError> {
            // 3.3 V at 50 ns linear ramp, then saturate.
            #[allow(clippy::cast_precision_loss)]
            let ns = target.as_nanoseconds() as f64;
            let value = 3.3 * (ns / 50.0).clamp(0.0, 1.0);
            self.samples.push((target, value));
            self.checkpoints.push(target);
            Ok(AnalogStepReport {
                time_reached: target,
                checkpoint_saved: true,
            })
        }

        fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
            self.samples.retain(|(t, _)| *t <= target);
            self.checkpoints.retain(|t| *t <= target);
            Ok(())
        }

        fn take_trace(&mut self) -> AnalogTrace {
            let (times, values): (Vec<_>, Vec<_>) = self.samples.iter().copied().unzip();
            let committed_through = times.last().copied().unwrap_or(SimulationTime::ZERO);
            AnalogTrace {
                waveforms: vec![Waveform::new(self.observed, times, values)],
                committed_through,
            }
        }
    }

    // -- Adapter-level unit tests ------------------------------------------

    /// `adapter_kind` must report `IcarusVerilog` so downstream
    /// consumers (e.g. the scheduler's metadata, a result-side router)
    /// can route by adapter family.
    #[test]
    fn icarus_adapter_kind_is_icarus_verilog() {
        let transport = InMemoryVvp::new(Vec::<ScriptedEvent>::new(), vec![]);
        let adapter = IcarusVerilogAdapter::new(transport);
        assert_eq!(adapter.adapter_kind(), DigitalAdapterKind::IcarusVerilog);
    }

    /// `next_event_time` must round-trip the transport's prediction
    /// unmodified.
    #[test]
    fn next_event_time_forwards_transport_prediction() {
        let signals = vec![SignalName::new("dout")];
        let transport = InMemoryVvp::new(
            [ScriptedEvent {
                time: SimulationTime::from_nanoseconds(50),
                signals: signals.clone(),
            }],
            signals,
        );
        let mut adapter = IcarusVerilogAdapter::new(transport);

        let report = adapter.next_event_time().expect("transport must answer");
        assert_eq!(
            report.predicted_time,
            SimulationTime::from_nanoseconds(50),
            "adapter must forward VVP's prediction verbatim"
        );

        // And the transport recorded a single query.
        assert!(
            matches!(
                adapter.transport().log().first(),
                Some(VvpCall::QueryNextEvent(Some(_)))
            ),
            "transport must have logged the query"
        );
    }

    /// `next_event_time` surfaces transport exhaustion as
    /// `SchedulerError::DigitalAdapterFailed` — the scheduler's
    /// clean-halt signal.
    #[test]
    fn next_event_time_surfaces_transport_exhaustion() {
        let transport = InMemoryVvp::new(Vec::<ScriptedEvent>::new(), vec![]);
        let mut adapter = IcarusVerilogAdapter::new(transport);
        let err = adapter
            .next_event_time()
            .expect_err("empty script must error");
        assert!(
            matches!(err, SchedulerError::DigitalAdapterFailed(_)),
            "exhaustion must surface as DigitalAdapterFailed, got {err:?}"
        );
    }

    /// `confirm_event` on the correct-prediction path emits a
    /// `Confirmed` report and records the toggled signals in the
    /// adapter's event log.
    #[test]
    fn confirm_event_correct_prediction_path() {
        let din = SignalName::new("din");
        let dout = SignalName::new("dout");
        let transport = InMemoryVvp::new(
            [ScriptedEvent {
                time: SimulationTime::from_nanoseconds(50),
                signals: vec![din.clone(), dout.clone()],
            }],
            vec![din.clone(), dout.clone()],
        );
        let mut adapter = IcarusVerilogAdapter::new(transport);

        let _ = adapter.next_event_time().unwrap();
        let report = adapter
            .confirm_event(SimulationTime::from_nanoseconds(50))
            .expect("confirm_event must succeed");
        assert_eq!(
            report,
            DigitalStepReport::Confirmed {
                time: SimulationTime::from_nanoseconds(50),
            }
        );

        // The adapter must now hold one event per declared signal at
        // 50 ns; that surfaces in the drained DigitalEventTrace.
        let trace = adapter.take_trace();
        for sig in [din, dout] {
            assert_eq!(
                trace.events_for(&sig),
                Some(&[SimulationTime::from_nanoseconds(50)][..]),
                "signal {sig} must have one event at 50 ns",
            );
        }
        // And the VCD is minimal-well-formed.
        assert!(
            trace.vcd.contains("$timescale 1ps $end"),
            "VCD must declare timescale"
        );
        assert!(
            trace.vcd.contains("$enddefinitions $end"),
            "VCD must terminate declarations"
        );
        assert!(
            trace.vcd.contains(&format!("#{}", 50_000_i64)),
            "VCD must contain a #50000 timestamp record"
        );
    }

    /// `rollback_to_checkpoint` relays through the transport and
    /// prunes recorded events past the rollback target. Not exercised
    /// on the correct-prediction path; this test pins the wire-level
    /// behaviour so sibling task #44 can compose against it.
    #[test]
    fn rollback_to_checkpoint_prunes_recorded_events() {
        let din = SignalName::new("din");
        let transport = InMemoryVvp::new(
            [
                ScriptedEvent {
                    time: SimulationTime::from_nanoseconds(30),
                    signals: vec![din.clone()],
                },
                ScriptedEvent {
                    time: SimulationTime::from_nanoseconds(60),
                    signals: vec![din.clone()],
                },
            ],
            vec![din.clone()],
        );
        let mut adapter = IcarusVerilogAdapter::new(transport);

        // Confirm both events.
        let _ = adapter.next_event_time().unwrap();
        let _ = adapter
            .confirm_event(SimulationTime::from_nanoseconds(30))
            .unwrap();
        let _ = adapter.next_event_time().unwrap();
        let _ = adapter
            .confirm_event(SimulationTime::from_nanoseconds(60))
            .unwrap();

        // Roll back to 40 ns — the 60 ns event must be evicted.
        adapter
            .rollback_to_checkpoint(SimulationTime::from_nanoseconds(40))
            .expect("rollback must succeed");

        let trace = adapter.take_trace();
        assert_eq!(
            trace.events_for(&din),
            Some(&[SimulationTime::from_nanoseconds(30)][..]),
            "events past rollback target must be dropped from the trace",
        );
    }

    /// Witness: end-to-end run of the
    /// `optimistic-advance-with-correct-prediction` scenario with the
    /// Icarus adapter wired into the real scheduler.
    ///
    /// > Given SimulationEngineer has constructed a mixed-signal
    /// > Circuit with an analog front-end and a digital Verilog block
    /// > And the digital simulator predicts a next event at time 50 ns
    /// > When the Scheduler issues a run-until command to the analog
    /// > solver for 50 ns
    /// > And the digital simulator confirms an event at 50 ns
    /// > Then the Scheduler commits the analog state at 50 ns
    /// > And the Result contains analog Waveforms and digital event
    /// > traces synchronized at 50 ns
    /// > And no rollback occurs
    #[test]
    fn optimistic_advance_with_correct_prediction_via_icarus_adapter() {
        let vout = NodeId::new(1);
        let din = SignalName::new("din");
        let dout = SignalName::new("dout");

        let transport = InMemoryVvp::new(
            [ScriptedEvent {
                time: SimulationTime::from_nanoseconds(50),
                signals: vec![din.clone(), dout.clone()],
            }],
            vec![din.clone(), dout.clone()],
        );
        let digital = IcarusVerilogAdapter::new(transport);
        let analog = LinearRamp::new(vout);

        let boundaries = BoundarySignals {
            analog_to_digital: vec![(SignalName::new("vout"), din.clone())],
            digital_to_analog: vec![(dout.clone(), SignalName::new("vin"))],
        };

        let scheduler = MixedSignalScheduler::new(
            analog,
            digital,
            boundaries,
            SimulationTime::from_nanoseconds(100),
        );
        let result = scheduler.run().expect("scheduler.run must succeed");

        // — Then the Scheduler commits the analog state at 50 ns —
        assert_eq!(
            result.final_commit(),
            Some(SimulationTime::from_nanoseconds(50)),
        );
        assert_eq!(
            result.scheduler.commits,
            vec![SimulationTime::from_nanoseconds(50)],
        );

        // — And the Result contains analog Waveforms and digital
        // event traces synchronized at 50 ns —
        let analog_wf = result
            .analog
            .waveform_for(vout)
            .expect("analog trace must hold vout waveform");
        assert!(analog_wf
            .times
            .contains(&SimulationTime::from_nanoseconds(50)));
        assert!(result.digital.vcd.contains("$timescale 1ps $end"));
        assert!(result.digital.vcd.contains(&format!("#{}", 50_000_i64))); // 50 ns = 50_000 ps
        for sig in [din, dout] {
            assert_eq!(
                result.digital.events_for(&sig),
                Some(&[SimulationTime::from_nanoseconds(50)][..]),
            );
        }

        // — And no rollback occurs —
        assert!(result.rollback_free());
        assert!(result.scheduler.rollbacks.is_empty());
    }
}
