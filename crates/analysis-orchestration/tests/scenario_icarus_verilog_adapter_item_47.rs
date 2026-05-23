//! Integration witness for **tasks.md item #47** (Capability:
//! `mixed-signal-cosim`):
//!
//! > Implement Icarus Verilog adapter: next-event-time query, event
//! > delivery, rollback-to-checkpoint protocol via VVP runtime —
//! > @spec: mixed-signal-cosim#optimistic-advance-with-correct-prediction
//! > (depends on #42)
//!
//! Item #42 (sibling parent task) gave the `MixedSignalScheduler` its
//! `DigitalSimulator` trait and an in-crate `TestDouble`. Item #47's
//! contribution is the **real** [`DigitalSimulator`] implementation
//! that bridges the scheduler to an Icarus Verilog VVP runtime,
//! abstracted behind a transport so a live `vvp` child process can be
//! swapped in without changing the adapter's outward shape.
//!
//! This file is the per-tasks.md-item lane's complementary witness to
//! the in-crate adapter unit tests in
//! `analysis_orchestration::mixed_signal::icarus::tests`. It asserts
//! the three behavioural promises of item #47 from the **public**
//! `analysis_orchestration` API surface, against the
//! `optimistic-advance-with-correct-prediction` Gherkin block:
//!
//! 1. **Next-event-time query.** The adapter routes the scheduler's
//!    `next_event_time()` to the VVP transport's
//!    [`VvpTransport::query_next_event`].
//! 2. **Event delivery.** On `confirm_event(boundary)` the adapter
//!    advances the transport to that boundary and records the toggled
//!    signals into its accumulated VCD trace.
//! 3. **Rollback-to-checkpoint protocol.** The adapter relays a
//!    `rollback_to_checkpoint(target)` call to the transport and
//!    prunes any post-target events from the trace, so the resulting
//!    VCD is consistent with the digital state after the rollback.
//!
//! ADR refs: ADR-0004 (mixed-signal scheduler ownership; the
//! consequences section explicitly names the digital-side adapter
//! requirement met here), ADR-0010 (unstable v1 API surface — these
//! re-exports are tracked so a breaking change is caught by this
//! witness's test breakage).

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, IcarusVerilogAdapter, InMemoryVvp, MixedSignalScheduler, NextEventReport,
    SchedulerError, ScriptedEvent, VvpAdvanceReport, VvpCall, VvpTransport,
};
use circuit_solver_types::{AnalogTrace, NodeId, SignalName, SimulationTime, Waveform};

// ---------------------------------------------------------------------------
// Stand-in analog solver for the witness (not the production solver).
// ---------------------------------------------------------------------------

struct WitnessAnalog {
    observed: NodeId,
    samples: Vec<(SimulationTime, f64)>,
    checkpoints: Vec<SimulationTime>,
}

impl WitnessAnalog {
    fn new(observed: NodeId) -> Self {
        Self {
            observed,
            samples: vec![(SimulationTime::ZERO, 0.0)],
            checkpoints: Vec::new(),
        }
    }
}

impl AnalogSolver for WitnessAnalog {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        // Simple linear ramp 0 → 3.3 V across 50 ns then saturate.
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

// ---------------------------------------------------------------------------
// Witness 1 — Headline scenario via the Icarus adapter.
// ---------------------------------------------------------------------------

/// Drives the exact Gherkin block from the spec:
///
/// > Given SimulationEngineer has constructed a mixed-signal Circuit
/// > with an analog front-end and a digital Verilog block
/// > And the digital simulator predicts a next event at time 50 ns
/// > When the Scheduler issues a run-until command to the analog
/// > solver for 50 ns
/// > And the digital simulator confirms an event at 50 ns
/// > Then the Scheduler commits the analog state at 50 ns
/// > And the Result contains analog Waveforms and digital event
/// > traces synchronized at 50 ns
/// > And no rollback occurs
///
/// — but with `IcarusVerilogAdapter<InMemoryVvp>` wired in as the
/// scheduler's `DigitalSimulator` rather than the in-crate
/// `TestDouble`. This is the spec's terminal acceptance shape with the
/// real adapter on the digital side.
#[test]
fn item_47_icarus_adapter_drives_correct_prediction_scenario() {
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
    let analog = WitnessAnalog::new(vout);

    // Boundary signals as the spec describes.
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
        "scheduler must commit at the predicted boundary",
    );
    assert_eq!(
        result.scheduler.commits,
        vec![SimulationTime::from_nanoseconds(50)],
        "exactly one commit, at 50 ns",
    );

