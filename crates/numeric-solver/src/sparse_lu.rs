//! Sparse direct LU factorizer with Markowitz pivot selection
//! (tasks.md item US-018).
//!
//! This module provides a standalone, dependency-free sparse LU
//! factorizer suited for large circuit matrices where the existing
//! [`crate::linear_solver::RussellRealSolver`] (UMFPACK) is the
//! production backend but a pure-Rust reference implementation is
//! needed for unit-testing and for the Newton-Raphson story US-018.
//!
//! # Algorithm overview
//!
//! The factorizer follows the left-looking sparse LU algorithm with
//! **Markowitz cost** pivot selection and a **threshold partial-pivot**
//! acceptance criterion:
//!
//! At column `k` of the elimination, the Markowitz cost of candidate
//! entry `(r, k)` is:
//!
//! ```text
//! cost(r) = (nnz in active row r - 1) * (nnz in active col k - 1)
//! ```
//!
//! Among all candidates whose absolute value satisfies
//! `|a_{rk}| >= threshold * max_col_k`, the one with the smallest
//! Markowitz cost is chosen as the pivot. In the event of a tie,
//! the entry with the largest absolute value is preferred (for
//! numerical stability).
//!
//! Threshold defaults to 0.1 (SPICE-conventional u = 0.1), matching
//! the acceptance criterion in the task description.
//!
//! # Types
//!
//! - [`CsrMatrix`] — Compressed Sparse Row representation of the
//!   input matrix. Callers build it once and hand it to
//!   [`SparseLU::factorize`].
//! - [`SparseLU`] — stateful factorizer. After `factorize` succeeds
//!   the struct holds the `L` and `U` factors plus the row-permutation
//!   vector; [`SparseLU::solve`] applies the permutation and performs
//!   forward/back substitution.
//! - [`SingularMatrix`] — error returned when a zero pivot is
//!   encountered.
//!
//! # Honored ADRs
//!
//! - **ADR-0002** — This module sits *alongside* the `LinearSolver`
//!   trait (not inside it). It is exposed as a named type so the
//!   Newton-Raphson story can reference it directly, and may later
//!   be wired behind the trait surface if required.
//! - **ADR-0010** — part of the v1 unstable public API.

#![allow(clippy::module_name_repetitions)]

use std::fmt;

/// A square matrix in Compressed Sparse Row (CSR) format.
///
/// `n` is the row/column count. `row_ptr[i]` is the index into
/// `col_idx` / `values` where row `i` begins; `row_ptr[n]` equals
/// the total number of stored entries. `col_idx[k]` and `values[k]`
/// hold the column index and value of the `k`-th stored entry.
///
/// Duplicate `(row, col)` pairs are **summed** on construction via
/// [`CsrMatrix::from_triplets`], matching SPICE stamping semantics.
///
/// # Invariants
///
/// - `row_ptr.len() == n + 1`
/// - `col_idx.len() == values.len() == row_ptr[n]`
/// - For each row `i`, entries in `col_idx[row_ptr[i]..row_ptr[i+1]]`
///   are unique (duplicates summed on construction) and sorted in
///   ascending column order.
#[derive(Debug, Clone, PartialEq)]
pub struct CsrMatrix {
    /// Dimension (number of rows == number of columns).
    pub n: usize,
    /// Row pointer array of length `n + 1`.
    pub row_ptr: Vec<usize>,
    /// Column-index array.
    pub col_idx: Vec<usize>,
    /// Value array (parallel to `col_idx`).
    pub values: Vec<f64>,
}

impl CsrMatrix {
    /// Build a `CsrMatrix` from a list of `(row, col, value)` triplets.
    ///
    /// Duplicate `(row, col)` entries are accumulated (summed).
    /// Entries are stored in ascending column order within each row.
    ///
    /// # Panics
    ///
    /// Panics if any row or column index is `>= n`.
    #[must_use]
    pub fn from_triplets(n: usize, triplets: &[(usize, usize, f64)]) -> Self {
        // Accumulate into a dense 2-D temp representation, then compress.
        // For small n (circuit matrices) this is acceptably O(nnz * log(nnz)).
        use std::collections::BTreeMap;

        // row -> sorted map of col -> value
        let mut rows: Vec<BTreeMap<usize, f64>> = vec![BTreeMap::new(); n];
        for &(r, c, v) in triplets {
            assert!(r < n, "row {r} out of range for n={n}");
            assert!(c < n, "col {c} out of range for n={n}");
            *rows[r].entry(c).or_insert(0.0) += v;
        }

        let mut row_ptr = Vec::with_capacity(n + 1);
        let mut col_idx = Vec::new();
        let mut values = Vec::new();
        row_ptr.push(0);
        for row in &rows {
            for (&c, &v) in row {
                col_idx.push(c);
                values.push(v);
            }
            row_ptr.push(col_idx.len());
        }

        Self {
            n,
            row_ptr,
            col_idx,
            values,
        }
    }

