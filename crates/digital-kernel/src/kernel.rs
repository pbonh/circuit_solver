//! Native digital kernel — the in-process event-driven engine mandated by ADR-0006.
//!
//! ADR-0006 ("Native Event-Driven Digital Engine") replaces external
//! co-simulation (ADR-0004) with a native, in-process DEVS-style event
//! queue. The [`DigitalKernel`] is the top-level type that the Mixed-Signal
//! Scheduler drives via `run_until` — no IPC, no external process.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │            DigitalKernel                     │
//! │                                              │
//! │  ┌─────────────┐    ┌──────────────────┐    │
//! │  │ EventQueue   │    │ NetState         │    │
//! │  │ (min-heap)   │    │ (net → value)   │    │
//! │  └──────┬──────┘    └────────┬─────────┘    │
//! │         │                    │               │
//! │         └────────┬───────────┘              │
//! │                  │                          │
//! │         run_until(target)                    │
//! │         → KernelRunReport                    │
//! │                                              │
//! │  ┌─────────────────────────────────────┐    │
//! │  │ CombinationalEvaluator (optional)   │    │
//! │  │ + SettleConfig                       │    │
//! │  │ → delta-cycle settling per time pt   │    │
//! │  └─────────────────────────────────────┘    │
//! └─────────────────────────────────────────────┘
//! ```
//!
//! - [`EventQueue`] — binary min-heap of time-ordered events.
//! - [`NetState`] — current value of every digital net (wire).
//! - [`DigitalKernel`] — composes the two, exposes the `run_until` API.
//! - [`CombinationalEvaluator`] + [`SettleConfig`] — delta-cycle
//!   combinational settling with oscillation detection (task #12).
//!
//! # Delta-cycle settling (task #12)
//!
//! When a [`CombinationalEvaluator`] is installed, `run_until` processes
//! events **one time point at a time**, running delta-cycle settling after
//! each time point. This propagates zero-delay combinational logic until
//! the net state stabilizes. Oscillation is detected via state hashing
//! and a hard delta-cycle limit, guaranteeing the kernel **never hangs**.
//!
//! Without an evaluator, `run_until` behaves exactly as in task #11
//! (backward compatible).
//!
//! # Integration with analysis-orchestration
//!
//! The kernel is designed to implement the `DigitalSimulator` trait
//! (defined in `analysis-orchestration::mixed_signal`). Task #17 will
//! wire that impl; this crate provides the standalone kernel so that
//! task #12 (delta-cycle settling) and task #13 (checkpoint/restore)
//! can build on it without depending on the orchestration crate.

use circuit_solver_types::SimulationTime;
use core::fmt;

use crate::event_queue::{DigitalEvent, EventQueue, EventQueueCheckpoint, LogicValue, NetId};
use crate::settle::{self, CombinationalEvaluator, SettleConfig, SettleOutcome};

// ---------------------------------------------------------------------------
// Net state
// ---------------------------------------------------------------------------

/// The current value of every digital net (wire) in the kernel.
///
/// Net state is updated as events are processed during `run_until`.
/// After processing, the net state reflects the values that combinational
/// settling (task #12) would compute. The kernel's checkpoint/restore
/// mechanism (task #13) snapshots net state alongside the event queue.
#[derive(Debug, Clone, PartialEq)]
pub struct NetState {
    /// Net values indexed by [`NetId::index`].
    values: Vec<LogicValue>,
}

impl Default for NetState {
    fn default() -> Self {
        Self::new()
    }
}

impl NetState {
    /// Create an empty net state.
    #[must_use]
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Create a net state pre-allocated for `n` nets, all initialized
    /// to [`LogicValue::Unknown`].
    #[must_use]
    pub fn with_nets(n: usize) -> Self {
        Self {
            values: vec![LogicValue::Unknown; n],
        }
    }

    /// Get the current value of net `id`.
    ///
    /// Returns [`LogicValue::Unknown`] for nets that have not been
    /// assigned.
    #[must_use]
    pub fn get(&self, id: NetId) -> LogicValue {
        self.values
            .get(id.index() as usize)
            .copied()
            .unwrap_or(LogicValue::Unknown)
    }

    /// Set the value of net `id`. Grows the internal vector if needed.
    pub fn set(&mut self, id: NetId, value: LogicValue) {
        let idx = id.index() as usize;
        if idx >= self.values.len() {
            self.values.resize(idx + 1, LogicValue::Unknown);
        }
        self.values[idx] = value;
    }