    // — And the Result contains analog Waveforms and digital event
    //   traces synchronized at 50 ns —
    let analog_wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain vout waveform");
    assert!(
        analog_wf
            .times
            .contains(&SimulationTime::from_nanoseconds(50)),
        "analog waveform must include the 50 ns sample",
    );
    assert!(
        result.digital.vcd.contains("$timescale 1ps $end"),
        "VCD must declare a timescale",
    );
    assert!(
        result.digital.vcd.contains("$enddefinitions $end"),
        "VCD must terminate its declarations block",
    );
    // 50 ns = 50_000 ps.
    assert!(
        result.digital.vcd.contains(&format!("#{}", 50_000_i64)),
        "VCD must contain a #50000 timestamp record",
    );
    for sig in [din, dout] {
        assert_eq!(
            result.digital.events_for(&sig),
            Some(&[SimulationTime::from_nanoseconds(50)][..]),
            "digital trace must record an event at 50 ns for {sig}",
        );
    }

    // — And no rollback occurs —
    assert!(
        result.rollback_free(),
        "no rollback events should be recorded on the correct-prediction path",
    );
    assert!(result.scheduler.rollbacks.is_empty());
}

// ---------------------------------------------------------------------------
// Witness 2 — Next-event-time query routes to the VVP transport.
// ---------------------------------------------------------------------------

/// The adapter must surface the transport's `query_next_event` answer
/// verbatim through `DigitalSimulator::next_event_time`. This is the
/// first of item #47's three promises.
#[test]
fn item_47_next_event_time_routes_to_transport() {
    let din = SignalName::new("din");
    let transport = InMemoryVvp::new(
        [ScriptedEvent {
            time: SimulationTime::from_nanoseconds(75),
            signals: vec![din.clone()],
        }],
        vec![din],
    );
    let mut adapter = IcarusVerilogAdapter::new(transport);

    let report = adapter.next_event_time().expect("transport must answer");
    assert_eq!(
        report,
        NextEventReport {
            predicted_time: SimulationTime::from_nanoseconds(75),
        },
        "adapter must forward the transport's prediction verbatim",
    );

    // And the transport recorded exactly one query.
    let log = adapter.transport().log();
    assert_eq!(
        log,
        &[VvpCall::QueryNextEvent(Some(
            SimulationTime::from_nanoseconds(75)
        ))],
        "transport log must contain one query, with the predicted time",
    );
}

// ---------------------------------------------------------------------------
// Witness 3 — Event delivery records VCD events with toggled signals.
// ---------------------------------------------------------------------------

/// `confirm_event(boundary)` on the correct-prediction path must:
///
/// 1. Return `DigitalStepReport::Confirmed { time }`.
/// 2. Cause the transport to log an `AdvanceAndReport(boundary)` call.
/// 3. Record the toggled signals in the adapter's accumulated trace,
///    visible after `take_trace`.
#[test]
fn item_47_confirm_event_delivers_signals_to_vcd_trace() {
    let din = SignalName::new("din");
    let dout = SignalName::new("dout");
    let transport = InMemoryVvp::new(
        [ScriptedEvent {
            time: SimulationTime::from_nanoseconds(40),
            signals: vec![din.clone(), dout.clone()],
        }],
        vec![din.clone(), dout.clone()],
    );
    let mut adapter = IcarusVerilogAdapter::new(transport);

    let _ = adapter.next_event_time().expect("query succeeds");
    let report = adapter
        .confirm_event(SimulationTime::from_nanoseconds(40))
        .expect("advance succeeds");
    assert_eq!(
        report,
        DigitalStepReport::Confirmed {
            time: SimulationTime::from_nanoseconds(40),
        },
    );

    // Verify the transport call log.
    let log: Vec<VvpCall> = adapter.transport().log().to_vec();
    assert!(
        log.contains(&VvpCall::AdvanceAndReport(
            SimulationTime::from_nanoseconds(40),
        )),
        "transport must have been asked to advance to 40 ns; log: {log:?}",
    );

    // Drain the trace and confirm both signals show an event at 40 ns.
    let trace = adapter.take_trace();
    for sig in [din, dout] {
        assert_eq!(
            trace.events_for(&sig),
            Some(&[SimulationTime::from_nanoseconds(40)][..]),
            "signal {sig} must have an event at 40 ns",
        );
    }
}

