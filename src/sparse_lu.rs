//! SparseLU: direct LU factorisation for [`CsrMatrix`].
//!
//! Uses a dense n×n working copy with Markowitz-threshold partial pivoting.
//! Appropriate for circuit-class matrices (n up to a few thousand nodes).
//!
//! # Algorithm
//! Doolittle LU with threshold partial pivoting:
//! - At elimination step `k`, choose the pivot row `r` from the active column
//!   such that `|a[r][k]| >= threshold * col_max`.  Among eligible rows the
//!   one with the smallest Markowitz cost is chosen.
//! - `threshold = 0.1` (SPICE conventional value).
//! - If no pivot is found (singular or near-singular matrix), returns
//!   [`Err(SingularMatrix)`].

use crate::CsrMatrix;

/// Error returned when the matrix is (structurally or numerically) singular.
#[derive(Debug, Clone, PartialEq)]
pub struct SingularMatrix {
    /// Elimination step at which no valid pivot was found.
    pub step: usize,
}

impl std::fmt::Display for SingularMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "singular matrix at elimination step {}", self.step)
    }
}

/// Factored LU representation ready for back-substitution.
///
/// The LU factors are stored in a single flat `n×n` dense array in row-major
/// order.  The permutation vector `perm` maps each logical row index to its
/// physical row after all row-interchanges.
pub struct SparseLU {
    /// Matrix dimension.
    n: usize,
    /// Dense LU factors in row-major order (length n*n).
    /// Lower triangular part (excluding diagonal) holds the L multipliers;
    /// upper triangular part (including diagonal) holds U.
    lu: Vec<f64>,
    /// Row permutation: `perm[i] = j` means logical row `i` is physical row
    /// `j` in the original system.  Applied as forward permutation during solve.
    perm: Vec<usize>,
}

impl SparseLU {
    /// Threshold for Markowitz partial pivoting (SPICE conventional = 0.1).
    const THRESHOLD: f64 = 0.1;

    /// Factorise `a` into L·U form.
    ///
    /// Returns `Ok(SparseLU)` on success or `Err(SingularMatrix)` if the matrix
    /// is singular.
    pub fn factorize(a: &CsrMatrix) -> Result<Self, SingularMatrix> {
        let n = a.size;

        // --- copy CSR into a dense n×n working buffer -------------------------
        let mut lu = vec![0.0f64; n * n];
        for r in 0..n {
            for k in a.row_ptr[r]..a.row_ptr[r + 1] {
                let c = a.col_idx[k];
                lu[r * n + c] = a.values[k];
            }
        }

        // --- identity permutation -------------------------------------------
        let mut perm: Vec<usize> = (0..n).collect();

        // --- Doolittle elimination with Markowitz-threshold pivot -------------
        for step in 0..n {
            // Find column maximum in the active column (rows >= step).
            let col_max = (step..n)
                .map(|r| lu[perm[r] * n + step].abs())
                .fold(0.0_f64, f64::max);

            if col_max < f64::EPSILON {
                return Err(SingularMatrix { step });
            }

            let threshold_abs = Self::THRESHOLD * col_max;

            // Among rows eligible by threshold, pick the one with the smallest
            // Markowitz cost in the active [step..n, step..n] submatrix.
            let mut best_row = None;
            let mut best_cost = usize::MAX;

            for r in step..n {
                let phys_r = perm[r];
                if lu[phys_r * n + step].abs() >= threshold_abs {
                    // Count active non-zeros in row phys_r from step..n.
                    let row_nnz = (step..n)
                        .filter(|&c| lu[phys_r * n + c].abs() > 0.0)
                        .count();
                    // Count active non-zeros in column `step` from step..n.
                    let col_nnz = (step..n)
                        .filter(|&rr| lu[perm[rr] * n + step].abs() > 0.0)
                        .count();
                    let cost = (row_nnz.saturating_sub(1)) * (col_nnz.saturating_sub(1));
                    if cost < best_cost {
                        best_cost = cost;
                        best_row = Some(r);
                    }
                }
            }

            let pivot_logical = best_row.ok_or(SingularMatrix { step })?;
            // Swap logical rows `step` and `pivot_logical` in the permutation.
            perm.swap(step, pivot_logical);

            let phys_pivot = perm[step];
            let pivot_val = lu[phys_pivot * n + step];

            // Eliminate rows below the pivot.
            #[allow(clippy::needless_range_loop)]
            for r in (step + 1)..n {
                let phys_r = perm[r];
                let factor = lu[phys_r * n + step] / pivot_val;
                if factor == 0.0 {
                    continue;
                }
                lu[phys_r * n + step] = factor; // store L multiplier
                for c in (step + 1)..n {
                    let update = factor * lu[phys_pivot * n + c];
                    lu[phys_r * n + c] -= update;
                }
            }
        }

        Ok(SparseLU { n, lu, perm })
    }

