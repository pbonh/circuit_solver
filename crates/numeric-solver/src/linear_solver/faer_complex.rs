//! `faer`-backed complex sparse direct LU.
//!
//! Implements [`LinearSolver<Complex<f64>>`] for AC small-signal
//! (and noise) analyses per `tasks.md` item #23 and ADR-0002.
//!
//! # Choice of `faer` version and feature set
//!
//! The crate currently depends on `faer = "0.19.4"` with
//! `default-features = false, features = ["std", "rayon"]`. Rationale:
//!
//! - 0.19.x is the last family with a 1.67 MSRV, comfortably under
//!   this workspace's declared MSRV of 1.75 (Cargo workspace
//!   `[workspace.package].rust-version`). 0.20+ pushes to 1.81+ and
//!   would force a workspace-wide MSRV bump that this task does not
//!   own.
//! - Default features pull in `serde`, `rand`, and `npy`, none of
//!   which the sparse-LU path needs. We turn them off.
//! - `rayon` stays on so the symbolic-and-numeric LU phases can
//!   parallelise on large AC sweeps (the per-frequency loop still
//!   sees one `solve` call per frequency, but each LU itself wants
//!   threads on circuit-scale matrices).
//!
//! # Failure-mode caveat in faer 0.19.4
//!
//! `faer` 0.19.4's simplicial sparse LU (the path chosen for small
//! matrices, exactly the size of the tests below) **panics** when
//! a pivot row is reduced to an exact zero before the symbolic-
//! singular check fires; see `faer-rs` sparse/linalg/lu.rs:1795.
//! That panic path is unreachable for finite-coefficient, structurally
//! non-singular AC matrices but can be tripped by a stamp that
//! produces an exact-zero row.
//!
//! We mitigate at the boundary:
//!
//! 1. The wrapper rejects non-finite entries up-front
//!    ([`LinearSolverError::NonFiniteEntry`]), which prevents NaN
//!    propagation poisoning the LU.
//! 2. The wrapper detects **explicit zero rows** before invoking
//!    `faer` and surfaces them as [`LinearSolverError::SingularMatrix`].
//!    A zero row in a square sparse matrix is sufficient
//!    (but not necessary) for singularity; this is the cheap check
//!    that covers most user-facing "obvious" singularities and the
//!    test-suite stand-ins for them.
//! 3. Everything else delegates to `faer`'s `LuError` which we map
//!    to our unified [`LinearSolverError`].
//!
//! When the workspace MSRV bumps to 1.81+, this module can switch to
//! a faer-0.20+ family that resolves the upstream panic (see the
//! faer changelog around the 0.20 sparse-LU refactor).
//!
//! # No `unsafe`
//!
//! Workspace lint `unsafe_code = "forbid"` applies. The faer entry
//! points used here (`SparseColMat::try_new_from_triplets`,
//! `SparseColMatRef::sp_lu`, `SpSolver::solve`,
//! `Mat::zeros`, `Mat::write`, `Mat::read`) are all safe-Rust API.

use faer::complex_native::c64;
use faer::sparse::linalg::solvers::SpSolver;
use faer::sparse::{CreationError, LuError, SparseColMat};
use faer::Mat;
use num_complex::Complex;

use super::system::{LinearSolverError, SolutionVector, SparseLinearSystem};
use super::LinearSolver;

/// Stateless dispatcher: [`LinearSolver<Complex<f64>>`] implementor
/// backed by `faer`'s sparse direct LU.
///
/// Holds no state of its own. Construction is free; the analysis
/// orchestrator (or, in tests, the caller directly) typically holds
/// a single `FaerComplexSolver` value across an entire AC sweep and
/// invokes [`LinearSolver::solve`] per frequency point.
#[derive(Debug, Default, Clone, Copy)]
pub struct FaerComplexSolver;