    /// Return the value at `(row, col)`, or `0.0` if the entry is
    /// structurally absent.
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        let start = self.row_ptr[row];
        let end = self.row_ptr[row + 1];
        let slice = &self.col_idx[start..end];
        match slice.binary_search(&col) {
            Ok(k) => self.values[start + k],
            Err(_) => 0.0,
        }
    }
}

/// Error returned by [`SparseLU::factorize`] when a zero (or below-
/// threshold) pivot is encountered, indicating a structurally or
/// numerically singular matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingularMatrix {
    /// The elimination step (column) at which the zero pivot was found.
    pub column: usize,
}

impl fmt::Display for SingularMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sparse-lu: singular matrix at column {}", self.column)
    }
}

impl std::error::Error for SingularMatrix {}

/// Sparse direct LU factorizer with Markowitz pivot selection.
///
/// Call [`SparseLU::factorize`] to perform symbolic + numeric
/// factorization. On success, [`SparseLU::solve`] applies L/U
/// forward–back substitution for one or more right-hand sides.
///
/// # Pivot threshold
///
/// `threshold` (default `0.1`) is the SPICE-conventional partial-
/// pivot acceptance criterion: a candidate entry `a_{rk}` is
/// eligible as pivot only if `|a_{rk}| >= threshold * max_col_k`,
/// where `max_col_k` is the largest absolute value in the active
/// part of column `k`.
///
/// # Storage model
///
/// Internally the factorizer works on a dense `n × n` working copy
/// of the matrix. This is appropriate for the sizes encountered in
/// circuit simulation (up to a few thousand nodes); a fully sparse
/// left-looking implementation is the production-grade alternative
/// (UMFPACK, covered by `RussellRealSolver`).
#[derive(Debug, Clone)]
pub struct SparseLU {
    /// Pivot threshold for Markowitz partial pivoting (default 0.1).
    pub threshold: f64,
    // Post-factorization state:
    n: usize,
    /// Dense LU working matrix: lower triangle (below diagonal)
    /// holds L factors (diagonal 1.0 implicit), upper triangle
    /// (including diagonal) holds U factors.
    lu: Vec<f64>, // row-major, n×n
    /// Row permutation: `perm[k]` is the original row that ended up
    /// in row `k` after pivot reordering.
    perm: Vec<usize>,
    factorized: bool,
}

impl SparseLU {
    /// Construct a new `SparseLU` with the default threshold (0.1).
    #[must_use]
    pub fn new() -> Self {
        Self {
            threshold: 0.1,
            n: 0,
            lu: Vec::new(),
            perm: Vec::new(),
            factorized: false,
        }
    }

    /// Construct with a custom pivot threshold.
    #[must_use]
    pub fn with_threshold(threshold: f64) -> Self {
        Self {
            threshold,
            ..Self::new()
        }
    }