    /// Solve `A·x = rhs` using the stored LU factorisation.
    ///
    /// Returns the solution vector `x`.
    pub fn solve(&self, rhs: &[f64]) -> Vec<f64> {
        let n = self.n;

        // --- apply row permutation to rhs -----------------------------------
        let mut b: Vec<f64> = (0..n).map(|i| rhs[self.perm[i]]).collect();

        // --- forward substitution  L·y = b  (L has unit diagonal) ----------
        for i in 0..n {
            let phys_i = self.perm[i];
            for j in 0..i {
                b[i] -= self.lu[phys_i * n + j] * b[j];
            }
        }

        // --- back substitution  U·x = y  ------------------------------------
        for i in (0..n).rev() {
            let phys_i = self.perm[i];
            for j in (i + 1)..n {
                b[i] -= self.lu[phys_i * n + j] * b[j];
            }
            b[i] /= self.lu[phys_i * n + i];
        }

        b
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MnaMatrix;

    fn make_csr(n: usize, triplets: &[(usize, usize, f64)], rhs: &[f64]) -> CsrMatrix {
        let mut m = MnaMatrix::new(n);
        for &(r, c, v) in triplets {
            m.stamp(r, c, v);
        }
        for (i, &v) in rhs.iter().enumerate() {
            m.stamp_rhs(i, v);
        }
        m.to_csr()
    }

    /// 2×2 trivial diagonal system: [2 0; 0 3] · x = [6; 9]  → x = [3; 3]
    #[test]
    fn solve_2x2_diagonal() {
        let a = make_csr(2, &[(0, 0, 2.0), (1, 1, 3.0)], &[6.0, 9.0]);
        let lu = SparseLU::factorize(&a).expect("factorize");
        let x = lu.solve(&a.rhs);
        assert!((x[0] - 3.0).abs() < 1e-10, "x[0]={}", x[0]);
        assert!((x[1] - 3.0).abs() < 1e-10, "x[1]={}", x[1]);
    }

    /// 3×3 well-conditioned system: 3x3 tridiagonal-like with unique solution.
    /// Use the banded diagonal-dominant system: [3 -1 0; -1 3 -1; 0 -1 3]·x = [2;1;2]
    /// Exact solution: x = [1; 1; 1].
    #[test]
    fn solve_3x3_tridiagonal() {
        let triplets = [
            (0, 0, 3.0), (0, 1, -1.0),
            (1, 0, -1.0), (1, 1, 3.0), (1, 2, -1.0),
            (2, 1, -1.0), (2, 2, 3.0),
        ];
        let rhs = [2.0, 1.0, 2.0];
        let a = make_csr(3, &triplets, &rhs);
        let lu = SparseLU::factorize(&a).expect("factorize");
        let x = lu.solve(&a.rhs);
        // Exact solution x = [1, 1, 1]; verify within 1e-10
        for i in 0..3 {
            assert!((x[i] - 1.0).abs() < 1e-10, "x[{i}]={} expected 1.0", x[i]);
        }
    }

    /// Singular matrix returns Err.
    #[test]
    fn singular_returns_err() {
        let a = make_csr(2, &[(0, 0, 1.0), (1, 0, 2.0)], &[1.0, 2.0]);
        let result = SparseLU::factorize(&a);
        assert!(result.is_err(), "expected Err for singular matrix");
    }

    /// 5×5 acceptance test: solve a non-trivial dense system within 1e-10.
    #[test]
    fn us_018_solve_5x5_sparse_system_within_1e10_of_exact() {
        // Hilbert-like but well-conditioned for small n.
        // Use a simple banded system whose exact solution is known.
        let n = 5;
        let mut m = MnaMatrix::new(n);
        // Diagonal dominant 5×5: A[i][i]=4, A[i][i±1]=-1
        for i in 0..n {
            m.stamp(i, i, 4.0);
            if i > 0 { m.stamp(i, i - 1, -1.0); }
            if i + 1 < n { m.stamp(i, i + 1, -1.0); }
        }
        // RHS: all 1s → solved by Thomas-tridiagonal, but SparseLU should handle it.
        for i in 0..n { m.stamp_rhs(i, 1.0); }
        let csr = m.to_csr();
        let lu = SparseLU::factorize(&csr).expect("factorize 5×5");
        let x = lu.solve(&csr.rhs);
        // Verify residual ||A·x - b||∞ < 1e-10
        for r in 0..n {
            let ax: f64 = (0..n).map(|c| csr.get(r, c) * x[c]).sum();
            assert!(
                (ax - 1.0).abs() < 1e-10,
                "row {r}: residual {}", (ax - 1.0).abs()
            );
        }
    }
}