    /// Number of nets tracked (may include unassigned slots).
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// True iff no nets are tracked.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Checkpoint the net state for rollback.
    #[must_use]
    pub fn checkpoint(&self) -> NetStateCheckpoint {
        NetStateCheckpoint {
            values: self.values.clone(),
        }
    }

    /// Restore from a previously captured checkpoint.
    pub fn restore_from_checkpoint(&mut self, cp: NetStateCheckpoint) {
        self.values = cp.values;
    }
}

/// A snapshot of [`NetState`] for rollback (task #13).
#[derive(Debug, Clone, PartialEq)]
pub struct NetStateCheckpoint {
    values: Vec<LogicValue>,
}

// ---------------------------------------------------------------------------
// Kernel checkpoint
// ---------------------------------------------------------------------------

/// A combined checkpoint of the [`DigitalKernel`]'s event queue and net
/// state, sufficient for the optimistic rollback mechanism (task #13).
///
/// Produced by [`DigitalKernel::checkpoint`] and consumed by
/// [`DigitalKernel::restore_from_checkpoint`].
#[derive(Debug, Clone, PartialEq)]
pub struct KernelCheckpoint {
    /// Snapshot of the event queue.
    pub queue: EventQueueCheckpoint,
    /// Snapshot of the net state.
    pub net_state: NetStateCheckpoint,
}

// ---------------------------------------------------------------------------
// Settle report per time point
// ---------------------------------------------------------------------------

/// Settle result for a single time point within a `run_until` call.
#[derive(Debug, Clone, PartialEq)]
pub struct TimePointSettleReport {
    /// The simulation time at which settling occurred.
    pub time: SimulationTime,
    /// The settle outcome.
    pub outcome: SettleOutcome,
}

// ---------------------------------------------------------------------------
// Digital kernel
// ---------------------------------------------------------------------------

/// The native, in-process event-driven digital kernel (ADR-0006).
///
/// The kernel composes an [`EventQueue`] (time-ordered event scheduling)
/// with a [`NetState`] (current value of every digital net) and an
/// optional [`CombinationalEvaluator`] (delta-cycle settling). The
/// Mixed-Signal Scheduler drives the kernel via [`run_until`] — no IPC,
/// no external process.
///
/// # In-process run-until API
///
/// [`run_until(target)`] advances the kernel's simulation clock to
/// `target`, processing all scheduled events at or before `target`.
///
/// **Without an evaluator** (task #11 behavior): events are processed in
/// bulk, updating net state directly.
///
/// **With an evaluator** (task #12 behavior): events are processed one
/// time point at a time, and delta-cycle settling runs after each time
/// point. This propagates zero-delay combinational logic until the net
/// state stabilizes. Oscillation is detected and reported — the kernel
/// never hangs.
///
/// # Builder pattern
///
/// Use [`with_evaluator`] and [`with_settle_config`] to configure
/// delta-cycle settling:
///
/// ```
/// # use digital_kernel::{DigitalKernel, NetId, LogicValue, SettleConfig};
/// # use digital_kernel::settle::FnEvaluator;
/// let kernel = DigitalKernel::new()
///     .with_evaluator(FnEvaluator::new(|_, _| vec![]))
///     .with_settle_config(SettleConfig::with_max_delta_cycles(50));
/// ```
///
/// [`run_until`]: DigitalKernel::run_until
/// [`run_until(target)`]: DigitalKernel::run_until
/// [`with_evaluator`]: DigitalKernel::with_evaluator
/// [`with_settle_config`]: DigitalKernel::with_settle_config
pub struct DigitalKernel {
    /// The event queue (scheduling and processing).
    queue: EventQueue,
    /// The current value of every digital net.
    net_state: NetState,
    /// Optional combinational evaluator for delta-cycle settling.
    evaluator: Option<Box<dyn CombinationalEvaluator>>,
    /// Configuration for delta-cycle settling. Used only when
    /// `evaluator` is `Some`.
    settle_config: SettleConfig,
}

impl Default for DigitalKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for DigitalKernel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DigitalKernel")
            .field("queue", &self.queue)
            .field("net_state", &self.net_state)
            .field("settle_config", &self.settle_config)
            .field("has_evaluator", &self.evaluator.is_some())
            .finish()
    }
}

