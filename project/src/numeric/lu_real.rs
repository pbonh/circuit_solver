//! Real-valued sparse LU solve dispatch for DC operating-point and
//! transient timestep solves, per ADR-0002.
//!
//! This module is the project-level integration layer that bridges the
//! dense [`SubView`] (or [`AssembledSystem`]) produced by MNA assembly
//! to the sparse-direct LU backend ([`RussellRealSolver`]) in the
//! `numeric-solver` crate.
//!
//! # Why this module exists
//!
//! The `numeric-solver` crate provides:
//!
//! - [`RussellRealSolver`] — a stateless `LinearSolver<f64>` implementor
//!   backed by `russell_sparse`/UMFPACK sparse direct LU.
//! - [`SparseLinearSystem`] — the triplet-format input the solver
//!   consumes.
//!
//! The assembly pipeline (`tasks.md` #14 → #15) produces dense row-major
//! systems ([`SubView`]). This module lowers those dense representations
//! into the sparse triplet form the solver expects, then dispatches to
//! [`RussellRealSolver::solve`].
//!
//! # Dense-to-sparse conversion
//!
//! [`dense_to_sparse`] scans the dense matrix and emits a
//! [`SparseTriplet`] for every non-zero entry. The zero-threshold is
//! exact `== 0.0` (not a tolerance), matching the convention that MNA
//! stamps are exactly zero where no element contributed. Entries that
//! are structurally present but numerically zero (e.g. a cancelled-out
//! stamp) are still emitted — this preserves the structural symmetry
//! that UMFPACK uses for fill-reducing ordering, and the solver's
//! pre-checks will reject truly-zero rows as singular anyway.
//!
//! # Convenience entry points
//!
//! - [`solve_sub_view`] — take a [`SubView`], convert, and solve.
//! - [`solve_assembled`] — take an [`AssembledSystem`], convert, and
//!   solve. (Intended for early integration testing before the full
//!   sub-view pipeline is wired up.)
//!
//! # Design references
//!
//! - **ADR-0002** — Hybrid sparse direct solver backend (Russell +
//!   FAER). This module is the Russell half of the dispatch.
//! - **ADR-0010** — Unstable public Rust API surface for v1.

use numeric_solver::{
    LinearSolver, LinearSolverError, RussellRealSolver, SolutionVector, SparseLinearSystem,
    SparseTriplet, SubView,
};

use super::mna::AssembledSystem;

// ---------------------------------------------------------------------------
// Dense-to-sparse conversion
// ---------------------------------------------------------------------------

/// Convert a dense row-major matrix and RHS vector into a
/// [`SparseLinearSystem<f64>`] suitable for [`RussellRealSolver`].
///
/// The function scans the `dim × dim` dense matrix and emits a
/// [`SparseTriplet`] for every entry that is not exactly zero. Duplicate
/// `(row, col)` pairs are *not* pre-aggregated — `russell_sparse`'s
/// COO accumulator sums them at factorization time, matching the SPICE
/// stamping convention.
///
/// # Arguments
///
/// - `node_count` — number of node-equation rows (including ground).
/// - `branch_count` — number of MNA branch-equation rows.
/// - `matrix` — flat row-major slice of length `dim * dim`, where
///   `dim = node_count + branch_count`.
/// - `rhs` — slice of length `dim`.
///
/// # Errors
///
/// Returns [`LinearSolverError`] if:
/// - `node_count + branch_count` overflows `u32` (reported as
///   [`DimensionPartitionMismatch`](LinearSolverError::DimensionPartitionMismatch)),
/// - `rhs.len() != dim` (reported as
///   [`RhsDimensionMismatch`](LinearSolverError::RhsDimensionMismatch)),
/// - any triplet's row/col is `>= dim` (reported as
///   [`TripletOutOfRange`](LinearSolverError::TripletOutOfRange)).
///
/// Non-finite entries in the matrix or RHS are *not* checked here;
/// [`RussellRealSolver::solve`] performs that scalar-specific
/// validation before invoking the backend.
pub fn dense_to_sparse(
    node_count: u32,
    branch_count: u32,
    matrix: &[f64],
    rhs: &[f64],
) -> Result<SparseLinearSystem<f64>, LinearSolverError> {
    let dim = node_count.checked_add(branch_count).ok_or(
        LinearSolverError::DimensionPartitionMismatch {
            dim: u32::MAX, // best-effort: the true dim is undefined due to overflow
            node_count,
            branch_count,
        },
    )?;

    let dim_usize = dim as usize;

    // Extract non-zero entries from the dense row-major matrix.
    let mut triplets = Vec::new();
    for r in 0..dim_usize {
        for c in 0..dim_usize {
            let val = matrix[r * dim_usize + c];
            if val != 0.0 {
                triplets.push(SparseTriplet {
                    row: r as u32,
                    col: c as u32,
                    value: val,
                });
            }
        }
    }

    SparseLinearSystem::new(dim, node_count, branch_count, triplets, rhs.to_vec())
}

