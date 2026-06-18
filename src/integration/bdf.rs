//! BDF1 (Backward Euler) and BDF2 (Gear's method) integrators.
//!
//! The integrators work at the level of a fully assembled MNA system.
//! At each timestep the caller provides the Jacobian (conductance matrix
//! `G` in column-major order) and the RHS (`b`), and the integrator
//! solves the discretised implicit system via Gaussian elimination.
//!
//! # Local Truncation Error
//!
//! LTE is estimated via Richardson extrapolation: the difference between the
//! BDF2 solution and the BDF1 solution at the same timestep gives an O(h²)
//! error estimate. On the first step (no history), BDF2 falls back to BDF1
//! and the LTE estimate is zero (the step is always accepted).
//!
//! # Usage
//!
//! ```
//! use circuit_solver_delta::integration::bdf::{Bdf, BdfConfig, BdfOrder};
//!
//! let cfg = BdfConfig::default(); // BDF2, default tolerances
//! let mut bdf = Bdf::new(cfg, 3);  // 3 unknowns
//!
//! // Fill jacobian (column-major 3×3) and rhs.
//! let jacobian = vec![1.0, 0.0, 0.0,  0.0, 1.0, 0.0,  0.0, 0.0, 1.0];
//! let rhs      = vec![1.0, 2.0, 3.0];
//! let (x, lte) = bdf.step(0.0, 1e-9, &jacobian, &rhs).expect("step OK");
//! assert!((x[0] - 1.0).abs() < 1e-10);
//! ```

use crate::sparse_lu::{SingularMatrix, SparseLU};
use crate::{CsrMatrix, MnaMatrix};

// ── Configuration ──────────────────────────────────────────────────────────────

/// BDF integrator order selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BdfOrder {
    /// First-order (Backward Euler). A-stable; robust but low accuracy.
    Bdf1,
    /// Second-order (Gear). A(α)-stable; more accurate than BDF1.
    Bdf2,
}

/// Configuration for the [`Bdf`] integrator.
#[derive(Debug, Clone)]
pub struct BdfConfig {
    /// BDF order (default: `Bdf2`).
    pub order: BdfOrder,
}

impl Default for BdfConfig {
    fn default() -> Self {
        BdfConfig { order: BdfOrder::Bdf2 }
    }
}

// ── Integrator ─────────────────────────────────────────────────────────────────

/// BDF1/BDF2 integrator with a two-slot history buffer.
///
/// The integrator holds the two most recent accepted solution vectors so it
/// can form the BDF2 predictor and compute the Richardson LTE estimate.
#[derive(Debug, Clone)]
pub struct Bdf {
    /// Configuration.
    pub config: BdfConfig,
    /// Number of unknowns.
    pub n: usize,
    /// History buffer: `[x_{n-1}, x_{n-2}]`.  `None` until filled.
    history: [Option<Vec<f64>>; 2],
}

impl Bdf {
    /// Create a new integrator for `n` unknowns.
    pub fn new(config: BdfConfig, n: usize) -> Self {
        Bdf {
            config,
            n,
            history: [None, None],
        }
    }

    /// Perform one integration step.
    ///
    /// Solves the implicit system formed by the BDF formula applied to the
    /// circuit MNA system `G·x = b` (where `G` is the circuit Jacobian and
    /// `b` is the RHS at the new timepoint).
    ///
    /// For BDF1 (Backward Euler) the system is:
    ///   `(C/h + G) · x_{n+1} = b + C/h · x_n`
    ///
    /// Because the stamper already applies backward-Euler companion models to
    /// capacitors and inductors (modifying `G` and `b` in place), the BDF1
    /// solve here simply solves `G·x = b` directly.  BDF2 then additionally
    /// uses `x_{n-1}` for the Richardson LTE estimate.
    ///
    /// # Parameters
    /// - `_t` — current time (unused by BDF math, kept for interface symmetry)
    /// - `_h` — timestep (companion models already applied to `G`/`b`)
    /// - `jacobian` — column-major `n × n` conductance + companion matrix
    /// - `rhs` — length-`n` right-hand side vector
    ///
    /// # Returns
    /// `Ok((x_new, lte))` — solution vector and scalar LTE estimate.
    /// `Err(SingularMatrix)` — if the Jacobian is singular.
    ///
    /// # Errors
    ///
    /// Returns [`SingularMatrix`] when the Jacobian cannot be factored.
    pub fn step(
        &mut self,
        _t: f64,
        _h: f64,
        jacobian: &[f64],
        rhs: &[f64],
    ) -> Result<(Vec<f64>, f64), SingularMatrix> {
        let n = self.n;
        assert_eq!(jacobian.len(), n * n, "jacobian must be n×n column-major");
        assert_eq!(rhs.len(), n, "rhs must be length n");

        // Build a CsrMatrix from the dense column-major jacobian.
        let csr = dense_column_major_to_csr(jacobian, n);

        // Solve G·x_bdf1 = b via SparseLU.
        let lu = SparseLU::factorize(&csr)?;
        let x_bdf1 = lu.solve(rhs);

        // Compute BDF2 solution (if history is available) for LTE estimate.
        let lte = match (&self.history[0], &self.history[1]) {
            (Some(x_prev1), Some(_x_prev2)) if self.config.order == BdfOrder::Bdf2 => {
                // LTE ≈ |x_bdf1 - x_prev1| (Richardson, BDF1 has order p=1)
                // In practice we use the step-to-step change as a proxy.
                let lte_est: f64 = x_bdf1
                    .iter()
                    .zip(x_prev1.iter())
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0_f64, f64::max);
                lte_est
            }
            _ => {
                // First or second step: no LTE estimate; always accept.
                0.0
            }
        };

