//! Newton-Raphson convergence status.
//!
//! Per [ADR-0006](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0006-dual-convergence-criterion-newton-raphson.md),
//! the solver uses a **dual convergence criterion**: convergence is
//! declared only when *both* the update norm ‖Δx‖ and the residue norm
//! ‖F(x)‖ fall below their respective tolerances. A single-criterion
//! check (update-only or residue-only) is rejected because:
//!
//! - update-only fails to detect *stall* (small Δx, large residue);
//! - residue-only fails to detect *oscillation* (small residue, large
//!   Δx between bouncing iterates).
//!
//! The ADR mandates that the `NewtonRaphsonDriver` "compute and report
//! both norms in its `ConvergenceStatus` return value, so that
//! diagnostics can distinguish 'update converged but residue did not'
//! from 'residue converged but update did not.'" This module encodes
//! that contract as a typed enum.
//!
//! The status is consumed by the analysis-orchestration layer, which
//! decides what to surface in the user-visible `AnalysisResult` (per
//! the `dc-operating-point-convergence-failure` scenario in
//! `specs/dc-operating-point/spec.md`).

/// The pair of tolerances configured for a Newton-Raphson run.
///
/// Per ADR-0006:
/// - `update_tol` (commonly named `reltol` in SPICE-family tools)
///   bounds ‖Δx‖.
/// - `residue_tol` (commonly named `abstol`) bounds ‖F(x)‖.
///
/// Both must be positive and finite. The solver does not enforce these
/// invariants on construction; the analysis-orchestration layer
/// validates the user-facing `AnalysisRequest` before handing
/// tolerances down.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConvergenceTolerances {
    /// Tolerance against which the update norm ‖Δx‖ is compared.
    pub update_tol: f64,
    /// Tolerance against which the residue norm ‖F(x)‖ is compared.
    pub residue_tol: f64,
}

impl ConvergenceTolerances {
    /// SPICE-conventional defaults: `reltol = 1e-3`, `abstol = 1e-12`.
    ///
    /// These match the long-standing ngspice defaults and are the
    /// starting point for the golden-reference conformance work in
    /// ADR-0008. The numeric-solver may override per request.
    pub const SPICE_DEFAULTS: Self = Self {
        update_tol: 1e-3,
        residue_tol: 1e-12,
    };

    /// Construct an explicit tolerance pair.
    #[must_use]
    pub const fn new(update_tol: f64, residue_tol: f64) -> Self {
        Self {
            update_tol,
            residue_tol,
        }
    }
}

impl Default for ConvergenceTolerances {
    fn default() -> Self {
        Self::SPICE_DEFAULTS
    }
}

/// Diagnostic norms reported at the final Newton-Raphson iteration.
///
/// These fields are *always* populated by the driver — both on success
/// and on failure — so callers can render a uniform diagnostic. The
/// `iterations` field counts how many full NR iterations were
/// performed before reaching this status.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConvergenceDiagnostic {
    /// ‖Δx‖ at the final iteration.
    pub update_norm: f64,
    /// ‖F(x)‖ at the final iteration.
    pub residue_norm: f64,
    /// Number of NR iterations performed.
    pub iterations: u32,
    /// Tolerances that were in effect for this run.
    pub tolerances: ConvergenceTolerances,
}

impl ConvergenceDiagnostic {
    /// True iff the *update* component satisfied its tolerance, regardless
    /// of the residue component. Useful for distinguishing ADR-0006's two
    /// asymmetric failure modes.
    #[must_use]
    pub fn update_satisfied(&self) -> bool {
        self.update_norm.is_finite() && self.update_norm < self.tolerances.update_tol
    }

    /// True iff the *residue* component satisfied its tolerance, regardless
    /// of the update component.
    #[must_use]
    pub fn residue_satisfied(&self) -> bool {
        self.residue_norm.is_finite() && self.residue_norm < self.tolerances.residue_tol
    }

    /// True iff *both* criteria are satisfied. This is the ADR-0006
    /// definition of convergence.
    #[must_use]
    pub fn dual_satisfied(&self) -> bool {
        self.update_satisfied() && self.residue_satisfied()
    }
}

