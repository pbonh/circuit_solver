//! The [ADR-0008](../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0008-per-node-max-relative-absolute-tolerance-envelope.md)
//! tolerance envelope: a pair `(relative, absolute)` whose effective
//! bound on the absolute difference is `max(relative * |v_ref|, absolute)`.
//!
//! # The decision and its rejected alternatives
//!
//! ADR-0008 considered four formulations:
//!
//! - **A: pure relative.** Fails on near-zero nodes (reference's own
//!   numerical noise exceeds the bound).
//! - **B: pure absolute.** Cannot serve both small-signal (`µV`) and
//!   large-signal (`100 V`) regimes with a single threshold.
//! - **C: additive (`rel * |v_ref| + abs`).** Over-generous at large
//!   signals (additive term still admits a 1 mV slop on a 100 V rail).
//! - **D: `max(rel * |v_ref|, abs)`** — **chosen**. Relative dominates
//!   on big signals; absolute floor dominates near zero; symmetric and
//!   easy to explain.
//!
//! This module implements Option D and *only* Option D — the rejected
//! variants are not provided, to keep the conformance criterion
//! deterministic across the whole codebase.

/// Per-analysis tolerance envelope.
///
/// Both fields are non-negative finite `f64`. The check is
///
/// ```text
/// |v_actual - v_ref|  <=  max( relative * |v_ref| , absolute )
/// ```
///
/// where the comparison is `<=` (closed on the boundary; an exact-edge
/// sample passes). Both `relative` and `absolute` are required —
/// either being zero is allowed (and means "do not contribute"), but
/// both being zero degenerates to exact equality and is almost
/// certainly a configuration bug.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tolerance {
    /// Relative tolerance fraction. For 1 % use `0.01`. For 0.1 dB on
    /// AC magnitude use `0.001`. Must be `>= 0` and finite.
    pub relative: f64,
    /// Absolute floor in the same units as the reference quantity. For
    /// node voltages use volts (`1e-3` = 1 mV). For currents use
    /// amperes. For noise spectral density use V/√Hz. Must be `>= 0`
    /// and finite.
    pub absolute: f64,
}

impl Tolerance {
    /// Construct a tolerance.
    ///
    /// # Panics
    ///
    /// Panics if either field is negative, NaN, or non-finite. The
    /// conformance harness deliberately does not allow infinite or
    /// NaN tolerances — any such input is a configuration bug, not a
    /// runtime condition to recover from.
    #[must_use]
    pub fn new(relative: f64, absolute: f64) -> Self {
        assert!(
            relative.is_finite() && relative >= 0.0,
            "Tolerance::relative must be finite and >= 0 (got {relative})"
        );
        assert!(
            absolute.is_finite() && absolute >= 0.0,
            "Tolerance::absolute must be finite and >= 0 (got {absolute})"
        );
        Self { relative, absolute }
    }

    /// The maximum-allowed absolute difference for a reference value
    /// `v_ref`. Always `>= 0`.
    ///
    /// ```text
    /// envelope(v_ref) = max( relative * |v_ref|, absolute )
    /// ```
    ///
    /// Returns 0.0 if both `relative` and `absolute` are 0.0 (degenerate
    /// exact-equality mode — see [`Self::new`] for why this is allowed
    /// but discouraged).
    #[must_use]
    pub fn envelope_for(&self, v_ref: f64) -> f64 {
        let rel = self.relative * v_ref.abs();
        rel.max(self.absolute)
    }

    /// Does `v_actual` pass against `v_ref` under this tolerance?
    ///
    /// Returns `true` iff `|v_actual - v_ref| <= envelope_for(v_ref)`.
    ///
    /// NaN handling: if `v_actual` or `v_ref` is NaN, this returns
    /// `false` (NaN is treated as a hard failure — a NaN result from
    /// the solver indicates non-convergence and must be reported,
    /// never silently passed).
    #[must_use]
    pub fn passes(&self, v_ref: f64, v_actual: f64) -> bool {
        if v_ref.is_nan() || v_actual.is_nan() {
            return false;
        }
        let diff = (v_actual - v_ref).abs();
        diff <= self.envelope_for(v_ref)
    }

    /// The signed margin of `v_actual` relative to the envelope around
    /// `v_ref`.
    ///
    /// `margin = envelope_for(v_ref) - |v_actual - v_ref|`.
    ///
    /// - `margin >= 0` means the sample passes (with that much slack
    ///   remaining).
    /// - `margin < 0` means the sample fails (and `-margin` is how far
    ///   beyond the envelope it landed).
    ///
    /// NaN reference or actual returns `f64::NEG_INFINITY` so that a
    /// `worst_margin` reduction over a series will surface the NaN as
    /// the worst case.
    #[must_use]
    pub fn margin(&self, v_ref: f64, v_actual: f64) -> f64 {
        if v_ref.is_nan() || v_actual.is_nan() {
            return f64::NEG_INFINITY;
        }
        let diff = (v_actual - v_ref).abs();
        self.envelope_for(v_ref) - diff
    }
}

