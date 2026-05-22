//! Result envelopes for mixed-signal analyses.
//!
//! The `mixed-signal-cosim` capability spec requires the unified Result
//! to contain *both* analog `Waveform`s and digital event traces in
//! VCD format, time-synchronized at every cycle boundary. This module
//! models the minimum surface to satisfy the
//! `optimistic-advance-with-correct-prediction` scenario; sibling
//! scenarios (mis-prediction rollback, VCD-trace shape, conformance)
//! will extend this same envelope.
//!
//! The types here are intentionally serialization-friendly (`Clone +
//! PartialEq + Debug`) and decoupled from any solver internals — they
//! are the contract between the Mixed-Signal Scheduler and the
//! application frontend.

use crate::{NodeId, SignalName, SimulationTime};

/// A time-indexed waveform for a single analog node.
///
/// Per the inlined Glossary, a Waveform is "a time-domain voltage or
/// current signal." This struct keeps the time axis as a sorted vector
/// of `SimulationTime` values and the sample axis as a parallel
/// vector of `f64` voltages (in volts). Invariant: `times.len() ==
/// values.len()`, and `times` is monotonically non-decreasing.
#[derive(Debug, Clone, PartialEq)]
pub struct Waveform {
    /// The node whose voltage this waveform records.
    pub node: NodeId,
    /// Monotonically non-decreasing sample times.
    pub times: Vec<SimulationTime>,
    /// Parallel sample values (volts).
    pub values: Vec<f64>,
}

impl Waveform {
    /// Construct a new waveform for a node.
    ///
    /// # Panics
    ///
    /// Panics if `times.len() != values.len()`. The two vectors must be
    /// parallel sample axes.
    #[must_use]
    pub fn new(node: NodeId, times: Vec<SimulationTime>, values: Vec<f64>) -> Self {
        assert_eq!(
            times.len(),
            values.len(),
            "Waveform: times and values must be the same length"
        );
        Self {
            node,
            times,
            values,
        }
    }

    /// Number of samples.
    #[must_use]
    pub fn len(&self) -> usize {
        self.times.len()
    }

    /// True iff the waveform has no samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.times.is_empty()
    }

    /// The last time the waveform was sampled, or `None` if empty.
    #[must_use]
    pub fn last_time(&self) -> Option<SimulationTime> {
        self.times.last().copied()
    }
}

/// The full analog-side trace for a mixed-signal analysis: a Waveform
/// per observed node.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalogTrace {
    /// Waveforms keyed by observed node, in deterministic order.
    pub waveforms: Vec<Waveform>,
    /// The final commit time of the analog solver, i.e., the most
    /// recent synchronization point at which the analog state was
    /// committed (no rollback pending past this point).
    pub committed_through: SimulationTime,
}

impl AnalogTrace {
    /// Look up a waveform by node.
    #[must_use]
    pub fn waveform_for(&self, node: NodeId) -> Option<&Waveform> {
        self.waveforms.iter().find(|w| w.node == node)
    }
}

/// A digital event-trace in VCD (Value Change Dump) format.
///
/// VCD is the canonical interchange format named by the spec ("the
/// Result contains analog Waveforms and digital event traces in VCD
/// format"). We carry the trace as text rather than parsed events at
/// this layer; downstream consumers (conformance harness, VCD readers)
/// parse it back. This keeps the Result envelope round-trippable
/// against tools like `gtkwave` or `verilator --trace` consumers.
///
/// The `events_by_signal` index is a *summary* of the same data the
/// VCD text encodes, populated by the Icarus or Verilator adapter as
/// it relays events to the scheduler. The scenario's "And the Result
/// contains [...] digital event traces synchronized at 50 ns"
/// acceptance condition is checked against this summary so tests do
/// not have to parse VCD.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DigitalEventTrace {
    /// Raw VCD text, suitable for handing to a standard VCD reader.
    pub vcd: String,
    /// Per-signal event times, in scheduler time. Populated alongside
    /// `vcd` by the digital adapter.
    pub events_by_signal: Vec<(SignalName, Vec<SimulationTime>)>,
}

impl DigitalEventTrace {
    /// Look up the event times recorded for a named signal.
    #[must_use]
    pub fn events_for(&self, signal: &SignalName) -> Option<&[SimulationTime]> {
        self.events_by_signal
            .iter()
            .find(|(name, _)| name == signal)
            .map(|(_, times)| times.as_slice())
    }

