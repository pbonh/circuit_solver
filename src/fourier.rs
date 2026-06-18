//! Fourier analysis of transient waveforms.
//!
//! [`FourierAnalysis`] resamples a non-uniform transient waveform to N uniform
//! points using monotone cubic (Fritsch-Carlson) spline interpolation, then
//! computes the DFT via a radix-2 Cooley-Tukey FFT.  The result is a
//! [`FourierSolution`] containing per-bin frequencies, magnitudes, and phases.
//!
//! # Algorithm
//!
//! 1. **Resample** the input `(times, values)` waveform (possibly non-uniform)
//!    onto `n_points` evenly-spaced points in `[t_start, t_stop]` using
//!    monotone cubic spline interpolation (Fritsch-Carlson slope limiting).
//! 2. **FFT**: apply radix-2 DIT (decimation-in-time) Cooley-Tukey FFT to the
//!    resampled values.  `n_points` is rounded up to the next power of two.
//! 3. **Normalise**: compute per-bin magnitude `= |X[k]| / N` and
//!    phase `= atan2(Im, Re)` (radians).  Only the positive half-spectrum
//!    (`k = 0..N/2`) is returned.
//! 4. **Frequencies**: `freq[k] = k * fs / N` where `fs = N / (t_stop - t_start)`.
//!
//! # Example
//!
//! ```
//! use circuit_solver_delta::fourier::{FourierAnalysis, FourierSolution};
//!
//! // 10-point, 1 kHz sine at 10 kHz sample rate
//! let n = 1024_usize;
//! let fs = 10_000.0_f64;
//! let dt = 1.0 / fs;
//! let times: Vec<f64> = (0..n).map(|i| i as f64 * dt).collect();
//! let values: Vec<f64> = times.iter().map(|&t| (2.0 * std::f64::consts::PI * 1000.0 * t).sin()).collect();
//! let t_stop = *times.last().unwrap() + dt;
//!
//! let sol = FourierAnalysis::new(&times, &values, n)
//!     .expect("non-empty waveform")
//!     .run();
//!
//! // The 1 kHz bin should be the dominant peak.
//! let peak_bin = sol.magnitude.iter().enumerate()
//!     .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
//!     .map(|(i, _)| i)
//!     .unwrap();
//! let peak_freq = sol.freqs[peak_bin];
//! assert!((peak_freq - 1000.0).abs() < fs / n as f64,
//!         "peak at {peak_freq} Hz, expected 1 kHz");
//! ```

use std::f64::consts::PI;

// ── FourierSolution ──────────────────────────────────────────────────────────

/// Result of a Fourier analysis.
///
/// Only the positive-frequency half of the spectrum is included
/// (`k = 0..n_fft/2`), so `freqs.len() == magnitude.len() == phase.len() == n_fft/2`.
#[derive(Debug, Clone)]
pub struct FourierSolution {
    /// Frequency of each bin (Hz).  `freq[k] = k * fs / n_fft`.
    pub freqs: Vec<f64>,
    /// Magnitude of each bin: `|X[k]| / n_fft`.
    pub magnitude: Vec<f64>,
    /// Phase of each bin: `atan2(Im[k], Re[k])` (radians, range `−π..π`).
    pub phase: Vec<f64>,
}

// ── FourierAnalysis ──────────────────────────────────────────────────────────

/// Builder / runner for FFT-based spectral analysis.
///
/// Takes a (possibly non-uniform) transient waveform, resamples it to
/// `n_points` via monotone cubic spline, and computes the DFT.
#[derive(Debug)]
pub struct FourierAnalysis<'a> {
    times: &'a [f64],
    values: &'a [f64],
    n_points: usize,
}

/// Error returned when [`FourierAnalysis::new`] rejects its inputs.
#[derive(Debug, Clone, PartialEq)]
pub enum FourierError {
    /// `times` and `values` have different lengths.
    LengthMismatch { times_len: usize, values_len: usize },
    /// The waveform has fewer than 2 samples (spline requires at least 2).
    TooFewSamples { len: usize },
    /// `n_points` is zero.
    ZeroPoints,
}

impl std::fmt::Display for FourierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FourierError::LengthMismatch { times_len, values_len } => {
                write!(f, "times length {times_len} != values length {values_len}")
            }
            FourierError::TooFewSamples { len } => {
                write!(f, "waveform has only {len} sample(s); need at least 2")
            }
            FourierError::ZeroPoints => write!(f, "n_points must be > 0"),
        }
    }
}

