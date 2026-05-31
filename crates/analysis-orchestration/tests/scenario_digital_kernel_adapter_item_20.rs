//! Integration witness for **tasks.md item #20** (Capability:
//! `mixed-signal-cosim`):
//!
//! > Mixed-Signal Scheduler: optimistic time advance + checkpoint/rollback
//! > across the analog/digital boundary. (depends on #17, #18)
//! > <!-- traces-spec: mixed-signal-cosim#digital-driven-analog-load-rollback -->
//! > <!-- traces-adr: ADR-0006 -->
//!
//! Item #20 extends the `MixedSignalScheduler` run loop so that it
//! saves a digital checkpoint at every predicted boundary and rolls
//! the digital kernel back alongside the analog solver on
//! misprediction. This file asserts the end-to-end behaviour using
//! the real [`DigitalKernel`] (ADR-0006: in-process `run-until`, no
//! IPC) wrapped in [`DigitalKernelAdapter`].
//!
//! # Scenarios
//!
//! 1. **Happy path** — digital-driven analog load: the digital kernel
//!    has events at 50 ns and 100 ns; the scheduler advances the
//!    analog solver to each boundary, exchanges signals, and produces
//!    correct committed-through traces. No rollbacks occur.
//!
//! 2. **Checkpoint round-trip** — the adapter's `save_checkpoint` /
//!    `rollback_to` cycle correctly snapshots and restores the
//!    kernel's internal event queue and net state.
//!
//! 3. **Rollback in scheduler run loop** — when the digital adapter
//!    reports a misprediction, the scheduler rolls back *both* the
//!    analog solver and the digital kernel. This uses a test double
//!    that wraps a real `DigitalKernel` but overrides
//!    `confirm_event` to inject a misprediction at a specific step.
//!
//! ADR refs: ADR-0004 (optimistic advance + rollback), ADR-0006
//! (native DEVS kernel), ADR-0010 (unstable v1 API surface).

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalKernelAdapter,
    DigitalSimulator, DigitalStepReport, MixedSignalScheduler, NextEventReport,
    SchedulerError, SparseCheckpoint,
};
use circuit_solver_types::{
    AnalogTrace, DigitalEventTrace, NodeId, RollbackEvent, SignalName, SimulationTime, Waveform,
};
use digital_kernel::{DigitalEvent, DigitalKernel, LogicValue, NetId};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn t_ns(ns: i64) -> SimulationTime {
    SimulationTime::from_nanoseconds(ns)
}

// ---------------------------------------------------------------------------
// Analog solver double (reused from item_44 pattern)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum AnalogCall {
    RunUntil(SimulationTime),
    RollbackTo(SimulationTime),
}

struct AnalogDouble {
    observed: NodeId,
    calls: Rc<RefCell<Vec<AnalogCall>>>,
    samples: Vec<(SimulationTime, f64)>,
}