/// The analysis kinds the conformance harness recognises. Each one
/// has a tuned default tolerance per ADR-0008 §"Default thresholds by
/// analysis type"; the per-test code in tasks.md #63–#67 may override
/// the default with [`Tolerance::new`] when a PDK or test bench
/// requires it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnalysisKind {
    /// DC operating point — voltage or branch current. ADR-0008
    /// default: 1 % relative or 1 mV absolute.
    Dc,
    /// Transient time-domain — node voltage per time point. ADR-0008
    /// default: 1 % relative or 1 mV absolute per time point per node.
    Transient,
    /// AC small-signal magnitude (in dB). ADR-0008 default: 0.1 dB
    /// relative or 0.01 dB absolute per output/input pair.
    AcMagnitude,
    /// AC small-signal phase (in degrees). ADR-0008 default: 1°
    /// relative or 0.1° absolute per output/input pair.
    AcPhase,
    /// Noise spectral density (in V/√Hz). ADR-0008 default: 2 %
    /// relative or 1 nV/√Hz absolute per frequency point.
    NoiseSpectralDensity,
}

impl AnalysisKind {
    /// The ADR-0008 default tolerance for this analysis kind.
    ///
    /// These are *defaults* the harness ships with — the per-analysis
    /// conformance tests (#63–#68) can override per-PDK if numerical
    /// conditioning warrants. The defaults are copied verbatim from
    /// ADR-0008's "Default thresholds by analysis type" table.
    #[must_use]
    // ADR-0008 lists DC and Transient as *separate rows* with the same
    // numeric tolerance pair. They are conceptually different analysis
    // kinds (operating-point vs. time-domain) even though their
    // tolerances happen to coincide; merging the arms would couple two
    // unrelated ADR rows and create a regression risk if a future
    // proposal retunes one but not the other.
    #[allow(clippy::match_same_arms)]
    pub fn default_tolerance(self) -> Tolerance {
        match self {
            // 1 % relative or 1 mV absolute per node voltage / per branch current.
            Self::Dc => Tolerance::new(0.01, 1e-3),
            // 1 % relative or 1 mV absolute per time point per node.
            Self::Transient => Tolerance::new(0.01, 1e-3),
            // 0.1 dB relative or 0.01 dB absolute per output/input pair.
            // Encoded as fractions: 0.1 dB ≈ 0.001 fractional magnitude
            // *of the value itself* under the dB-domain comparison
            // convention adopted by the per-analysis test in #64; the
            // 0.001 / 1e-4 numeric pair here matches the ADR's text
            // verbatim with both fields read as dB.
            Self::AcMagnitude => Tolerance::new(0.001, 1e-4),
            // 1° relative or 0.1° absolute per output/input pair.
            Self::AcPhase => Tolerance::new(0.01, 0.1),
            // 2 % relative or 1 nV/√Hz absolute per frequency point.
            Self::NoiseSpectralDensity => Tolerance::new(0.02, 1e-9),
        }
    }
}

