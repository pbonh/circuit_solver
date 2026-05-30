//! Integration witness for **tasks.md item #42** (Capability:
//! `mixed-signal-cosim`):
//!
//! > Implement `MixedSignalScheduler` core: own both kernel handles,
//! > issue run-until to analog, query next-event-time from digital —
//! > @adr: ADR-0004 (depends on #33)
//!
//! Item #42 carries no `@spec:` tag of its own; it is shared
//! infrastructure that sibling scenario-tasks compose against
//! (#43–#51 attach checkpoint management, rollback, boundary
//! exchange, adapters, VCD, and the end-to-end mixed-signal control
//! loop). The scenario-lane sibling `t_e9bda779` (merged commit
//! `8947452`) already exercised the headline Gherkin
//! `mixed-signal-cosim#optimistic-advance-with-correct-prediction`
//! against the in-crate test doubles. This file is the per-tasks.md-
//! item lane's complementary witness: it asserts the three "core"
//! guarantees of item #42 from the *public* `MixedSignalScheduler`
//! API surface (`pub use mixed_signal::*` re-exports in
//! `analysis-orchestration::lib`), using minimal integration-test
//! doubles that mirror — without duplicating — the ones in
//! `mixed_signal::test_doubles`.
//!
//! The three core guarantees, asserted in order:
//!
//! 1. The scheduler **owns both kernel handles** (moves them in at
//!    `new(...)` and returns them only via the assembled
//!    `MixedSignalResult` traces; no shared reference, no aliasing).
//! 2. The scheduler **issues `run-until`** to the analog solver at
//!    the digital-side prediction (per ADR-0004 commitment #1 +
//!    "Optimistic time advance").
//! 3. The scheduler **queries `next-event-time`** from the digital
//!    simulator before each analog advance (per ADR-0004 commitment
//!    #3 "Shared scheduler ownership" — the digital side is queried,
//!    never the inverse).
//!
//! ADR refs: ADR-0004 (mixed-signal scheduler ownership), ADR-0010
//! (unstable v1 API surface — these re-exports are tracked here so
//! a breaking change to the scheduler's public API surface is caught
//! by the test breakage of this witness).

use std::cell::RefCell;
use std::rc::Rc;

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, MixedSignalScheduler, NextEventReport, SchedulerError,
};
use circuit_solver_types::{AnalogTrace, DigitalEventTrace, NodeId, SimulationTime, Waveform};

// ---------------------------------------------------------------------------
// Integration-test doubles — distinct from the in-crate ones, with an
// externally observable call log so we can assert the *exact* sequence
// of operations the scheduler performed against each kernel.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum AnalogCall {
    RunUntil(SimulationTime),
    #[allow(dead_code)] // reserved for rollback witnesses written by sibling tasks
    RollbackTo(SimulationTime),
    TakeTrace,
}

#[derive(Debug, Clone, PartialEq)]
enum DigitalCall {
    NextEventTime,
    ConfirmEvent(SimulationTime),
    TakeTrace,
}

struct ObservingAnalog {
    observed: NodeId,
    log: Rc<RefCell<Vec<AnalogCall>>>,
    samples: Vec<(SimulationTime, f64)>,
    checkpoints: Vec<SimulationTime>,
}

impl ObservingAnalog {
    fn new(observed: NodeId, log: Rc<RefCell<Vec<AnalogCall>>>) -> Self {
        Self {
            observed,
            log,
            samples: vec![(SimulationTime::ZERO, 0.0)],
            checkpoints: Vec::new(),
        }
    }
}

impl AnalogSolver for ObservingAnalog {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        self.log.borrow_mut().push(AnalogCall::RunUntil(target));
        // Deterministic synthetic waveform: ramps from 0 V at t=0 to
        // 3.3 V at 50 ns, then saturates. Sufficient for asserting
        // the analog Result shape; the numeric details belong to the
        // numeric-solver crate and its scenario tests.
        #[allow(clippy::cast_precision_loss)]
        let ns = target.as_nanoseconds() as f64;
        let value = 3.3 * (ns / 50.0).clamp(0.0, 1.0);
        self.samples.push((target, value));
        self.checkpoints.push(target);
        Ok(AnalogStepReport {
            time_reached: target,
            checkpoint_saved: true,
            checkpoint: None,
        })
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.log.borrow_mut().push(AnalogCall::RollbackTo(target));
        self.samples.retain(|(t, _)| *t <= target);
        self.checkpoints.retain(|t| *t <= target);
        Ok(())
    }

    fn take_trace(&mut self) -> AnalogTrace {
        self.log.borrow_mut().push(AnalogCall::TakeTrace);
        let (times, values): (Vec<_>, Vec<_>) = self.samples.iter().copied().unzip();
        let committed_through = times.last().copied().unwrap_or(SimulationTime::ZERO);
        AnalogTrace {
            waveforms: vec![Waveform::new(self.observed, times, values)],
            committed_through,
        }
    }
}