    /// Total number of recorded events across all signals.
    #[must_use]
    pub fn total_events(&self) -> usize {
        self.events_by_signal.iter().map(|(_, ts)| ts.len()).sum()
    }
}

/// A rollback diagnostic record. The
/// `optimistic-advance-with-correct-prediction` scenario asserts
/// `no rollback occurs`, so a passing run produces an empty rollback
/// list. Sibling scenarios populate this with `RollbackEvent` entries.
#[derive(Debug, Clone, PartialEq)]
pub struct RollbackEvent {
    /// The predicted boundary time that turned out to be wrong.
    pub mispredicted_at: SimulationTime,
    /// The corrected event time the digital simulator actually
    /// reported.
    pub corrected_to: SimulationTime,
    /// The checkpoint time the analog solver rolled back to.
    pub checkpoint_at: SimulationTime,
    /// Human-readable reason (e.g., `"contract-violation"`,
    /// `"no-event-confirmed"`).
    pub reason: String,
}

/// Scheduler-attached metadata on a `MixedSignalResult`. Acts as the
/// audit trail for analog/digital interaction.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SchedulerMetadata {
    /// Synchronization points the scheduler committed at, in order.
    pub commits: Vec<SimulationTime>,
    /// Rollback events recorded during the run. Empty on
    /// correct-prediction runs.
    pub rollbacks: Vec<RollbackEvent>,
    /// Diagnostic log lines emitted by the scheduler (e.g.,
    /// "contract-violation" warnings). Plain strings for now;
    /// structured tracing is sibling-task work.
    pub diagnostics: Vec<String>,
}

impl SchedulerMetadata {
    /// True iff no rollback events were recorded.
    #[must_use]
    pub fn rollback_free(&self) -> bool {
        self.rollbacks.is_empty()
    }
}

/// The unified Result of a mixed-signal analysis.
///
/// Per the spec's acceptance criteria, this Result must contain both
/// analog Waveforms and digital event traces in VCD format. It is the
/// canonical handoff from the Mixed-Signal Scheduler to the
/// application frontend.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MixedSignalResult {
    /// Analog-side time-series.
    pub analog: AnalogTrace,
    /// Digital-side event trace in VCD format.
    pub digital: DigitalEventTrace,
    /// Scheduler-attached audit trail.
    pub scheduler: SchedulerMetadata,
}

impl MixedSignalResult {
    /// Convenience: did the scheduler complete without any rollbacks?
    #[must_use]
    pub fn rollback_free(&self) -> bool {
        self.scheduler.rollback_free()
    }

    /// Convenience: the final synchronization point committed by the
    /// scheduler.
    #[must_use]
    pub fn final_commit(&self) -> Option<SimulationTime> {
        self.scheduler.commits.last().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waveform_new_enforces_parallel_lengths() {
        let w = Waveform::new(
            NodeId::new(1),
            vec![SimulationTime::ZERO, SimulationTime::from_nanoseconds(50)],
            vec![0.0, 3.3],
        );
        assert_eq!(w.len(), 2);
        assert_eq!(w.last_time(), Some(SimulationTime::from_nanoseconds(50)));
    }

    #[test]
    #[should_panic(expected = "same length")]
    fn waveform_new_panics_on_mismatched_lengths() {
        let _ = Waveform::new(NodeId::new(1), vec![SimulationTime::ZERO], vec![]);
    }

    #[test]
    fn digital_trace_indexes_by_signal() {
        let trace = DigitalEventTrace {
            vcd: String::new(),
            events_by_signal: vec![(
                SignalName::new("din"),
                vec![SimulationTime::from_nanoseconds(50)],
            )],
        };
        assert_eq!(trace.total_events(), 1);
        assert_eq!(
            trace.events_for(&SignalName::new("din")),
            Some(&[SimulationTime::from_nanoseconds(50)][..])
        );
    }

    #[test]
    fn scheduler_metadata_rollback_free_when_empty() {
        let meta = SchedulerMetadata::default();
        assert!(meta.rollback_free());
    }

    #[test]
    fn scheduler_metadata_not_rollback_free_with_event() {
        let mut meta = SchedulerMetadata::default();
        meta.rollbacks.push(RollbackEvent {
            mispredicted_at: SimulationTime::from_nanoseconds(100),
            corrected_to: SimulationTime::from_nanoseconds(80),
            checkpoint_at: SimulationTime::from_nanoseconds(50),
            reason: "no-event-confirmed".into(),
        });
        assert!(!meta.rollback_free());
    }
}