impl<'a> FourierAnalysis<'a> {
    /// Construct a new [`FourierAnalysis`].
    ///
    /// # Parameters
    ///
    /// - `times`: sample times (seconds).  Must be strictly increasing with at
    ///   least 2 elements.
    /// - `values`: waveform amplitude at each time.  Must have the same length
    ///   as `times`.
    /// - `n_points`: number of uniform resampling points (determines FFT
    ///   resolution).  Rounded up to the next power of two internally.
    ///
    /// # Errors
    ///
    /// Returns [`FourierError`] if any input constraint is violated.
    pub fn new(times: &'a [f64], values: &'a [f64], n_points: usize) -> Result<Self, FourierError> {
        if times.len() != values.len() {
            return Err(FourierError::LengthMismatch {
                times_len: times.len(),
                values_len: values.len(),
            });
        }
        if times.len() < 2 {
            return Err(FourierError::TooFewSamples { len: times.len() });
        }
        if n_points == 0 {
            return Err(FourierError::ZeroPoints);
        }
        Ok(FourierAnalysis { times, values, n_points })
    }

    /// Run the analysis: resample → FFT → return [`FourierSolution`].
    pub fn run(&self) -> FourierSolution {
        let t_start = self.times[0];
        let t_stop  = *self.times.last().unwrap();
        let duration = t_stop - t_start;

        // Round n_points up to the next power of two for radix-2 FFT.
        let n_fft = next_pow2(self.n_points);

        // 1. Resample onto n_fft uniform points in [t_start, t_stop].
        let resampled = monotone_cubic_resample(self.times, self.values, n_fft, t_start, t_stop);

        // 2. FFT (in-place radix-2 DIT on complex buffer).
        let mut buf: Vec<[f64; 2]> = resampled.iter().map(|&v| [v, 0.0]).collect();
        fft_radix2(&mut buf);

        // 3. Build solution (positive half-spectrum only: k = 0..n_fft/2).
        let n = n_fft as f64;
        let fs = n / duration; // sample rate
        let half = n_fft / 2;

        let mut freqs     = Vec::with_capacity(half);
        let mut magnitude = Vec::with_capacity(half);
        let mut phase     = Vec::with_capacity(half);

        for (k, &slot) in buf.iter().enumerate().take(half) {
            let re = slot[0];
            let im = slot[1];
            freqs.push(k as f64 * fs / n);
            magnitude.push((re * re + im * im).sqrt() / n);
            phase.push(im.atan2(re));
        }

        FourierSolution { freqs, magnitude, phase }
    }
}

// ── Monotone cubic spline (Fritsch-Carlson) ──────────────────────────────────