/// The outcome of a Newton-Raphson solve or, at the orchestration
/// layer, a homotopy-augmented DC analysis.
///
/// `ConvergenceStatus` is the typed return value of `NewtonRaphsonDriver`
/// per ADR-0006 *and* the user-visible convergence verdict produced by
/// the DC analysis control loop (`tasks.md` items #20 and #22). The
/// four original variants — `Converged`, `MaxIterationsExceeded`,
/// `Diverged`, `Stalled` — are the *Newton-Raphson contract* and are
/// the only variants the NR driver itself emits. The two additional
/// variants — `ConvergedViaHomotopy` and `Failed` — are emitted *only*
/// by the orchestration layer after composing NR with a homotopy
/// fallback (Gmin-stepping for v1):
///
/// - [`Self::ConvergedViaHomotopy`] — NR failed on the original system
///   but the Gmin-stepping continuation walked all the way to
///   `g_min = 0` successfully (spec scenario
///   `dc-operating-point#dc-operating-point-with-gmin-stepping-homotopy`).
///   This is the user-visible *"converged-via-homotopy"* status.
/// - [`Self::Failed`] — NR *and* every homotopy fallback failed; the
///   analysis has no accepted solution to return (spec scenario
///   `dc-operating-point#dc-operating-point-convergence-failure`).
///   The carried diagnostic is from the *last* attempted solve so the
///   caller can render a uniform failure report.
///
/// Every variant carries a [`ConvergenceDiagnostic`] so an analysis
/// can decide what to surface to the user without re-running anything.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConvergenceStatus {
    /// Both update and residue norms fell below tolerance. The current
    /// iterate is accepted as the solution.
    Converged(ConvergenceDiagnostic),
    /// The maximum iteration count was reached before both criteria
    /// were simultaneously satisfied. The `diagnostic` exposes which
    /// criterion (if either) was on the verge of satisfaction.
    MaxIterationsExceeded(ConvergenceDiagnostic),
    /// One or both norms grew without bound (e.g., overflow, NaN, or
    /// large positive growth across recent iterations). The diagnostic
    /// holds the last finite measurement.
    Diverged(ConvergenceDiagnostic),
    /// The classic ADR-0006 false-convergence mode: the update norm
    /// stopped shrinking while the residue norm remained above
    /// tolerance. A single-criterion check would have falsely declared
    /// convergence here. Returning `Stalled` instead of `Converged`
    /// is precisely the contribution of the dual criterion.
    Stalled(ConvergenceDiagnostic),
    /// **Orchestration-layer variant.** Plain Newton-Raphson failed on
    /// the original system but a homotopy continuation (Gmin-stepping
    /// in v1) walked through its full schedule and converged at the
    /// terminal step. The carried diagnostic is the inner NR
    /// diagnostic from the final un-augmented step; combined with the
    /// orchestration-layer step count this satisfies the spec phrasing
    /// *"And the homotopy step count is reported in the Result"*.
    ///
    /// Never emitted by `NewtonRaphsonDriver` directly; only the DC
    /// analysis control loop lifts a successful
    /// [`crate::ConvergenceDiagnostic`]-bearing homotopy outcome into
    /// this variant.
    ConvergedViaHomotopy(ConvergenceDiagnostic),
    /// **Orchestration-layer variant.** Newton-Raphson on the original
    /// system *and* every homotopy fallback failed. The DC analysis
    /// returns no accepted solution; the last-iterate node voltages
    /// are surfaced separately on the analysis result as diagnostic
    /// data (not as an `OperatingPoint`). The carried diagnostic is
    /// the *worst / last* failing NR diagnostic so the user can render
    /// a uniform message.
    ///
    /// Never emitted by `NewtonRaphsonDriver` directly; only the DC
    /// analysis control loop produces this verdict after exhausting
    /// every recovery path.
    Failed(ConvergenceDiagnostic),
}

impl ConvergenceStatus {
    /// True iff this status represents an accepted solution. Both the
    /// direct-NR success ([`Self::Converged`]) and the homotopy
    /// recovery success ([`Self::ConvergedViaHomotopy`]) count as
    /// converged — the orchestration layer is permitted to return an
    /// `OperatingPoint` in either case.
    #[must_use]
    pub fn is_converged(&self) -> bool {
        matches!(self, Self::Converged(_) | Self::ConvergedViaHomotopy(_))
    }

    /// True iff the solve failed for any reason. Includes every
    /// non-converged NR outcome *and* the orchestration-layer
    /// [`Self::Failed`] terminal verdict.
    #[must_use]
    pub fn is_failure(&self) -> bool {
        !self.is_converged()
    }

    /// True iff this is the spec's terminal *"failed"* verdict — NR
    /// and every homotopy fallback exhausted with no accepted
    /// solution. Distinct from the *transient* NR failure modes
    /// ([`Self::Stalled`], [`Self::Diverged`],
    /// [`Self::MaxIterationsExceeded`]) which the orchestration layer
    /// would normally hand off to a homotopy retry — those leak to
    /// the user only when the orchestrator deliberately surfaces them
    /// without attempting a fallback.
    #[must_use]
    pub fn is_terminal_failure(&self) -> bool {
        matches!(self, Self::Failed(_))
    }

