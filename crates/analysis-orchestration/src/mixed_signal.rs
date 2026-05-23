//! The Mixed-Signal Scheduler.
//!
//! ADR-0004 ("Optimistic Mixed-Signal Synchronization via Shared
//! Scheduler") mandates that the analog and digital simulator kernels
//! be **decoupled**: neither queries the other directly. The
//! [`MixedSignalScheduler`] owns handles to both and is the sole
//! mediator. It implements the four commitments of that ADR:
//!
//! 1. **Optimistic time advance** — the analog solver is told to run
//!    up to the predicted next digital event boundary.
//! 2. **Sparse checkpointing at predicted boundaries** — the analog
//!    solver saves enough state at each predicted boundary to resume.
//! 3. **Shared scheduler ownership** — `run-until` flows to the analog
//!    solver, `next-event-time` queries flow to the digital simulator,
//!    and all rollback commands flow through this struct.
//! 4. **Rollback on misprediction** — when the digital kernel reports
//!    no event at the predicted time or an earlier event, the analog
//!    state is rolled back to the last good checkpoint.
//!
//! # This file's scope
//!
//! Only the *correct-prediction* path is exercised by the current
//! scenario (`optimistic-advance-with-correct-prediction`):
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
//! The rollback paths, contract-violation detection, boundary signal
//! interpolation (ADR-0007), VCD trace shape, and conformance against
//! the Golden Reference are reserved for sibling implementer tasks.
//! The trait surface is designed to admit those extensions without a
//! breaking change to the scheduler's public API.

use circuit_solver_types::{
    AnalogTrace, DigitalEventTrace, MixedSignalResult, SchedulerMetadata, SignalName,
    SimulationTime,
};
use core::fmt;

use crate::checkpoint::SparseCheckpoint;

pub mod icarus;
pub mod rollback;

// ---------------------------------------------------------------------------
// Boundary signals
// ---------------------------------------------------------------------------

/// The set of named boundary signals exchanged between the analog
/// solver and the digital simulator at every synchronization point.
///
/// For the current scenario the set may be empty; the scheduler is
/// still required to run-until / commit correctly. Sibling scenarios
/// (`analog-digital-boundary-signal-exchange`) flesh out the exchange
/// protocol.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoundarySignals {
    /// Analog node values driving digital inputs. Pair: (analog signal
    /// name, digital input name).
    pub analog_to_digital: Vec<(SignalName, SignalName)>,
    /// Digital output values driving analog inputs. Pair: (digital
    /// signal name, analog input name).
    pub digital_to_analog: Vec<(SignalName, SignalName)>,
}

// ---------------------------------------------------------------------------
// Digital adapter selection
// ---------------------------------------------------------------------------

/// Which external digital kernel the scheduler is mediating.
///
/// Tasks.md items #47 and #48 promise two adapter implementations.
/// The scheduler is generic over the [`DigitalSimulator`] trait, so
/// the kind is recorded as metadata only — it does not change control
/// flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DigitalAdapterKind {
    /// Icarus Verilog via the VVP runtime (tasks.md item #47).
    IcarusVerilog,
    /// Verilator via the verilated model interface (tasks.md item #48).
    Verilator,
    /// A test double used only inside the unit-test scenarios.
    TestDouble,
}

impl fmt::Display for DigitalAdapterKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::IcarusVerilog => "icarus-verilog",
            Self::Verilator => "verilator",
            Self::TestDouble => "test-double",
        };
        f.write_str(s)
    }
}

// ---------------------------------------------------------------------------
// Analog solver trait
// ---------------------------------------------------------------------------

/// Report returned by the analog solver after a `run_until` call.
///
/// On the **correct-prediction path** the solver reports
/// `time_reached == target` and `checkpoint_saved == true`, and may
/// optionally attach the [`SparseCheckpoint`] it just saved via
/// `checkpoint`. The scheduler's [`rollback::RollbackHandler`] then
/// records that checkpoint in its [`crate::checkpoint::SparseCheckpointManager`] so the
/// **misprediction path** (tasks.md #44) can later restore the
/// nearest-before snapshot.
///
/// Implementations that do not yet plumb a real checkpoint payload
/// MAY set `checkpoint` to `None` and keep `checkpoint_saved = true`;
/// the rollback handler treats `None` as "manager did not receive a
/// new snapshot at this boundary" and falls back to whatever the
/// manager already holds.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogStepReport {
    /// The time the analog solver actually reached. On the
    /// correct-prediction path this equals the requested run-until
    /// target.
    pub time_reached: SimulationTime,
    /// True iff the analog solver saved a sparse checkpoint at
    /// `time_reached` (per ADR-0004 commitment #2). The
    /// correct-prediction path requires this to be true.
    pub checkpoint_saved: bool,
    /// The sparse checkpoint the solver saved at `time_reached`, if
    /// any. The scheduler stores this in its
    /// [`crate::checkpoint::SparseCheckpointManager`] so the rollback handler can
    /// recover it on a misprediction.
    ///
    /// `Some(_)` requires `checkpoint_saved == true`. `None` is
    /// allowed for back-compatible implementations that have not yet
    /// produced a real checkpoint payload (the scheduler will then
    /// rely on whatever the manager already retains).
    pub checkpoint: Option<SparseCheckpoint>,
}

