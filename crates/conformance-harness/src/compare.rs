//! Comparator: produce a [`ConformanceReport`] from an actual series
//! set against a [`crate::GoldenReference`] under a chosen
//! [`crate::Tolerance`].
//!
//! # Comparison shape
//!
//! Per ADR-0008 the conformance criterion is *per-node, per-sample*.
//! Pseudocode:
//!
//! ```text
//! for var in golden.variables:
//!     actual = lookup actual series with same name
//!     for p in 0 .. n_points:
//!         pass = tol.passes(golden.values[p], actual.values[p])
//!         track worst margin
//!     emit VariableSummary { name, n_failures, worst_margin, worst_point }
//! verdict = Pass iff every variable has zero failures and every
//!           required-variable lookup succeeded; else Fail.
//! ```
//!
//! # Required vs optional variables
//!
//! The caller supplies the actual series via [`compare`]'s
//! `actual_by_name` argument. If a golden variable is missing from the
//! actual map, that variable's summary reports `MISSING_FROM_ACTUAL`
//! as its failure mode and the overall verdict is `Fail`. Extra
//! actuals not in the golden are *ignored* — the harness is a one-way
//! "is the solver at least as right as ngspice" check, not a
//! bidirectional set-equality.
//!
//! # Why this returns a report, not a bool
//!
//! ADR-0008 §"Positive consequences" lists "Per-node checking means a
//! single outlier node does not cause a global failure; the
//! conformance report can list the worst-case nodes and their
//! margins." The report-shaped return is load-bearing for the per-
//! analysis tests in #63–#68 to print actionable diagnostics; a
//! bare `bool` would force them to re-implement margin tracking.

use crate::golden::GoldenReference;
use crate::tolerance::Tolerance;

/// Outcome of comparing one variable.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableSummary {
    /// Verbatim variable name from the golden file (e.g., `v(out)`).
    pub name: String,
    /// Number of golden sample points compared. Zero if the variable
    /// was missing from the actual map.
    pub n_points: usize,
    /// Number of points that failed the tolerance check.
    pub n_failures: usize,
    /// Worst (most-negative) margin observed across all points,
    /// using [`Tolerance::margin`]'s sign convention. `f64::INFINITY`
    /// if there were no points (variable missing). `>= 0` means the
    /// variable passed; `< 0` means it failed and `-worst_margin` is
    /// how far the worst sample landed outside the envelope.
    pub worst_margin: f64,
    /// 0-based sweep index where `worst_margin` was observed.
    /// `usize::MAX` if there were no points.
    pub worst_point: usize,
    /// The first up-to-`max_failures_per_variable` failed points, for
    /// diagnostic reporting. Empty if `n_failures == 0` or the
    /// variable was missing.
    pub failures: Vec<PointFailure>,
    /// Set if the variable was declared by the golden but not
    /// supplied by the caller. When `true`, the other fields except
    /// `name` carry their "no data" sentinels.
    pub missing_from_actual: bool,
}

/// One per-sample failure record.
#[derive(Debug, Clone, PartialEq)]
pub struct PointFailure {
    /// 0-based sweep index.
    pub point: usize,
    /// Sweep-axis value at this point (time in s, frequency in Hz, …).
    pub sweep_value: f64,
    /// Golden reference value at this point.
    pub reference: f64,
    /// Actual (solver) value at this point.
    pub actual: f64,
    /// Signed margin (negative = outside envelope by this much).
    pub margin: f64,
}

/// Pass/fail verdict for a [`ConformanceReport`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceVerdict {
    /// Every variable in the golden was matched by the actual series
    /// and every per-point check passed.
    Pass,
    /// At least one variable was missing from the actual, or at least
    /// one per-point check failed.
    Fail,
}