// ---------------------------------------------------------------------------
// Witness 4 — Rollback-to-checkpoint protocol relays through transport
// and prunes the VCD trace.
// ---------------------------------------------------------------------------

/// `rollback_to_checkpoint(target)` must:
///
/// 1. Route through the transport's `rollback_to(target)`.
/// 2. Drop any events strictly later than `target` from the adapter's
///    accumulated trace.
///
/// Not exercised on the correct-prediction Gherkin path, but item #47
/// explicitly names "rollback-to-checkpoint protocol via VVP runtime"
/// as part of its scope; sibling task #44 will compose against this
/// behaviour when it implements the scheduler-side rollback handler.
#[test]
fn item_47_rollback_to_checkpoint_relays_and_prunes() {
    let din = SignalName::new("din");
    let transport = InMemoryVvp::new(
        [
            ScriptedEvent {
                time: SimulationTime::from_nanoseconds(20),
                signals: vec![din.clone()],
            },
            ScriptedEvent {
                time: SimulationTime::from_nanoseconds(70),
                signals: vec![din.clone()],
            },
        ],
        vec![din.clone()],
    );
    let mut adapter = IcarusVerilogAdapter::new(transport);

    let _ = adapter.next_event_time().unwrap();
    let _ = adapter
        .confirm_event(SimulationTime::from_nanoseconds(20))
        .unwrap();
    let _ = adapter.next_event_time().unwrap();
    let _ = adapter
        .confirm_event(SimulationTime::from_nanoseconds(70))
        .unwrap();

    // Roll back past 70 ns but not past 20 ns.
    adapter
        .rollback_to_checkpoint(SimulationTime::from_nanoseconds(50))
        .expect("rollback must succeed");

    // Confirm the transport saw the rollback request.
    let log = adapter.transport().log();
    assert!(
        log.iter().any(
            |c| matches!(c, VvpCall::RollbackTo(t) if *t == SimulationTime::from_nanoseconds(50))
        ),
        "transport must have logged a RollbackTo(50 ns); log: {log:?}",
    );

    // And the surviving trace only records the 20 ns event.
    let trace = adapter.take_trace();
    assert_eq!(
        trace.events_for(&din),
        Some(&[SimulationTime::from_nanoseconds(20)][..]),
        "events past the rollback target must be evicted from the trace",
    );
}

// ---------------------------------------------------------------------------
// Witness 5 — Public API surface stability (ADR-0010, pinned by tests).
// ---------------------------------------------------------------------------

/// Item #47's public surface is the set of types re-exported from
/// `analysis_orchestration::mixed_signal::icarus`. Sibling tasks
/// (#44, #49, #50) will compose against this surface; they must not
/// silently drop a re-export. This test compiles iff the headline
/// names are present and visible from a downstream crate.
#[test]
fn item_47_public_api_surface_is_visible() {
    // The `VvpTransport` trait must be visible and usable as a bound
    // from downstream — we verify this by naming it on a local helper
    // function. (Hoisted to the top to satisfy
    // `clippy::items_after_statements`.)
    fn _trait_object_compiles<T: VvpTransport>(_t: T) {}

    // The mere act of naming these in `use ...` above and constructing
    // them here is the assertion. Pin a few construction shapes too:
    let _: InMemoryVvp = InMemoryVvp::new(
        [ScriptedEvent {
            time: SimulationTime::ZERO,
            signals: vec![],
        }],
        vec![],
    );
    let _: VvpAdvanceReport = VvpAdvanceReport::Confirmed {
        time: SimulationTime::ZERO,
        toggled: vec![],
    };
    let _: VvpAdvanceReport = VvpAdvanceReport::Postponed {
        new_prediction: SimulationTime::ZERO,
    };
    let _: VvpAdvanceReport = VvpAdvanceReport::Mispredicted {
        actual_time: SimulationTime::ZERO,
        toggled: vec![],
    };
    let _: VvpCall = VvpCall::QueryNextEvent(None);
    let _: VvpCall = VvpCall::AdvanceAndReport(SimulationTime::ZERO);
    let _: VvpCall = VvpCall::RollbackTo(SimulationTime::ZERO);

    // And the adapter is `DigitalSimulator`; we already exercise it in
    // witnesses 1–4. Pin the `adapter_kind` selector for completeness.
    let transport = InMemoryVvp::new(Vec::<ScriptedEvent>::new(), vec![]);
    let adapter = IcarusVerilogAdapter::new(transport);
    assert_eq!(adapter.adapter_kind(), DigitalAdapterKind::IcarusVerilog);
}