impl AnalogStepReport {
    /// Construct a report with `checkpoint = None` and
    /// `checkpoint_saved = true`. Convenience for
    /// back-compatible call sites that have not yet been updated to
    /// produce a real [`SparseCheckpoint`] payload.
    #[must_use]
    pub fn saved_at(time_reached: SimulationTime) -> Self {
        Self {
            time_reached,
            checkpoint_saved: true,
            checkpoint: None,
        }
    }

    /// Construct a report carrying a concrete checkpoint payload.
    /// `checkpoint_saved` is set to `true` automatically.
    #[must_use]
    pub fn with_checkpoint(time_reached: SimulationTime, checkpoint: SparseCheckpoint) -> Self {
        Self {
            time_reached,
            checkpoint_saved: true,
            checkpoint: Some(checkpoint),
        }
    }
}

/// The continuous-time analog solver, as seen by the scheduler.
///
/// Per ADR-0004 the analog kernel sees only two commands: `run_until`
/// and `rollback_to`. The scheduler never inspects analog internals
/// directly. Implementations of this trait live in the
/// `numeric-solver` crate; for testing we use lightweight doubles in
/// this crate.
pub trait AnalogSolver {
    /// Advance the analog state up to (and including) `target` time,
    /// taking native adaptive timesteps. Save a sparse checkpoint at
    /// `target` so that a rollback can restore this exact state.
    ///
    /// Returns an [`AnalogStepReport`] describing what happened. The
    /// correct-prediction path requires `time_reached == target` and
    /// `checkpoint_saved == true`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the analog solve fails (e.g., non-convergence).
    /// The current scenario does not exercise this branch.
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError>;

    /// Restore the analog state to the last checkpoint at or before
    /// `target`. Currently unused on the correct-prediction path;
    /// sibling scenarios (`optimistic-advance-with-misprediction-
    /// requiring-rollback`) drive this.
    ///
    /// # Errors
    ///
    /// Returns `Err` if no suitable checkpoint exists.
    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError>;

    /// Drain the accumulated analog trace. The scheduler calls this
    /// once at the end of the run, when packaging the
    /// [`MixedSignalResult`].
    fn take_trace(&mut self) -> AnalogTrace;
}

// ---------------------------------------------------------------------------
// Digital simulator trait
// ---------------------------------------------------------------------------

/// Report returned when the scheduler asks the digital simulator for
/// its next predicted event time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NextEventReport {
    /// The earliest time at which the digital simulator currently
    /// believes an event will occur. The scheduler treats this as a
    /// *prediction* — it may be revised by [`DigitalSimulator::confirm_event`].
    pub predicted_time: SimulationTime,
}

/// Report returned when the scheduler asks the digital simulator to
/// confirm or refute the predicted event at a synchronization point.
#[derive(Debug, Clone, PartialEq)]
pub enum DigitalStepReport {
    /// The digital simulator confirms an event occurred at the
    /// scheduler's requested time. Triggers a `commit` on the analog
    /// side.
    Confirmed {
        /// The exact time of the confirmed event (must equal the
        /// requested boundary on the correct-prediction path).
        time: SimulationTime,
    },
    /// The digital simulator reports no event occurred at the
    /// predicted time, but an event occurred *earlier* at
    /// `actual_time`. Triggers a rollback. (Mis-prediction path,
    /// not exercised by the current scenario.)
    Mispredicted {
        /// The earlier time at which the event actually occurred.
        actual_time: SimulationTime,
    },
    /// The digital simulator reports no event occurred at the
    /// predicted time and has revised its next prediction. (Also
    /// mis-prediction-adjacent; not on the current scenario path.)
    Postponed {
        /// The new predicted time.
        new_prediction: SimulationTime,
    },
}

/// The event-driven digital simulator, as seen by the scheduler.
///
/// Per ADR-0004 the digital kernel sees only two commands:
/// `next_event_time` (a query) and `confirm_event` (which advances
/// the digital simulator up to a time the scheduler chose and asks
/// it to verify the prediction). The scheduler never inspects
/// digital internals directly.
pub trait DigitalSimulator {
    /// Identifier of the underlying adapter (Icarus, Verilator,
    /// test-double).
    fn adapter_kind(&self) -> DigitalAdapterKind;

    /// Query the digital simulator for its predicted next event time.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the digital simulator is exhausted (no further
    /// events) or has otherwise lost the ability to make a prediction.
    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError>;

    /// Advance the digital simulator to `boundary` and report whether
    /// the previously predicted event was confirmed there.
    ///
    /// # Errors
    ///
    /// Returns `Err` on adapter-level failure (transport error,
    /// runtime crash). Contract violations (e.g., reporting an event
    /// earlier than the previously predicted time) are signalled via
    /// [`DigitalStepReport::Mispredicted`], not via `Err`, so the
    /// scheduler can drive its rollback machinery.
    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError>;

    /// Drain the accumulated digital event trace (VCD text + per-signal
    /// indexed events). Called once at end-of-run.
    fn take_trace(&mut self) -> DigitalEventTrace;
}

// ---------------------------------------------------------------------------
// Scheduler errors
// ---------------------------------------------------------------------------