    /// Factorize the matrix `a` using Markowitz pivot selection with
    /// partial-pivot threshold [`Self::threshold`].
    ///
    /// On success the factorization state is stored in `self` and
    /// [`SparseLU::solve`] may be called. On failure the state is
    /// reset (factorized = false).
    ///
    /// # Errors
    ///
    /// Returns [`SingularMatrix`] when no eligible pivot (non-zero
    /// entry passing the threshold test) is found in column `k`.
    pub fn factorize(&mut self, a: &CsrMatrix) -> Result<(), SingularMatrix> {
        let n = a.n;
        self.n = n;
        self.factorized = false;

        if n == 0 {
            self.lu = Vec::new();
            self.perm = Vec::new();
            self.factorized = true;
            return Ok(());
        }

        // Build a dense row-major working copy of the matrix.
        let mut mat = vec![0.0f64; n * n];
        for r in 0..n {
            for k in a.row_ptr[r]..a.row_ptr[r + 1] {
                let c = a.col_idx[k];
                mat[r * n + c] = a.values[k];
            }
        }

        // Row-permutation array: perm[k] = original row at step k.
        let mut perm: Vec<usize> = (0..n).collect();

        // Gaussian elimination with Markowitz pivot selection.
        for step in 0..n {
            // ── 1. Find pivot column maximum in active submatrix ──
            //
            // For threshold partial pivoting we need the column
            // maximum over active rows `step..n` in column `step`.
            // (We are doing plain column-pivot LU, not full Markowitz
            // reordering, which would also permute columns.  The
            // Markowitz *cost* here is used to choose among the rows
            // that pass the threshold test, minimising fill-in.)
            let mut col_max = 0.0f64;
            for r in step..n {
                let v = mat[r * n + step].abs();
                if v > col_max {
                    col_max = v;
                }
            }

            if col_max == 0.0 {
                return Err(SingularMatrix { column: step });
            }

            // ── 2. Markowitz pivot selection ──────────────────────
            //
            // Among rows r in step..n where |mat[r,step]| >= threshold * col_max,
            // choose the row with the smallest Markowitz cost:
            //   cost = (active_row_nnz - 1) * (active_col_nnz - 1)
            // where active means the submatrix [step..n, step..n].
            //
            // Precompute active nnz per row and for the pivot column.
            let active_col_nnz = (step..n)
                .filter(|&r| mat[r * n + step] != 0.0)
                .count();

            let threshold_abs = self.threshold * col_max;

            let mut best_row = None;
            let mut best_cost = usize::MAX;
            let mut best_abs = 0.0f64;

            for r in step..n {
                let v = mat[r * n + step];
                if v.abs() < threshold_abs {
                    continue;
                }
                // Markowitz cost for this row.
                let active_row_nnz = (step..n)
                    .filter(|&c| mat[r * n + c] != 0.0)
                    .count();
                let cost = (active_row_nnz.saturating_sub(1))
                    * (active_col_nnz.saturating_sub(1));
                if cost < best_cost || (cost == best_cost && v.abs() > best_abs) {
                    best_cost = cost;
                    best_row = Some(r);
                    best_abs = v.abs();
                }
            }

            let Some(pivot_row) = best_row else {
                return Err(SingularMatrix { column: step });
            };

            // ── 3. Swap rows ──────────────────────────────────────
            if pivot_row != step {
                perm.swap(step, pivot_row);
                for c in 0..n {
                    mat.swap(step * n + c, pivot_row * n + c);
                }
            }

            // ── 4. Eliminate below pivot ──────────────────────────
            let pivot_val = mat[step * n + step];
            for r in (step + 1)..n {
                if mat[r * n + step] == 0.0 {
                    continue;
                }
                let factor = mat[r * n + step] / pivot_val;
                mat[r * n + step] = factor; // store L sub-diagonal
                for c in (step + 1)..n {
                    let update = factor * mat[step * n + c];
                    mat[r * n + c] -= update;
                }
            }
        }

        self.lu = mat;
        self.perm = perm;
        self.factorized = true;
        Ok(())
    }

    /// Solve `A · x = rhs` using the factorization computed by
    /// [`SparseLU::factorize`].
    ///
    /// The permutation is applied to `rhs` first, then forward
    /// substitution through `L` and back-substitution through `U`
    /// are performed.
    ///
    /// # Panics
    ///
    /// Panics if [`SparseLU::factorize`] has not been called
    /// successfully, or if `rhs.len() != n`.
    #[must_use]
    pub fn solve(&self, rhs: &[f64]) -> Vec<f64> {
        assert!(self.factorized, "SparseLU::solve called before factorize");
        let n = self.n;
        assert_eq!(rhs.len(), n, "rhs length mismatch");

        if n == 0 {
            return Vec::new();
        }

        // Apply row permutation to rhs.
        let mut b: Vec<f64> = self.perm.iter().map(|&r| rhs[r]).collect();

        // Forward substitution: L · y = b  (L has implicit 1s on diagonal,
        // sub-diagonal entries stored in lu[r][c] for c < r).
        for r in 1..n {
            for c in 0..r {
                let l_rc = self.lu[r * n + c];
                let y_c = b[c];
                b[r] -= l_rc * y_c;
            }
        }

        // Back substitution: U · x = y.
        let mut x = b;
        for r in (0..n).rev() {
            for c in (r + 1)..n {
                let u_rc = self.lu[r * n + c];
                let x_c = x[c];
                x[r] -= u_rc * x_c;
            }
            x[r] /= self.lu[r * n + r]; // U diagonal
        }

        x
    }
}