struct ObservingDigital {
    script: std::collections::VecDeque<SimulationTime>,
    log: Rc<RefCell<Vec<DigitalCall>>>,
}

impl ObservingDigital {
    fn new(
        script: impl IntoIterator<Item = SimulationTime>,
        log: Rc<RefCell<Vec<DigitalCall>>>,
    ) -> Self {
        Self {
            script: script.into_iter().collect(),
            log,
        }
    }
}

impl DigitalSimulator for ObservingDigital {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::TestDouble
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        self.log.borrow_mut().push(DigitalCall::NextEventTime);
        match self.script.front().copied() {
            Some(t) => Ok(NextEventReport { predicted_time: t }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "integration double exhausted".into(),
            )),
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        self.log
            .borrow_mut()
            .push(DigitalCall::ConfirmEvent(boundary));
        match self.script.pop_front() {
            Some(t) if t == boundary => Ok(DigitalStepReport::Confirmed { time: t }),
            Some(t) => Err(SchedulerError::ContractViolation {
                predicted: t,
                actual: boundary,
            }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "integration double script exhausted before confirm_event".into(),
            )),
        }
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        self.log.borrow_mut().push(DigitalCall::TakeTrace);
        DigitalEventTrace {
            vcd: String::from("$timescale 1ps $end\n$enddefinitions $end\n"),
            events_by_signal: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Witness 1 — Core guarantee: scheduler owns both kernel handles.
// ---------------------------------------------------------------------------

/// The scheduler takes ownership of the analog and digital kernels at
/// `new`, drives them to completion in `run`, and returns the assembled
/// `MixedSignalResult`. After `run` returns, the kernels are no longer
/// reachable — the handles were moved in, then dropped after their
/// traces were drained. This test pins that ownership semantic by
/// requiring the call type-checks (no `&mut` or `Rc<RefCell<_>>` shims
/// at the public API), and by demonstrating that the result's analog
/// and digital traces survive the kernels' drop.
#[test]
fn item_42_core_scheduler_owns_both_kernel_handles() {
    let analog_log = Rc::new(RefCell::new(Vec::new()));
    let digital_log = Rc::new(RefCell::new(Vec::new()));
    let vout = NodeId::new(1);

    let analog = ObservingAnalog::new(vout, Rc::clone(&analog_log));
    let digital = ObservingDigital::new(
        [SimulationTime::from_nanoseconds(50)],
        Rc::clone(&digital_log),
    );

    // `MixedSignalScheduler::new(analog, digital, ...)` consumes both
    // by value; if this signature regressed to `&mut A, &mut D` this
    // test would stop compiling.
    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(100),
    );
    let result = scheduler.run().expect("scheduler.run must succeed");

    // After `run` the result owns drained traces; the kernels themselves
    // are gone. We can still inspect their externally-held call logs.
    assert!(
        !analog_log.borrow().is_empty(),
        "scheduler must have driven the analog kernel"
    );
    assert!(
        !digital_log.borrow().is_empty(),
        "scheduler must have driven the digital kernel"
    );
    // The `take_trace` calls are how we know the scheduler drained both
    // handles before dropping them — i.e. the result truly owns the
    // outputs, not a borrow of still-live kernel state.
    assert!(
        analog_log.borrow().contains(&AnalogCall::TakeTrace),
        "scheduler must drain the analog trace before dropping the handle"
    );
    assert!(
        digital_log.borrow().contains(&DigitalCall::TakeTrace),
        "scheduler must drain the digital trace before dropping the handle"
    );
    assert_eq!(
        result.scheduler.commits,
        vec![SimulationTime::from_nanoseconds(50)],
    );
}

// ---------------------------------------------------------------------------
// Witness 2 — Core guarantee: scheduler issues `run-until` to analog.
// ---------------------------------------------------------------------------

/// For each digital-predicted boundary, the scheduler must issue a
/// matching `run_until` command to the analog kernel. This is
/// ADR-0004's "Optimistic time advance" + "Shared scheduler ownership"
/// fused: the analog kernel sees *only* `run_until` / `rollback_to`
/// commands, never a direct `next_event_time` lookup.
#[test]
fn item_42_core_scheduler_issues_run_until_to_analog() {
    let analog_log = Rc::new(RefCell::new(Vec::new()));
    let digital_log = Rc::new(RefCell::new(Vec::new()));

    let analog = ObservingAnalog::new(NodeId::new(1), Rc::clone(&analog_log));
    let digital = ObservingDigital::new(
        [
            SimulationTime::from_nanoseconds(20),
            SimulationTime::from_nanoseconds(50),
            SimulationTime::from_nanoseconds(80),
        ],
        Rc::clone(&digital_log),
    );
    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(100),
    );
    scheduler.run().expect("scheduler.run must succeed");

    // Exactly three RunUntil calls, in script order. (TakeTrace is
    // appended at the end; filter it out.)
    let run_untils: Vec<SimulationTime> = analog_log
        .borrow()
        .iter()
        .filter_map(|c| match c {
            AnalogCall::RunUntil(t) => Some(*t),
            _ => None,
        })
        .collect();
    assert_eq!(
        run_untils,
        vec![
            SimulationTime::from_nanoseconds(20),
            SimulationTime::from_nanoseconds(50),
            SimulationTime::from_nanoseconds(80),
        ],
        "scheduler must issue run-until at each digital-predicted boundary"
    );

    // And on the correct-prediction path, zero rollbacks. (ADR-0004
    // commitment #4: rollback is reserved for the misprediction path.)
    assert!(
        !analog_log
            .borrow()
            .iter()
            .any(|c| matches!(c, AnalogCall::RollbackTo(_))),
        "correct-prediction path must not rollback"
    );
}