impl DigitalKernel {
    /// Create a new kernel with the simulation clock at t=0 and no
    /// nets or events. No combinational evaluator is installed.
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: EventQueue::new(),
            net_state: NetState::new(),
            evaluator: None,
            settle_config: SettleConfig::default(),
        }
    }

    /// Create a kernel pre-allocated for `n` nets, all initialized
    /// to [`LogicValue::Unknown`].
    #[must_use]
    pub fn with_nets(n: usize) -> Self {
        Self {
            queue: EventQueue::new(),
            net_state: NetState::with_nets(n),
            evaluator: None,
            settle_config: SettleConfig::default(),
        }
    }

    /// Install a combinational evaluator for delta-cycle settling.
    ///
    /// When an evaluator is installed, `run_until` processes events
    /// one time point at a time and invokes settling after each.
    /// Without an evaluator, `run_until` uses the bulk-processing
    /// path from task #11.
    #[must_use]
    pub fn with_evaluator(mut self, evaluator: impl CombinationalEvaluator + 'static) -> Self {
        self.evaluator = Some(Box::new(evaluator));
        self
    }

    /// Override the settle configuration.
    ///
    /// Only used when an evaluator is installed. The default config
    /// allows up to 100 delta cycles.
    #[must_use]
    pub fn with_settle_config(mut self, config: SettleConfig) -> Self {
        self.settle_config = config;
        self
    }

    // ----- Queries -----

    /// The current simulation clock value.
    #[must_use]
    pub fn current_time(&self) -> SimulationTime {
        self.queue.current_time()
    }

    /// The earliest scheduled event time, or `None` if no events
    /// are pending.
    #[must_use]
    pub fn next_event_time(&self) -> Option<SimulationTime> {
        self.queue.next_event_time()
    }

    /// Get the current value of net `id`.
    #[must_use]
    pub fn net_value(&self, id: NetId) -> LogicValue {
        self.net_state.get(id)
    }

    /// Number of pending events in the queue.
    #[must_use]
    pub fn pending_event_count(&self) -> usize {
        self.queue.pending_count()
    }

    /// True iff no pending events remain.
    #[must_use]
    pub fn is_idle(&self) -> bool {
        self.queue.is_empty()
    }

    /// Reference to the underlying event queue (for inspection).
    #[must_use]
    pub fn queue(&self) -> &EventQueue {
        &self.queue
    }

    /// Reference to the underlying net state (for inspection).
    #[must_use]
    pub fn net_state(&self) -> &NetState {
        &self.net_state
    }

    /// Whether a combinational evaluator is installed.
    #[must_use]
    pub fn has_evaluator(&self) -> bool {
        self.evaluator.is_some()
    }

    /// The settle configuration.
    #[must_use]
    pub fn settle_config(&self) -> &SettleConfig {
        &self.settle_config
    }

    // ----- Scheduling -----

    /// Schedule an event on the kernel's event queue.
    ///
    /// The event's net and value will be applied when the event is
    /// processed during [`run_until`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::event_queue::EventQueueError::TimeTravel`]
    /// if the event time is before the current simulation clock.
    ///
    /// [`run_until`]: DigitalKernel::run_until
    pub fn schedule(
        &mut self,
        event: DigitalEvent,
    ) -> Result<(), crate::event_queue::EventQueueError> {
        self.queue.schedule(event)
    }

    // ----- In-process run-until -----

    /// Advance the kernel to `target`, processing all events at or
    /// before `target`.
    ///
    /// This is the **in-process run-until API** that ADR-0006 mandates.
    /// The Mixed-Signal Scheduler calls this directly — no IPC, no
    /// external process.
    ///
    /// # Without an evaluator (task #11 behavior)
    ///
    /// Events are processed in bulk. For each event processed, the
    /// kernel updates the net state and records the event for trace
    /// assembly.
    ///
    /// # With an evaluator (task #12 behavior)
    ///
    /// Events are processed **one time point at a time**. After
    /// processing all events at a given time T, the kernel runs
    /// delta-cycle settling:
    ///
    /// 1. The evaluator is invoked with the nets that changed at T.
    /// 2. Any new assignments are applied and the changed nets are
    ///    fed back to the evaluator.
    /// 3. This repeats until either the evaluator returns no new
    ///    assignments (**settled**) or the delta-cycle limit is
    ///    exceeded / a state cycle is detected (**oscillating**).
    ///
    /// Oscillation is recorded in the report but does **not** stop
    /// processing — the kernel continues to subsequent time points
    /// (it never hangs).
    ///
    /// # Panics
    ///
    /// Panics if `target` is before the current simulation clock.
    pub fn run_until(&mut self, target: SimulationTime) -> KernelRunReport {
        match &self.evaluator {
            None => self.run_until_bulk(target),
            Some(_) => self.run_until_with_settling(target),
        }
    }

    /// Bulk-processing path (task #11, no evaluator).
    fn run_until_bulk(&mut self, target: SimulationTime) -> KernelRunReport {
        let report = self.queue.run_until(target);

        // Apply each processed event to the net state.
        for event in &report.events_processed {
            self.net_state.set(event.net, event.value);
        }

        KernelRunReport {
            time_reached: report.time_reached,
            events_processed: report.events_processed,
            next_event_time: report.next_event_time,
            settle_reports: vec![],
        }
    }

    /// Per-timepoint path with delta-cycle settling (task #12).
    fn run_until_with_settling(&mut self, target: SimulationTime) -> KernelRunReport {
        assert!(
            target >= self.queue.current_time(),
            "run_until target {target} is before current time {}",
            self.queue.current_time()
        );

        let mut all_events_processed: Vec<DigitalEvent> = Vec::new();
        let mut settle_reports: Vec<TimePointSettleReport> = Vec::new();

        // Process one time point at a time.
        loop {
            // Peek at the next event time.
            let next_t = match self.queue.next_event_time() {
                Some(t) if t <= target => t,
                _ => break, // No more events at or before target.
            };

            // Process all events at time next_t.
            let queue_report = self.queue.run_until(next_t);

            // Track which nets changed value at this time point.
            let mut changed_nets: Vec<NetId> = Vec::new();
            for event in &queue_report.events_processed {
                let old = self.net_state.get(event.net);
                if old != event.value {
                    changed_nets.push(event.net);
                }
                self.net_state.set(event.net, event.value);
            }
            all_events_processed.extend(queue_report.events_processed.clone());

            // Run delta-cycle settling.
            let outcome = settle::settle(
                &mut self.net_state,
                self.evaluator.as_ref().unwrap().as_ref(),
                changed_nets,
                &self.settle_config,
            );

            settle_reports.push(TimePointSettleReport {
                time: next_t,
                outcome,
            });

            // Continue to next time point (even if oscillation detected).
        }

        // Advance the clock to target if it's not already there.
        if self.queue.current_time() < target {
            let _ = self.queue.run_until(target);
        }

        KernelRunReport {
            time_reached: target,
            events_processed: all_events_processed,
            next_event_time: self.queue.next_event_time(),
            settle_reports,
        }
    }

    // ----- Trace -----

    /// Drain the accumulated processed events, returning them in
    /// processing order.
    ///
    /// Called at end-of-run to assemble the digital event trace.
    #[must_use]
    pub fn take_processed_events(&mut self) -> Vec<DigitalEvent> {
        self.queue.take_processed_events()
    }

    // ----- Checkpoint / restore (task #13 foundation) -----

    /// Checkpoint the kernel's complete state (event queue + net state)
    /// for the optimistic rollback mechanism.
    #[must_use]
    pub fn checkpoint(&self) -> KernelCheckpoint {
        KernelCheckpoint {
            queue: self.queue.checkpoint(),
            net_state: self.net_state.checkpoint(),
        }
    }

    /// Restore the kernel to a previously captured checkpoint.
    ///
    /// Used by the rollback mechanism (task #13) to reset the kernel
    /// to a known-good state after a misprediction.
    pub fn restore_from_checkpoint(&mut self, cp: KernelCheckpoint) {
        self.queue.restore_from_checkpoint(cp.queue);
        self.net_state.restore_from_checkpoint(cp.net_state);
    }
}