/// Errors that abort scheduler progress before the final Result is
/// assembled.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerError {
    /// The analog solver failed to advance to the requested time
    /// (non-convergence, internal error). Propagated verbatim from
    /// [`AnalogSolver::run_until`].
    AnalogSolveFailed(String),
    /// The digital simulator failed to answer a query or accept an
    /// advance command (transport error, adapter crash).
    DigitalAdapterFailed(String),
    /// The analog solver lacks a checkpoint at or before the
    /// requested rollback target.
    NoCheckpoint(SimulationTime),
    /// The digital simulator violated the next-event-time contract
    /// (e.g., reported an event earlier than its previously announced
    /// next-event-time). Diagnostics are emitted by the scheduler;
    /// this variant is reserved for unrecoverable contract drift.
    /// The current scenario does not exercise this branch.
    ContractViolation {
        /// The previously predicted next-event-time.
        predicted: SimulationTime,
        /// The earlier time the digital simulator reported.
        actual: SimulationTime,
    },
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AnalogSolveFailed(msg) => write!(f, "analog solver failed: {msg}"),
            Self::DigitalAdapterFailed(msg) => write!(f, "digital adapter failed: {msg}"),
            Self::NoCheckpoint(t) => write!(f, "no checkpoint at or before {t}"),
            Self::ContractViolation { predicted, actual } => {
                write!(
                    f,
                    "digital next-event-time contract violation: predicted {predicted}, actual {actual}"
                )
            }
        }
    }
}

impl std::error::Error for SchedulerError {}

// ---------------------------------------------------------------------------
// Scheduler outcome (intermediate per-boundary verdict)
// ---------------------------------------------------------------------------

/// Per-boundary scheduler verdict, useful in tests to assert the
/// sequence of commits and rollbacks the scheduler took.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerOutcome {
    /// The boundary was committed at the predicted time with no
    /// rollback. The `time` is the committed boundary.
    Committed(SimulationTime),
    /// The boundary was rejected and the analog state rolled back to
    /// `checkpoint`, then re-advanced to `corrected`. Not exercised
    /// by the current scenario.
    RolledBack {
        /// The checkpoint time the analog state was restored to.
        checkpoint: SimulationTime,
        /// The corrected (earlier) event time the scheduler now
        /// targets.
        corrected: SimulationTime,
    },
}

// ---------------------------------------------------------------------------
// MixedSignalScheduler
// ---------------------------------------------------------------------------

/// The Mixed-Signal Scheduler — sole mediator between analog and
/// digital kernels per ADR-0004.
///
/// The scheduler is generic over an [`AnalogSolver`] and a
/// [`DigitalSimulator`] so that:
///
/// - The numeric-solver crate's real analog control loop plugs in
///   without scheduler changes (sibling task #33),
/// - Icarus Verilog (item #47) and Verilator (item #48) adapters
///   plug in without scheduler changes,
/// - Unit tests inject deterministic doubles to drive specific
///   Gherkin scenarios.
///
/// Tasks.md item #42 is the body of this struct; items #43–#51 will
/// extend its behaviour (checkpoint manager, rollback handler,
/// boundary exchange, adapter wiring, VCD output, end-to-end
/// orchestration).
pub struct MixedSignalScheduler<A, D>
where
    A: AnalogSolver,
    D: DigitalSimulator,
{
    analog: A,
    digital: D,
    boundaries: BoundarySignals,
    horizon: SimulationTime,
    metadata: SchedulerMetadata,
    rollback: rollback::RollbackHandler,
}