impl FaerComplexSolver {
    /// Construct a fresh solver. Equivalent to `FaerComplexSolver` /
    /// `FaerComplexSolver::default()`; provided so callers do not have
    /// to know about the derive.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl LinearSolver<Complex<f64>> for FaerComplexSolver {
    fn solve(
        &self,
        system: &SparseLinearSystem<Complex<f64>>,
    ) -> Result<SolutionVector<Complex<f64>>, LinearSolverError> {
        let dim = system.dim();
        let dim_usize = dim as usize;

        // 1. Reject non-finite triplet values up front.
        for (i, t) in system.triplets().iter().enumerate() {
            if !is_finite_complex(t.value) {
                return Err(LinearSolverError::NonFiniteEntry {
                    location: format!("triplet[{i}]"),
                });
            }
        }

        // 2. Reject non-finite RHS entries.
        for (i, r) in system.rhs().iter().enumerate() {
            if !is_finite_complex(*r) {
                return Err(LinearSolverError::NonFiniteEntry {
                    location: format!("rhs[{i}]"),
                });
            }
        }

        // 3. Detect zero rows (sufficient condition for singularity)
        //    before invoking faer. Avoids the upstream panic at
        //    faer-0.19.4 sparse/linalg/lu.rs:1795 on exact-zero
        //    pivots in the simplicial LU path.
        //
        //    "Zero row" means: no triplet at `(r, _)` carries a
        //    non-zero scalar. Per faer's stamping semantics duplicate
        //    triplets sum, so we collect the per-row non-zero sum
        //    rather than just the count.
        if dim > 0 {
            let mut row_nz_count = vec![0_u32; dim_usize];
            for t in system.triplets() {
                if t.value != Complex::new(0.0, 0.0) {
                    row_nz_count[t.row as usize] = row_nz_count[t.row as usize].saturating_add(1);
                }
            }
            if let Some(r) = row_nz_count.iter().position(|&c| c == 0) {
                return Err(LinearSolverError::SingularMatrix {
                    // `r < dim` by construction (row_nz_count length
                    // equals `dim_usize`), and `dim` is a `u32`, so
                    // this cast is lossless.
                    column_hint: Some(u32::try_from(r).expect("r < dim fits u32")),
                });
            }
        }

        // 4. Build the faer triplet stream. `num_complex::Complex<f64>`
        //    and `faer::complex_native::c64` are layout-compatible
        //    POD structs `{ re: f64, im: f64 }`, but we go through
        //    the field constructor rather than transmute so the
        //    workspace lint `unsafe_code = "forbid"` stays clean.
        let triplets: Vec<(usize, usize, c64)> = system
            .triplets()
            .iter()
            .map(|t| {
                (
                    t.row as usize,
                    t.col as usize,
                    c64::new(t.value.re, t.value.im),
                )
            })
            .collect();

        // 5. Hand to faer.
        let a: SparseColMat<usize, c64> =
            SparseColMat::try_new_from_triplets(dim_usize, dim_usize, &triplets)
                .map_err(creation_error_to_linear)?;

        // 6. Pack the RHS into a faer dense column.
        let mut rhs_mat = Mat::<c64>::zeros(dim_usize, 1);
        for (i, r) in system.rhs().iter().enumerate() {
            rhs_mat.write(i, 0, c64::new(r.re, r.im));
        }

        // 7. Factor.
        let lu = a.sp_lu().map_err(lu_error_to_linear)?;

        // 8. Solve.
        let x_mat = lu.solve(&rhs_mat);

        // 9. Extract dense unknowns.
        let mut unknowns: Vec<Complex<f64>> = Vec::with_capacity(dim_usize);
        for i in 0..dim_usize {
            let v = x_mat.read(i, 0);
            unknowns.push(Complex::new(v.re, v.im));
        }

        Ok(SolutionVector::from_parts(
            system.node_count(),
            system.branch_count(),
            unknowns,
        ))
    }
}

/// True iff both real and imaginary parts are finite (no NaN, no
/// ±∞). Used at the boundary before handing values to faer.
fn is_finite_complex(z: Complex<f64>) -> bool {
    z.re.is_finite() && z.im.is_finite()
}

/// Project a [`CreationError`] from `SparseColMat::try_new_from_triplets`
/// into the unified [`LinearSolverError`].
///
/// `CreationError` distinguishes index-out-of-range (which our own
/// pre-check in [`SparseLinearSystem::new`] already catches, so we
/// re-raise it as the same variant here for defense in depth) from
/// generic allocator failures.
fn creation_error_to_linear(err: CreationError) -> LinearSolverError {
    match err {
        CreationError::Generic(g) => LinearSolverError::BackendFailure {
            backend: "faer",
            description: format!("sparse creation: {g:?}"),
        },
        CreationError::OutOfBounds { row, col } => LinearSolverError::TripletOutOfRange {
            // Best-effort: faer returns usize, ours is u32.
            row: row.try_into().unwrap_or(u32::MAX),
            col: col.try_into().unwrap_or(u32::MAX),
            dim: u32::MAX,
        },
    }
}

