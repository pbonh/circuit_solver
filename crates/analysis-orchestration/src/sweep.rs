//! Logarithmic frequency `Sweep` generator.
//!
//! This module covers `tasks.md` item #28 of
//! `circuit-solver/2026-05-21-v1-spec` and witnesses scenario
//! `ac-small-signal#ac-frequency-sweep-over-multiple-decades`.
//!
//! Per the inlined glossary, a `Sweep` is *"a sequence of analysis
//! points (voltage, frequency, or time)"*. This module supplies the
//! **frequency** flavor with **logarithmic** spacing and a
//! **configurable points-per-decade** density, which is the standard
//! presentation grid for Bode-style AC small-signal results.
//!
//! # Spec contract
//!
//! The witnessing scenario:
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit with a bandpass filter topology
//! And the frequency Sweep is logarithmic from 1 kHz to 1 GHz with 100 points per decade
//! When CircuitDesigner submits an AC small-signal Analysis request
//! Then the Result contains TransferFunction data at every frequency point
//! And the bandpass center frequency and Q factor match the Golden Reference within tolerance
//! And the complex-valued solves use the faer sparse-direct backend
//! ```
//!
//! The "logarithmic from f_start to f_stop with N points per decade"
//! contract resolves to a strictly increasing geometric progression
//! whose ratio is `10^(1/N)`. The endpoints `f_start` and `f_stop` are
//! both included so callers can pin the result frame exactly.
//!
//! # Design decisions
//!
//! - **Inclusive endpoints.** `f_start` and `f_stop` are both
//!   emitted. This matches the common engineering reading of
//!   "1 kHz to 1 GHz" and ngspice's `dec` sweep convention.
//! - **Exact decade points.** When `(log10(f_stop) - log10(f_start))`
//!   is an integer and `points_per_decade` divides evenly, the decade
//!   boundaries appear exactly in the output (modulo `f64` round-off).
//!   The implementation uses `log10` + linear interpolation in the
//!   log domain + `powf` rather than repeated multiplication so error
//!   does not accumulate point-to-point.
//! - **Strict positivity.** Logarithmic sweeps are undefined at and
//!   below DC; `f_start <= 0` (or non-finite) is a caller-bug
//!   error rather than silently clamped.
//! - **f_start == f_stop.** Allowed; emits a single-point sweep at
//!   that frequency. This degenerate case is useful for scripts that
//!   parameterize the sweep range and only sometimes degenerate it.
//! - **Per ADR-0010** the public API surface is unstable at v1.
//!
//! # Wiring with [`ac_analysis`](super::ac_analysis)
//!
//! The output [`Vec<f64>`] feeds directly into
//! [`AcAnalysisRequest::frequencies_hz`](super::AcAnalysisRequest::frequencies_hz):
//!
//! ```ignore
//! use analysis_orchestration::{ac_analysis, AcAnalysisRequest, LogSweep};
//! # let (system, structure, graph, outputs) = todo!();
//! let frequencies_hz = LogSweep::new(1.0e3, 1.0e9, 100).unwrap().frequencies();
//! let result = ac_analysis(AcAnalysisRequest {
//!     system, structure, graph,
//!     frequencies_hz: &frequencies_hz,
//!     outputs, ground: None,
//! }).unwrap();
//! ```

#![allow(clippy::module_name_repetitions)]
// Numerical-test pragmas: pedantic float-compare and cast-precision
// lints are noisy and unhelpful in this module. Strict float
// equality is the *right* assertion when checking that endpoints
// are pinned exactly (we use the same f64 bits the constructor
// stored). Casts between f64/u64/usize/i32 are bounded by the
// constructor's validation, not by lint-visible runtime data; the
// `len`/`frequencies` paths cap the intermediate at
// `u32::MAX · ~308` worst-case (log10 of f64::MAX), well inside
// every target's usize range and well inside f64's exact-integer
// range. The single-char binding lint trips on our scenario-witness
// `r,l,c` electrical quantities, which is the universally
// understood mathematical convention. The `needless_range_loop`
// false-positive in the -3 dB scan is intentional reverse iteration
// from the peak.
#![allow(
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names,
    clippy::needless_range_loop,
    clippy::too_many_lines,
    clippy::doc_markdown
)]

