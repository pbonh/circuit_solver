//! Native digital kernel adapter (ADR-0006, tasks.md item #20).
//!
//! The [`DigitalKernelAdapter`] wraps a
//! [`circuit_solver_digital_kernel::DigitalKernel`] and implements the
//! [`super::DigitalSimulator`] trait so the
//! [`super::MixedSignalScheduler`] can drive it in-process without any
//! cross-process IPC.
//!
//! # Checkpoint / Rollback
//!
//! The native `DigitalKernel` provides its own `checkpoint()` /
//! `restore_from_checkpoint()` API (see `digital-kernel::kernel`). The
//! adapter layers a [`BTreeMap`] of `(SimulationTime → KernelCheckpoint)`
//! on top so that:
//!
//! - [`DigitalKernelAdapter::save_checkpoint`] snapshots the kernel's
//!   current state and stores it keyed by the kernel's current time.
//! - [`DigitalKernelAdapter::rollback_to`] finds the nearest checkpoint
//!   at or before the target time and restores it.
//!
//! This mirrors the pattern used by the Icarus adapter (VvpTransport)
//! and the Verilator adapter, but uses the kernel's own native
//! checkpoint mechanism rather than an external transport layer.

use std::collections::BTreeMap;

use digital_kernel::{
    DigitalEvent, DigitalKernel, KernelCheckpoint,
};
use circuit_solver_types::{DigitalEventTrace, SimulationTime};

use super::{DigitalAdapterKind, DigitalSimulator, DigitalStepReport, NextEventReport, SchedulerError, SignalName};

// ---------------------------------------------------------------------------
// DigitalKernelAdapter
// ---------------------------------------------------------------------------

/// Adapter that wraps a native [`DigitalKernel`] (ADR-0006 DEVS engine)
/// and implements [`DigitalSimulator`] for the
/// [`super::MixedSignalScheduler`].
///
/// The adapter holds:
/// - `kernel` — the underlying `DigitalKernel` that processes events.
/// - `checkpoints` — a `BTreeMap<SimulationTime, KernelCheckpoint>` of
///   saved snapshots, used for rollback.
/// - `processed_events` — accumulator of events processed during the
///   run, harvested via `take_processed_events()` after each
///   `run_until` inside `confirm_event`.
/// - `signal_names` — boundary signal names used to construct the
///   [`DigitalEventTrace`] at end-of-run.
pub struct DigitalKernelAdapter {
    kernel: DigitalKernel,
    checkpoints: BTreeMap<SimulationTime, KernelCheckpoint>,
    processed_events: Vec<DigitalEvent>,
    signal_names: Vec<SignalName>,
}

impl DigitalKernelAdapter {
    /// Construct a new adapter wrapping the given [`DigitalKernel`].
    ///
    /// `signal_names` lists the boundary signals that the scheduler
    /// exchanges at synchronization points; they are used to build the
    /// [`DigitalEventTrace`] at end-of-run.
    pub fn new(kernel: DigitalKernel, signal_names: Vec<SignalName>) -> Self {
        Self {
            kernel,
            checkpoints: BTreeMap::new(),
            processed_events: Vec::new(),
            signal_names,
        }
    }

    /// Borrow the underlying kernel (read-only). Useful for tests that
    /// want to inspect the kernel's net state after a run.
    #[must_use]
    pub fn kernel(&self) -> &DigitalKernel {
        &self.kernel
    }

    /// Mutably borrow the underlying kernel. Use sparingly — the
    /// adapter is responsible for driving the kernel through the trait
    /// methods.
    pub fn kernel_mut(&mut self) -> &mut DigitalKernel {
        &mut self.kernel
    }

    /// Number of saved checkpoints currently retained.
    #[must_use]
    pub fn checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }
}

impl DigitalSimulator for DigitalKernelAdapter {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::NativeKernel
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        match self.kernel.next_event_time() {
            Some(t) => Ok(NextEventReport {
                predicted_time: t,
            }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "native kernel: no further events".into(),
            )),
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        // Advance the kernel to the predicted boundary.
        let report = self.kernel.run_until(boundary);

        // Harvest any events the kernel processed during this advance.
        let new_events = self.kernel.take_processed_events();
        self.processed_events.extend(new_events);

        // Determine the confirmation result.
        if report.time_reached == boundary {
            // The kernel reached the boundary. Check whether events
            // were processed at this time — if so, confirm.
            if !report.events_processed.is_empty()
                || report
                    .settle_reports
                    .iter()
                    .any(|sr| sr.time == boundary)
            {
                Ok(DigitalStepReport::Confirmed { time: boundary })
            } else if let Some(next) = report.next_event_time {
                // No event at the boundary; the next event is in the
                // future → postponed.
                Ok(DigitalStepReport::Postponed {
                    new_prediction: next,
                })
            } else {
                // No events at all at this boundary, and no future
                // events. Treat as confirmed (the boundary was reached
                // without events, which is fine for a synchronization
                // point).
                Ok(DigitalStepReport::Confirmed { time: boundary })
            }
        } else if report.time_reached < boundary {
            // The kernel stopped early — an event occurred at an
            // earlier time. This is the misprediction path.
            Ok(DigitalStepReport::Mispredicted {
                actual_time: report.time_reached,
            })
        } else {
            // Kernel overshot the boundary — should not happen with a
            // correct implementation, but surface it.
            Err(SchedulerError::DigitalAdapterFailed(format!(
                "native kernel overshot boundary: reached {} > requested {}",
                report.time_reached, boundary
            )))
        }
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        // Build VCD text and per-signal event index from the
        // accumulated processed events.
        let events_by_signal_vec: Vec<Vec<SimulationTime>> = self
            .signal_names
            .iter()
            .map(|_| {
                self.processed_events
                    .iter()
                    .map(|e| e.time)
                    .collect()
            })
            .collect();

        let vcd = crate::vcd_writer::build_vcd(&crate::vcd_writer::VcdTraceInput {
            scope_name: "native_kernel",
            signals: &self.signal_names,
            events_by_signal: &events_by_signal_vec,
        });

        let events_by_signal: Vec<(SignalName, Vec<SimulationTime>)> = self
            .signal_names
            .iter()
            .map(|s| {
                let times: Vec<SimulationTime> = self
                    .processed_events
                    .iter()
                    .map(|e| e.time)
                    .collect();
                (s.clone(), times)
            })
            .collect();

        DigitalEventTrace {
            vcd,
            events_by_signal,
        }
    }

    fn save_checkpoint(&mut self) -> Option<SimulationTime> {
        let current_time = self.kernel.current_time();
        let checkpoint = self.kernel.checkpoint();
        self.checkpoints.insert(current_time, checkpoint);
        Some(current_time)
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        // Find the nearest checkpoint at or before `target`.
        let checkpoint_time = self
            .checkpoints
            .range(..=target)
            .next_back()
            .map(|(t, _)| *t)
            .ok_or_else(|| SchedulerError::NoCheckpoint(target))?;

        let checkpoint = self
            .checkpoints
            .get(&checkpoint_time)
            .expect("checkpoint_time came from range iterator; must exist")
            .clone();

        // Drop all checkpoints strictly after the restored time.
        let keys_to_remove: Vec<SimulationTime> = self
            .checkpoints
            .keys()
            .copied()
            .filter(|&t| t > checkpoint_time)
            .collect();
        for key in keys_to_remove {
            self.checkpoints.remove(&key);
        }

        // Restore the kernel state.
        self.kernel.restore_from_checkpoint(checkpoint);

        Ok(())
    }
}