// ---------------------------------------------------------------------------
// Kernel run report
// ---------------------------------------------------------------------------

/// Report returned by [`DigitalKernel::run_until`] describing what
/// happened during the advance.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelRunReport {
    /// The simulation time the kernel was advanced to. Equals `target`
    /// on the normal path.
    pub time_reached: SimulationTime,
    /// Events processed during this `run_until` call, in the order
    /// they were processed.
    pub events_processed: Vec<DigitalEvent>,
    /// The next scheduled event time after `target`, if any.
    pub next_event_time: Option<SimulationTime>,
    /// Settle reports for each time point where settling occurred.
    /// Empty when no evaluator is installed (task #11 behavior).
    pub settle_reports: Vec<TimePointSettleReport>,
}

impl KernelRunReport {
    /// Whether any time point reported oscillation.
    #[must_use]
    pub fn has_oscillation(&self) -> bool {
        self.settle_reports
            .iter()
            .any(|r| matches!(r.outcome, SettleOutcome::Oscillating { .. }))
    }

    /// Total delta cycles across all time points.
    #[must_use]
    pub fn total_delta_cycles(&self) -> u32 {
        self.settle_reports
            .iter()
            .map(|r| match r.outcome {
                SettleOutcome::Settled { delta_cycles }
                | SettleOutcome::Oscillating { delta_cycles, .. } => delta_cycles,
            })
            .sum()
    }
}