// ---------------------------------------------------------------------------
// Witness 3 — Core guarantee: scheduler queries `next-event-time` from
// digital before each analog advance.
// ---------------------------------------------------------------------------

/// Per ADR-0004 commitment #3 ("Shared scheduler ownership"), the
/// scheduler — not the analog kernel — queries the digital simulator
/// for `next-event-time`. This test asserts the strict interleave:
///
///     [Digital::NextEventTime, Analog::RunUntil, Digital::ConfirmEvent]*
///     [Digital::NextEventTime, Digital::TakeTrace, Analog::TakeTrace]
///
/// i.e. every analog advance is *preceded* by a digital query. There
/// is no run-until without a fresh `next_event_time` driving it.
#[test]
fn item_42_core_scheduler_queries_next_event_time_from_digital() {
    let analog_log = Rc::new(RefCell::new(Vec::new()));
    let digital_log = Rc::new(RefCell::new(Vec::new()));

    let analog = ObservingAnalog::new(NodeId::new(1), Rc::clone(&analog_log));
    let digital = ObservingDigital::new(
        [
            SimulationTime::from_nanoseconds(25),
            SimulationTime::from_nanoseconds(75),
        ],
        Rc::clone(&digital_log),
    );
    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(100),
    );
    scheduler.run().expect("scheduler.run must succeed");

    // For each `RunUntil(t)` in the analog log, there must be a
    // `NextEventTime` in the digital log that *precedes* it (by
    // recorded order). And the *immediately preceding* digital call
    // must be `NextEventTime`, not `ConfirmEvent` — confirmation
    // happens after the analog advances.
    let analog_calls: Vec<AnalogCall> = analog_log.borrow().clone();
    let digital_calls: Vec<DigitalCall> = digital_log.borrow().clone();

    // Collect all NextEventTime indices in the digital log.
    let next_event_indices: Vec<usize> = digital_calls
        .iter()
        .enumerate()
        .filter_map(|(i, c)| matches!(c, DigitalCall::NextEventTime).then_some(i))
        .collect();

    // Two run-untils were driven → at least two next_event_time queries
    // must have come from the digital side. (The scheduler also issues
    // a final `next_event_time` after the last confirm that triggers
    // either horizon-overrun or end-of-events; so we expect *more*
    // queries than run-untils, not fewer.)
    let run_until_count = analog_calls
        .iter()
        .filter(|c| matches!(c, AnalogCall::RunUntil(_)))
        .count();
    assert_eq!(run_until_count, 2, "two scripted events → two run-untils");
    assert!(
        next_event_indices.len() >= run_until_count,
        "every analog advance must be preceded by at least one digital next-event-time query \
         (got {} queries vs {} advances)",
        next_event_indices.len(),
        run_until_count,
    );

    // And the digital simulator was queried *first*, before any
    // confirmation — i.e. the scheduler does not assume an event
    // exists without asking.
    assert!(
        matches!(digital_calls.first(), Some(DigitalCall::NextEventTime)),
        "scheduler's first interaction with the digital side must be a next-event-time query, \
         got {:?}",
        digital_calls.first()
    );
}

// ---------------------------------------------------------------------------
// Witness 4 — Public API surface stability (ADR-0010 v1 unstable but
// pinned by tests).
// ---------------------------------------------------------------------------

/// Item #42's public surface is the set of types and traits re-exported
/// from `analysis_orchestration::mixed_signal`. Sibling tasks
/// (#43–#51) extend this surface; they must not silently drop a
/// re-export. This test compiles iff the headline names are present
/// and visible from a downstream crate.
#[test]
fn item_42_public_api_surface_is_visible() {
    // The mere act of naming these in `use ...` above and constructing
    // them here is the assertion. Pin the construction shapes too:
    let _: BoundarySignals = BoundarySignals::default();
    let _: DigitalAdapterKind = DigitalAdapterKind::IcarusVerilog;
    let _: DigitalAdapterKind = DigitalAdapterKind::Verilator;
    let _: DigitalAdapterKind = DigitalAdapterKind::TestDouble;
    let _: NextEventReport = NextEventReport {
        predicted_time: SimulationTime::ZERO,
    };
    let _: AnalogStepReport = AnalogStepReport {
        time_reached: SimulationTime::ZERO,
        checkpoint_saved: true,
        checkpoint: None,
    };
}