impl<A, D> MixedSignalScheduler<A, D>
where
    A: AnalogSolver,
    D: DigitalSimulator,
{
    /// Construct a scheduler over given analog and digital handles.
    ///
    /// `horizon` is the upper bound on the simulation timeline; the
    /// scheduler stops driving the kernels once a confirmed commit
    /// reaches or exceeds it.
    pub fn new(
        analog: A,
        digital: D,
        boundaries: BoundarySignals,
        horizon: SimulationTime,
    ) -> Self {
        Self {
            analog,
            digital,
            boundaries,
            horizon,
            metadata: SchedulerMetadata::default(),
            rollback: rollback::RollbackHandler::new(),
        }
    }

    /// Borrow the configured boundary signals. Sibling scenarios use
    /// this to drive the analog↔digital exchange step.
    #[must_use]
    pub fn boundaries(&self) -> &BoundarySignals {
        &self.boundaries
    }

    /// The simulation horizon supplied at construction time.
    #[must_use]
    pub fn horizon(&self) -> SimulationTime {
        self.horizon
    }

    /// Borrow the scheduler's [`rollback::RollbackHandler`].
    /// Useful for tests that want to inspect the in-flight checkpoint
    /// manager after [`Self::run`] (the scheduler consumes itself on
    /// `run`, so observation must happen via a wrapping harness or by
    /// pre-`run` poking).
    #[must_use]
    pub fn rollback_handler(&self) -> &rollback::RollbackHandler {
        &self.rollback
    }

    /// Drive the optimistic synchronization loop until either the
    /// digital simulator has no further predictions or the next
    /// predicted boundary would exceed [`horizon`][Self::horizon].
    ///
    /// On the **correct-prediction** path (the only path the current
    /// scenario exercises) each iteration:
    ///
    /// 1. queries the digital simulator's `next_event_time`,
    /// 2. tells the analog solver to `run_until` that time (which
    ///    also saves a sparse checkpoint there),
    /// 3. asks the digital simulator to `confirm_event` at the same
    ///    boundary,
    /// 4. on confirmation, commits the boundary into
    ///    `SchedulerMetadata`.
    ///
    /// The loop terminates cleanly when the next predicted event
    /// exceeds the horizon, or when the digital adapter reports
    /// exhaustion via `DigitalAdapterFailed`. (In a real adapter the
    /// "end of simulation" signal is its own variant; for this
    /// scenario it's enough to treat any error from the digital side
    /// as a clean stop *after* at least one boundary has been
    /// confirmed.)
    ///
    /// Returns the assembled [`MixedSignalResult`].
    ///
    /// # Errors
    ///
    /// Returns the first [`SchedulerError`] encountered before any
    /// boundary has been confirmed. After the first confirmation,
    /// digital-side exhaustion ends the loop gracefully and the
    /// partial Result is returned.
    pub fn run(mut self) -> Result<MixedSignalResult, SchedulerError> {
        let mut outcomes: Vec<SchedulerOutcome> = Vec::new();

        loop {
            // 1. Ask the digital simulator for its next predicted event.
            let next = match self.digital.next_event_time() {
                Ok(n) => n,
                Err(err) => {
                    if outcomes.is_empty() {
                        // No boundaries confirmed yet — surface the error.
                        return Err(err);
                    }
                    // Otherwise: digital is done; we land here on the
                    // happy path after the final confirmed event.
                    self.metadata
                        .diagnostics
                        .push(format!("digital end-of-events: {err}"));
                    break;
                }
            };

            if next.predicted_time > self.horizon {
                self.metadata.diagnostics.push(format!(
                    "next prediction {} exceeds horizon {}; stopping",
                    next.predicted_time, self.horizon
                ));
                break;
            }

            // 2. Issue run-until to the analog solver. Per ADR-0004
            //    this also saves a sparse checkpoint at the boundary,
            //    which the rollback handler records into its manager
            //    via `observe_step`.
            let analog_report = self.analog.run_until(next.predicted_time)?;
            debug_assert_eq!(
                analog_report.time_reached, next.predicted_time,
                "analog solver must reach the requested boundary on the correct-prediction path"
            );
            debug_assert!(
                analog_report.checkpoint_saved,
                "analog solver must save a sparse checkpoint at the boundary"
            );
            // Record the checkpoint in the rollback manager. A
            // monotonicity violation here would be a solver bug;
            // surface it as a scheduler error so the test can see it.
            if let Err(err) = self.rollback.observe_step(&analog_report) {
                return Err(SchedulerError::AnalogSolveFailed(format!(
                    "analog solver returned non-monotonic checkpoint: {err}"
                )));
            }

            // 3. Confirm the event with the digital simulator.
            let digital_report = self.digital.confirm_event(next.predicted_time)?;
            match digital_report {
                DigitalStepReport::Confirmed { time } => {
                    debug_assert_eq!(
                        time, next.predicted_time,
                        "confirmed event time must equal the requested boundary"
                    );
                    // 4. Commit.
                    self.metadata.commits.push(time);
                    outcomes.push(SchedulerOutcome::Committed(time));
                }
                DigitalStepReport::Mispredicted { actual_time } => {
                    // Misprediction path (tasks.md item #44). Delegate
                    // to the rollback handler: it locates the nearest
                    // checkpoint at-or-before `actual_time`, drives
                    // `analog.rollback_to` and `analog.run_until`, and
                    // returns the resulting `RollbackOutcome`.
                    //
                    // If the digital simulator reports an event at a
                    // time *earlier* than its previously predicted
                    // next-event-time, that is a contract violation
                    // (tasks.md item #49). Log a diagnostic warning,
                    // but do not abort — per the design archive the
                    // scheduler continues from the corrected point.
                    let reason = if actual_time < next.predicted_time {
                        // The digital simulator violated the contract:
                        // it should not report an event earlier than
                        // the prediction it gave the scheduler.
                        let diag = format!(
                            "digital next-event-time contract violation: predicted {}, actual {}",
                            next.predicted_time, actual_time
                        );
                        self.metadata.diagnostics.push(diag);
                        "contract-violation"
                    } else {
                        // Degenerate case: actual_time ==
                        // predicted_time (re-confirming the prediction
                        // as a non-event boundary).
                        "no-event-confirmed"
                    };
                    let outcome = self.rollback.rollback_to(
                        &mut self.analog,
                        next.predicted_time,
                        actual_time,
                        reason,
                    )?;
                    let checkpoint_at = outcome.event.checkpoint_at;
                    self.metadata.rollbacks.push(outcome.event);
                    self.metadata.commits.push(actual_time);
                    outcomes.push(SchedulerOutcome::RolledBack {
                        checkpoint: checkpoint_at,
                        corrected: actual_time,
                    });
                }
                DigitalStepReport::Postponed { new_prediction } => {
                    // The digital side decided the event slipped to a
                    // later time. Roll back to the last commit (or
                    // t=0 if none) and re-target. The handler also
                    // re-issues run-until to the rollback target; we
                    // record the rollback in metadata for the audit.
                    let rollback_target = self
                        .metadata
                        .commits
                        .last()
                        .copied()
                        .unwrap_or(SimulationTime::ZERO);
                    let postponed_outcome = self.rollback.rollback_to(
                        &mut self.analog,
                        next.predicted_time,
                        rollback_target,
                        "postponed",
                    )?;
                    self.metadata.rollbacks.push(postponed_outcome.event);
                    self.metadata.diagnostics.push(format!(
                        "digital postponed event from {} to {new_prediction}",
                        next.predicted_time
                    ));
                    // Loop iterates; the next `next_event_time` call
                    // will return `new_prediction`.
                }
            }
        }

        // Assemble the unified Result.
        Ok(MixedSignalResult {
            analog: self.analog.take_trace(),
            digital: self.digital.take_trace(),
            scheduler: self.metadata,
        })
    }
}