/// Resample `(times, values)` onto `n_out` evenly-spaced points in
/// `[t_start, t_stop]` using monotone cubic spline interpolation.
///
/// Reference: Fritsch & Carlson (1980), "Monotone Piecewise Cubic Interpolation",
/// SIAM J. Numer. Anal. 17(2), pp. 238–246.
///
/// The algorithm computes secant slopes, initialises tangent estimates, then
/// limits them so the interpolant is monotone in each sub-interval.  Clamps
/// the query to `[t_start, t_stop]` so no extrapolation is needed.
fn monotone_cubic_resample(
    t: &[f64],
    y: &[f64],
    n_out: usize,
    t_start: f64,
    t_stop: f64,
) -> Vec<f64> {
    let n = t.len();

    // --- Step 1: secant slopes ---
    let mut delta = vec![0.0_f64; n - 1];
    for i in 0..n - 1 {
        delta[i] = (y[i + 1] - y[i]) / (t[i + 1] - t[i]);
    }

    // --- Step 2: initial tangent estimates (average of neighbouring secants) ---
    let mut m = vec![0.0_f64; n];
    m[0] = delta[0];
    m[n - 1] = delta[n - 2];
    for i in 1..n - 1 {
        m[i] = 0.5 * (delta[i - 1] + delta[i]);
    }

    // --- Step 3: Fritsch-Carlson monotonicity limiting ---
    for i in 0..n - 1 {
        if delta[i].abs() < f64::EPSILON {
            // Flat segment: force zero tangents on both endpoints.
            m[i]     = 0.0;
            m[i + 1] = 0.0;
        } else {
            let alpha = m[i]     / delta[i];
            let beta  = m[i + 1] / delta[i];
            let rho   = (alpha * alpha + beta * beta).sqrt();
            if rho > 3.0 {
                let tau = 3.0 / rho;
                m[i]     = tau * alpha * delta[i];
                m[i + 1] = tau * beta  * delta[i];
            }
        }
    }

    // --- Step 4: evaluate at n_out uniform query points ---
    let mut out = Vec::with_capacity(n_out);
    let dt_out = if n_out > 1 { (t_stop - t_start) / (n_out - 1) as f64 } else { 0.0 };

    // Binary search cursor (persistent, since queries are monotone).
    let mut seg = 0_usize;

    for k in 0..n_out {
        let tq = (t_start + k as f64 * dt_out).clamp(t_start, t_stop);

        // Advance segment until `t[seg] <= tq < t[seg+1]`.
        while seg + 1 < n - 1 && tq >= t[seg + 1] {
            seg += 1;
        }

        let h  = t[seg + 1] - t[seg];
        let s  = (tq - t[seg]) / h; // normalised position in [0, 1]

        // Hermite basis functions.
        let h00 =  2.0 * s * s * s - 3.0 * s * s + 1.0;
        let h10 =       s * s * s - 2.0 * s * s + s;
        let h01 = -2.0 * s * s * s + 3.0 * s * s;
        let h11 =       s * s * s -       s * s;

        let v = h00 * y[seg] + h10 * h * m[seg] + h01 * y[seg + 1] + h11 * h * m[seg + 1];
        out.push(v);
    }

    out
}

// ── Radix-2 DIT FFT (Cooley-Tukey) ──────────────────────────────────────────

/// Return the smallest power of two ≥ `n`.
fn next_pow2(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut p = 1_usize;
    while p < n {
        p <<= 1;
    }
    p
}

/// In-place radix-2 decimation-in-time FFT.
///
/// `buf` must have length equal to a power of two.  Each element is `[re, im]`.
/// On return, `buf[k]` contains the DFT coefficient `X[k]`.
fn fft_radix2(buf: &mut [[f64; 2]]) {
    let n = buf.len();
    debug_assert!(n.is_power_of_two(), "FFT length must be a power of two");

    // --- Bit-reversal permutation ---
    let bits = n.trailing_zeros() as usize;
    for i in 0..n {
        let j = bit_reverse(i, bits);
        if j > i {
            buf.swap(i, j);
        }
    }

    // --- Cooley-Tukey butterfly stages ---
    let mut len = 2_usize;
    while len <= n {
        let half = len / 2;
        let ang  = -2.0 * PI / len as f64; // DFT convention: e^{-j 2π k/N}
        let wlen = [ang.cos(), ang.sin()];

        let mut i = 0;
        while i < n {
            let mut w = [1.0_f64, 0.0_f64]; // twiddle factor w = 1 at start of each group
            for j in 0..half {
                let u = buf[i + j];
                let v = complex_mul(buf[i + j + half], w);
                buf[i + j]        = [u[0] + v[0], u[1] + v[1]];
                buf[i + j + half] = [u[0] - v[0], u[1] - v[1]];
                w = complex_mul(w, wlen);
            }
            i += len;
        }
        len <<= 1;
    }
}

/// Multiply two complex numbers represented as `[re, im]`.
#[inline(always)]
fn complex_mul(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] * b[0] - a[1] * b[1], a[0] * b[1] + a[1] * b[0]]
}