/// A logarithmic frequency sweep specification.
///
/// Construct with [`LogSweep::new`], then call [`LogSweep::frequencies`]
/// to materialize the frequency vector for an AC analysis request.
///
/// Invariants (enforced at construction time):
///
/// - `f_start_hz` is finite and `> 0`.
/// - `f_stop_hz` is finite and `>= f_start_hz`.
/// - `points_per_decade >= 1`.
///
/// The materialized vector contains
/// `ceil(N · log10(f_stop / f_start)) + 1` points when
/// `f_stop > f_start`, with the last point pinned to exactly
/// `f_stop_hz` (modulo `f64` representation). When `f_stop == f_start`
/// the vector has length 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogSweep {
    f_start_hz: f64,
    f_stop_hz: f64,
    points_per_decade: u32,
}

impl LogSweep {
    /// Construct a logarithmic sweep from `f_start_hz` to `f_stop_hz`
    /// (inclusive) at `points_per_decade` points per decade.
    ///
    /// # Errors
    ///
    /// - [`LogSweepError::NonFiniteFrequency`] — either bound is NaN
    ///   or ±∞.
    /// - [`LogSweepError::NonPositiveStart`] — `f_start_hz <= 0`.
    ///   Logarithmic sweeps are undefined at and below DC.
    /// - [`LogSweepError::StopBelowStart`] — `f_stop_hz < f_start_hz`.
    /// - [`LogSweepError::ZeroPointsPerDecade`] — `points_per_decade == 0`.
    pub fn new(
        f_start_hz: f64,
        f_stop_hz: f64,
        points_per_decade: u32,
    ) -> Result<Self, LogSweepError> {
        if !f_start_hz.is_finite() {
            return Err(LogSweepError::NonFiniteFrequency {
                frequency_hz: f_start_hz,
            });
        }
        if !f_stop_hz.is_finite() {
            return Err(LogSweepError::NonFiniteFrequency {
                frequency_hz: f_stop_hz,
            });
        }
        if f_start_hz <= 0.0 {
            return Err(LogSweepError::NonPositiveStart {
                frequency_hz: f_start_hz,
            });
        }
        if f_stop_hz < f_start_hz {
            return Err(LogSweepError::StopBelowStart {
                f_start_hz,
                f_stop_hz,
            });
        }
        if points_per_decade == 0 {
            return Err(LogSweepError::ZeroPointsPerDecade);
        }
        Ok(Self {
            f_start_hz,
            f_stop_hz,
            points_per_decade,
        })
    }

    /// Start of the sweep (Hz).
    #[must_use]
    pub fn f_start_hz(&self) -> f64 {
        self.f_start_hz
    }

    /// End of the sweep (Hz), inclusive.
    #[must_use]
    pub fn f_stop_hz(&self) -> f64 {
        self.f_stop_hz
    }

    /// Density of the sweep (samples per decade).
    #[must_use]
    pub fn points_per_decade(&self) -> u32 {
        self.points_per_decade
    }

    /// Number of frequency points the materialized [`Self::frequencies`]
    /// vector will contain, without allocating it.
    ///
    /// `1` when `f_start_hz == f_stop_hz`; otherwise
    /// `ceil(N · log10(f_stop / f_start)) + 1`.
    #[must_use]
    pub fn len(&self) -> usize {
        if self.f_start_hz == self.f_stop_hz {
            return 1;
        }
        let decades = (self.f_stop_hz / self.f_start_hz).log10();
        // ceil(N·decades) yields the number of *intervals*; +1 for
        // the inclusive endpoint.
        // `decades > 0` since stop > start (positive) was already
        // enforced; `points_per_decade >= 1` likewise. So the
        // product is finite and strictly positive.
        let n_intervals = (f64::from(self.points_per_decade) * decades).ceil();
        // Cast through u64 first to avoid the silent saturation
        // `as usize` does on extreme inputs; `n_intervals` is
        // bounded by `u32::MAX · ~308` (log10 of f64::MAX) which
        // fits in u64 easily.
        let n_intervals_u64 = n_intervals as u64;
        (n_intervals_u64 as usize).saturating_add(1)
    }