/// The full report from one [`compare`] call.
///
/// `verdict == Pass` iff `n_failed_variables == 0`. The
/// [`VariableSummary`] list is in the order the golden declared the
/// variables — stable across runs for downstream diagnostic formatting.
#[derive(Debug, Clone, PartialEq)]
pub struct ConformanceReport {
    /// Overall pass/fail.
    pub verdict: ConformanceVerdict,
    /// The tolerance used.
    pub tolerance: Tolerance,
    /// Total variables declared by the golden.
    pub n_variables: usize,
    /// Variables with at least one failure or marked missing.
    pub n_failed_variables: usize,
    /// Per-variable summary, in golden's declaration order.
    pub variables: Vec<VariableSummary>,
    /// Cross-variable worst margin (most negative). `f64::INFINITY`
    /// if the golden declared no variables.
    pub worst_margin: f64,
    /// The variable name that produced `worst_margin`. Empty string
    /// if no variables.
    pub worst_variable: String,
}

impl ConformanceReport {
    /// True iff `verdict == Pass`.
    #[must_use]
    pub fn is_pass(&self) -> bool {
        matches!(self.verdict, ConformanceVerdict::Pass)
    }
}

/// Compare an actual series set against a [`GoldenReference`] under
/// a [`Tolerance`].
///
/// # Arguments
///
/// - `golden`: the parsed reference (typically from
///   [`crate::load_ngspice_ascii`]).
/// - `actual_by_name`: an iterator of `(variable_name, values)`
///   pairs. The `values` slice must be parallel to
///   `golden.sweep_axis`. Variables not in this map will be reported
///   as `missing_from_actual`.
/// - `tolerance`: the per-analysis envelope. The per-analysis
///   default can be obtained from
///   [`crate::AnalysisKind::default_tolerance`].
/// - `max_failures_per_variable`: cap on how many [`PointFailure`]
///   records to retain per variable, for diagnostic memory bounding.
///   Pass `usize::MAX` to keep them all.
///
/// # Length mismatch
///
/// If an actual series' length differs from `golden.n_points()`,
/// that variable is reported as if every point failed: `n_failures
/// == n_points`, `worst_margin = f64::NEG_INFINITY`, and a single
/// synthetic [`PointFailure`] at point 0 with the mismatch flagged
/// via `reference = NaN, actual = NaN`. This keeps the report shape
/// well-formed without introducing a third "shape error" variant.
#[allow(clippy::too_many_lines)] // Four exhaustive cases (missing / shape
                                 // mismatch / matched-length / extra) in a single
                                 // pass — extracting them into helpers would mean
                                 // duplicating the worst-margin / per-variable
                                 // bookkeeping shared between the matched and
                                 // shape-mismatch paths.