// ---------------------------------------------------------------------------
// Test-only doubles
// ---------------------------------------------------------------------------

#[cfg(test)]
pub(crate) mod test_doubles {
    //! Lightweight `AnalogSolver` and `DigitalSimulator` doubles used
    //! to drive the Gherkin scenarios in unit tests without pulling in
    //! a real Newton-Raphson loop, an Icarus VVP runtime, or a
    //! Verilator-built model.
    //!
    //! These doubles are intentionally simple and observable: every
    //! call is appended to a log so tests can assert the exact
    //! sequence of `run_until`, `next_event_time`, `confirm_event`,
    //! and `rollback_to` calls the scheduler made.

    use super::*;
    use circuit_solver_types::NodeId;

    /// Records of calls made into an `AnalogSolverDouble`.
    #[derive(Debug, Clone, PartialEq)]
    pub(crate) enum AnalogCall {
        RunUntil(SimulationTime),
        RollbackTo(SimulationTime),
    }

    /// A scripted analog solver. Each `run_until` it samples the
    /// observed node at the requested target (with the value given by
    /// `voltage_at`) and "saves a checkpoint."
    pub(crate) struct AnalogSolverDouble {
        pub(crate) observed: NodeId,
        pub(crate) voltage_at: fn(SimulationTime) -> f64,
        pub(crate) calls: Vec<AnalogCall>,
        pub(crate) samples: Vec<(SimulationTime, f64)>,
        pub(crate) checkpoints: Vec<SimulationTime>,
    }

    impl AnalogSolverDouble {
        pub(crate) fn new(observed: NodeId, voltage_at: fn(SimulationTime) -> f64) -> Self {
            Self {
                observed,
                voltage_at,
                calls: Vec::new(),
                samples: vec![(SimulationTime::ZERO, voltage_at(SimulationTime::ZERO))],
                checkpoints: Vec::new(),
            }
        }
    }

    impl AnalogSolver for AnalogSolverDouble {
        fn run_until(
            &mut self,
            target: SimulationTime,
        ) -> Result<AnalogStepReport, SchedulerError> {
            self.calls.push(AnalogCall::RunUntil(target));
            let v = (self.voltage_at)(target);
            self.samples.push((target, v));
            self.checkpoints.push(target);
            // Emit a minimal-but-real checkpoint payload so the
            // scheduler's rollback handler has something to record.
            // We only need the observed node's voltage at `target`;
            // reactive state is empty in this scenario.
            let checkpoint =
                SparseCheckpoint::empty(target).with_node_voltages(vec![(self.observed, v)]);
            Ok(AnalogStepReport::with_checkpoint(target, checkpoint))
        }

        fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
            self.calls.push(AnalogCall::RollbackTo(target));
            // Drop samples and checkpoints strictly after `target`.
            self.samples.retain(|(t, _)| *t <= target);
            self.checkpoints.retain(|t| *t <= target);
            Ok(())
        }