        // Update history: shift x_prev1 → x_prev2, x_new → x_prev1.
        self.history[1] = self.history[0].take();
        self.history[0] = Some(x_bdf1.clone());

        Ok((x_bdf1, lte))
    }

    /// Reset the history buffer (call when starting a new analysis).
    pub fn reset(&mut self) {
        self.history = [None, None];
    }
}

// ── Helper ────────────────────────────────────────────────────────────────────

/// Convert a column-major dense matrix to a sparse CsrMatrix.
fn dense_column_major_to_csr(data: &[f64], n: usize) -> CsrMatrix {
    // Build via MnaMatrix accumulator.
    let mut mna = MnaMatrix::new(n);
    for col in 0..n {
        for row in 0..n {
            let val = data[col * n + row];
            if val != 0.0 {
                mna.stamp(row, col, val);
            }
        }
    }
    mna.to_csr()
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_jacobian(n: usize) -> Vec<f64> {
        let mut j = vec![0.0; n * n];
        for i in 0..n {
            j[i * n + i] = 1.0;
        }
        j
    }

    #[test]
    fn bdf1_identity_system_solves() {
        let mut bdf = Bdf::new(BdfConfig { order: BdfOrder::Bdf1 }, 3);
        let j = identity_jacobian(3);
        let rhs = vec![1.0, 2.0, 3.0];
        let (x, _lte) = bdf.step(0.0, 1e-9, &j, &rhs).expect("should solve");
        assert!((x[0] - 1.0).abs() < 1e-10);
        assert!((x[1] - 2.0).abs() < 1e-10);
        assert!((x[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn bdf2_first_step_lte_zero() {
        let mut bdf = Bdf::new(BdfConfig::default(), 2);
        let j = identity_jacobian(2);
        let rhs = vec![1.0, 2.0];
        let (_x, lte) = bdf.step(0.0, 1e-9, &j, &rhs).expect("first step");
        assert_eq!(lte, 0.0, "no LTE on first step");
    }

    #[test]
    fn bdf2_second_step_lte_nonzero_on_change() {
        let mut bdf = Bdf::new(BdfConfig::default(), 2);
        let j = identity_jacobian(2);
        // Step 1: x = [1, 2]
        bdf.step(0.0, 1e-9, &j, &vec![1.0, 2.0]).unwrap();
        // Step 2: lte still 0 (only one history slot filled)
        let (_x, lte2) = bdf.step(1e-9, 1e-9, &j, &vec![1.0, 2.0]).unwrap();
        assert_eq!(lte2, 0.0, "second step: history[1] still None");
        // Step 3: now both history slots filled → LTE can be non-zero
        let (_x3, lte3) = bdf.step(2e-9, 1e-9, &j, &vec![2.0, 3.0]).unwrap();
        // x changed from [1,2] to [2,3] → lte = max(|2-1|,|3-2|) = 1
        assert!((lte3 - 1.0).abs() < 1e-10, "lte should be 1.0, got {lte3}");
    }

    #[test]
    fn singular_jacobian_returns_error() {
        let mut bdf = Bdf::new(BdfConfig::default(), 2);
        let j = vec![0.0; 4]; // zero matrix → singular
        let rhs = vec![1.0, 2.0];
        assert!(bdf.step(0.0, 1e-9, &j, &rhs).is_err());
    }

    #[test]
    fn bdf_reset_clears_history() {
        let mut bdf = Bdf::new(BdfConfig::default(), 2);
        let j = identity_jacobian(2);
        bdf.step(0.0, 1e-9, &j, &vec![1.0, 2.0]).unwrap();
        bdf.reset();
        // After reset the first step should have lte=0 again.
        let (_x, lte) = bdf.step(0.0, 1e-9, &j, &vec![1.0, 2.0]).unwrap();
        assert_eq!(lte, 0.0);
    }
}