pub fn compare<'a, I>(
    golden: &GoldenReference,
    actual_by_name: I,
    tolerance: Tolerance,
    max_failures_per_variable: usize,
) -> ConformanceReport
where
    I: IntoIterator<Item = (&'a str, &'a [f64])>,
{
    // Materialize the iterator into a vec for O(N) name lookup against
    // each golden variable. Callers passing many variables can build a
    // sorted slice for O(log N) lookup, but the typical SPICE testbench
    // size (≤ 200 vars) makes the linear scan fine.
    let actuals: Vec<(&str, &[f64])> = actual_by_name.into_iter().collect();

    let mut variables: Vec<VariableSummary> = Vec::with_capacity(golden.n_variables());
    let mut n_failed_variables = 0usize;
    let mut global_worst_margin = f64::INFINITY;
    let mut global_worst_variable = String::new();

    for var in &golden.variables {
        let actual_slice = actuals
            .iter()
            .find(|(name, _)| *name == var.name)
            .map(|(_, v)| *v);

        // Case 1: variable not supplied by caller.
        let Some(actual) = actual_slice else {
            variables.push(VariableSummary {
                name: var.name.clone(),
                n_points: 0,
                n_failures: 0,
                worst_margin: f64::NEG_INFINITY,
                worst_point: usize::MAX,
                failures: Vec::new(),
                missing_from_actual: true,
            });
            n_failed_variables += 1;
            if f64::NEG_INFINITY < global_worst_margin {
                global_worst_margin = f64::NEG_INFINITY;
                global_worst_variable.clone_from(&var.name);
            }
            continue;
        };

        // Case 2: shape mismatch — treat as total failure.
        if actual.len() != golden.n_points() {
            let mut failures = Vec::new();
            if max_failures_per_variable > 0 {
                failures.push(PointFailure {
                    point: 0,
                    sweep_value: golden.sweep_axis.first().copied().unwrap_or(f64::NAN),
                    reference: f64::NAN,
                    actual: f64::NAN,
                    margin: f64::NEG_INFINITY,
                });
            }
            variables.push(VariableSummary {
                name: var.name.clone(),
                n_points: golden.n_points(),
                n_failures: golden.n_points(),
                worst_margin: f64::NEG_INFINITY,
                worst_point: 0,
                failures,
                missing_from_actual: false,
            });
            n_failed_variables += 1;
            if f64::NEG_INFINITY < global_worst_margin {
                global_worst_margin = f64::NEG_INFINITY;
                global_worst_variable.clone_from(&var.name);
            }
            continue;
        }

        // Case 3: matched length — per-point comparison.
        let mut n_failures = 0usize;
        let mut worst_margin = f64::INFINITY;
        let mut worst_point = usize::MAX;
        let mut failures: Vec<PointFailure> = Vec::new();
        for (p, (&v_ref, &v_act)) in var.values.iter().zip(actual.iter()).enumerate() {
            let m = tolerance.margin(v_ref, v_act);
            if m < worst_margin {
                worst_margin = m;
                worst_point = p;
            }
            if !tolerance.passes(v_ref, v_act) {
                n_failures += 1;
                if failures.len() < max_failures_per_variable {
                    failures.push(PointFailure {
                        point: p,
                        sweep_value: golden.sweep_axis[p],
                        reference: v_ref,
                        actual: v_act,
                        margin: m,
                    });
                }
            }
        }

        let var_failed = n_failures > 0;
        if var_failed {
            n_failed_variables += 1;
        }
        if worst_margin < global_worst_margin {
            global_worst_margin = worst_margin;
            global_worst_variable.clone_from(&var.name);
        }
        variables.push(VariableSummary {
            name: var.name.clone(),
            n_points: var.values.len(),
            n_failures,
            worst_margin,
            worst_point,
            failures,
            missing_from_actual: false,
        });
    }

    let verdict = if n_failed_variables == 0 {
        ConformanceVerdict::Pass
    } else {
        ConformanceVerdict::Fail
    };
    ConformanceReport {
        verdict,
        tolerance,
        n_variables: golden.n_variables(),
        n_failed_variables,
        variables,
        worst_margin: global_worst_margin,
        worst_variable: global_worst_variable,
    }
}