/// Reverse the lower `bits` bits of `x`.
fn bit_reverse(mut x: usize, bits: usize) -> usize {
    let mut result = 0_usize;
    for _ in 0..bits {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: run a full 1 kHz sine transient and return the FourierSolution.
    fn sine_1khz_fourier(n: usize, fs: f64) -> FourierSolution {
        let dt = 1.0 / fs;
        let times: Vec<f64>  = (0..n).map(|i| i as f64 * dt).collect();
        let values: Vec<f64> = times.iter()
            .map(|&t| (2.0 * PI * 1000.0 * t).sin())
            .collect();

        FourierAnalysis::new(&times, &values, n)
            .expect("valid input")
            .run()
    }

    // ── 1 kHz sine: DFT magnitude peak within 0.1 dB of 0.5 (single-sided) ──

    /// Acceptance criterion for US-036:
    ///
    /// A 1 kHz sine wave sampled at `fs = 102_400 Hz` for N=1024 points is
    /// resampled to 1024 uniform points and FFT'd.  With fs chosen so that
    /// 1 kHz falls on an exact FFT bin (1000 * 1024 / 102400 = 10.0 cycles),
    /// there is no spectral leakage and the dominant bin magnitude equals 0.5
    /// (the theoretical single-sided amplitude of a unit-amplitude sine).
    ///
    /// The dominant bin must correspond to 1 kHz ± one bin width, and its
    /// magnitude must be within 0.1 dB of 0.5.
    ///
    /// 0.1 dB tolerance → ratio bounds [0.5 / 10^(0.1/20), 0.5 * 10^(0.1/20)]
    ///                               ≈ [0.4944, 0.5058].
    #[test]
    fn sine_1khz_dft_peak_at_1khz_within_0_1_db() {
        // fs chosen so 1 kHz lands exactly on bin 10 (no spectral leakage).
        // 1000 Hz * N / fs = 1000 * 1024 / 102400 = 10.0  (integer bin)
        let fs   = 102_400.0_f64; // Hz
        let n    = 1024_usize;    // points (power of two → no padding)
        let f_sig = 1000.0_f64;  // 1 kHz

        let sol = sine_1khz_fourier(n, fs);

        // Find the peak bin in the positive-frequency half.
        let (peak_bin, &peak_mag) = sol.magnitude.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .expect("non-empty spectrum");

        let peak_freq = sol.freqs[peak_bin];
        let bin_width = fs / n as f64; // Hz per bin

        // Peak frequency must be within one bin of 1 kHz.
        assert!(
            (peak_freq - f_sig).abs() <= bin_width,
            "peak at {peak_freq:.1} Hz, expected {f_sig} Hz ± {bin_width} Hz"
        );

        // Magnitude must be within 0.1 dB of 0.5.
        let expected_mag  = 0.5_f64;           // theoretical single-sided amplitude
        let tol_linear    = 10_f64.powf(0.1 / 20.0); // 0.1 dB → ×1.01153
        let mag_lo        = expected_mag / tol_linear;
        let mag_hi        = expected_mag * tol_linear;

        assert!(
            peak_mag >= mag_lo && peak_mag <= mag_hi,
            "peak magnitude {peak_mag:.6} not within 0.1 dB of {expected_mag:.6} \
             (range [{mag_lo:.6}, {mag_hi:.6}])"
        );
    }

    // ── FourierSolution field lengths are consistent ──────────────────────────

    #[test]
    fn fourier_solution_field_lengths_match() {
        let sol = sine_1khz_fourier(512, 50_000.0);
        assert_eq!(sol.freqs.len(), sol.magnitude.len());
        assert_eq!(sol.freqs.len(), sol.phase.len());
    }

    // ── freqs start at 0 and are monotonically increasing ─────────────────────

    #[test]
    fn fourier_freqs_monotone_from_zero() {
        let sol = sine_1khz_fourier(64, 10_000.0);
        assert!((sol.freqs[0]).abs() < f64::EPSILON, "first bin should be DC (0 Hz)");
        for w in sol.freqs.windows(2) {
            assert!(w[1] > w[0], "freqs must be strictly increasing");
        }
    }

    // ── positive half-spectrum length = n_fft / 2 ────────────────────────────

    #[test]
    fn fourier_output_is_half_spectrum() {
        let n     = 128_usize;
        let n_fft = next_pow2(n); // = 128 (already a power of two)
        let sol   = sine_1khz_fourier(n, 50_000.0);
        assert_eq!(sol.freqs.len(), n_fft / 2);
    }

    // ── DC signal: only bin 0 is non-negligible ───────────────────────────────

    #[test]
    fn dc_signal_dominant_at_bin_zero() {
        let n  = 256_usize;
        let fs = 10_000.0_f64;
        let dt = 1.0 / fs;
        let dc = 2.5_f64;
        let times:  Vec<f64> = (0..n).map(|i| i as f64 * dt).collect();
        let values: Vec<f64> = vec![dc; n];

        let sol = FourierAnalysis::new(&times, &values, n)
            .unwrap()
            .run();

        // Bin 0 magnitude should ≈ dc (single-sided: |X[0]|/N = dc*N/N = dc).
        // DC: no factor-of-2 correction; magnitude[0] = dc for a constant signal.
        assert!(
            (sol.magnitude[0] - dc).abs() < 1e-10,
            "DC magnitude {:.6} should equal {dc}", sol.magnitude[0]
        );
        // All other bins should be effectively zero.
        for (k, &m) in sol.magnitude.iter().enumerate().skip(1) {
            assert!(
                m < 1e-10,
                "bin {k} magnitude {m:.3e} should be negligible for a DC signal"
            );
        }
    }

    // ── FourierError: length mismatch ─────────────────────────────────────────

    #[test]
    fn fourier_error_length_mismatch() {
        let t = vec![0.0, 1.0, 2.0];
        let v = vec![0.0, 1.0];
        let err = FourierAnalysis::new(&t, &v, 4).unwrap_err();
        assert_eq!(err, FourierError::LengthMismatch { times_len: 3, values_len: 2 });
    }

    // ── FourierError: too few samples ─────────────────────────────────────────

    #[test]
    fn fourier_error_too_few_samples() {
        let t = vec![0.0];
        let v = vec![1.0];
        let err = FourierAnalysis::new(&t, &v, 4).unwrap_err();
        assert_eq!(err, FourierError::TooFewSamples { len: 1 });
    }

    // ── FourierError: zero points ─────────────────────────────────────────────

    #[test]
    fn fourier_error_zero_points() {
        let t = vec![0.0, 1.0];
        let v = vec![0.0, 1.0];
        let err = FourierAnalysis::new(&t, &v, 0).unwrap_err();
        assert_eq!(err, FourierError::ZeroPoints);
    }

    // ── Non-uniform input times: spline resamples correctly ───────────────────

    /// Feed a 1 kHz sine sampled at non-uniform intervals (jittered times)
    /// and verify the FFT still places the peak near 1 kHz.
    #[test]
    fn fourier_nonuniform_times_peak_at_1khz() {
        let f_sig = 1000.0_f64;
        let fs    = 50_000.0_f64;
        let n     = 512_usize;
        let dt    = 1.0 / fs;

        // Jitter each sample time by ±10% of dt (deterministic, no rand dep).
        let times: Vec<f64> = (0..n).map(|i| {
            let t_nominal = i as f64 * dt;
            let jitter = dt * 0.1 * if i % 2 == 0 { 1.0 } else { -1.0 };
            (t_nominal + jitter).max(0.0)
        }).collect();

        // Ensure times are strictly increasing (sort and deduplicate any ties).
        let mut sorted_times = times.clone();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let values: Vec<f64> = sorted_times.iter()
            .map(|&t| (2.0 * PI * f_sig * t).sin())
            .collect();

        let sol = FourierAnalysis::new(&sorted_times, &values, n)
            .expect("valid input")
            .run();

        let (peak_bin, _) = sol.magnitude.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        let peak_freq = sol.freqs[peak_bin];
        let bin_width = fs / n as f64;

        assert!(
            (peak_freq - f_sig).abs() <= 2.0 * bin_width,
            "non-uniform: peak at {peak_freq:.1} Hz, expected ~{f_sig} Hz"
        );
    }

    // ── Monotone spline helper: linearly-spaced input is exact ────────────────

    #[test]
    fn spline_linear_input_reconstructs_exactly() {
        let t: Vec<f64>  = (0..8).map(|i| i as f64).collect();
        let y: Vec<f64>  = t.iter().map(|&x| 2.0 * x + 1.0).collect();
        let out = monotone_cubic_resample(&t, &y, 8, 0.0, 7.0);
        for (i, (&expected, &got)) in y.iter().zip(out.iter()).enumerate() {
            assert!(
                (got - expected).abs() < 1e-10,
                "sample {i}: expected {expected}, got {got}"
            );
        }
    }

    // ── next_pow2 corner cases ─────────────────────────────────────────────────

    #[test]
    fn next_pow2_rounds_up() {
        assert_eq!(next_pow2(0),    1);
        assert_eq!(next_pow2(1),    1);
        assert_eq!(next_pow2(2),    2);
        assert_eq!(next_pow2(3),    4);
        assert_eq!(next_pow2(1023), 1024);
        assert_eq!(next_pow2(1024), 1024);
        assert_eq!(next_pow2(1025), 2048);
    }
}