impl AnalogDouble {
    fn new(observed: NodeId, calls: Rc<RefCell<Vec<AnalogCall>>>) -> Self {
        Self {
            observed,
            calls,
            samples: vec![(SimulationTime::ZERO, voltage_at(SimulationTime::ZERO))],
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn voltage_at(t: SimulationTime) -> f64 {
    let ns = t.as_nanoseconds() as f64;
    (ns / 100.0).clamp(0.0, 1.0)
}

impl AnalogSolver for AnalogDouble {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        self.calls.borrow_mut().push(AnalogCall::RunUntil(target));
        let v = voltage_at(target);
        self.samples.push((target, v));
        let checkpoint =
            SparseCheckpoint::empty(target).with_node_voltages(vec![(self.observed, v)]);
        Ok(AnalogStepReport::with_checkpoint(target, checkpoint))
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.calls.borrow_mut().push(AnalogCall::RollbackTo(target));
        self.samples.retain(|(t, _)| *t <= target);
        Ok(())
    }

    fn take_trace(&mut self) -> AnalogTrace {
        let (times, values): (Vec<_>, Vec<_>) = self.samples.iter().copied().unzip();
        let committed_through = times.last().copied().unwrap_or(SimulationTime::ZERO);
        let waveform = Waveform::new(self.observed, times, values);
        AnalogTrace {
            waveforms: vec![waveform],
            committed_through,
        }
    }
}

// ---------------------------------------------------------------------------
// Scenario 1 — Happy path: digital-driven analog load with real
// DigitalKernel
// ---------------------------------------------------------------------------

/// The digital kernel has scheduled events at 50 ns (net 0 → One)
/// and 100 ns (net 0 → Zero). The scheduler should:
///
/// 1. Query `next_event_time` → 50 ns.
/// 2. Save a digital checkpoint (at time 0).
/// 3. Advance the analog solver to 50 ns.
/// 4. Confirm the event → Confirmed.
/// 5. Query `next_event_time` → 100 ns.
/// 6. Save a digital checkpoint (at time 50 ns).
/// 7. Advance the analog solver to 100 ns.
/// 8. Confirm the event → Confirmed.
/// 9. Query `next_event_time` → no more events.
/// 10. Return a result with two commits [50 ns, 100 ns], zero rollbacks.
#[test]
fn digital_driven_analog_load_happy_path_with_native_kernel() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));

    // Build a real digital kernel with two events.
    let mut kernel = DigitalKernel::new();
    let net0 = NetId::new(0);
    kernel
        .schedule(DigitalEvent::new(t_ns(50), net0, LogicValue::One))
        .expect("schedule event at 50 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(100), net0, LogicValue::Zero))
        .expect("schedule event at 100 ns");

    let signals = vec![SignalName::new("din")];
    let adapter = DigitalKernelAdapter::new(kernel, signals);

    let analog = AnalogDouble::new(vout, Rc::clone(&analog_calls));
    let scheduler = MixedSignalScheduler::new(
        analog,
        adapter,
        BoundarySignals::default(),
        t_ns(200),
    );
    let result = scheduler
        .run()
        .expect("scheduler.run must succeed on happy path");

    // Two commits at the digital event boundaries.
    assert_eq!(
        result.scheduler.commits,
        vec![t_ns(50), t_ns(100)],
        "scheduler must commit at each digital event boundary"
    );

    // No rollbacks on the correct-prediction path.
    assert!(
        result.scheduler.rollbacks.is_empty(),
        "happy path must have zero rollbacks"
    );
    assert!(
        result.rollback_free(),
        "rollback_free must be true when no rollbacks occurred"
    );

    // The analog solver was advanced to 50 ns and 100 ns.
    let run_untils: Vec<SimulationTime> = analog_calls
        .borrow()
        .iter()
        .filter_map(|c| match c {
            AnalogCall::RunUntil(t) => Some(*t),
            _ => None,
        })
        .collect();
    assert_eq!(
        run_untils,
        vec![t_ns(50), t_ns(100)],
        "analog solver must receive run-until at each boundary"
    );

    // No analog rollbacks on the happy path.
    assert!(
        !analog_calls
            .borrow()
            .iter()
            .any(|c| matches!(c, AnalogCall::RollbackTo(_))),
        "happy path must not rollback the analog solver"
    );

    // The digital trace is non-empty.
    assert!(
        !result.digital.vcd.is_empty(),
        "digital VCD trace must be populated"
    );

    // The analog trace includes samples at 50 ns and 100 ns.
    let wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain vout waveform");
    assert!(
        wf.times.contains(&t_ns(50)),
        "analog waveform must include the 50 ns sample"
    );
    assert!(
        wf.times.contains(&t_ns(100)),
        "analog waveform must include the 100 ns sample"
    );
}

// ---------------------------------------------------------------------------
// Scenario 2 — Checkpoint round-trip with real DigitalKernel
// ---------------------------------------------------------------------------