#[cfg(test)]
// Same rationale as the tolerance::tests allow: every float comparison
// in this module's tests is against a sentinel (f64::NEG_INFINITY) or an
// integer-representable threshold built into ADR-0008 — exactness is
// the assertion, not an artifact.
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::golden::{GoldenReference, GoldenVariable, SweepKind};
    use crate::tolerance::AnalysisKind;

    /// Build a 3-point transient golden with two variables.
    fn golden_transient() -> GoldenReference {
        let mut g = GoldenReference::new(
            "test".into(),
            SweepKind::Transient,
            "time".into(),
            "time".into(),
        );
        g.sweep_axis = vec![0.0, 1e-9, 2e-9];
        g.push_variable(GoldenVariable {
            name: "v(n1)".into(),
            kind: "voltage".into(),
            values: vec![0.0, 3.3, 3.3],
        })
        .unwrap();
        g.push_variable(GoldenVariable {
            name: "v(out)".into(),
            kind: "voltage".into(),
            values: vec![0.0, 1.65, 3.3],
        })
        .unwrap();
        g
    }

    // ---------- Pass / fail verdicts ----------

    #[test]
    fn exact_match_passes_with_zero_margin_at_worst_point() {
        let g = golden_transient();
        let actual_n1: Vec<f64> = vec![0.0, 3.3, 3.3];
        let actual_out: Vec<f64> = vec![0.0, 1.65, 3.3];
        let report = compare(
            &g,
            [
                ("v(n1)", actual_n1.as_slice()),
                ("v(out)", actual_out.as_slice()),
            ],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert_eq!(report.verdict, ConformanceVerdict::Pass);
        assert_eq!(report.n_failed_variables, 0);
        // Worst margin is the smallest *positive* margin we saw —
        // determined by the smallest envelope (at v_ref=0 the envelope
        // is 1 mV; the actual is also 0 so diff=0, margin=1e-3).
        assert!(report.worst_margin >= 0.0);
    }

    #[test]
    fn within_envelope_passes() {
        let g = golden_transient();
        // Slight perturbation well inside 1% / 1 mV at 3.3 V (≈ 33 mV
        // envelope).
        let actual_n1: Vec<f64> = vec![0.0, 3.310, 3.305];
        let actual_out: Vec<f64> = vec![0.0001, 1.660, 3.295];
        let report = compare(
            &g,
            [
                ("v(n1)", actual_n1.as_slice()),
                ("v(out)", actual_out.as_slice()),
            ],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert_eq!(report.verdict, ConformanceVerdict::Pass);
    }

    #[test]
    fn one_bad_sample_flips_verdict_to_fail() {
        let g = golden_transient();
        // v(n1) sample 1: reference 3.3 V, actual 3.5 V → diff = 200 mV,
        // envelope = max(0.033, 0.001) = 33 mV → fails by 167 mV.
        let actual_n1: Vec<f64> = vec![0.0, 3.5, 3.3];
        let actual_out: Vec<f64> = vec![0.0, 1.65, 3.3];
        let report = compare(
            &g,
            [
                ("v(n1)", actual_n1.as_slice()),
                ("v(out)", actual_out.as_slice()),
            ],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert_eq!(report.verdict, ConformanceVerdict::Fail);
        assert_eq!(report.n_failed_variables, 1);
        assert_eq!(report.worst_variable, "v(n1)");
        // worst_margin < 0
        assert!(report.worst_margin < 0.0);
        let v_n1 = &report.variables[0];
        assert_eq!(v_n1.n_failures, 1);
        assert_eq!(v_n1.worst_point, 1);
        assert_eq!(v_n1.failures.len(), 1);
        assert_eq!(v_n1.failures[0].point, 1);
        assert!((v_n1.failures[0].reference - 3.3).abs() < 1e-12);
        assert!((v_n1.failures[0].actual - 3.5).abs() < 1e-12);
    }

    // ---------- Missing-from-actual ----------

    #[test]
    fn missing_actual_variable_marks_failure() {
        let g = golden_transient();
        let actual_n1: Vec<f64> = vec![0.0, 3.3, 3.3];
        // No v(out) supplied.
        let report = compare(
            &g,
            [("v(n1)", actual_n1.as_slice())],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert_eq!(report.verdict, ConformanceVerdict::Fail);
        let v_out = report
            .variables
            .iter()
            .find(|s| s.name == "v(out)")
            .unwrap();
        assert!(v_out.missing_from_actual);
        assert_eq!(v_out.n_failures, 0); // no per-point work happened
        assert_eq!(v_out.n_points, 0);
    }

    // ---------- Shape mismatch ----------

    #[test]
    fn length_mismatch_treated_as_total_failure() {
        let g = golden_transient();
        let actual_n1: Vec<f64> = vec![0.0, 3.3]; // too short
        let actual_out: Vec<f64> = vec![0.0, 1.65, 3.3];
        let report = compare(
            &g,
            [
                ("v(n1)", actual_n1.as_slice()),
                ("v(out)", actual_out.as_slice()),
            ],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert_eq!(report.verdict, ConformanceVerdict::Fail);
        let v_n1 = &report.variables[0];
        assert_eq!(v_n1.n_failures, v_n1.n_points);
        assert_eq!(v_n1.worst_margin, f64::NEG_INFINITY);
    }

    // ---------- Extra actuals are ignored ----------

    #[test]
    fn extra_actual_variables_are_ignored() {
        let g = golden_transient();
        let actual_n1: Vec<f64> = vec![0.0, 3.3, 3.3];
        let actual_out: Vec<f64> = vec![0.0, 1.65, 3.3];
        let actual_extra: Vec<f64> = vec![999.0, 999.0, 999.0];
        let report = compare(
            &g,
            [
                ("v(n1)", actual_n1.as_slice()),
                ("v(out)", actual_out.as_slice()),
                ("v(noise_floor_probe)", actual_extra.as_slice()),
            ],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert_eq!(report.verdict, ConformanceVerdict::Pass);
        // The extra is not in the report.
        assert!(report
            .variables
            .iter()
            .all(|s| s.name != "v(noise_floor_probe)"));
    }

    // ---------- Failure cap ----------

    #[test]
    fn max_failures_per_variable_caps_diagnostics() {
        let g = golden_transient();
        // Both deps wildly off.
        let actual_n1: Vec<f64> = vec![10.0, 10.0, 10.0];
        let actual_out: Vec<f64> = vec![10.0, 10.0, 10.0];
        let report = compare(
            &g,
            [
                ("v(n1)", actual_n1.as_slice()),
                ("v(out)", actual_out.as_slice()),
            ],
            AnalysisKind::Transient.default_tolerance(),
            1, // keep only the first failure per variable
        );
        assert_eq!(report.verdict, ConformanceVerdict::Fail);
        for v in &report.variables {
            // Every variable should report n_failures equal to all 3 points,
            // but the diagnostic vec should be capped at 1.
            assert_eq!(v.n_failures, 3);
            assert_eq!(v.failures.len(), 1);
        }
    }

    // ---------- NaN handling ----------

    #[test]
    fn nan_in_actual_surfaces_as_failure() {
        let g = golden_transient();
        let actual_n1: Vec<f64> = vec![0.0, f64::NAN, 3.3];
        let actual_out: Vec<f64> = vec![0.0, 1.65, 3.3];
        let report = compare(
            &g,
            [
                ("v(n1)", actual_n1.as_slice()),
                ("v(out)", actual_out.as_slice()),
            ],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert_eq!(report.verdict, ConformanceVerdict::Fail);
        let v_n1 = &report.variables[0];
        assert!(v_n1.n_failures >= 1);
        // The NaN point should be the worst (margin = -infinity).
        assert_eq!(v_n1.worst_margin, f64::NEG_INFINITY);
    }

    // ---------- Worst-case bookkeeping ----------

    #[test]
    fn worst_variable_identifies_max_violation_node() {
        let g = golden_transient();
        // v(n1) sample 1 off by 100 mV (small violation),
        // v(out) sample 2 off by 1 V (huge violation).
        let actual_n1: Vec<f64> = vec![0.0, 3.4, 3.3];
        let actual_out: Vec<f64> = vec![0.0, 1.65, 4.3];
        let report = compare(
            &g,
            [
                ("v(n1)", actual_n1.as_slice()),
                ("v(out)", actual_out.as_slice()),
            ],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert_eq!(report.verdict, ConformanceVerdict::Fail);
        assert_eq!(report.worst_variable, "v(out)");
        // v(out) at p=2 violated by ≈ 1 V − 33 mV = 967 mV; margin
        // should be roughly that, negative.
        assert!(report.worst_margin < -0.9);
    }

    #[test]
    fn is_pass_helper_agrees_with_verdict() {
        let g = golden_transient();
        let n1: Vec<f64> = vec![0.0, 3.3, 3.3];
        let o: Vec<f64> = vec![0.0, 1.65, 3.3];
        let r = compare(
            &g,
            [("v(n1)", n1.as_slice()), ("v(out)", o.as_slice())],
            AnalysisKind::Transient.default_tolerance(),
            16,
        );
        assert!(r.is_pass());
    }
}
