//! Transient-analysis Result envelope and adaptive-timestepping
//! metadata.
//!
//! This module realizes the Gherkin scenario
//! `transient-time-domain#adaptive-timestepping-rejects-and-re-solves`
//! terminal step:
//!
//! ```gherkin
//! And the final Result contains only accepted time points
//! And the timestep history is available in the Result metadata
//! ```
//!
//! It hosts:
//!
//! 1. [`StepOutcome`] — the closed-enum Accept/Reject discriminator
//!    that the adaptive controller produces and the metadata
//!    preserves verbatim.
//! 2. [`TimestepHistoryEntry`] — one log entry per attempted
//!    timestep (accepted *or* rejected), with diagnostic
//!    `worst_ratio` and optional `worst_index` per the tasks.md #35
//!    reviewer note on tasks.md #32.
//! 3. [`TimestepHistoryMetadata`] — the read-only Result-side
//!    handle that carries the entries plus convenience derivations
//!    (`accepted_times`, `counts`).
//! 4. [`TransientResult`] — the unified envelope returned by the
//!    transient analysis: Waveforms (only at accepted time points,
//!    per the scenario's penultimate Then) and the metadata block.
//!
//! # Why this lives in `circuit-solver-types`, not `numeric-solver`
//!
//! The numeric solver produces a `TimestepHistory` (and its sibling
//! `TimestepRecord`) as a pure-compute byproduct of the LTE
//! controller. The transient-analysis-frontend boundary, however,
//! must not pull the numeric solver into its types layer — the
//! frontend depends only on `circuit-solver-types`. So the
//! metadata exposed *to the user* lives here, and `numeric-solver`
//! provides a conversion (see `numeric_solver::TimestepHistory::into`
//! and the `From<&TimestepHistory>` impl in that crate) to lift its
//! internal type into this stable shape. The two types are
//! deliberately structurally similar so the conversion is a
//! field-by-field map, not a transformation.
//!
//! # ADR alignment
//!
//! - **ADR-0006** (Dual NR convergence) — vacuous (Result-shape
//!   only).
//! - **ADR-0007** (Zero-order-hold A/D boundary) — vacuous (no A/D
//!   surface).
//! - **ADR-0008** (max(rel, abs) tolerance envelope) — vacuous; the
//!   metadata reports `worst_ratio` which the controller already
//!   normalized against the envelope. The metadata itself is just
//!   carrying the controller's verdict.
//! - **ADR-0009** (Topology checker) — vacuous.
//! - **ADR-0010** (Unstable v1 API) — honored. All new public
//!   types are part of the unstable v1 surface; consumers must pin.

use crate::result::Waveform;

/// Whether an adaptive-timestepping attempt was accepted or
/// rejected by the LTE controller.
///
/// Mirrors `numeric_solver::StepOutcome` for the
/// Result-metadata side. Kept here so the `circuit-solver-types`
/// crate is dependency-free and the transient-analysis-frontend
/// boundary need not pull in the numeric solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepOutcome {
    /// LTE was within the tolerance envelope; the time point
    /// appears in the Result Waveforms.
    Accept,
    /// LTE exceeded the tolerance envelope; the time point does
    /// *not* appear in the Result Waveforms (per the scenario's
    /// "only accepted time points" Then), but the attempt is
    /// logged in the metadata for diagnostic value.
    Reject,
}

/// One log entry recording an attempted timestep — accepted or
/// rejected.
///
/// The transient analysis control loop (tasks.md #33) builds one
/// of these after every adaptive [`StepOutcome`] decision and
/// passes it up to the [`TimestepHistoryMetadata`] carried on the
/// [`TransientResult`].
///
/// # Fields
///
/// - `t_attempt` — the time at which the attempt was made, in
///   seconds. For an accepted step at `t_n + h`, this is `t_n + h`.
///   For a rejected attempt at the same time, it is also `t_n + h`
///   (rejected attempts are logged at the *would-be* time, not at
///   the shrunk re-solve time, so the log shows what was tried).
/// - `h_attempt` — the step size that was attempted, in seconds.
/// - `outcome` — whether the attempt was accepted or rejected.
/// - `worst_ratio` — the worst-case LTE / threshold ratio across
///   all observed nodes from this attempt. Values `<= 1.0`
///   correspond to `Accept`; `> 1.0` to `Reject`. Surfaced for
///   "which step came closest to rejection?" diagnostic plotting.
/// - `worst_node` — index of the observed node that produced
///   `worst_ratio`, if the producer recorded it. The transient
///   control loop populates this from
///   `numeric_solver::StepDecision::worst_index`; legacy /
///   hand-built entries may set `None`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimestepHistoryEntry {
    /// Attempted time `t_n + h` in seconds.
    pub t_attempt: f64,
    /// Step size attempted in seconds.
    pub h_attempt: f64,
    /// Accepted-or-rejected outcome from the LTE controller.
    pub outcome: StepOutcome,
    /// Worst-case LTE / threshold ratio across observed nodes.
    pub worst_ratio: f64,
    /// Index of the node that drove `worst_ratio`, if known.
    pub worst_node: Option<usize>,
}