/// The adapter's `save_checkpoint` snapshots the kernel state, and
/// `rollback_to` restores it. After restoring, the kernel should
/// resume from the checkpoint time and re-process events that were
/// originally after the checkpoint.
#[test]
fn native_kernel_adapter_checkpoint_round_trip() {
    let mut kernel = DigitalKernel::new();
    let net0 = NetId::new(0);
    kernel
        .schedule(DigitalEvent::new(t_ns(50), net0, LogicValue::One))
        .expect("schedule at 50 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(80), net0, LogicValue::Zero))
        .expect("schedule at 80 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(120), net0, LogicValue::One))
        .expect("schedule at 120 ns");

    let signals = vec![SignalName::new("din")];
    let mut adapter = DigitalKernelAdapter::new(kernel, signals);

    // Advance the kernel to 50 ns via the trait method.
    let next = adapter.next_event_time().expect("must have event at 50 ns");
    assert_eq!(next.predicted_time, t_ns(50));

    // Save a checkpoint at time 0 (before any advance).
    let cp_time = adapter
        .save_checkpoint()
        .expect("save_checkpoint must return Some at time 0");
    assert_eq!(cp_time, SimulationTime::ZERO, "checkpoint time must be 0");

    // Confirm event at 50 ns → runs the kernel to 50 ns.
    let step = adapter.confirm_event(t_ns(50)).expect("confirm at 50 ns");
    assert!(
        matches!(step, DigitalStepReport::Confirmed { time } if time == t_ns(50)),
        "event at 50 ns must be confirmed"
    );

    // Save another checkpoint at 50 ns.
    let cp_time_50 = adapter
        .save_checkpoint()
        .expect("save_checkpoint must return Some at time 50 ns");
    assert_eq!(cp_time_50, t_ns(50));

    // Advance to 80 ns.
    let next2 = adapter.next_event_time().expect("must have event at 80 ns");
    assert_eq!(next2.predicted_time, t_ns(80));
    let step2 = adapter.confirm_event(t_ns(80)).expect("confirm at 80 ns");
    assert!(
        matches!(step2, DigitalStepReport::Confirmed { time } if time == t_ns(80)),
        "event at 80 ns must be confirmed"
    );

    // Verify the kernel's current time is 80 ns.
    assert_eq!(adapter.kernel().current_time(), t_ns(80));

    // Now roll back to 50 ns.
    adapter
        .rollback_to(t_ns(50))
        .expect("rollback to 50 ns must succeed");

    // After rollback, the kernel should be at 50 ns.
    assert_eq!(
        adapter.kernel().current_time(),
        t_ns(50),
        "kernel must be at 50 ns after rollback"
    );

    // The next event should still be the one at 80 ns (it was
    // re-scheduled when the kernel's event queue was restored).
    let next3 = adapter.next_event_time().expect("must have event at 80 ns after rollback");
    assert_eq!(
        next3.predicted_time,
        t_ns(80),
        "next event after rollback must be 80 ns"
    );

    // Confirm at 80 ns again.
    let step3 = adapter.confirm_event(t_ns(80)).expect("confirm at 80 ns after rollback");
    assert!(
        matches!(step3, DigitalStepReport::Confirmed { time } if time == t_ns(80)),
        "event at 80 ns must be confirmed after rollback"
    );

    // Advance to 120 ns.
    let next4 = adapter.next_event_time().expect("must have event at 120 ns");
    assert_eq!(next4.predicted_time, t_ns(120));
    let step4 = adapter
        .confirm_event(t_ns(120))
        .expect("confirm at 120 ns");
    assert!(
        matches!(step4, DigitalStepReport::Confirmed { time } if time == t_ns(120)),
        "event at 120 ns must be confirmed"
    );
}

// ---------------------------------------------------------------------------
// Scenario 3 — Rollback in the scheduler run loop
// ---------------------------------------------------------------------------

/// A `MispredictingKernelAdapter` wraps a real `DigitalKernel` but
/// overrides `confirm_event` to inject a misprediction at a chosen
/// step. This allows us to exercise the scheduler's rollback path
/// with a real kernel underneath.
struct MispredictingKernelAdapter {
    inner: DigitalKernelAdapter,
    confirm_script: VecDeque<DigitalStepReport>,
}

impl MispredictingKernelAdapter {
    fn new(kernel: DigitalKernel, signal_names: Vec<SignalName>) -> Self {
        Self {
            inner: DigitalKernelAdapter::new(kernel, signal_names),
            confirm_script: VecDeque::new(),
        }
    }

    fn with_confirm_script(mut self, script: Vec<DigitalStepReport>) -> Self {
        self.confirm_script = script.into_iter().collect();
        self
    }
}

impl DigitalSimulator for MispredictingKernelAdapter {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        self.inner.adapter_kind()
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        self.inner.next_event_time()
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        // If the script has an override, use it; otherwise delegate.
        match self.confirm_script.pop_front() {
            Some(report) => Ok(report),
            None => self.inner.confirm_event(boundary),
        }
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        self.inner.take_trace()
    }

    fn save_checkpoint(&mut self) -> Option<SimulationTime> {
        self.inner.save_checkpoint()
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.inner.rollback_to(target)
    }
}

/// The scheduler predicts 100 ns (from `next_event_time`), advances
/// the analog solver to 100 ns, but the digital adapter reports
/// `Mispredicted { actual_time: 80 ns }`. The scheduler must:
///
/// 1. Roll back the analog solver to the nearest checkpoint before 80 ns.
/// 2. Roll back the digital kernel to the nearest checkpoint before 80 ns.
/// 3. Re-advance to 80 ns.
/// 4. Record the rollback in the result metadata.
#[test]
fn digital_driven_analog_load_rollback_with_native_kernel() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));

    // Build a real kernel with events at 50 ns and 80 ns.
    let mut kernel = DigitalKernel::new();
    let net0 = NetId::new(0);
    kernel
        .schedule(DigitalEvent::new(t_ns(50), net0, LogicValue::One))
        .expect("schedule at 50 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(80), net0, LogicValue::Zero))
        .expect("schedule at 80 ns");

    let signals = vec![SignalName::new("din")];

    // The adapter wraps the real kernel, but we override confirm_event
    // to inject a misprediction at the 100 ns boundary (the scheduler
    // will predict 100 ns because we override next_event_time on the
    // second call). The first confirm at 50 ns is genuine.
    //
    // Actually, the kernel's real next_event_time returns 50 ns then
    // 80 ns. To trigger a rollback, we need the scheduler to predict
    // a time that differs from the actual event. We can do this by:
    // - Having the kernel predict 50 ns (real), confirm → Confirmed.
    // - Having the kernel predict 80 ns (real), but we override
    //   confirm_event to say Mispredicted(actual=70ns).
    //   However, that requires an event at 70ns that doesn't exist.
    //
    // A cleaner approach: use a scripted next_event_time that
    // predicts 100 ns, then mispredicts with actual 80 ns.
    // But we also want the real kernel to have events.
    //
    // Simplest correct approach: schedule events at 50 ns and 80 ns,
    // let the first event (50 ns) go through correctly (seed a
    // checkpoint), then on the second event, the adapter overrides
    // next_event_time to predict 100 ns (instead of the real 80 ns),
    // and on confirm_event returns Mispredicted { actual_time: 80 ns }.
    // This exercises the full rollback path.

    // We need a more sophisticated adapter for this. Let's build one
    // that scripts both next_event_time and confirm_event.
    let adapter = ScriptedRollbackAdapter::new(
        kernel,
        signals,
        // next_event_time predictions: 50 ns (real), 100 ns (wrong),
        // 80 ns (corrected after rollback), then exhausted.
        vec![t_ns(50), t_ns(100), t_ns(80)],
        // confirm_event responses: Confirmed(50 ns),
        // Mispredicted(actual=80 ns), Confirmed(80 ns).
        vec![
            DigitalStepReport::Confirmed { time: t_ns(50) },
            DigitalStepReport::Mispredicted {
                actual_time: t_ns(80),
            },
            DigitalStepReport::Confirmed { time: t_ns(80) },
        ],
    );

    let analog = AnalogDouble::new(vout, Rc::clone(&analog_calls));
    let scheduler = MixedSignalScheduler::new(
        analog,
        adapter,
        BoundarySignals::default(),
        t_ns(200),
    );
    let result = scheduler
        .run()
        .expect("scheduler.run must succeed with rollback");

    // Exactly one rollback event.
    assert_eq!(
        result.scheduler.rollbacks.len(),
        1,
        "must have exactly one rollback"
    );
    let rb = &result.scheduler.rollbacks[0];
    assert_eq!(rb.mispredicted_at, t_ns(100));
    assert_eq!(rb.corrected_to, t_ns(80));
    assert_eq!(rb.checkpoint_at, t_ns(50), "nearest checkpoint before 80 ns is 50 ns");

    // Two commits: 50 ns (confirmed) and 80 ns (post-rollback).
    assert_eq!(result.scheduler.commits, vec![t_ns(50), t_ns(80)]);

    // The analog solver was rolled back.
    assert!(
        analog_calls
            .borrow()
            .iter()
            .any(|c| matches!(c, AnalogCall::RollbackTo(_))),
        "analog solver must have been rolled back"
    );

    // The analog trace does not contain the rolled-back 100 ns sample.
    let wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain vout waveform");
    assert!(
        wf.times.contains(&t_ns(80)),
        "analog waveform must include the 80 ns sample (post-rollback)"
    );
    assert!(
        !wf.times.contains(&t_ns(100)),
        "analog waveform must NOT contain the rolled-back 100 ns sample"
    );

    // Not rollback-free.
    assert!(
        !result.rollback_free(),
        "rollback_free must be false when a rollback occurred"
    );
}