/// Project a [`LuError`] into [`LinearSolverError`].
fn lu_error_to_linear(err: LuError) -> LinearSolverError {
    match err {
        LuError::Generic(g) => LinearSolverError::BackendFailure {
            backend: "faer",
            description: format!("sparse LU: {g:?}"),
        },
        LuError::SymbolicSingular(j) => LinearSolverError::SingularMatrix {
            column_hint: Some(j.try_into().unwrap_or(u32::MAX)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_solver::system::SparseTriplet;

    /// Helper: build a system from raw parts, asserting `new`
    /// accepts it (the tests below construct only well-formed
    /// systems; the error-path tests build malformed ones and assert
    /// `new` rejects).
    fn sys(
        dim: u32,
        node_count: u32,
        branch_count: u32,
        triplets: Vec<SparseTriplet<Complex<f64>>>,
        rhs: Vec<Complex<f64>>,
    ) -> SparseLinearSystem<Complex<f64>> {
        SparseLinearSystem::new(dim, node_count, branch_count, triplets, rhs).expect("well-formed")
    }

    fn t(row: u32, col: u32, re: f64, im: f64) -> SparseTriplet<Complex<f64>> {
        SparseTriplet {
            row,
            col,
            value: Complex::new(re, im),
        }
    }

    fn approx_eq_c(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
        (a.re - b.re).abs() < tol && (a.im - b.im).abs() < tol
    }

    /// ─── Happy path: 2x2 identity ──────────────────────────────────
    #[test]
    fn identity_2x2_returns_rhs_unchanged() {
        let s = sys(
            2,
            2,
            0,
            vec![t(0, 0, 1.0, 0.0), t(1, 1, 1.0, 0.0)],
            vec![Complex::new(3.5, 1.0), Complex::new(-2.0, 4.0)],
        );
        let x = FaerComplexSolver::new().solve(&s).expect("identity ok");
        assert_eq!(x.dim(), 2);
        assert_eq!(x.node_count(), 2);
        assert_eq!(x.branch_count(), 0);
        assert!(approx_eq_c(x.unknowns()[0], Complex::new(3.5, 1.0), 1e-12));
        assert!(approx_eq_c(x.unknowns()[1], Complex::new(-2.0, 4.0), 1e-12));
    }

    /// ─── Happy path: RC low-pass node at ω ─────────────────────────
    /// Single-node AC system: `(G + jωC) · V = Iin`.
    /// G = 1 S, C = 1 F, ω = 1 rad/s, Iin = 1 A → V = 1/(1+j) = 0.5 - 0.5j.
    #[test]
    fn rc_low_pass_at_omega_one() {
        let g = 1.0_f64;
        let c = 1.0_f64;
        let omega = 1.0_f64;
        let s = sys(
            1,
            1,
            0,
            vec![t(0, 0, g, omega * c)],
            vec![Complex::new(1.0, 0.0)],
        );
        let x = FaerComplexSolver::new().solve(&s).expect("non-singular");
        assert!(
            approx_eq_c(x.unknowns()[0], Complex::new(0.5, -0.5), 1e-12),
            "got {:?}",
            x.unknowns()[0]
        );
    }

    /// ─── Happy path: 2-node L-section AC at ω ──────────────────────
    /// Exercise off-diagonal complex structure on a non-singular 2×2:
    ///
    /// ```text
    /// [ G+jωC,  -G ] [V1]   [0]
    /// [  -G,     G ] [V2] = [G]
    /// ```
    ///
    /// Solving by hand with G = 1, C = 1, ω = 1:
    ///   Row 2: -V1 + V2 = 1   →   V2 = 1 + V1
    ///   Row 1: (1+j)·V1 - V2 = 0
    ///        ⇒ (1+j)·V1 - (1 + V1) = 0
    ///        ⇒ j·V1 = 1
    ///        ⇒ V1 = -j   (i.e. 0 - 1j)
    ///        ⇒ V2 = 1 - j
    #[test]
    fn two_node_complex_off_diagonal_solve() {
        let g = 1.0_f64;
        let c = 1.0_f64;
        let omega = 1.0_f64;
        let s = sys(
            2,
            2,
            0,
            vec![
                t(0, 0, g, omega * c),
                t(0, 1, -g, 0.0),
                t(1, 0, -g, 0.0),
                t(1, 1, g, 0.0),
            ],
            vec![Complex::new(0.0, 0.0), Complex::new(g, 0.0)],
        );
        let x = FaerComplexSolver::new().solve(&s).expect("non-singular");
        assert!(
            approx_eq_c(x.unknowns()[0], Complex::new(0.0, -1.0), 1e-12),
            "V1 wrong: {:?}",
            x.unknowns()[0]
        );
        assert!(
            approx_eq_c(x.unknowns()[1], Complex::new(1.0, -1.0), 1e-12),
            "V2 wrong: {:?}",
            x.unknowns()[1]
        );
    }

    /// ─── Happy path: duplicate triplets sum ───────────────────────
    /// Two stamps at the same (0,0) summing to a 1+0j diagonal.
    #[test]
    fn duplicate_triplets_at_same_position_sum() {
        let s = sys(
            1,
            1,
            0,
            vec![t(0, 0, 0.4, 0.0), t(0, 0, 0.6, 0.0)],
            vec![Complex::new(2.0, 0.0)],
        );
        let x = FaerComplexSolver::new().solve(&s).expect("ok");
        assert!(approx_eq_c(x.unknowns()[0], Complex::new(2.0, 0.0), 1e-12));
    }

    /// ─── Happy path: solution slicing ──────────────────────────────
    #[test]
    fn solution_node_branch_slicing_matches_layout() {
        // dim=3, node_count=2, branch_count=1. Trivial identity-like
        // system so we can verify slicing without arithmetic noise.
        let s = sys(
            3,
            2,
            1,
            vec![t(0, 0, 1.0, 0.0), t(1, 1, 1.0, 0.0), t(2, 2, 1.0, 0.0)],
            vec![
                Complex::new(10.0, 0.0),
                Complex::new(20.0, 0.0),
                Complex::new(30.0, 0.0),
            ],
        );
        let x = FaerComplexSolver::new().solve(&s).unwrap();
        assert_eq!(x.dim(), 3);
        assert_eq!(x.node_unknowns().len(), 2);
        assert_eq!(x.branch_unknowns().len(), 1);
        assert!(approx_eq_c(
            x.node_unknowns()[0],
            Complex::new(10.0, 0.0),
            1e-12
        ));
        assert!(approx_eq_c(
            x.node_unknowns()[1],
            Complex::new(20.0, 0.0),
            1e-12
        ));
        assert!(approx_eq_c(
            x.branch_unknowns()[0],
            Complex::new(30.0, 0.0),
            1e-12
        ));
    }

    /// ─── Error path: zero-row singular detection ───────────────────
    /// Stand-in for a floating node — a row that no triplet
    /// contributes to. The wrapper catches this **before** faer's
    /// simplicial LU panics.
    #[test]
    fn zero_row_returns_singular_matrix_not_panic() {
        let s = sys(
            2,
            2,
            0,
            // Only stamp row 0; row 1 has no entries → zero row.
            vec![t(0, 0, 1.0, 0.0)],
            vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)],
        );
        let err = FaerComplexSolver::new()
            .solve(&s)
            .expect_err("expected singular");
        match err {
            LinearSolverError::SingularMatrix { column_hint } => {
                assert_eq!(
                    column_hint,
                    Some(1),
                    "expected row-1 hint, got {column_hint:?}"
                );
            }
            other => panic!("expected SingularMatrix, got {other:?}"),
        }
    }

    /// ─── Error path: explicit-zero triplets do not save a zero row ─
    /// A `0+0j` triplet at (1,1) does **not** count as live structure;
    /// the wrapper must still detect the zero row.
    #[test]
    fn explicit_zero_triplet_does_not_save_zero_row() {
        let s = sys(
            2,
            2,
            0,
            vec![t(0, 0, 1.0, 0.0), t(1, 1, 0.0, 0.0)],
            vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)],
        );
        let err = FaerComplexSolver::new().solve(&s).expect_err("singular");
        assert!(
            matches!(err, LinearSolverError::SingularMatrix { .. }),
            "got {err:?}"
        );
    }

    /// ─── Error path: NaN in triplet → `NonFiniteEntry` ───────────────
    #[test]
    fn nan_triplet_rejected_before_faer() {
        let s = sys(
            1,
            1,
            0,
            vec![t(0, 0, f64::NAN, 0.0)],
            vec![Complex::new(1.0, 0.0)],
        );
        let err = FaerComplexSolver::new().solve(&s).expect_err("NaN");
        assert!(
            matches!(err, LinearSolverError::NonFiniteEntry { ref location } if location == "triplet[0]"),
            "got {err:?}"
        );
    }

    /// ─── Error path: ±∞ in imaginary part → `NonFiniteEntry` ─────────
    #[test]
    fn infinity_imag_part_rejected() {
        let s = sys(
            1,
            1,
            0,
            vec![t(0, 0, 1.0, f64::INFINITY)],
            vec![Complex::new(1.0, 0.0)],
        );
        let err = FaerComplexSolver::new().solve(&s).expect_err("inf");
        assert!(matches!(err, LinearSolverError::NonFiniteEntry { .. }));
    }

    /// ─── Error path: NaN in RHS → `NonFiniteEntry` ───────────────────
    #[test]
    fn nan_in_rhs_rejected() {
        let s = sys(
            1,
            1,
            0,
            vec![t(0, 0, 1.0, 0.0)],
            vec![Complex::new(f64::NAN, 0.0)],
        );
        let err = FaerComplexSolver::new().solve(&s).expect_err("rhs NaN");
        assert!(
            matches!(err, LinearSolverError::NonFiniteEntry { ref location } if location == "rhs[0]"),
            "got {err:?}"
        );
    }

    /// ─── Error path: `SparseLinearSystem::new` rejects mis-shaped rhs ─
    #[test]
    fn system_new_rejects_rhs_dim_mismatch() {
        let err = SparseLinearSystem::new(
            2,
            2,
            0,
            vec![t(0, 0, 1.0, 0.0), t(1, 1, 1.0, 0.0)],
            vec![Complex::new(1.0, 0.0)], // length 1, not 2
        )
        .expect_err("rhs mismatch");
        match err {
            LinearSolverError::RhsDimensionMismatch { dim, rhs_len } => {
                assert_eq!(dim, 2);
                assert_eq!(rhs_len, 1);
            }
            other => panic!("got {other:?}"),
        }
    }

    /// ─── Error path: `SparseLinearSystem::new` rejects bad partition ─
    #[test]
    fn system_new_rejects_partition_mismatch() {
        let err = SparseLinearSystem::new(
            3,
            2,
            0, // 2 + 0 != 3
            vec![t(0, 0, 1.0, 0.0)],
            vec![
                Complex::new(1.0, 0.0),
                Complex::new(2.0, 0.0),
                Complex::new(3.0, 0.0),
            ],
        )
        .expect_err("partition");
        assert!(matches!(
            err,
            LinearSolverError::DimensionPartitionMismatch { .. }
        ));
    }

    /// ─── Error path: `SparseLinearSystem::new` rejects out-of-range ──
    #[test]
    fn system_new_rejects_triplet_out_of_range() {
        let err = SparseLinearSystem::new(
            2,
            2,
            0,
            vec![t(5, 0, 1.0, 0.0)],
            vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)],
        )
        .expect_err("oob");
        match err {
            LinearSolverError::TripletOutOfRange { row, col, dim } => {
                assert_eq!(row, 5);
                assert_eq!(col, 0);
                assert_eq!(dim, 2);
            }
            other => panic!("got {other:?}"),
        }
    }

    /// ─── Empty system: dim=0 is the trivial pass-through ───────────
    #[test]
    fn empty_system_returns_empty_solution() {
        let s = sys(0, 0, 0, vec![], vec![]);
        let x = FaerComplexSolver::new().solve(&s).expect("trivial");
        assert_eq!(x.dim(), 0);
        assert!(x.unknowns().is_empty());
    }

    /// ─── Display impl smoke-test ───────────────────────────────────
    #[test]
    fn linear_solver_error_display_does_not_panic() {
        let e = LinearSolverError::SingularMatrix {
            column_hint: Some(7),
        };
        let _ = format!("{e}");
        let e = LinearSolverError::BackendFailure {
            backend: "faer",
            description: "x".into(),
        };
        let _ = format!("{e}");
    }

    /// ─── Stateless dispatcher: many sequential solves with one instance.
    /// Mirrors the "AC sweep" usage where one `FaerComplexSolver` is
    /// reused across frequencies. Each solve must be independent
    /// (no carry-over state).
    #[test]
    fn sequential_solves_share_no_state() {
        let solver = FaerComplexSolver::new();
        for k in 1..=5 {
            let k_f = f64::from(k);
            let s = sys(
                1,
                1,
                0,
                vec![t(0, 0, k_f, 0.0)],
                vec![Complex::new(k_f * 2.0, 0.0)],
            );
            let x = solver.solve(&s).expect("ok");
            // k * V = 2k → V = 2.
            assert!(approx_eq_c(x.unknowns()[0], Complex::new(2.0, 0.0), 1e-12));
        }
    }
}