    /// True iff the materialized vector would have zero points. By
    /// construction this is never the case (constructor enforces a
    /// non-empty sweep), but the method is supplied for symmetry
    /// with `len`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Materialize the frequency vector for this sweep.
    ///
    /// The vector is strictly increasing (except for the degenerate
    /// single-point case), with `frequencies()[0] == f_start_hz` and
    /// `frequencies().last() == f_stop_hz` (modulo `f64` round-off).
    ///
    /// Computed in the log domain (`log10` + linear interp + `powf`)
    /// so that round-off does not accumulate across points; every
    /// emitted frequency is `f_start · 10^(k / points_per_decade)`
    /// for integer `k`.
    #[must_use]
    pub fn frequencies(&self) -> Vec<f64> {
        if self.f_start_hz == self.f_stop_hz {
            return vec![self.f_start_hz];
        }
        let log_start = self.f_start_hz.log10();
        let log_stop = self.f_stop_hz.log10();
        let decades = log_stop - log_start;
        let ppd = f64::from(self.points_per_decade);
        let n_intervals = (ppd * decades).ceil() as u64;
        let n_points = (n_intervals as usize).saturating_add(1);
        let step = 1.0 / ppd;
        let mut out = Vec::with_capacity(n_points);
        // Emit f_start · 10^(k · step) for k=0..n_intervals-1, then
        // pin the last point to f_stop exactly so the inclusive-end
        // contract holds even when `ppd · decades` is not integral.
        for k in 0..n_intervals {
            // log10(f_k) = log_start + k · step.
            let log_f = log_start + (k as f64) * step;
            out.push(10f64.powf(log_f));
        }
        out.push(self.f_stop_hz);
        out
    }
}

/// Errors raised by [`LogSweep::new`].
#[derive(Debug, Clone, PartialEq)]
pub enum LogSweepError {
    /// One of the supplied frequencies was NaN or ±∞.
    NonFiniteFrequency {
        /// The offending value (Hz).
        frequency_hz: f64,
    },
    /// `f_start_hz` was zero or negative. Logarithmic sweeps are
    /// undefined at and below DC.
    NonPositiveStart {
        /// The offending value (Hz).
        frequency_hz: f64,
    },
    /// `f_stop_hz` was strictly less than `f_start_hz`. The sweep
    /// would run backward, which is not how the spec defines a
    /// frequency `Sweep`.
    StopBelowStart {
        /// The sweep start (Hz).
        f_start_hz: f64,
        /// The sweep stop (Hz).
        f_stop_hz: f64,
    },
    /// `points_per_decade == 0`. A zero-density sweep has no points
    /// and would yield an empty vector, which the AC analysis
    /// control loop rejects upstream with `EmptySweep`; we surface it
    /// here at construction time so the misuse is reported at its
    /// source.
    ZeroPointsPerDecade,
}

impl core::fmt::Display for LogSweepError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteFrequency { frequency_hz } => write!(
                f,
                "log-sweep: frequency {frequency_hz} Hz is non-finite (NaN or ±∞)"
            ),
            Self::NonPositiveStart { frequency_hz } => write!(
                f,
                "log-sweep: f_start_hz must be > 0, got {frequency_hz} Hz \
                 (logarithmic sweeps are undefined at and below DC)"
            ),
            Self::StopBelowStart {
                f_start_hz,
                f_stop_hz,
            } => write!(
                f,
                "log-sweep: f_stop_hz ({f_stop_hz} Hz) must be >= f_start_hz ({f_start_hz} Hz)"
            ),
            Self::ZeroPointsPerDecade => {
                write!(f, "log-sweep: points_per_decade must be >= 1, got 0")
            }
        }
    }
}