// ---------------------------------------------------------------------------
// Scripted rollback adapter — wraps a real DigitalKernel but
// overrides both next_event_time and confirm_event with scripted
// values to inject a misprediction while still using the kernel's
// real checkpoint/rollback.
// ---------------------------------------------------------------------------

struct ScriptedRollbackAdapter {
    inner: DigitalKernelAdapter,
    next_predictions: VecDeque<SimulationTime>,
    confirm_responses: VecDeque<DigitalStepReport>,
}

impl ScriptedRollbackAdapter {
    fn new(
        kernel: DigitalKernel,
        signal_names: Vec<SignalName>,
        next_predictions: Vec<SimulationTime>,
        confirm_responses: Vec<DigitalStepReport>,
    ) -> Self {
        Self {
            inner: DigitalKernelAdapter::new(kernel, signal_names),
            next_predictions: next_predictions.into_iter().collect(),
            confirm_responses: confirm_responses.into_iter().collect(),
        }
    }
}

impl DigitalSimulator for ScriptedRollbackAdapter {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        self.inner.adapter_kind()
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        match self.next_predictions.pop_front() {
            Some(t) => Ok(NextEventReport {
                predicted_time: t,
            }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "scripted adapter: predictions exhausted".into(),
            )),
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        match self.confirm_responses.pop_front() {
            Some(report) => {
                // If the report is Confirmed, also advance the real
                // kernel so its internal state stays consistent for
                // checkpoints.
                if let DigitalStepReport::Confirmed { time } = report {
                    let _ = self.inner.kernel_mut().run_until(time);
                    let _ = self.inner.kernel_mut().take_processed_events();
                }
                Ok(report)
            }
            None => Err(SchedulerError::DigitalAdapterFailed(
                "scripted adapter: confirm responses exhausted".into(),
            )),
        }
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        self.inner.take_trace()
    }

    fn save_checkpoint(&mut self) -> Option<SimulationTime> {
        self.inner.save_checkpoint()
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.inner.rollback_to(target)
    }
}

// ---------------------------------------------------------------------------
// Scenario 4 — Public API surface witness (ADR-0010)
// ---------------------------------------------------------------------------

/// The `DigitalKernelAdapter` and `NativeKernel` variant must be
/// visible from the crate root. This test compiles iff the re-exports
/// are present.
#[test]
fn item_20_public_api_surface_is_visible() {
    let _: DigitalAdapterKind = DigitalAdapterKind::NativeKernel;
    let _: DigitalAdapterKind = DigitalAdapterKind::IcarusVerilog;
    let _: DigitalAdapterKind = DigitalAdapterKind::Verilator;
    let _: DigitalAdapterKind = DigitalAdapterKind::TestDouble;

    // Construct a DigitalKernelAdapter (requires a kernel).
    let kernel = DigitalKernel::new();
    let _adapter = DigitalKernelAdapter::new(kernel, vec![SignalName::new("test")]);
}