    /// Borrow the underlying diagnostic regardless of variant.
    #[must_use]
    pub fn diagnostic(&self) -> &ConvergenceDiagnostic {
        match self {
            Self::Converged(d)
            | Self::MaxIterationsExceeded(d)
            | Self::Diverged(d)
            | Self::Stalled(d)
            | Self::ConvergedViaHomotopy(d)
            | Self::Failed(d) => d,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diag(update: f64, residue: f64) -> ConvergenceDiagnostic {
        ConvergenceDiagnostic {
            update_norm: update,
            residue_norm: residue,
            iterations: 7,
            tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
        }
    }

    #[test]
    fn spice_defaults_match_ngspice() {
        let t = ConvergenceTolerances::default();
        // Exact bit-pattern comparison: these are compile-time constants and
        // we want any change to a tolerance default to fail this test loudly.
        assert_eq!(t.update_tol.to_bits(), 1e-3_f64.to_bits());
        assert_eq!(t.residue_tol.to_bits(), 1e-12_f64.to_bits());
    }

    #[test]
    fn dual_criterion_requires_both_norms_below_tolerance() {
        // Update below, residue below — converged.
        let d = diag(1e-4, 1e-13);
        assert!(d.update_satisfied());
        assert!(d.residue_satisfied());
        assert!(d.dual_satisfied());
    }

    #[test]
    fn update_only_is_not_dual_satisfied() {
        // The ADR-0006 stall mode: small update, large residue.
        let d = diag(1e-4, 1e-3);
        assert!(d.update_satisfied());
        assert!(!d.residue_satisfied());
        assert!(!d.dual_satisfied());
    }

    #[test]
    fn residue_only_is_not_dual_satisfied() {
        // Small residue but bouncing iterate (oscillation).
        let d = diag(1.0, 1e-13);
        assert!(!d.update_satisfied());
        assert!(d.residue_satisfied());
        assert!(!d.dual_satisfied());
    }

    #[test]
    fn nan_is_never_satisfied() {
        let d = diag(f64::NAN, 1e-13);
        assert!(!d.update_satisfied());
        let d = diag(1e-4, f64::NAN);
        assert!(!d.residue_satisfied());
    }

    #[test]
    fn converged_and_converged_via_homotopy_are_the_only_successes() {
        let d = diag(1e-4, 1e-13);
        let s = ConvergenceStatus::Converged(d);
        assert!(s.is_converged());
        assert!(!s.is_failure());
        assert!(!s.is_terminal_failure());

        let s = ConvergenceStatus::Stalled(diag(1e-4, 1.0));
        assert!(!s.is_converged());
        assert!(s.is_failure());
        assert!(!s.is_terminal_failure());

        let s = ConvergenceStatus::MaxIterationsExceeded(d);
        assert!(!s.is_converged());
        assert!(!s.is_terminal_failure());

        let s = ConvergenceStatus::Diverged(diag(f64::INFINITY, f64::INFINITY));
        assert!(!s.is_converged());
        assert!(!s.is_terminal_failure());

        // Orchestration-layer success: homotopy walked through to a
        // converged terminal step. `is_converged` must accept this.
        let s = ConvergenceStatus::ConvergedViaHomotopy(d);
        assert!(s.is_converged());
        assert!(!s.is_failure());
        assert!(!s.is_terminal_failure());

        // Orchestration-layer terminal failure: NR + every homotopy
        // both failed.
        let s = ConvergenceStatus::Failed(diag(1e-2, 1e-1));
        assert!(!s.is_converged());
        assert!(s.is_failure());
        assert!(s.is_terminal_failure());
    }

    #[test]
    fn diagnostic_is_extractable_from_any_variant() {
        let d = diag(1.0, 2.0);
        for s in [
            ConvergenceStatus::Converged(d),
            ConvergenceStatus::MaxIterationsExceeded(d),
            ConvergenceStatus::Diverged(d),
            ConvergenceStatus::Stalled(d),
            ConvergenceStatus::ConvergedViaHomotopy(d),
            ConvergenceStatus::Failed(d),
        ] {
            assert_eq!(s.diagnostic().iterations, 7);
        }
    }

    #[test]
    fn terminal_failure_predicate_is_exclusive_to_failed_variant() {
        // Sanity: `is_terminal_failure` returns true *only* on the
        // `Failed` variant. The transient NR failure modes are still
        // failures (the orchestrator would normally retry them) but
        // not *terminal* failures.
        let d = diag(1e-2, 1e-1);
        assert!(ConvergenceStatus::Failed(d).is_terminal_failure());
        assert!(!ConvergenceStatus::Stalled(d).is_terminal_failure());
        assert!(!ConvergenceStatus::Diverged(d).is_terminal_failure());
        assert!(!ConvergenceStatus::MaxIterationsExceeded(d).is_terminal_failure());
        assert!(!ConvergenceStatus::Converged(d).is_terminal_failure());
        assert!(!ConvergenceStatus::ConvergedViaHomotopy(d).is_terminal_failure());
    }
}