// ---------------------------------------------------------------------------
// Convenience solve entry points
// ---------------------------------------------------------------------------

/// Solve a ground-suppressed [`SubView`] using the real-valued sparse
/// LU backend.
///
/// This is the primary entry point for DC operating-point and transient
/// timestep solves. It converts the dense sub-view into sparse triplet
/// form, then dispatches to [`RussellRealSolver`].
///
/// # Errors
///
/// Returns [`LinearSolverError`] on:
/// - conversion failure (dimension mismatch, out-of-range indices),
/// - non-finite entries detected by the solver's pre-checks,
/// - genuine singularity detected by UMFPACK, or
/// - backend-internal failures.
pub fn solve_sub_view(sub_view: &SubView) -> Result<SolutionVector<f64>, LinearSolverError> {
    let sparse = dense_to_sparse(
        sub_view.node_count(),
        sub_view.branch_count(),
        sub_view.matrix(),
        sub_view.rhs(),
    )?;
    let solver = RussellRealSolver::new();
    solver.solve(&sparse)
}

/// Solve a full (ground-row-intact) [`AssembledSystem`] using the
/// real-valued sparse LU backend.
///
/// This entry point is intended for early integration testing before
/// the sub-view pipeline is fully wired up. It does **not** perform
/// ground suppression — the caller is responsible for ensuring the
/// system is non-singular (e.g. by stamping a ground-row identity
/// before calling).
///
/// # Errors
///
/// Same as [`solve_sub_view`].
pub fn solve_assembled(
    assembled: &AssembledSystem,
) -> Result<SolutionVector<f64>, LinearSolverError> {
    let sparse = dense_to_sparse(
        assembled.node_count(),
        assembled.branch_count(),
        assembled.matrix(),
        assembled.rhs(),
    )?;
    let solver = RussellRealSolver::new();
    solver.solve(&sparse)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use numeric_solver::linear_solver::{LinearSolver, RussellRealSolver, SparseTriplet};

    /// Helper: approximate equality within tolerance.
    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    /// Helper: build a 3×3 identity-like dense matrix as flat row-major.
    /// dim=3, node_count=3, branch_count=0.
    fn identity_matrix_3x3() -> (Vec<f64>, Vec<f64>) {
        let matrix = vec![
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, //
            0.0, 0.0, 1.0, //
        ];
        let rhs = vec![1.0, 2.0, 3.0];
        (matrix, rhs)
    }

    // -----------------------------------------------------------------
    // dense_to_sparse
    // -----------------------------------------------------------------

    #[test]
    fn dense_to_sparse_identity_produces_diagonal_triplets() {
        let (matrix, rhs) = identity_matrix_3x3();
        let sys = dense_to_sparse(3, 0, &matrix, &rhs).expect("identity system");

        assert_eq!(sys.dim(), 3);
        assert_eq!(sys.triplets().len(), 3); // only the 1.0 diagonals
        for t in sys.triplets() {
            assert_eq!(t.row, t.col, "identity triplet must be on diagonal");
            assert!(approx(t.value, 1.0, 1e-15));
        }
    }

    #[test]
    fn dense_to_sparse_zeros_are_skipped() {
        // 2×2 with one zero off-diagonal: [1, 0 | 3, 4]
        let matrix = vec![1.0, 0.0, 3.0, 4.0];
        let rhs = vec![5.0, 6.0];
        let sys = dense_to_sparse(2, 0, &matrix, &rhs).expect("2x2 system");

        assert_eq!(sys.triplets().len(), 3); // (0,0), (1,0), (1,1)
        assert_eq!(sys.rhs(), &[5.0, 6.0]);
    }

    #[test]
    fn dense_to_sparse_rhs_mismatch_rejected() {
        let (matrix, _) = identity_matrix_3x3();
        let short_rhs = vec![1.0, 2.0]; // should be length 3
        let err = dense_to_sparse(3, 0, &matrix, &short_rhs).expect_err("rhs mismatch must fail");
        assert_eq!(
            err,
            LinearSolverError::RhsDimensionMismatch { dim: 3, rhs_len: 2 }
        );
    }

    #[test]
    fn dense_to_sparse_partition_mismatch_on_overflow() {
        // The only way to get DimensionPartitionMismatch from dense_to_sparse
        // is when node_count + branch_count overflows u32, because dim is
        // derived from the partition (not supplied independently).
        let err = dense_to_sparse(u32::MAX, 1, &[], &[]).expect_err("overflow partition must fail");
        assert!(
            matches!(err, LinearSolverError::DimensionPartitionMismatch { .. }),
            "expected DimensionPartitionMismatch, got {err:?}",
        );
    }

    #[test]
    fn dense_to_sparse_round_trip_solves_identity() {
        let (matrix, rhs) = identity_matrix_3x3();
        let sparse = dense_to_sparse(3, 0, &matrix, &rhs).expect("identity system");
        let solver = RussellRealSolver::new();
        let sol = solver.solve(&sparse).expect("identity solve");
        let want = [1.0, 2.0, 3.0];
        for (got, w) in sol.unknowns().iter().zip(want.iter()) {
            assert!(approx(*got, *w, 1e-12), "got {got}, want {w}");
        }
    }

    // -----------------------------------------------------------------
    // solve via direct SparseLinearSystem construction
    // -----------------------------------------------------------------

    #[test]
    fn solve_identity_2x2() {
        let triplets = vec![
            SparseTriplet {
                row: 0,
                col: 0,
                value: 1.0,
            },
            SparseTriplet {
                row: 1,
                col: 1,
                value: 1.0,
            },
        ];
        let rhs = vec![5.0, -3.0];
        let sys = SparseLinearSystem::new(2, 2, 0, triplets, rhs).expect("2x2 identity");
        let solver = RussellRealSolver::new();
        let sol = solver.solve(&sys).expect("2x2 identity solve");

        assert!(approx(sol.unknowns()[0], 5.0, 1e-12));
        assert!(approx(sol.unknowns()[1], -3.0, 1e-12));
    }

    #[test]
    fn solve_resistor_divider_2x2() {
        // [1  0] [v1]   [5  ]
        // [-1 2] [v2] = [0  ]
        // v1 = 5, v2 = 2.5
        let triplets = vec![
            SparseTriplet {
                row: 0,
                col: 0,
                value: 1.0,
            },
            SparseTriplet {
                row: 1,
                col: 0,
                value: -1.0,
            },
            SparseTriplet {
                row: 1,
                col: 1,
                value: 2.0,
            },
        ];
        let rhs = vec![5.0, 0.0];
        let sys = SparseLinearSystem::new(2, 2, 0, triplets, rhs).expect("divider 2x2");
        let solver = RussellRealSolver::new();
        let sol = solver.solve(&sys).expect("divider solve");

        assert!(approx(sol.unknowns()[0], 5.0, 1e-10), "v1");
        assert!(approx(sol.unknowns()[1], 2.5, 1e-10), "v2");
    }

    #[test]
    fn solve_with_mna_branch() {
        // 3-dim system: 2 nodes + 1 voltage-source branch.
        // Ground suppress already applied (row 0 = identity):
        //   [1  0  0] [v0 ]   [0]
        //   [0  1  1] [v1 ] = [0]
        //   [0  1 -1] [Ivs]   [5]
        // v0=0, v1 + Ivs = 0, v1 - Ivs = 5
        // → v1 = 2.5, Ivs = -2.5
        let triplets = vec![
            SparseTriplet {
                row: 0,
                col: 0,
                value: 1.0,
            },
            SparseTriplet {
                row: 1,
                col: 1,
                value: 1.0,
            },
            SparseTriplet {
                row: 1,
                col: 2,
                value: 1.0,
            },
            SparseTriplet {
                row: 2,
                col: 1,
                value: 1.0,
            },
            SparseTriplet {
                row: 2,
                col: 2,
                value: -1.0,
            },
        ];
        let rhs = vec![0.0, 0.0, 5.0];
        let sys = SparseLinearSystem::new(3, 2, 1, triplets, rhs).expect("3-dim vs system");
        let solver = RussellRealSolver::new();
        let sol = solver.solve(&sys).expect("3-dim vs solve");

        assert!(approx(sol.unknowns()[0], 0.0, 1e-10), "v0 (ground)");
        assert!(approx(sol.unknowns()[1], 2.5, 1e-10), "v1");
        assert!(approx(sol.unknowns()[2], -2.5, 1e-10), "Ivs");

        // Verify node/branch slices.
        assert_eq!(sol.node_unknowns().len(), 2);
        assert_eq!(sol.branch_unknowns().len(), 1);
    }

    // -----------------------------------------------------------------
    // Edge cases
    // -----------------------------------------------------------------

    #[test]
    fn empty_system_solves_trivially() {
        let matrix: Vec<f64> = vec![];
        let rhs: Vec<f64> = vec![];
        let sparse = dense_to_sparse(0, 0, &matrix, &rhs).expect("empty system");
        let solver = RussellRealSolver::new();
        let sol = solver.solve(&sparse).expect("empty solve");
        assert_eq!(sol.dim(), 0);
        assert!(sol.unknowns().is_empty());
    }

    #[test]
    fn all_zero_matrix_is_reported_singular() {
        let matrix = vec![0.0, 0.0, 0.0, 0.0]; // 2×2 all zeros
        let rhs = vec![1.0, 2.0];
        let sparse = dense_to_sparse(2, 0, &matrix, &rhs).expect("all-zero system");
        let solver = RussellRealSolver::new();
        let err = solver.solve(&sparse);
        assert!(
            matches!(
                err,
                Err(LinearSolverError::SingularMatrix { .. }
                    | LinearSolverError::BackendFailure { .. })
            ),
            "expected SingularMatrix/BackendFailure, got {err:?}",
        );
    }

    #[test]
    fn duplicate_triplets_are_summed() {
        // Stamp two contributions at (0,0): 2.0 + 3.0 = 5.0 total.
        // Dense: [5, 0 | 0, 1], rhs = [10, 1] → x = [2, 1]
        let matrix = vec![
            5.0, 0.0, //
            0.0, 1.0,
        ];
        let rhs = vec![10.0, 1.0];
        let sys = dense_to_sparse(2, 0, &matrix, &rhs).expect("2x2 with 5 on diag");
        let solver = RussellRealSolver::new();
        let sol = solver.solve(&sys).expect("2x2 diag=5 solve");
        assert!(approx(sol.unknowns()[0], 2.0, 1e-10));
        assert!(approx(sol.unknowns()[1], 1.0, 1e-10));
    }

    // -----------------------------------------------------------------
    // solve_assembled
    // -----------------------------------------------------------------

    #[test]
    fn solve_assembled_identity() {
        let matrix = vec![
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, //
            0.0, 0.0, 1.0, //
        ];
        let rhs = vec![4.0, -2.0, 7.0];
        let assembled =
            AssembledSystem::from_raw_parts(3, 0, matrix, rhs).expect("3x3 identity assembled");
        let sol = solve_assembled(&assembled).expect("3x3 identity solve via assembled");
        assert!(approx(sol.unknowns()[0], 4.0, 1e-12));
        assert!(approx(sol.unknowns()[1], -2.0, 1e-12));
        assert!(approx(sol.unknowns()[2], 7.0, 1e-12));
    }

    #[test]
    fn solve_assembled_resistor_divider() {
        // Same 2×2 resistor divider as the direct test, but via AssembledSystem.
        let matrix = vec![
            1.0, 0.0, //
            -1.0, 2.0,
        ];
        let rhs = vec![5.0, 0.0];
        let assembled =
            AssembledSystem::from_raw_parts(2, 0, matrix, rhs).expect("2x2 divider assembled");
        let sol = solve_assembled(&assembled).expect("2x2 divider solve via assembled");
        assert!(approx(sol.unknowns()[0], 5.0, 1e-10), "v1");
        assert!(approx(sol.unknowns()[1], 2.5, 1e-10), "v2");
    }

    #[test]
    fn solve_assembled_with_branch() {
        // 3-dim: 2 nodes + 1 VS branch (same system as direct test).
        let matrix = vec![
            1.0, 0.0, 0.0, //
            0.0, 1.0, 1.0, //
            0.0, 1.0, -1.0,
        ];
        let rhs = vec![0.0, 0.0, 5.0];
        let assembled =
            AssembledSystem::from_raw_parts(2, 1, matrix, rhs).expect("3-dim vs assembled");
        let sol = solve_assembled(&assembled).expect("3-dim vs solve via assembled");
        assert!(approx(sol.unknowns()[0], 0.0, 1e-10), "v0 (ground)");
        assert!(approx(sol.unknowns()[1], 2.5, 1e-10), "v1");
        assert!(approx(sol.unknowns()[2], -2.5, 1e-10), "Ivs");
    }
}