/// The adaptive-timestepping audit trail attached to a
/// [`TransientResult`].
///
/// Realizes the Gherkin scenario's terminal Then "the timestep
/// history is available in the Result metadata." The metadata is
/// read-only from the user's perspective; constructors are
/// `pub`-fn entry points that the analysis-orchestration layer
/// (tasks.md #33) calls once at the end of the run.
///
/// # Invariants
///
/// The entries are stored in *attempt order*, not time-sorted.
/// A rejected attempt at `t = 1 ns` followed by a successful
/// re-solve at `t = 0.5 ns` appears in that order (reject first,
/// accept second), so the metadata preserves the controller's
/// trajectory.
///
/// `accepted_times()` filters out rejected entries and returns
/// the `t_attempt` of every accepted entry in attempt order; this
/// must match the time axis of every [`Waveform`] in the parent
/// [`TransientResult`]. The orchestration layer enforces that
/// alignment; the metadata itself doesn't double-check.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimestepHistoryMetadata {
    entries: Vec<TimestepHistoryEntry>,
}

impl TimestepHistoryMetadata {
    /// Construct an empty metadata block. Useful for non-adaptive
    /// runs (e.g. fixed-step Backward Euler) where the
    /// orchestrator may still emit a `TransientResult` carrying an
    /// empty history block.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Construct a metadata block from a sequence of entries.
    ///
    /// The orchestrator typically builds the entries up by
    /// converting from `numeric_solver::TimestepHistory` (see the
    /// `From` impl in that crate); this constructor exists for
    /// tests and for non-numeric-solver producers.
    #[must_use]
    pub fn from_entries(entries: Vec<TimestepHistoryEntry>) -> Self {
        Self { entries }
    }

    /// All entries in attempt order (accepted *and* rejected).
    #[must_use]
    pub fn entries(&self) -> &[TimestepHistoryEntry] {
        &self.entries
    }

    /// Number of recorded entries (accepted + rejected).
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True iff no entries are recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Counts of accepted vs. rejected entries.
    ///
    /// Returns `(accepted, rejected)`. Useful for diagnostic
    /// reporting like "adaptive controller accepted 412 / 437
    /// attempts".
    #[must_use]
    pub fn counts(&self) -> (usize, usize) {
        let accepted = self
            .entries
            .iter()
            .filter(|e| e.outcome == StepOutcome::Accept)
            .count();
        let rejected = self.entries.len() - accepted;
        (accepted, rejected)
    }

    /// The `t_attempt` values of *only* the accepted entries, in
    /// attempt order.
    ///
    /// By construction this is the time axis of the parent
    /// [`TransientResult`]'s [`Waveform`]s. Rejected attempts are
    /// excluded per the scenario's "only accepted time points"
    /// Then.
    #[must_use]
    pub fn accepted_times(&self) -> Vec<f64> {
        self.entries
            .iter()
            .filter(|e| e.outcome == StepOutcome::Accept)
            .map(|e| e.t_attempt)
            .collect()
    }

    /// True iff at least one rejection appears in the log.
    ///
    /// The adaptive controller is *expected* to reject sometimes
    /// (that is the whole point of the scenario); this predicate
    /// is for diagnostics, not for failure detection.
    #[must_use]
    pub fn had_rejection(&self) -> bool {
        self.entries
            .iter()
            .any(|e| e.outcome == StepOutcome::Reject)
    }
}

/// The unified Result envelope returned by a transient analysis.
///
/// Per the Glossary, `Result` is "the unified output structure for
/// any analysis." This struct is the transient-analysis specialization:
/// it carries Waveforms at every accepted time point, plus the
/// adaptive-timestepping metadata block.
///
/// # Acceptance scenario alignment
///
/// `transient-time-domain#adaptive-timestepping-rejects-and-re-solves`
/// terminal lines:
///
/// > And the final Result contains only accepted time points
/// > And the timestep history is available in the Result metadata
///
/// Both lines are satisfied structurally by this type:
///
/// - [`TransientResult::waveforms`] holds [`Waveform`]s whose `times`
///   axes were populated *only* on accepted steps (the orchestration
///   layer enforces this by deferring sample append until after
///   `StepOutcome::Accept`).
/// - [`TransientResult::timestep_history`] holds the
///   [`TimestepHistoryMetadata`] that records *both* accepted and
///   rejected attempts, so a downstream diagnostic / plotting layer
///   can show "the controller tried 1 ns, was rejected, retried at
///   0.5 ns and succeeded."
///
/// # ADR-0010 stability
///
/// Public, unstable at v1.0.0. Consumers must pin exact versions.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransientResult {
    /// One Waveform per observed analog node, sampled only at
    /// accepted time points (per the scenario's penultimate Then).
    pub waveforms: Vec<Waveform>,
    /// Adaptive-timestepping audit trail, available in the Result
    /// metadata (per the scenario's terminal Then).
    pub timestep_history: TimestepHistoryMetadata,
}