impl fmt::Display for KernelRunReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "run_until(t={}): {} events processed, next_event={}",
            self.time_reached,
            self.events_processed.len(),
            match self.next_event_time {
                Some(t) => format!("Some({t})"),
                None => "None".to_string(),
            }
        )?;
        if !self.settle_reports.is_empty() {
            let settled = self
                .settle_reports
                .iter()
                .filter(|r| matches!(r.outcome, SettleOutcome::Settled { .. }))
                .count();
            let oscillating = self
                .settle_reports
                .iter()
                .filter(|r| matches!(r.outcome, SettleOutcome::Oscillating { .. }))
                .count();
            write!(
                f,
                ", settle: {settled} settled, {oscillating} oscillating, {} total delta cycles",
                self.total_delta_cycles()
            )?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settle::FnEvaluator;

    // -- Task #11 backward compatibility tests --

    #[test]
    fn new_kernel_starts_at_zero() {
        let k = DigitalKernel::new();
        assert_eq!(k.current_time(), SimulationTime::ZERO);
        assert!(k.is_idle());
        assert_eq!(k.next_event_time(), None);
    }

    #[test]
    fn with_nets_initializes_unknown() {
        let k = DigitalKernel::with_nets(4);
        for i in 0..4u32 {
            assert_eq!(k.net_value(NetId::new(i)), LogicValue::Unknown);
        }
    }

    #[test]
    fn schedule_and_run_until_applies_net_state() {
        let mut k = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let net_a = NetId::new(0);
        let net_b = NetId::new(1);

        k.schedule(DigitalEvent::new(t50, net_a, LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(t50, net_b, LogicValue::Zero))
            .unwrap();

        let report = k.run_until(t50);
        assert_eq!(report.time_reached, t50);
        assert_eq!(report.events_processed.len(), 2);
        assert_eq!(k.net_value(net_a), LogicValue::One);
        assert_eq!(k.net_value(net_b), LogicValue::Zero);
    }

    #[test]
    fn run_until_partial_advance() {
        let mut k = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(t100, NetId::new(1), LogicValue::Zero))
            .unwrap();

        // Advance to 50 ns: only the first event should fire.
        let report = k.run_until(t50);
        assert_eq!(report.events_processed.len(), 1);
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::One);
        // Net 1 still unknown (not yet processed).
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::Unknown);
        assert_eq!(report.next_event_time, Some(t100));
    }

    #[test]
    fn checkpoint_and_restore_roundtrip() {
        let mut k = DigitalKernel::with_nets(2);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(t100, NetId::new(1), LogicValue::Zero))
            .unwrap();

        // Checkpoint before running.
        let cp = k.checkpoint();

        // Run to 100 ns.
        let _ = k.run_until(t100);
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::One);
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::Zero);
        assert_eq!(k.current_time(), t100);

        // Restore: kernel should be back to t=0 with 2 pending events.
        k.restore_from_checkpoint(cp);
        assert_eq!(k.current_time(), SimulationTime::ZERO);
        assert_eq!(k.pending_event_count(), 2);
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::Unknown);
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::Unknown);
    }

    #[test]
    fn take_processed_events_drains() {
        let mut k = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(t50);

        let events = k.take_processed_events();
        assert_eq!(events.len(), 1);
        assert!(k.take_processed_events().is_empty());
    }

    #[test]
    fn net_state_auto_grows_on_set() {
        let mut ns = NetState::new();
        assert!(ns.is_empty());
        ns.set(NetId::new(5), LogicValue::One);
        assert_eq!(ns.len(), 6); // indices 0..=5
        assert_eq!(ns.get(NetId::new(5)), LogicValue::One);
        // Uninitialized slots are Unknown.
        assert_eq!(ns.get(NetId::new(0)), LogicValue::Unknown);
    }

    #[test]
    fn kernel_run_report_display() {
        let report = KernelRunReport {
            time_reached: SimulationTime::from_nanoseconds(50),
            events_processed: vec![DigitalEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(0),
                LogicValue::One,
            )],
            next_event_time: Some(SimulationTime::from_nanoseconds(100)),
            settle_reports: vec![],
        };
        let s = format!("{report}");
        assert!(s.contains("1 events processed"));
        assert!(s.contains("next_event=Some"));
    }

    // -- Task #12: Delta-cycle settling tests --

    #[test]
    fn kernel_without_evaluator_no_settle_reports() {
        let mut k = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        let report = k.run_until(t50);
        assert!(report.settle_reports.is_empty());
        assert!(!report.has_oscillation());
        assert_eq!(report.total_delta_cycles(), 0);
    }

    #[test]
    fn kernel_with_evaluator_settles_simple() {
        // Net 0 changes to One; evaluator propagates to net 1.
        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) && ns.get(NetId::new(0)) == LogicValue::One {
                    out.push((NetId::new(1), LogicValue::Zero));
                }
            }
            out
        });

        let mut k = DigitalKernel::with_nets(2)
            .with_evaluator(eval)
            .with_settle_config(SettleConfig::with_max_delta_cycles(10));

        let t50 = SimulationTime::from_nanoseconds(50);
        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();

        let report = k.run_until(t50);
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::One);
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::Zero);
        assert_eq!(report.settle_reports.len(), 1);
        assert_eq!(
            report.settle_reports[0].outcome,
            SettleOutcome::Settled { delta_cycles: 1 }
        );
        assert!(!report.has_oscillation());
    }

    #[test]
    fn kernel_with_cascading_evaluator_settles() {
        // Net 0 → net 1 → net 2, two delta cycles.
        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) {
                    out.push((NetId::new(1), LogicValue::Zero));
                }
                if net == NetId::new(1) && ns.get(NetId::new(1)) == LogicValue::Zero {
                    out.push((NetId::new(2), LogicValue::One));
                }
            }
            out
        });

        let mut k = DigitalKernel::with_nets(3)
            .with_evaluator(eval)
            .with_settle_config(SettleConfig::with_max_delta_cycles(10));

        let t50 = SimulationTime::from_nanoseconds(50);
        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();

        let report = k.run_until(t50);
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::One);
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::Zero);
        assert_eq!(k.net_value(NetId::new(2)), LogicValue::One);
        assert_eq!(
            report.settle_reports[0].outcome,
            SettleOutcome::Settled { delta_cycles: 2 }
        );
    }

    #[test]
    fn kernel_oscillation_detected_but_continues() {
        // Self-feeding inverter on net 0: always flips.
        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                let v = ns.get(net);
                out.push((
                    net,
                    match v {
                        LogicValue::One => LogicValue::Zero,
                        LogicValue::Zero => LogicValue::One,
                        _ => LogicValue::One,
                    },
                ));
            }
            out
        });

        let mut k = DigitalKernel::with_nets(2)
            .with_evaluator(eval)
            .with_settle_config(SettleConfig::with_max_delta_cycles(100));

        // Schedule two events: one at t=50 (will oscillate) and
        // one at t=100 on a different net (should still process).
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);
        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(t100, NetId::new(1), LogicValue::One))
            .unwrap();

        let report = k.run_until(t100);
        // Should have reached t=100 despite oscillation at t=50.
        assert_eq!(report.time_reached, t100);
        // Net 1 should have been updated.
        // (It may have oscillated too since the evaluator flips all
        //  changed nets, but the key point is we didn't hang.)
        assert!(report.has_oscillation());
    }

    #[test]
    fn kernel_multiple_time_points_each_settle() {
        // Two time points with different propagation behavior.
        // Evaluator: net 0→One drives net 1 to Zero; net 0→Zero drives
        // net 1 to One (inverter chain).
        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) {
                    let v = ns.get(NetId::new(0));
                    // Invert: net 0=One → net 1=Zero; net 0=Zero → net 1=One.
                    let driven = match v {
                        LogicValue::One => LogicValue::Zero,
                        LogicValue::Zero => LogicValue::One,
                        _ => LogicValue::Unknown,
                    };
                    out.push((NetId::new(1), driven));
                }
            }
            out
        });

        let mut k = DigitalKernel::with_nets(2)
            .with_evaluator(eval)
            .with_settle_config(SettleConfig::with_max_delta_cycles(10));

        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        // First event: net 0 → One (drives net 1 to Zero).
        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        // Second event: net 0 → Zero (drives net 1 to One).
        k.schedule(DigitalEvent::new(t100, NetId::new(0), LogicValue::Zero))
            .unwrap();

        let report = k.run_until(t100);
        assert_eq!(report.settle_reports.len(), 2);

        // First time point: net 0=One, cascade → net 1=Zero.
        assert!(matches!(
            &report.settle_reports[0].outcome,
            SettleOutcome::Settled { delta_cycles: 1 }
        ));
        // Second time point: net 0=Zero, cascade → net 1=One.
        assert!(matches!(
            &report.settle_reports[1].outcome,
            SettleOutcome::Settled { delta_cycles: 1 }
        ));

        // Final state.
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::Zero);
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::One);
    }

    #[test]
    fn kernel_with_no_events_no_settle() {
        let eval = FnEvaluator::new(|_, _| vec![]);
        let k = DigitalKernel::new()
            .with_evaluator(eval)
            .with_settle_config(SettleConfig::default());

        let mut k = k;
        let t50 = SimulationTime::from_nanoseconds(50);
        let report = k.run_until(t50);
        assert!(report.settle_reports.is_empty());
        assert_eq!(report.time_reached, t50);
    }

    #[test]
    fn kernel_run_report_display_with_settling() {
        let report = KernelRunReport {
            time_reached: SimulationTime::from_nanoseconds(50),
            events_processed: vec![DigitalEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(0),
                LogicValue::One,
            )],
            next_event_time: None,
            settle_reports: vec![TimePointSettleReport {
                time: SimulationTime::from_nanoseconds(50),
                outcome: SettleOutcome::Settled { delta_cycles: 2 },
            }],
        };
        let s = format!("{report}");
        assert!(s.contains("1 settled"));
        assert!(s.contains("0 oscillating"));
        assert!(s.contains("2 total delta cycles"));
    }

    #[test]
    fn kernel_run_report_display_with_oscillation() {
        let report = KernelRunReport {
            time_reached: SimulationTime::from_nanoseconds(50),
            events_processed: vec![],
            next_event_time: None,
            settle_reports: vec![
                TimePointSettleReport {
                    time: SimulationTime::from_nanoseconds(50),
                    outcome: SettleOutcome::Settled { delta_cycles: 1 },
                },
                TimePointSettleReport {
                    time: SimulationTime::from_nanoseconds(50),
                    outcome: SettleOutcome::Oscillating {
                        delta_cycles: 3,
                        oscillating_nets: vec![NetId::new(0)],
                    },
                },
            ],
        };
        let s = format!("{report}");
        assert!(s.contains("1 settled"));
        assert!(s.contains("1 oscillating"));
        assert!(report.has_oscillation());
    }

    #[test]
    fn has_evaluator_reflects_state() {
        let k = DigitalKernel::new();
        assert!(!k.has_evaluator());

        let k = DigitalKernel::new().with_evaluator(FnEvaluator::new(|_, _| vec![]));
        assert!(k.has_evaluator());
    }

    #[test]
    fn settle_config_accessible() {
        let k = DigitalKernel::new();
        assert_eq!(k.settle_config().max_delta_cycles, 100);

        let k = DigitalKernel::new().with_settle_config(SettleConfig::with_max_delta_cycles(50));
        assert_eq!(k.settle_config().max_delta_cycles, 50);
    }

    #[test]
    fn kernel_debug_includes_has_evaluator() {
        let k = DigitalKernel::new();
        let debug = format!("{k:?}");
        assert!(debug.contains("has_evaluator: false"));

        let k = DigitalKernel::new().with_evaluator(FnEvaluator::new(|_, _| vec![]));
        let debug = format!("{k:?}");
        assert!(debug.contains("has_evaluator: true"));
    }
}