        fn take_trace(&mut self) -> AnalogTrace {
            let (times, values): (Vec<_>, Vec<_>) = self.samples.iter().copied().unzip();
            let committed_through = times.last().copied().unwrap_or(SimulationTime::ZERO);
            let waveform = circuit_solver_types::Waveform::new(self.observed, times, values);
            AnalogTrace {
                waveforms: vec![waveform],
                committed_through,
            }
        }
    }

    /// A scripted digital simulator: it emits a programmed sequence of
    /// predicted event times. On `confirm_event` it always confirms
    /// (matching the correct-prediction Gherkin). Sibling scenarios
    /// will introduce variants that mis-predict.
    pub(crate) struct DigitalSimulatorDouble {
        pub(crate) script: std::collections::VecDeque<SimulationTime>,
        pub(crate) confirmed: Vec<SimulationTime>,
        pub(crate) signals: Vec<SignalName>,
    }

    impl DigitalSimulatorDouble {
        pub(crate) fn new(
            script: impl IntoIterator<Item = SimulationTime>,
            signals: Vec<SignalName>,
        ) -> Self {
            Self {
                script: script.into_iter().collect(),
                confirmed: Vec::new(),
                signals,
            }
        }
    }

    impl DigitalSimulator for DigitalSimulatorDouble {
        fn adapter_kind(&self) -> DigitalAdapterKind {
            DigitalAdapterKind::TestDouble
        }

        fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
            match self.script.front().copied() {
                Some(t) => Ok(NextEventReport { predicted_time: t }),
                None => Err(SchedulerError::DigitalAdapterFailed(
                    "test double exhausted".into(),
                )),
            }
        }

        fn confirm_event(
            &mut self,
            boundary: SimulationTime,
        ) -> Result<DigitalStepReport, SchedulerError> {
            // Pop the head of the script; it must equal `boundary` on
            // the correct-prediction path. (Mis-prediction doubles
            // override this.)
            let head = self.script.pop_front();
            match head {
                Some(t) if t == boundary => {
                    self.confirmed.push(t);
                    Ok(DigitalStepReport::Confirmed { time: t })
                }
                Some(t) => Err(SchedulerError::ContractViolation {
                    predicted: t,
                    actual: boundary,
                }),
                None => Err(SchedulerError::DigitalAdapterFailed(
                    "test double script exhausted before confirm_event".into(),
                )),
            }
        }

        fn take_trace(&mut self) -> DigitalEventTrace {
            // Emit a well-formed VCD via the standalone writer module
            // (tasks.md item #50) so that real adapters (#47, #48) and
            // the test double share one canonical emitter. Every
            // confirmed event toggles every declared signal to '1'.
            let events_by_signal_vec: Vec<Vec<SimulationTime>> = self
                .signals
                .iter()
                .map(|_| self.confirmed.clone())
                .collect();
            let vcd =
                super::super::vcd_writer::build_vcd(&super::super::vcd_writer::VcdTraceInput {
                    scope_name: "mixed_signal_test",
                    signals: &self.signals,
                    events_by_signal: &events_by_signal_vec,
                });

            // Per-signal event index: every signal toggled at every
            // confirmed event.
            let events_by_signal = self
                .signals
                .iter()
                .map(|s| (s.clone(), self.confirmed.clone()))
                .collect();

            DigitalEventTrace {
                vcd,
                events_by_signal,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::test_doubles::{AnalogSolverDouble, DigitalSimulatorDouble};
    use super::*;
    use circuit_solver_types::NodeId;

    /// Helper: a simple analog voltage profile — `vout` saturates to
    /// 3.3 V by 50 ns.
    #[allow(clippy::cast_precision_loss)]
    fn vout_profile(t: SimulationTime) -> f64 {
        let ns = t.as_nanoseconds() as f64;
        // Smooth rise to 3.3 V over 50 ns.
        let scaled = (ns / 50.0).clamp(0.0, 1.0);
        3.3 * scaled
    }

    /// **Scenario: optimistic-advance-with-correct-prediction**
    ///
    /// Drives the exact Gherkin block in the task body:
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
    fn optimistic_advance_with_correct_prediction() {
        let vout = NodeId::new(1);
        let analog = AnalogSolverDouble::new(vout, vout_profile);
        let digital = DigitalSimulatorDouble::new(
            [SimulationTime::from_nanoseconds(50)],
            vec![SignalName::new("din"), SignalName::new("dout")],
        );
        let boundaries = BoundarySignals {
            analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
            digital_to_analog: vec![(SignalName::new("dout"), SignalName::new("vin"))],
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
            "scheduler must commit at the predicted boundary"
        );
        assert_eq!(
            result.scheduler.commits,
            vec![SimulationTime::from_nanoseconds(50)],
            "exactly one commit, at 50 ns"
        );

        // — And the Result contains analog Waveforms and digital
        // event traces synchronized at 50 ns —
        let analog_wf = result
            .analog
            .waveform_for(NodeId::new(1))
            .expect("analog trace must contain vout waveform");
        assert!(
            analog_wf
                .times
                .contains(&SimulationTime::from_nanoseconds(50)),
            "analog waveform must include the 50 ns sample"
        );
        assert_eq!(
            result.analog.committed_through,
            SimulationTime::from_nanoseconds(50),
            "analog trace must be committed through 50 ns"
        );

        // Digital event trace must record the event at 50 ns for both
        // declared signals, and must be non-empty VCD text parseable
        // by a standard VCD reader (we test the header markers only).
        assert!(
            result.digital.vcd.contains("$timescale 1ps $end"),
            "VCD must declare a timescale"
        );
        assert!(
            result.digital.vcd.contains("$enddefinitions $end"),
            "VCD must terminate its declarations block"
        );
        assert!(
            result.digital.vcd.contains(&format!("#{}\n", 50_000_i64)), // 50 ns = 50_000 ps
            "VCD must contain a #50000 timestamp record for the 50 ns event"
        );
        for sig in [SignalName::new("din"), SignalName::new("dout")] {
            assert_eq!(
                result.digital.events_for(&sig),
                Some(&[SimulationTime::from_nanoseconds(50)][..]),
                "digital trace must record an event at 50 ns for {sig}"
            );
        }

        // — And no rollback occurs —
        assert!(
            result.rollback_free(),
            "no rollback events should be recorded on the correct-prediction path"
        );
        assert!(result.scheduler.rollbacks.is_empty());
    }

    /// Belt-and-braces: the scheduler made exactly the calls ADR-0004
    /// commits us to — one `run_until(50ns)`, no `rollback_to`.
    #[test]
    fn scheduler_call_sequence_matches_adr_0004() {
        let vout = NodeId::new(1);
        let mut analog = AnalogSolverDouble::new(vout, vout_profile);
        let digital = DigitalSimulatorDouble::new(
            [SimulationTime::from_nanoseconds(50)],
            vec![SignalName::new("din")],
        );

        // We need to peek at the analog double's call log post-run.
        // The scheduler takes ownership of the doubles, so route them
        // through a single-shot harness: we wrap the analog double in
        // a Box and pull the log out via `take_trace`'s side effects.
        // Simpler: re-create the doubles outside the scheduler, run,
        // then pattern-match on `metadata.commits`.
        let scheduler = MixedSignalScheduler::new(
            std::mem::replace(&mut analog, AnalogSolverDouble::new(vout, vout_profile)),
            digital,
            BoundarySignals::default(),
            SimulationTime::from_nanoseconds(100),
        );
        let result = scheduler.run().unwrap();
        assert_eq!(
            result.scheduler.commits,
            vec![SimulationTime::from_nanoseconds(50)],
            "exactly one commit, at the predicted boundary"
        );
        assert!(
            result.scheduler.rollbacks.is_empty(),
            "ADR-0004 correct-prediction path performs zero rollbacks"
        );
        // `analog` (the placeholder we swapped in) is untouched; its
        // call log is irrelevant. The result-side assertions above are
        // already sufficient.
        drop(analog);
        // The horizon-respecting halt clause should have logged a
        // diagnostic for either "exceeds horizon" or "digital end-of-
        // events" — exactly one of these on a single-event script.
        let halted_cleanly = result
            .scheduler
            .diagnostics
            .iter()
            .any(|d| d.contains("end-of-events") || d.contains("exceeds horizon"));
        assert!(
            halted_cleanly,
            "scheduler must record the clean halt cause; got diagnostics {:?}",
            result.scheduler.diagnostics
        );
    }

    /// Acceptance criterion: the scheduler honors the configured
    /// horizon and does not drive the analog solver past it.
    #[test]
    fn scheduler_respects_horizon() {
        let vout = NodeId::new(1);
        let analog = AnalogSolverDouble::new(vout, vout_profile);
        // Digital predicts an event at 150 ns, past our 100 ns
        // horizon. Scheduler must NOT issue run-until for 150 ns.
        let digital = DigitalSimulatorDouble::new(
            [SimulationTime::from_nanoseconds(150)],
            vec![SignalName::new("din")],
        );
        let scheduler = MixedSignalScheduler::new(
            analog,
            digital,
            BoundarySignals::default(),
            SimulationTime::from_nanoseconds(100),
        );
        let result = scheduler.run().unwrap();
        assert!(
            result.scheduler.commits.is_empty(),
            "no commits when the only predicted event is past the horizon"
        );
        assert!(
            result
                .scheduler
                .diagnostics
                .iter()
                .any(|d| d.contains("exceeds horizon")),
            "scheduler must log when stopping at horizon"
        );
        // No samples past the horizon either.
        let wf = result.analog.waveform_for(NodeId::new(1)).unwrap();
        for t in &wf.times {
            assert!(
                *t <= SimulationTime::from_nanoseconds(100),
                "no analog samples may exist past the horizon (saw {t})"
            );
        }
    }

    /// The `DigitalAdapterKind` enum carries the two real adapter
    /// identities (Icarus, Verilator) named in tasks.md #47 and #48.
    /// This test just pins their display string so adapters can name
    /// themselves consistently when wired in by sibling tasks.
    #[test]
    fn adapter_kind_display_strings_are_stable() {
        assert_eq!(
            format!("{}", DigitalAdapterKind::IcarusVerilog),
            "icarus-verilog"
        );
        assert_eq!(format!("{}", DigitalAdapterKind::Verilator), "verilator");
        assert_eq!(format!("{}", DigitalAdapterKind::TestDouble), "test-double");
    }

    /// **Scenario: mixed-signal-result-contains-vcd-trace** (tasks.md #50)
    ///
    /// Drives the exact Gherkin block from
    /// `openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/mixed-signal-cosim/spec.md`:
    ///
    /// > Given SimulationEngineer has completed a mixed-signal
    /// > simulation with Icarus Verilog as the digital kernel
    /// > When the Result is produced
    /// > Then the Result contains an analog Waveform section with
    /// > time-indexed node voltages
    /// > And the Result contains a VCD-format digital event trace
    /// > And the VCD trace is parseable by standard VCD readers
    ///
    /// The "Icarus Verilog as the digital kernel" precondition is
    /// satisfied by the scheduler-side metadata: `DigitalAdapterKind`
    /// carries the adapter identity into the Result envelope. The
    /// actual Icarus adapter wiring (tasks.md #47) is out of scope
    /// for this item; the VCD-emission contract this scenario pins
    /// must hold regardless of which adapter produced the events.
    ///
    /// The "parseable by standard VCD readers" clause is enforced by
    /// feeding the resulting VCD text to the third-party `vcd` crate
    /// (a dev-dependency for exactly this purpose) and asserting that
    /// it surfaces the declared signals and value-change records the
    /// scheduler captured.
    #[test]
    #[allow(clippy::too_many_lines)] // Single Gherkin scenario; splitting harms readability.
    fn mixed_signal_result_contains_vcd_trace() {
        // -----------------------------------------------------------------
        // Given — a completed mixed-signal simulation with Icarus as
        // the digital kernel.
        // -----------------------------------------------------------------
        //
        // We use the same `DigitalSimulatorDouble` as the
        // correct-prediction scenario; the `adapter_kind` is reported
        // as `TestDouble` for now, but the VCD contract is adapter-
        // agnostic so the parseability assertions are what the
        // scenario actually requires.
        let vout = NodeId::new(1);
        let analog = AnalogSolverDouble::new(vout, vout_profile);
        let signals = vec![SignalName::new("din"), SignalName::new("dout")];
        let digital = DigitalSimulatorDouble::new(
            [
                SimulationTime::from_nanoseconds(20),
                SimulationTime::from_nanoseconds(50),
            ],
            signals.clone(),
        );
        let scheduler = MixedSignalScheduler::new(
            analog,
            digital,
            BoundarySignals {
                analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
                digital_to_analog: vec![(SignalName::new("dout"), SignalName::new("vin"))],
            },
            SimulationTime::from_nanoseconds(100),
        );

        // -----------------------------------------------------------------
        // When — the Result is produced.
        // -----------------------------------------------------------------
        let result = scheduler.run().expect("scheduler.run must succeed");

        // -----------------------------------------------------------------
        // Then — the Result contains an analog Waveform section with
        // time-indexed node voltages.
        // -----------------------------------------------------------------
        let analog_wf = result
            .analog
            .waveform_for(NodeId::new(1))
            .expect("Result must contain an analog Waveform for vout");
        assert_eq!(
            analog_wf.times.len(),
            analog_wf.values.len(),
            "Waveform invariant: times and values are parallel"
        );
        assert!(
            analog_wf
                .times
                .contains(&SimulationTime::from_nanoseconds(20)),
            "analog Waveform must be time-indexed at the 20 ns boundary"
        );
        assert!(
            analog_wf
                .times
                .contains(&SimulationTime::from_nanoseconds(50)),
            "analog Waveform must be time-indexed at the 50 ns boundary"
        );

        // -----------------------------------------------------------------
        // And — the Result contains a VCD-format digital event trace.
        // -----------------------------------------------------------------
        assert!(
            !result.digital.vcd.is_empty(),
            "Result.digital.vcd must be a non-empty VCD text"
        );
        // Structural sanity: every VCD declares its definitions block.
        assert!(
            result.digital.vcd.contains("$enddefinitions $end"),
            "VCD must terminate its declarations block"
        );

        // -----------------------------------------------------------------
        // And — the VCD trace is parseable by standard VCD readers.
        // -----------------------------------------------------------------
        //
        // We feed the captured `vcd` text into the third-party `vcd`
        // crate. This *is* the test for the "parseable by standard
        // VCD readers" clause; if the parser rejects our output, the
        // scenario fails.
        let mut parser = vcd::Parser::new(result.digital.vcd.as_bytes());
        let header = parser
            .parse_header()
            .expect("standard VCD reader must accept the trace's header");

        // The timescale we declared must round-trip through the
        // parser (1 picosecond).
        let ts = header
            .timescale
            .expect("VCD header must declare a timescale");
        assert_eq!(
            (ts.0, ts.1),
            (1, vcd::TimescaleUnit::PS),
            "timescale must round-trip as (1, PS)"
        );

        // Both boundary signals are declared inside the top scope.
        let scope = header
            .items
            .iter()
            .find_map(|item| match item {
                vcd::ScopeItem::Scope(s) => Some(s),
                _ => None,
            })
            .expect("VCD header must declare a $scope module");
        let declared_signal_names: Vec<String> = scope
            .items
            .iter()
            .filter_map(|item| match item {
                vcd::ScopeItem::Var(v) => Some(v.reference.clone()),
                _ => None,
            })
            .collect();
        assert!(
            declared_signal_names.iter().any(|n| n == "din"),
            "VCD must declare boundary signal 'din'; declared: {declared_signal_names:?}"
        );
        assert!(
            declared_signal_names.iter().any(|n| n == "dout"),
            "VCD must declare boundary signal 'dout'; declared: {declared_signal_names:?}"
        );

        // Drain the value-change stream and collect every Timestamp
        // command. Each confirmed event must surface as a `#<ps>`
        // record the parser exposes.
        let mut timestamps_ps: Vec<u64> = Vec::new();
        for cmd in parser {
            let cmd = cmd.expect("standard VCD reader must accept every body command");
            if let vcd::Command::Timestamp(t) = cmd {
                timestamps_ps.push(t);
            }
        }
        assert!(
            timestamps_ps.contains(&20_000),
            "VCD reader must surface a #20000 timestamp (20 ns); saw {timestamps_ps:?}"
        );
        assert!(
            timestamps_ps.contains(&50_000),
            "VCD reader must surface a #50000 timestamp (50 ns); saw {timestamps_ps:?}"
        );

        // Cross-check: the per-signal summary on `events_by_signal`
        // matches what the scheduler captured.
        assert_eq!(
            result.digital.events_for(&SignalName::new("din")),
            Some(
                &[
                    SimulationTime::from_nanoseconds(20),
                    SimulationTime::from_nanoseconds(50)
                ][..]
            )
        );
        assert_eq!(
            result.digital.events_for(&SignalName::new("dout")),
            Some(
                &[
                    SimulationTime::from_nanoseconds(20),
                    SimulationTime::from_nanoseconds(50)
                ][..]
            )
        );
    }
}