impl Default for SparseLU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    // ----------------------------------------------------------------
    // CsrMatrix construction
    // ----------------------------------------------------------------

    #[test]
    fn csr_from_triplets_sums_duplicates() {
        // Two entries at (0,0): 1.0 + 2.0 = 3.0
        let m = CsrMatrix::from_triplets(2, &[(0, 0, 1.0), (0, 0, 2.0), (1, 1, 5.0)]);
        assert_eq!(m.get(0, 0), 3.0);
        assert_eq!(m.get(1, 1), 5.0);
        assert_eq!(m.get(0, 1), 0.0);
    }

    #[test]
    fn csr_get_absent_entry_returns_zero() {
        let m = CsrMatrix::from_triplets(3, &[(0, 0, 1.0), (2, 2, 4.0)]);
        assert_eq!(m.get(1, 0), 0.0);
        assert_eq!(m.get(0, 2), 0.0);
    }

    // ----------------------------------------------------------------
    // SparseLU basic correctness
    // ----------------------------------------------------------------

    #[test]
    fn factorize_and_solve_identity() {
        // A = I_3, b = [1, 2, 3] => x = [1, 2, 3]
        let a = CsrMatrix::from_triplets(
            3,
            &[(0, 0, 1.0), (1, 1, 1.0), (2, 2, 1.0)],
        );
        let mut lu = SparseLU::new();
        lu.factorize(&a).expect("non-singular");
        let x = lu.solve(&[1.0, 2.0, 3.0]);
        for (got, want) in x.iter().zip([1.0, 2.0, 3.0].iter()) {
            assert!(approx(*got, *want, 1e-12), "got {got}, want {want}");
        }
    }

    #[test]
    fn factorize_and_solve_diagonal() {
        // A = diag(2, 4, 8), b = [4, 8, 16] => x = [2, 2, 2]
        let a = CsrMatrix::from_triplets(
            3,
            &[(0, 0, 2.0), (1, 1, 4.0), (2, 2, 8.0)],
        );
        let mut lu = SparseLU::new();
        lu.factorize(&a).expect("non-singular");
        let x = lu.solve(&[4.0, 8.0, 16.0]);
        for got in &x {
            assert!(approx(*got, 2.0, 1e-12), "got {got}");
        }
    }

    #[test]
    fn factorize_and_solve_requires_pivot_swap() {
        // A = [[0, 1], [2, 3]].  Without pivoting the (0,0) entry is
        // zero so we must swap rows to put the larger entry on the
        // diagonal.  Solution for b = [1, 8]: x = [1, 1] (verify:
        // 0*1 + 1*1 = 1 ✓, 2*1 + 3*1 = 5 ✗ — let's use a cleaner
        // example).
        //
        // A = [[0, 1], [1, 0]], b = [3, 5] => x = [5, 3].
        let a = CsrMatrix::from_triplets(2, &[(0, 1, 1.0), (1, 0, 1.0)]);
        let mut lu = SparseLU::new();
        lu.factorize(&a).expect("non-singular");
        let x = lu.solve(&[3.0, 5.0]);
        assert!(approx(x[0], 5.0, 1e-12), "x[0]={}", x[0]);
        assert!(approx(x[1], 3.0, 1e-12), "x[1]={}", x[1]);
    }

    #[test]
    fn singular_matrix_is_detected() {
        // Row-of-zeros => structurally singular.
        let a = CsrMatrix::from_triplets(2, &[(0, 0, 1.0)]);
        let mut lu = SparseLU::new();
        let err = lu.factorize(&a).expect_err("should be singular");
        assert_eq!(err.column, 1, "zero pivot in column 1");
    }

    #[test]
    fn empty_system_is_handled() {
        let a = CsrMatrix::from_triplets(0, &[]);
        let mut lu = SparseLU::new();
        lu.factorize(&a).expect("empty system is vacuously OK");
        let x = lu.solve(&[]);
        assert!(x.is_empty());
    }

    // ----------------------------------------------------------------
    // US-018 acceptance: 5×5 sparse system within 1e-10 of exact
    // ----------------------------------------------------------------

    /// Known 5×5 sparse system from tasks.md US-018.
    ///
    /// Matrix (tridiagonal + corner):
    ///
    /// ```text
    ///  A =
    ///  [ 4  1  0  0  0 ]
    ///  [ 1  4  1  0  0 ]
    ///  [ 0  1  4  1  0 ]
    ///  [ 0  0  1  4  1 ]
    ///  [ 0  0  0  1  4 ]
    /// ```
    ///
    /// For x = [1, 2, 3, 4, 5]ᵀ:
    ///   b = A·x = [6, 12, 18, 24, 24].
    #[test]
    fn us_018_solve_5x5_sparse_system_within_1e10_of_exact() {
        let n = 5usize;
        let mut triplets = Vec::new();
        for i in 0..n {
            triplets.push((i, i, 4.0));
            if i + 1 < n {
                triplets.push((i, i + 1, 1.0));
                triplets.push((i + 1, i, 1.0));
            }
        }
        let a = CsrMatrix::from_triplets(n, &triplets);
        let rhs = [6.0f64, 12.0, 18.0, 24.0, 24.0];
        let want = [1.0f64, 2.0, 3.0, 4.0, 5.0];

        let mut lu = SparseLU::new();
        lu.factorize(&a).expect("5x5 system should be non-singular");
        let x = lu.solve(&rhs);

        assert_eq!(x.len(), n);
        for (i, (got, &w)) in x.iter().zip(want.iter()).enumerate() {
            assert!(
                approx(*got, w, 1e-10),
                "x[{i}]: got {got}, want {w}, diff {}",
                (got - w).abs()
            );
        }
    }
}