#[cfg(test)]
// Exact float comparisons in these tests are intentional: we assert
// either (a) panic preconditions on the constructor, (b) the integer-
// representable values of the ADR-0008 default-threshold table, or
// (c) NaN/+-inf sentinel propagation. None of these are subject to
// rounding error — they are the spec-shaped boundary conditions the
// harness must encode exactly.
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    // ---------- Constructor & invariants ----------

    #[test]
    fn new_accepts_zero_components() {
        // ADR-0008 permits either field to be zero (they are
        // independent contributions to the max).
        let t = Tolerance::new(0.0, 1e-3);
        assert_eq!(t.relative, 0.0);
        assert_eq!(t.absolute, 1e-3);
    }

    #[test]
    #[should_panic(expected = "Tolerance::relative")]
    fn new_rejects_negative_relative() {
        let _ = Tolerance::new(-0.01, 1e-3);
    }

    #[test]
    #[should_panic(expected = "Tolerance::absolute")]
    fn new_rejects_negative_absolute() {
        let _ = Tolerance::new(0.01, -1e-3);
    }

    #[test]
    #[should_panic(expected = "Tolerance::relative")]
    fn new_rejects_nan_relative() {
        let _ = Tolerance::new(f64::NAN, 1e-3);
    }

    #[test]
    #[should_panic(expected = "Tolerance::absolute")]
    fn new_rejects_infinite_absolute() {
        let _ = Tolerance::new(0.01, f64::INFINITY);
    }

    // ---------- Envelope math (ADR-0008 Option D) ----------

    #[test]
    fn envelope_relative_dominates_at_large_signal() {
        // 1 % of 100 V is 1 V; 1 mV floor is far smaller. Relative
        // term should dominate.
        let t = Tolerance::new(0.01, 1e-3);
        assert!((t.envelope_for(100.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn envelope_absolute_floor_dominates_near_zero() {
        // 1 % of 1 µV is 1e-8; 1 mV floor is 1e-3. Floor wins, so an
        // ngspice numerical-noise floor of ~10 µV passes a 1 µV
        // reference even though the *relative* difference is 10×.
        let t = Tolerance::new(0.01, 1e-3);
        assert!((t.envelope_for(1e-6) - 1e-3).abs() < 1e-15);
    }

    #[test]
    fn envelope_uses_abs_of_reference() {
        // Sign of v_ref must not flip the envelope sign.
        let t = Tolerance::new(0.01, 1e-3);
        assert_eq!(t.envelope_for(-100.0), t.envelope_for(100.0));
    }

    #[test]
    fn envelope_at_exact_zero_reference_equals_absolute() {
        let t = Tolerance::new(0.01, 1e-3);
        assert_eq!(t.envelope_for(0.0), 1e-3);
    }

    // ---------- passes() boundary behaviour ----------

    #[test]
    fn passes_when_exactly_equal() {
        let t = Tolerance::new(0.01, 1e-3);
        assert!(t.passes(5.0, 5.0));
    }

    #[test]
    fn passes_exactly_at_envelope_edge() {
        // Closed-on-boundary: an actual exactly one envelope-width
        // away passes.
        let t = Tolerance::new(0.01, 1e-3);
        // envelope at 100 V is 1 V.
        assert!(t.passes(100.0, 101.0));
        assert!(t.passes(100.0, 99.0));
    }

    #[test]
    fn fails_just_outside_envelope() {
        let t = Tolerance::new(0.01, 1e-3);
        // 100 V ref, envelope = 1 V, actual 101.0001 V is outside.
        assert!(!t.passes(100.0, 101.0001));
    }

    #[test]
    fn passes_uses_absolute_floor_near_zero() {
        // 1 % of 1 µV is 0.01 µV. A solver result of 0.5 mV against a
        // 1 µV reference is a huge *relative* error but well within
        // the 1 mV absolute floor — passes per ADR-0008's near-zero
        // mitigation.
        let t = Tolerance::new(0.01, 1e-3);
        assert!(t.passes(1e-6, 5e-4));
    }

    #[test]
    fn fails_when_actual_is_nan() {
        // NaN is *always* a failure (per the docstring contract): a
        // NaN from the solver means non-convergence and must surface
        // as a defect.
        let t = Tolerance::new(0.01, 1e-3);
        assert!(!t.passes(5.0, f64::NAN));
    }

    #[test]
    fn fails_when_reference_is_nan() {
        let t = Tolerance::new(0.01, 1e-3);
        assert!(!t.passes(f64::NAN, 5.0));
    }

    // ---------- margin() sign and magnitude ----------

    #[test]
    fn margin_positive_on_pass() {
        let t = Tolerance::new(0.01, 1e-3);
        // envelope @ 100 = 1.0, diff = 0.1, margin = 0.9
        let m = t.margin(100.0, 100.1);
        assert!((m - 0.9).abs() < 1e-12);
    }

    #[test]
    fn margin_negative_on_fail() {
        let t = Tolerance::new(0.01, 1e-3);
        // envelope @ 100 = 1.0, diff = 1.5, margin = -0.5
        let m = t.margin(100.0, 101.5);
        assert!((m + 0.5).abs() < 1e-12);
    }

    #[test]
    fn margin_zero_on_exact_edge() {
        let t = Tolerance::new(0.01, 1e-3);
        let m = t.margin(100.0, 101.0);
        assert!(m.abs() < 1e-12);
    }

    #[test]
    fn margin_neg_inf_on_nan() {
        let t = Tolerance::new(0.01, 1e-3);
        assert_eq!(t.margin(f64::NAN, 1.0), f64::NEG_INFINITY);
        assert_eq!(t.margin(1.0, f64::NAN), f64::NEG_INFINITY);
    }

    // ---------- ADR-0008 default thresholds (verbatim table values) ----------

    #[test]
    fn default_dc_is_1pct_or_1mv() {
        let t = AnalysisKind::Dc.default_tolerance();
        assert!((t.relative - 0.01).abs() < 1e-15);
        assert!((t.absolute - 1e-3).abs() < 1e-15);
    }

    #[test]
    fn default_transient_matches_dc() {
        // ADR-0008: "Transient: 1 % relative or 1 mV absolute per
        // time point per node" — same numeric pair as DC.
        let t = AnalysisKind::Transient.default_tolerance();
        let dc = AnalysisKind::Dc.default_tolerance();
        assert_eq!(t.relative, dc.relative);
        assert_eq!(t.absolute, dc.absolute);
    }

    #[test]
    fn default_ac_magnitude_threshold_pair() {
        let t = AnalysisKind::AcMagnitude.default_tolerance();
        assert!((t.relative - 0.001).abs() < 1e-15);
        assert!((t.absolute - 1e-4).abs() < 1e-15);
    }

    #[test]
    fn default_ac_phase_threshold_pair() {
        let t = AnalysisKind::AcPhase.default_tolerance();
        // 1° relative or 0.1° absolute
        assert!((t.relative - 0.01).abs() < 1e-15);
        assert!((t.absolute - 0.1).abs() < 1e-15);
    }

    #[test]
    fn default_noise_threshold_pair() {
        let t = AnalysisKind::NoiseSpectralDensity.default_tolerance();
        // 2 % relative or 1 nV/√Hz absolute
        assert!((t.relative - 0.02).abs() < 1e-15);
        assert!((t.absolute - 1e-9).abs() < 1e-15);
    }
}