impl std::error::Error for LogSweepError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol.max(tol * a.abs().max(b.abs()))
    }

    // -------- constructor validation --------------------------------------

    #[test]
    fn new_rejects_nan_start() {
        let err = LogSweep::new(f64::NAN, 1.0e3, 10).unwrap_err();
        assert!(matches!(
            err,
            LogSweepError::NonFiniteFrequency { frequency_hz } if frequency_hz.is_nan()
        ));
    }

    #[test]
    fn new_rejects_inf_stop() {
        let err = LogSweep::new(1.0, f64::INFINITY, 10).unwrap_err();
        assert!(matches!(
            err,
            LogSweepError::NonFiniteFrequency { frequency_hz } if frequency_hz.is_infinite()
        ));
    }

    #[test]
    fn new_rejects_zero_start() {
        let err = LogSweep::new(0.0, 1.0e3, 10).unwrap_err();
        assert_eq!(err, LogSweepError::NonPositiveStart { frequency_hz: 0.0 });
    }

    #[test]
    fn new_rejects_negative_start() {
        let err = LogSweep::new(-1.0, 1.0e3, 10).unwrap_err();
        assert_eq!(err, LogSweepError::NonPositiveStart { frequency_hz: -1.0 });
    }

    #[test]
    fn new_rejects_stop_below_start() {
        let err = LogSweep::new(1.0e6, 1.0e3, 10).unwrap_err();
        assert_eq!(
            err,
            LogSweepError::StopBelowStart {
                f_start_hz: 1.0e6,
                f_stop_hz: 1.0e3,
            }
        );
    }

    #[test]
    fn new_rejects_zero_points_per_decade() {
        let err = LogSweep::new(1.0, 1.0e3, 0).unwrap_err();
        assert_eq!(err, LogSweepError::ZeroPointsPerDecade);
    }

    // -------- accessors round-trip ----------------------------------------

    #[test]
    fn accessors_roundtrip() {
        let s = LogSweep::new(1.0e3, 1.0e9, 100).expect("valid");
        assert_eq!(s.f_start_hz(), 1.0e3);
        assert_eq!(s.f_stop_hz(), 1.0e9);
        assert_eq!(s.points_per_decade(), 100);
    }

    // -------- materialization shape ---------------------------------------

    #[test]
    fn frequencies_one_decade_one_point_per_decade() {
        // 1 → 10 Hz at 1 point/decade: expect [1.0, 10.0].
        let s = LogSweep::new(1.0, 10.0, 1).expect("ok");
        let f = s.frequencies();
        assert_eq!(f.len(), 2);
        assert!(approx(f[0], 1.0, 1e-12));
        assert!(approx(f[1], 10.0, 1e-12));
        assert_eq!(s.len(), f.len());
    }

    #[test]
    fn frequencies_two_decades_two_points_per_decade() {
        // 1 → 100 Hz at 2 points/decade: 4 intervals → 5 points.
        // Step in log10 = 0.5, so points are
        // 10^0, 10^0.5, 10^1, 10^1.5, 10^2 = 1, ~3.162, 10, ~31.62, 100.
        let s = LogSweep::new(1.0, 100.0, 2).expect("ok");
        let f = s.frequencies();
        assert_eq!(f.len(), 5);
        assert!(approx(f[0], 1.0, 1e-12));
        assert!(approx(f[1], 10f64.powf(0.5), 1e-12));
        assert!(approx(f[2], 10.0, 1e-12));
        assert!(approx(f[3], 10f64.powf(1.5), 1e-12));
        assert!(approx(f[4], 100.0, 1e-12));
    }

    #[test]
    fn frequencies_inclusive_endpoints_pinned_exactly() {
        // Even when ppd · decades is non-integral the endpoints are
        // pinned exactly.
        let s = LogSweep::new(7.0, 13_000.0, 17).expect("ok");
        let f = s.frequencies();
        assert!(!f.is_empty());
        assert_eq!(f[0], 7.0);
        assert_eq!(*f.last().unwrap(), 13_000.0);
    }

    #[test]
    fn frequencies_strictly_increasing() {
        let s = LogSweep::new(1.0e3, 1.0e9, 100).expect("ok");
        let f = s.frequencies();
        for win in f.windows(2) {
            assert!(
                win[1] > win[0],
                "expected strict monotonicity: {} -> {}",
                win[0],
                win[1]
            );
        }
    }

    #[test]
    fn frequencies_count_matches_one_kilohertz_to_one_gigahertz_at_100_per_decade() {
        // 1 kHz → 1 GHz is 6 decades. 6 · 100 = 600 intervals → 601
        // points.
        let s = LogSweep::new(1.0e3, 1.0e9, 100).expect("ok");
        assert_eq!(s.len(), 601);
        let f = s.frequencies();
        assert_eq!(f.len(), 601);
        // Pinned endpoints.
        assert_eq!(f[0], 1.0e3);
        assert_eq!(*f.last().unwrap(), 1.0e9);
        // Decade boundaries land on exact powers of 10 (to f64
        // round-off): index 100 should be 10 kHz, index 200 should
        // be 100 kHz, ..., index 600 should be 1 GHz.
        for k in 0..=6 {
            let idx = k * 100;
            let want = 1.0e3 * 10f64.powi(k as i32);
            assert!(
                approx(f[idx], want, 1e-9),
                "decade boundary at idx {idx}: got {}, want {}",
                f[idx],
                want
            );
        }
    }

    #[test]
    fn frequencies_degenerate_single_point() {
        let s = LogSweep::new(42.0, 42.0, 10).expect("ok");
        assert_eq!(s.len(), 1);
        assert_eq!(s.frequencies(), vec![42.0]);
    }

    #[test]
    fn frequencies_partial_decade() {
        // 1 → 5 Hz (~0.699 decades) at 10 points/decade: ceil(6.99)=7
        // intervals → 8 points. Endpoints pinned.
        let s = LogSweep::new(1.0, 5.0, 10).expect("ok");
        let f = s.frequencies();
        assert_eq!(f.len(), 8);
        assert_eq!(f[0], 1.0);
        assert_eq!(*f.last().unwrap(), 5.0);
        // The last *internal* point (index 6) is the geometric
        // step before the pinned endpoint: 10^(6 · 0.1) = ~3.981.
        assert!(approx(f[6], 10f64.powf(0.6), 1e-9));
    }

    #[test]
    fn frequencies_log_domain_does_not_accumulate_error() {
        // 100 points/decade over 8 decades = 800 intervals → 801
        // points. Verify every interior point is consistent with
        // the log-domain identity log10(f_k) = log_start + k · step.
        let f_start = 1.0e-3;
        let f_stop = 1.0e5;
        let ppd = 100u32;
        let s = LogSweep::new(f_start, f_stop, ppd).expect("ok");
        let f = s.frequencies();
        let step = 1.0 / f64::from(ppd);
        for (k, &fk) in f.iter().enumerate() {
            if k == f.len() - 1 {
                // The endpoint is pinned to f_stop, not synthesized
                // from the log identity.
                continue;
            }
            let want = 10f64.powf(f_start.log10() + (k as f64) * step);
            assert!(
                approx(fk, want, 1e-12),
                "log-domain inconsistency at k={k}: got {fk}, want {want}"
            );
        }
    }

    // -------- integration with ac_analysis --------------------------------
    //
    // These tests live alongside the sweep unit tests rather than in
    // ac.rs so that the sweep module owns the witness for its own
    // scenario (`ac-frequency-sweep-over-multiple-decades`). They
    // verify the materialized vector is directly usable as input to
    // the AC control loop and produces the expected bandpass response.

    use crate::{ac_analysis, AcAnalysisRequest};
    use circuit_solver_types::flattened::FlattenedStructure;
    use circuit_solver_types::NodeId;
    use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
    use numeric_solver::{assemble, flatten, MnaSystem};

    fn rlc_series_bandpass(
        vsrc: f64,
        r_ohms: f64,
        l_henries: f64,
        c_farads: f64,
    ) -> (FlattenedStructure, CircuitGraph, MnaSystem) {
        // Series RLC: V1 → R → L → C → gnd. The output is tapped
        // across R (n_in to n_a), giving a bandpass response:
        //
        //   H(jω) = V_R / V_in = R / (R + jωL + 1/(jωC))
        //                     = jωRC / (1 + jωRC + (jω)² LC)
        //
        // This has |H| = 1 at the resonance ω0 = 1/√(LC) (the LC
        // reactances cancel and the resistor sees the whole drop)
        // and falls off on both sides. Q = (1/R)·√(L/C).
        //
        // To read V_R we need V_in (the source node) and V_a (the
        // R-to-L junction); the bandpass voltage is V_in - V_a.
        let mut b = CircuitBuilder::default();
        b.add_element(
            "V1",
            ElementKind::VoltageSource {
                voltage_volts: vsrc,
            },
            ["n_in", "0"],
            None,
        )
        .expect("add vsource");
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: r_ohms,
            },
            ["n_in", "n_a"],
            None,
        )
        .expect("add resistor");
        b.add_element(
            "L1",
            ElementKind::Inductor {
                inductance_henries: l_henries,
            },
            ["n_a", "n_b"],
            None,
        )
        .expect("add inductor");
        b.add_element(
            "C1",
            ElementKind::Capacitor {
                capacitance_farads: c_farads,
            },
            ["n_b", "0"],
            None,
        )
        .expect("add capacitor");
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        (fs, g, sys)
    }

    /// Scenario witness: `ac-small-signal#ac-frequency-sweep-over-multiple-decades`.
    ///
    /// ```gherkin
    /// Given CircuitDesigner has constructed a Circuit with a bandpass filter topology
    /// And the frequency Sweep is logarithmic from 1 kHz to 1 GHz with 100 points per decade
    /// When CircuitDesigner submits an AC small-signal Analysis request
    /// Then the Result contains TransferFunction data at every frequency point
    /// And the bandpass center frequency and Q factor match the Golden Reference within tolerance
    /// And the complex-valued solves use the faer sparse-direct backend
    /// ```
    ///
    /// We pick an RLC series with L = 1 µH, C = 1 nF, R = 1 Ω so the
    /// resonance falls at f0 = 1/(2π√(LC)) ≈ 5.033 MHz, comfortably
    /// inside the 1 kHz → 1 GHz sweep. Q = (1/R)·√(L/C) ≈ 31.6.
    ///
    /// We compute the analytic |H| at every swept frequency (the
    /// "Golden Reference" in scenario terms; for the v1 implementer
    /// task the analytic closed-form *is* the reference, with the
    /// ngspice-based Conformance step gated to a later task #102)
    /// and verify the simulator stays within a tight tolerance
    /// envelope at every point. We separately verify peak location
    /// (center frequency) and -3 dB bandwidth (Q factor).
    #[test]
    fn ac_frequency_sweep_over_multiple_decades_bandpass_witness() {
        let r = 1.0_f64;
        let l = 1.0e-6_f64;
        let c = 1.0e-9_f64;
        let (fs, g, sys) = rlc_series_bandpass(1.0, r, l, c);

        // The acceptance criterion uses "1 kHz to 1 GHz with 100
        // points per decade". Per ADR-0002 the complex-LU solver
        // is faer-backed; ac_analysis constructs FaerComplexSolver
        // internally so this branch is exercised automatically.
        let sweep = LogSweep::new(1.0e3, 1.0e9, 100).expect("valid sweep");
        let frequencies_hz = sweep.frequencies();
        assert_eq!(frequencies_hz.len(), 601, "6 decades · 100 ppd + 1");
        assert_eq!(frequencies_hz[0], 1.0e3);
        assert_eq!(*frequencies_hz.last().unwrap(), 1.0e9);

        // Node layout (CircuitBuilder uses insertion order; ground
        // is reserved as NodeId::GROUND): 0=gnd, 1=n_in, 2=n_a, 3=n_b.
        let n_in = NodeId::new(1);
        let n_a = NodeId::new(2);

        let result = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &frequencies_hz,
            outputs: &[n_in, n_a],
            ground: None,
        })
        .expect("ac_analysis ok");

        // -- TransferFunction data at every frequency point --
        assert_eq!(result.transfer_functions.len(), 2);
        let tf_in = result.transfer_for(n_in).expect("n_in tf");
        let tf_a = result.transfer_for(n_a).expect("n_a tf");
        assert_eq!(tf_in.frequencies_hz.len(), frequencies_hz.len());
        assert_eq!(tf_in.magnitude_db.len(), frequencies_hz.len());
        assert_eq!(tf_in.phase_degrees.len(), frequencies_hz.len());
        assert_eq!(tf_a.frequencies_hz.len(), frequencies_hz.len());
        assert_eq!(tf_a.magnitude_db.len(), frequencies_hz.len());
        assert_eq!(tf_a.phase_degrees.len(), frequencies_hz.len());

        // Construct the bandpass response V_R = V_in - V_a at each
        // frequency. Since V_in = 1∠0 (the unit voltage source) and
        // tf_in stays at 0 dB / 0° for all f, the bandpass magnitude
        // and phase can be read off V_a directly.
        // |H_bp(jω)|² = |1 - V_a(jω)|² · |V_in|²  (with V_in=1)
        //             = |1 - V_a|²
        // We reconstruct the analytic bandpass at every frequency:
        //   H(jω) = jωRC / (1 - ω²LC + jωRC)
        // and compare.
        let f0_hz = 1.0 / (2.0 * core::f64::consts::PI * (l * c).sqrt());
        let q = (1.0 / r) * (l / c).sqrt();

        // -- analytic vs solver pointwise --
        // Tolerance envelope: 0.1 dB magnitude and 1 degree phase
        // (matches the conformance scenario's bench, applied here
        // pointwise to the analytic closed-form as the v1
        // implementer's golden reference).
        let mag_tol_db = 0.1;
        let phase_tol_deg = 1.0;
        for (i, &f_hz) in frequencies_hz.iter().enumerate() {
            // Solver-derived bandpass: V_R = 1 - V_a.
            // To do this we need the complex value, but the TF
            // structure stores only |.| and arg(.). Reconstruct:
            let mag_va = 10f64.powf(tf_a.magnitude_db[i] / 20.0);
            let phase_va = tf_a.phase_degrees[i].to_radians();
            let va_re = mag_va * phase_va.cos();
            let va_im = mag_va * phase_va.sin();
            let bp_re = 1.0 - va_re;
            let bp_im = -va_im;
            let solver_bp_mag = (bp_re * bp_re + bp_im * bp_im).sqrt();
            let solver_bp_mag_db = 20.0 * solver_bp_mag.log10();
            let solver_bp_phase_deg = bp_im.atan2(bp_re).to_degrees();

            // Analytic bandpass at this frequency.
            let omega = 2.0 * core::f64::consts::PI * f_hz;
            let num_im = omega * r * c;
            let den_re = 1.0 - omega * omega * l * c;
            let den_im = omega * r * c;
            let den_mag2 = den_re * den_re + den_im * den_im;
            let h_re = (num_im * den_im) / den_mag2;
            let h_im = (num_im * den_re) / den_mag2;
            let ana_mag = (h_re * h_re + h_im * h_im).sqrt();
            let ana_mag_db = 20.0 * ana_mag.log10();
            let ana_phase_deg = h_im.atan2(h_re).to_degrees();

            // Magnitude check (only where the response is not in the
            // numerical floor; both endpoints of a 6-decade
            // bandpass are at extreme attenuation where 0.1 dB is
            // tighter than f64 round-off can deliver consistently).
            if ana_mag_db > -100.0 {
                assert!(
                    (solver_bp_mag_db - ana_mag_db).abs() < mag_tol_db,
                    "magnitude tolerance violated at f[{i}]={f_hz} Hz: \
                     solver={solver_bp_mag_db} dB, analytic={ana_mag_db} dB"
                );
                // Phase check only when magnitude is well above the
                // numerical floor (phase of a vanishing complex
                // number is meaningless).
                if ana_mag_db > -60.0 {
                    let phase_err = ((solver_bp_phase_deg - ana_phase_deg + 540.0) % 360.0) - 180.0;
                    assert!(
                        phase_err.abs() < phase_tol_deg,
                        "phase tolerance violated at f[{i}]={f_hz} Hz: \
                         solver={solver_bp_phase_deg}°, analytic={ana_phase_deg}°, \
                         err={phase_err}°"
                    );
                }
            }
        }

        // -- center frequency: locate the peak of the bandpass mag --
        let mut peak_idx = 0usize;
        let mut peak_db = f64::NEG_INFINITY;
        let bp_mag_db: Vec<f64> = (0..frequencies_hz.len())
            .map(|i| {
                let mag_va = 10f64.powf(tf_a.magnitude_db[i] / 20.0);
                let phase_va = tf_a.phase_degrees[i].to_radians();
                let bp_re = 1.0 - mag_va * phase_va.cos();
                let bp_im = -mag_va * phase_va.sin();
                10.0 * (bp_re * bp_re + bp_im * bp_im).log10()
            })
            .collect();
        for (i, &db) in bp_mag_db.iter().enumerate() {
            if db > peak_db {
                peak_db = db;
                peak_idx = i;
            }
        }
        let f_peak = frequencies_hz[peak_idx];
        // The center frequency should match the analytic f0 within
        // the sweep resolution. At 100 points/decade the grid
        // spacing at f0 ≈ 5.033 MHz is ~117 kHz, i.e. ~2.3% of f0.
        let f_peak_err_rel = (f_peak - f0_hz).abs() / f0_hz;
        assert!(
            f_peak_err_rel < 0.025,
            "center frequency off: peak at {f_peak} Hz vs f0={f0_hz} Hz \
             (rel err {f_peak_err_rel})"
        );
        // And the peak gain is ~0 dB for a series RLC bandpass.
        // The discretized peak is the grid sample nearest f0; for a
        // high-Q response (Q≈31.6 here) the gain falls off quickly
        // off-resonance, so even at 100 ppd the nearest grid sample
        // can be ~0.3 dB below the analytic peak. Use a tolerance
        // consistent with that grid resolution.
        assert!(peak_db.abs() < 0.5, "peak gain off: {peak_db} dB (want ~0)");

        // -- Q factor: read off the -3 dB bandwidth --
        // For a bandpass with peak at 0 dB, the -3 dB points bracket
        // the bandwidth Δf, and Q = f0 / Δf. Find the two crossings
        // either side of the peak.
        let target_db = peak_db - 3.0;
        let mut idx_lo = None;
        for i in (0..peak_idx).rev() {
            if bp_mag_db[i] <= target_db {
                idx_lo = Some(i);
                break;
            }
        }
        let mut idx_hi = None;
        for i in (peak_idx + 1)..bp_mag_db.len() {
            if bp_mag_db[i] <= target_db {
                idx_hi = Some(i);
                break;
            }
        }
        let idx_lo = idx_lo.expect("found lower -3 dB crossing");
        let idx_hi = idx_hi.expect("found upper -3 dB crossing");
        // Linear-in-log interpolation for each crossing.
        let interp = |i: usize, j: usize| -> f64 {
            let (a, b) = (bp_mag_db[i], bp_mag_db[j]);
            let (la, lb) = (frequencies_hz[i].log10(), frequencies_hz[j].log10());
            let t = (target_db - a) / (b - a);
            10f64.powf(la + t * (lb - la))
        };
        let f_lo = interp(idx_lo + 1, idx_lo);
        let f_hi = interp(idx_hi - 1, idx_hi);
        let bw = f_hi - f_lo;
        let q_measured = f_peak / bw;
        // Tolerance is wide because (a) the closed-form bandpass
        // -3 dB Q approximation is only first-order accurate for
        // narrowband (high-Q) responses, which this is, and (b) the
        // log-spaced grid resolution introduces ~1-2% interpolation
        // error. 10% relative tolerance is well within both.
        let q_err_rel = (q_measured - q).abs() / q;
        assert!(
            q_err_rel < 0.10,
            "Q factor off: measured Q={q_measured} vs analytic Q={q} \
             (rel err {q_err_rel})"
        );
    }

    #[test]
    fn log_sweep_feeds_ac_analysis_without_modification() {
        // Smoke test: the materialized vector is directly accepted
        // by ac_analysis (i.e. no shape adapter is needed). Uses the
        // simpler RC fixture from ac.rs's test space.
        let mut b = CircuitBuilder::default();
        b.add_element(
            "V1",
            ElementKind::VoltageSource { voltage_volts: 1.0 },
            ["n_in", "0"],
            None,
        )
        .expect("vsrc");
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: 1_000.0,
            },
            ["n_in", "n_out"],
            None,
        )
        .expect("r");
        b.add_element(
            "C1",
            ElementKind::Capacitor {
                capacitance_farads: 1.0e-6,
            },
            ["n_out", "0"],
            None,
        )
        .expect("c");
        let g = b.build().expect("build");
        let fs = flatten(&g).expect("flatten");
        let sys = assemble(&fs, &g, &[]).expect("assemble");

        let sweep = LogSweep::new(1.0, 1.0e5, 10).expect("sweep");
        let freqs = sweep.frequencies();
        let result = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &freqs,
            outputs: &[NodeId::new(2)],
            ground: None,
        })
        .expect("ac analysis");
        let tf = &result.transfer_functions[0];
        assert_eq!(tf.frequencies_hz.len(), freqs.len());
        assert_eq!(tf.magnitude_db.len(), freqs.len());
        assert_eq!(tf.phase_degrees.len(), freqs.len());
        // Endpoints are pinned by the sweep generator.
        assert_eq!(tf.frequencies_hz[0], 1.0);
        assert_eq!(*tf.frequencies_hz.last().unwrap(), 1.0e5);
    }
}