impl TransientResult {
    /// Construct a transient result from waveforms and history.
    ///
    /// Does **not** cross-validate that the waveform time axes
    /// match `timestep_history.accepted_times()`. That alignment
    /// is the orchestration layer's contract (tasks.md #33); this
    /// constructor is the cheap field-set used by both the
    /// orchestrator and tests.
    #[must_use]
    pub fn new(waveforms: Vec<Waveform>, timestep_history: TimestepHistoryMetadata) -> Self {
        Self {
            waveforms,
            timestep_history,
        }
    }

    /// Convenience: did the adaptive controller reject any
    /// attempts during the run?
    #[must_use]
    pub fn had_rejection(&self) -> bool {
        self.timestep_history.had_rejection()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NodeId, SimulationTime};

    fn entry(t: f64, h: f64, outcome: StepOutcome, ratio: f64) -> TimestepHistoryEntry {
        TimestepHistoryEntry {
            t_attempt: t,
            h_attempt: h,
            outcome,
            worst_ratio: ratio,
            worst_node: Some(0),
        }
    }

    #[test]
    fn empty_metadata_is_default() {
        let meta = TimestepHistoryMetadata::new();
        assert!(meta.is_empty());
        assert_eq!(meta.len(), 0);
        assert_eq!(meta.counts(), (0, 0));
        assert!(meta.accepted_times().is_empty());
        assert!(!meta.had_rejection());
    }

    #[test]
    fn from_entries_preserves_attempt_order() {
        // Mirrors the Gherkin scenario: 1 ns attempt rejected,
        // 0.5 ns re-solve accepted, 1.5 ns accepted.
        let entries = vec![
            entry(1.0e-9, 1.0e-9, StepOutcome::Reject, 5.0),
            entry(0.5e-9, 0.5e-9, StepOutcome::Accept, 0.2),
            entry(1.5e-9, 1.0e-9, StepOutcome::Accept, 0.5),
        ];
        let meta = TimestepHistoryMetadata::from_entries(entries);
        assert_eq!(meta.len(), 3);
        assert_eq!(meta.counts(), (2, 1));
        // Gherkin: "the final Result contains only accepted time
        // points" — the rejected 1.0 ns attempt must not appear in
        // the accepted-time axis.
        assert_eq!(meta.accepted_times(), vec![0.5e-9, 1.5e-9]);
        assert!(meta.had_rejection());
        // Gherkin: "the timestep history is available in the
        // Result metadata" — the rejected attempt is still in the
        // full entry log for diagnostics.
        assert_eq!(meta.entries()[0].outcome, StepOutcome::Reject);
    }

    #[test]
    fn counts_partitions_all_outcomes() {
        let entries = vec![
            entry(1.0e-9, 1.0e-9, StepOutcome::Accept, 0.1),
            entry(2.0e-9, 1.0e-9, StepOutcome::Reject, 2.5),
            entry(2.5e-9, 0.5e-9, StepOutcome::Accept, 0.3),
            entry(3.0e-9, 0.5e-9, StepOutcome::Reject, 1.7),
            entry(3.25e-9, 0.25e-9, StepOutcome::Accept, 0.4),
        ];
        let meta = TimestepHistoryMetadata::from_entries(entries);
        assert_eq!(meta.counts(), (3, 2));
        assert_eq!(meta.accepted_times().len(), 3);
    }

    #[test]
    fn transient_result_default_is_empty() {
        let r = TransientResult::default();
        assert!(r.waveforms.is_empty());
        assert!(r.timestep_history.is_empty());
        assert!(!r.had_rejection());
    }

    #[test]
    fn transient_result_carries_both_waveforms_and_metadata() {
        // Build a tiny 1-node waveform sampled at the accepted
        // times from the scenario.
        let waveform = Waveform::new(
            NodeId::new(1),
            vec![
                SimulationTime::from_picoseconds(500),
                SimulationTime::from_picoseconds(1500),
            ],
            vec![0.0, 3.3],
        );
        // History records the rejected 1 ns attempt plus both
        // accepted attempts.
        let meta = TimestepHistoryMetadata::from_entries(vec![
            entry(1.0e-9, 1.0e-9, StepOutcome::Reject, 5.0),
            entry(0.5e-9, 0.5e-9, StepOutcome::Accept, 0.2),
            entry(1.5e-9, 1.0e-9, StepOutcome::Accept, 0.5),
        ]);
        let result = TransientResult::new(vec![waveform], meta);

        // Scenario: only accepted time points appear in waveforms.
        assert_eq!(result.waveforms[0].times.len(), 2);
        // Scenario: timestep history is available in metadata.
        assert_eq!(result.timestep_history.len(), 3);
        // And the accepted-times axis of the metadata matches the
        // waveform's time axis (modulo unit conversion, which is
        // the orchestration layer's contract).
        assert_eq!(
            result.timestep_history.accepted_times(),
            vec![0.5e-9, 1.5e-9]
        );
        assert!(result.had_rejection());
    }

    #[test]
    fn entry_records_worst_node_when_known() {
        let mut e = entry(1.0e-9, 1.0e-9, StepOutcome::Reject, 7.0);
        e.worst_node = Some(3);
        assert_eq!(e.worst_node, Some(3));
        // None is also valid (legacy producers).
        e.worst_node = None;
        assert_eq!(e.worst_node, None);
    }
}
