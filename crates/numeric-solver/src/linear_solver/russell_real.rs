//! `russell_sparse`-backed real sparse direct LU.
//!
//! Implements [`LinearSolver<f64>`](super::LinearSolver) for DC
//! operating-point and transient timestep solves per `tasks.md`
//! item #16 and ADR-0002.
//!
//! # Choice of backend
//!
//! ADR-0002 commits to *`russell_sparse` for the real-valued half of
//! the hybrid backend*. We wrap [`russell_sparse::SolverUMFPACK`]
//! (UMFPACK over MUMPS) because:
//!
//! - UMFPACK is the recommended `russell_sparse` driver for circuit-
//!   style square sparse-LU workloads; MUMPS requires an MPI / Fortran
//!   toolchain that we do not want to push onto every developer.
//! - UMFPACK is part of `SuiteSparse`, which is broadly available
//!   across Linux package managers and Homebrew. The workspace
//!   `.cargo/config.toml` populates `CPATH` / `LIBRARY_PATH` /
//!   `RUNPATH` search lists for the four common install layouts
//!   (linuxbrew, macOS brew, `/usr/local`, system) so the build does
//!   not require callers to pre-set `LD_LIBRARY_PATH`.
//!
//! # Per-call factorization
//!
//! Each [`RussellRealSolver::solve`] call performs a fresh symbolic
//! and numeric factorization through `russell_sparse`'s `CooMatrix`
//! → `SolverUMFPACK::factorize` → `SolverUMFPACK::solve` pipeline.
//! `russell_sparse` documents that *"if the structure of the matrix
//! needs to be changed, the solver must be 'dropped' and a new
//! solver allocated"*. Because Newton-Raphson iterates (`tasks.md`
//! #17), DC homotopies (`tasks.md` #18 / #19), and transient time-
//! step changes can all rebuild the matrix between calls, we
//! allocate a fresh backend instance per `solve` rather than caching
//! one. A factorization-reuse layer is out of scope for #16 and
//! lands above the trait in the Newton driver task.
//!
//! # Failure-mode handling
//!
//! Caller-side errors (non-finite triplets, non-finite RHS,
//! dimension partition mismatch, out-of-range triplets, RHS length
//! mismatch) are caught here in pre-checks so the backend only ever
//! sees structurally and numerically clean inputs. `SparseLinearSystem`
//! already validates dim / RHS length / triplet ranges at construction;
//! we add the scalar-type-specific check (`f64::is_finite`) that
//! `system.rs` cannot perform without knowing `Scalar`.
//!
//! `russell_sparse`'s `factorize` returns `&'static str` diagnostics
//! that vary across `SuiteSparse` versions. We classify them
//! heuristically: any message containing `"singular"`, `"zero pivot"`,
//! or `"rank"` maps to [`LinearSolverError::SingularMatrix`]; everything
//! else maps to [`LinearSolverError::BackendFailure`]. The string
//! heuristic is documented as advisory in ADR-0002's discussion of
//! the convergence-failure path (`tasks.md` #22).
//!
//! # `dim == 0`
//!
//! `russell_sparse::CooMatrix::new` rejects `dim == 0`. We short-
//! circuit before invoking the backend and return an empty
//! [`SolutionVector`]. Empty systems are vacuously solvable
//! (`A · x = b` with all-zero dimension is satisfied by any vector,
//! and we return the canonical empty one), and the alternative —
//! erroring out — would leak a russell-internal precondition into
//! the unified trait surface.
//!
//! # No `unsafe`
//!
//! Workspace lint `unsafe_code = "forbid"` applies. The
//! `russell_sparse` / `russell_lab` entry points used here
//! (`CooMatrix::new`, `CooMatrix::put`, `SolverUMFPACK::new`,
//! `LinSolTrait::factorize`, `LinSolTrait::solve`, `Vector::new`,
//! `Vector::initialized`, `Vector::as_data`) are all safe-Rust API.

use russell_lab::Vector;
use russell_sparse::{CooMatrix, LinSolTrait, SolverUMFPACK, Sym};

use super::system::{LinearSolverError, SolutionVector, SparseLinearSystem};
use super::LinearSolver;

/// Stateless dispatcher: [`LinearSolver<f64>`] implementor backed by
/// `russell_sparse`'s UMFPACK sparse direct LU.
///
/// Holds no state of its own. Construction is free; the analysis
/// orchestrator (or, in tests, the caller directly) typically holds
/// a single `RussellRealSolver` value across an entire DC / transient
/// run and invokes [`LinearSolver::solve`] per Newton-Raphson iterate
/// or per timestep.
///
/// # Stability
///
/// Per ADR-0010 this type is part of the v1 *unstable* public Rust
/// API surface. The shape may change between v1.x.
#[derive(Debug, Default, Clone, Copy)]
pub struct RussellRealSolver;

impl RussellRealSolver {
    /// Construct a fresh solver. Equivalent to `RussellRealSolver` /
    /// `RussellRealSolver::default()`; provided so callers do not
    /// have to know about the derive.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl LinearSolver<f64> for RussellRealSolver {
    fn solve(
        &self,
        system: &SparseLinearSystem<f64>,
    ) -> Result<SolutionVector<f64>, LinearSolverError> {
        let dim = system.dim();
        let dim_usize = dim as usize;

        // 1. Trivial case: `dim == 0`. `russell_sparse::CooMatrix::new`
        //    rejects zero-dim matrices; the trait does not, so we
        //    short-circuit. Empty system is vacuously solved by the
        //    empty vector.
        if dim == 0 {
            return Ok(SolutionVector::from_parts(
                system.node_count(),
                system.branch_count(),
                Vec::new(),
            ));
        }

        // 2. Reject non-finite triplet values up front. (Index range
        //    and dimension partition are already enforced by
        //    `SparseLinearSystem::new`.)
        for (i, t) in system.triplets().iter().enumerate() {
            if !t.value.is_finite() {
                return Err(LinearSolverError::NonFiniteEntry {
                    location: format!("triplet[{i}]"),
                });
            }
        }

        // 3. Reject non-finite RHS entries.
        for (i, r) in system.rhs().iter().enumerate() {
            if !r.is_finite() {
                return Err(LinearSolverError::NonFiniteEntry {
                    location: format!("rhs[{i}]"),
                });
            }
        }

        // 4. Build the COO matrix.
        //
        //    `russell_sparse::CooMatrix` requires the caller to
        //    declare an upper bound on the number of stored entries
        //    up front, and rejects `nnz_max == 0`. Caller-supplied
        //    triplets may legitimately number zero (a fully-zero
        //    matrix is structurally singular but well-defined input),
        //    so we pad to `nnz_max >= 1` by stamping a single
        //    explicit zero at `(0, 0)`. Duplicate triplets are
        //    *summed* by russell at factorization time, matching
        //    SPICE-style stamp accumulation, so we never pre-aggregate.
        let nnz_max = system.triplets().len().max(1);
        let mut coo = CooMatrix::new(dim_usize, dim_usize, nnz_max, Sym::No).map_err(|e| {
            LinearSolverError::BackendFailure {
                backend: "russell",
                description: format!("CooMatrix::new: {e}"),
            }
        })?;

        if system.triplets().is_empty() {
            coo.put(0, 0, 0.0)
                .map_err(|e| LinearSolverError::BackendFailure {
                    backend: "russell",
                    description: format!("CooMatrix::put pad: {e}"),
                })?;
        } else {
            for t in system.triplets() {
                coo.put(t.row as usize, t.col as usize, t.value)
                    .map_err(|e| LinearSolverError::BackendFailure {
                        backend: "russell",
                        description: format!("CooMatrix::put: {e}"),
                    })?;
            }
        }

        // 5. Factorize and solve.
        let mut backend = SolverUMFPACK::new().map_err(|e| LinearSolverError::BackendFailure {
            backend: "russell",
            description: format!("SolverUMFPACK::new: {e}"),
        })?;
        backend
            .factorize(&coo, None)
            .map_err(classify_factorize_error)?;

        // Pack the RHS into a russell `Vector`. `Vector::initialized`
        // takes a closure producing each element.
        let rhs_slice = system.rhs();
        let rhs_vec = Vector::initialized(dim_usize, |i| rhs_slice[i]);
        let mut x = Vector::new(dim_usize);
        backend
            .solve(&mut x, &rhs_vec, false)
            .map_err(|e| LinearSolverError::BackendFailure {
                backend: "russell",
                description: format!("SolverUMFPACK::solve: {e}"),
            })?;

        Ok(SolutionVector::from_parts(
            system.node_count(),
            system.branch_count(),
            x.as_data().clone(),
        ))
    }
}

/// Heuristic mapping from `russell_sparse`'s string-typed `factorize`
/// errors to the structural variants of [`LinearSolverError`].
///
/// `russell_sparse` returns `&'static str` diagnostics that vary
/// across `SuiteSparse` versions. We classify the ones that carry
/// singularity information so callers (notably the DC convergence-
/// failure path, `tasks.md` #22) can react structurally rather than
/// parsing strings themselves; everything else is reported as a
/// generic backend failure with the original diagnostic preserved.
fn classify_factorize_error(msg: &str) -> LinearSolverError {
    let lower = msg.to_ascii_lowercase();
    if lower.contains("singular") || lower.contains("zero pivot") || lower.contains("rank") {
        LinearSolverError::SingularMatrix { column_hint: None }
    } else {
        LinearSolverError::BackendFailure {
            backend: "russell",
            description: msg.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_solver::system::SparseTriplet;

    /// Helper: round-equal within tolerance.
    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    /// Helper: build a well-formed system from raw parts, asserting
    /// `SparseLinearSystem::new` accepts it.
    fn sys(
        dim: u32,
        node_count: u32,
        branch_count: u32,
        triplets: Vec<SparseTriplet<f64>>,
        rhs: Vec<f64>,
    ) -> SparseLinearSystem<f64> {
        SparseLinearSystem::new(dim, node_count, branch_count, triplets, rhs)
            .expect("well-formed system")
    }

    fn t(row: u32, col: u32, value: f64) -> SparseTriplet<f64> {
        SparseTriplet { row, col, value }
    }

    // -----------------------------------------------------------------
    // Structural validation
    //
    // `SparseLinearSystem::new` performs dim/rhs/index range checks
    // up-front; the russell wrapper performs only the scalar-type-
    // specific finite check (and the `dim == 0` short-circuit).
    // -----------------------------------------------------------------

    #[test]
    fn empty_system_is_solved_trivially() {
        // `dim == 0` is a well-formed (if vacuous) system. The
        // wrapper returns an empty `SolutionVector` rather than
        // erroring out, hiding the `russell_sparse` precondition
        // that rejects zero-dim COO matrices.
        let system = sys(0, 0, 0, vec![], vec![]);
        let solver = RussellRealSolver::new();
        let solution = solver.solve(&system).expect("empty system is solvable");
        assert_eq!(solution.dim(), 0);
        assert_eq!(solution.unknowns().len(), 0);
    }

    #[test]
    fn rhs_dimension_mismatch_is_rejected_at_construction() {
        // The shared `SparseLinearSystem::new` validator catches
        // RHS-length errors before they reach the russell wrapper.
        let err = SparseLinearSystem::<f64>::new(3, 3, 0, vec![t(0, 0, 1.0)], vec![1.0, 2.0])
            .expect_err("rhs mismatch must error");
        assert_eq!(
            err,
            LinearSolverError::RhsDimensionMismatch { dim: 3, rhs_len: 2 }
        );
    }

    #[test]
    fn out_of_range_triplet_is_rejected_at_construction() {
        let err = SparseLinearSystem::<f64>::new(2, 2, 0, vec![t(2, 0, 1.0)], vec![1.0, 2.0])
            .expect_err("out-of-range must error");
        assert_eq!(
            err,
            LinearSolverError::TripletOutOfRange {
                row: 2,
                col: 0,
                dim: 2
            }
        );
    }

    #[test]
    fn non_finite_entry_is_rejected() {
        let system = sys(2, 2, 0, vec![t(0, 0, f64::NAN)], vec![1.0, 2.0]);
        let solver = RussellRealSolver::new();
        let err = solver.solve(&system).expect_err("NaN entry must error");
        assert!(
            matches!(err, LinearSolverError::NonFiniteEntry { ref location } if location == "triplet[0]"),
            "expected NonFiniteEntry at triplet[0], got {err:?}",
        );
    }

    #[test]
    fn non_finite_rhs_is_rejected() {
        let system = sys(2, 2, 0, vec![t(0, 0, 1.0)], vec![f64::INFINITY, 0.0]);
        let solver = RussellRealSolver::new();
        let err = solver.solve(&system).expect_err("inf rhs must error");
        assert!(
            matches!(err, LinearSolverError::NonFiniteEntry { ref location } if location == "rhs[0]"),
            "expected NonFiniteEntry at rhs[0], got {err:?}",
        );
    }

    // -----------------------------------------------------------------
    // Numerical correctness
    // -----------------------------------------------------------------

    #[test]
    fn solves_identity_system() {
        // A = I, b = [1, 2, 3]  =>  x = [1, 2, 3]
        let system = sys(
            3,
            3,
            0,
            vec![t(0, 0, 1.0), t(1, 1, 1.0), t(2, 2, 1.0)],
            vec![1.0, 2.0, 3.0],
        );
        let solver = RussellRealSolver::new();
        let solution = solver.solve(&system).expect("identity solve");
        let want = [1.0, 2.0, 3.0];
        for (got, want) in solution.unknowns().iter().zip(want.iter()) {
            assert!(approx(*got, *want, 1e-12), "got {got}, want {want}");
        }
    }

    #[test]
    fn solves_diagonal_system() {
        // A = diag(2, 4, 8), b = [4, 8, 16]  =>  x = [2, 2, 2]
        let system = sys(
            3,
            3,
            0,
            vec![t(0, 0, 2.0), t(1, 1, 4.0), t(2, 2, 8.0)],
            vec![4.0, 8.0, 16.0],
        );
        let solver = RussellRealSolver::new();
        let solution = solver.solve(&system).expect("diagonal solve");
        for got in solution.unknowns() {
            assert!(approx(*got, 2.0, 1e-12), "got {got}");
        }
    }

    #[test]
    fn solves_simple_resistor_divider_mna_subview() {
        // Hand-built MNA sub-view for a two-node DC system after
        // ground suppression. Original circuit:
        //   V1 = 5V between node 1 (+) and node 0 (gnd)
        //   R1 = 1Ω between node 1 and node 2
        //   R2 = 1Ω between node 2 and node 0 (gnd)
        //
        // After ground-row/column suppression and treating V1 as a
        // forced node (replace row 1 with `v1 = 5`), the reduced
        // unknowns are [v1, v2] and the system is:
        //   [ 1   0 ] [v1]   [5]
        //   [-1   2 ] [v2] = [0]
        // Solution: v1 = 5, v2 = 2.5.
        let system = sys(
            2,
            2,
            0,
            vec![t(0, 0, 1.0), t(1, 0, -1.0), t(1, 1, 2.0)],
            vec![5.0, 0.0],
        );
        let solver = RussellRealSolver::new();
        let solution = solver.solve(&system).expect("divider solve");
        assert!(approx(solution.unknowns()[0], 5.0, 1e-12));
        assert!(approx(solution.unknowns()[1], 2.5, 1e-12));
    }

    #[test]
    fn duplicate_entries_are_summed() {
        // Two entries that sum to the identity: caller stamps (0,0)
        // twice as 0.6 + 0.4 = 1.0. Same for (1,1).
        let system = sys(
            2,
            2,
            0,
            vec![t(0, 0, 0.6), t(0, 0, 0.4), t(1, 1, 0.5), t(1, 1, 0.5)],
            vec![7.0, -3.0],
        );
        let solver = RussellRealSolver::new();
        let solution = solver.solve(&system).expect("duplicate-entry solve");
        assert!(approx(solution.unknowns()[0], 7.0, 1e-12));
        assert!(approx(solution.unknowns()[1], -3.0, 1e-12));
    }

    #[test]
    fn solves_nontrivial_5x5_system() {
        // Manufactured system with a known integer solution so the
        // test is independent of any specific floating-point quirk
        // of the backend.
        //
        // A =
        //  [ 4  1  0  0  0 ]
        //  [ 1  4  1  0  0 ]
        //  [ 0  1  4  1  0 ]
        //  [ 0  0  1  4  1 ]
        //  [ 0  0  0  1  4 ]
        //
        // For x = [1, 2, 3, 4, 5]ᵀ:
        //   b = A x = [4+2, 1+8+3, 2+12+4, 3+16+5, 4+20]
        //           = [6, 12, 18, 24, 24].
        let dim: u32 = 5;
        let mut triplets = Vec::new();
        for i in 0..dim {
            triplets.push(t(i, i, 4.0));
            if i + 1 < dim {
                triplets.push(t(i, i + 1, 1.0));
                triplets.push(t(i + 1, i, 1.0));
            }
        }
        let system = sys(dim, dim, 0, triplets, vec![6.0, 12.0, 18.0, 24.0, 24.0]);
        let solver = RussellRealSolver::new();
        let solution = solver.solve(&system).expect("5x5 solve");
        let want = [1.0, 2.0, 3.0, 4.0, 5.0];
        for (got, want) in solution.unknowns().iter().zip(want.iter()) {
            assert!(approx(*got, *want, 1e-10), "got {got}, want {want}");
        }
    }

    #[test]
    fn reuses_solver_across_calls() {
        // Verify that a single `RussellRealSolver` can answer
        // multiple solves in sequence; this is the access pattern
        // the Newton driver (`tasks.md` #17) will use across
        // iterates.
        let solver = RussellRealSolver::new();

        let s1 = sys(2, 2, 0, vec![t(0, 0, 1.0), t(1, 1, 1.0)], vec![3.0, 5.0]);
        let x1 = solver.solve(&s1).expect("first solve");
        assert!(approx(x1.unknowns()[0], 3.0, 1e-12));
        assert!(approx(x1.unknowns()[1], 5.0, 1e-12));

        // Second call: different RHS, same matrix shape (re-factor).
        let s2 = sys(2, 2, 0, vec![t(0, 0, 1.0), t(1, 1, 1.0)], vec![-1.0, 7.0]);
        let x2 = solver.solve(&s2).expect("second solve");
        assert!(approx(x2.unknowns()[0], -1.0, 1e-12));
        assert!(approx(x2.unknowns()[1], 7.0, 1e-12));

        // Third call: different matrix entirely.
        let s3 = sys(2, 2, 0, vec![t(0, 0, 2.0), t(1, 1, 4.0)], vec![10.0, 20.0]);
        let x3 = solver.solve(&s3).expect("third solve");
        assert!(approx(x3.unknowns()[0], 5.0, 1e-12));
        assert!(approx(x3.unknowns()[1], 5.0, 1e-12));
    }

    #[test]
    fn singular_matrix_is_reported_structurally() {
        // A row of all zeros makes the matrix structurally singular.
        // Some backends report this at factorization time; others
        // report it at solve time as a zero pivot. Either way the
        // error should land in `LinearSolverError::SingularMatrix`
        // or `BackendFailure` (we accept both because mapping from
        // russell's string diagnostics is heuristic).
        let system = sys(
            2,
            2,
            0,
            // Row 1 has no live triplets (the explicit-zero entry at
            // (1, 0) sums to zero with no other contribution).
            vec![t(0, 0, 1.0), t(1, 0, 0.0)],
            vec![1.0, 2.0],
        );
        let solver = RussellRealSolver::new();
        let err = solver.solve(&system);
        assert!(
            matches!(
                err,
                Err(LinearSolverError::SingularMatrix { .. }
                    | LinearSolverError::BackendFailure { .. })
            ),
            "expected SingularMatrix/BackendFailure, got {err:?}",
        );
    }

    // -----------------------------------------------------------------
    // Integration with `MnaSystem` via the #15 `SubView` extractor.
    //
    // The on-main path `MnaSystem -> SubView -> SparseLinearSystem ->
    // RussellRealSolver` replaces the abandoned `extract_triplets`
    // helper that lived in the pre-#15 / pre-#23 implementation.
    // -----------------------------------------------------------------

    /// Walk the dense row-major matrix slice produced by
    /// [`crate::sub_view::SubView::matrix`] and emit the equivalent
    /// triplet list, skipping exact zeros. Tests use this to bridge
    /// the dense `SubView` surface to `SparseLinearSystem`'s triplet
    /// surface; production callers will get the same lowering once
    /// the sub-view extractor grows a direct `to_sparse_linear_system`
    /// method (out of scope for #16).
    fn dense_to_triplets(matrix: &[f64], dim: u32) -> Vec<SparseTriplet<f64>> {
        let dim_usize = dim as usize;
        let mut out = Vec::new();
        for r in 0..dim {
            for c in 0..dim {
                let v = matrix[r as usize * dim_usize + c as usize];
                // Exact 0.0 is the documented filter sentinel; this is
                // one of the rare cases where `clippy::float_cmp` is
                // correct to allow.
                #[allow(clippy::float_cmp)]
                let is_zero = v == 0.0;
                if !is_zero {
                    out.push(SparseTriplet {
                        row: r,
                        col: c,
                        value: v,
                    });
                }
            }
        }
        out
    }

    #[test]
    fn dense_subview_lowering_drops_exact_zeros() {
        // Build a tiny MNA system from a real CircuitGraph: one
        // resistor R=1Ω between node `n1` and ground. The property
        // under test is that exact-zero cells of the SubView are
        // *not* emitted as triplets while every nonzero stamp is.
        use crate::sub_view::SubViewBuilder;
        use netlist_graph::{CircuitBuilder, ElementKind};

        let mut b = CircuitBuilder::default();
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: 1.0,
            },
            ["n1", "0"],
            None,
        )
        .expect("add resistor");
        let g = b.build().expect("build ok");
        let fs = crate::flatten::flatten(&g).expect("flatten ok");
        let mna = crate::assemble::assemble(&fs, &g, &[]).expect("assemble ok");

        // Ground-suppressed sub-view: identity row + zeroed column at
        // the ground row/column, full MNA elsewhere.
        let ground = g.node_by_name("0").expect("ground node present").id();
        let view = SubViewBuilder::from_full(&mna)
            .with_ground_node(ground)
            .suppress_ground(true)
            .build()
            .expect("subview build");

        let triplets = dense_to_triplets(view.matrix(), view.dim());

        // Every emitted value must be nonzero and in-range.
        for entry in &triplets {
            #[allow(clippy::float_cmp)]
            let is_zero = entry.value == 0.0;
            assert!(!is_zero, "dense_to_triplets emitted a zero");
            assert!(entry.row < view.dim());
            assert!(entry.col < view.dim());
        }

        // Sum-of-dense vs sum-of-triplets must agree exactly: no
        // value dropped, no value invented.
        let dense_sum: f64 = view.matrix().iter().sum();
        let triplet_sum: f64 = triplets.iter().map(|e| e.value).sum();
        assert!(
            approx(dense_sum, triplet_sum, 1e-12),
            "dense_sum={dense_sum}, triplet_sum={triplet_sum}",
        );

        // Constructing a SparseLinearSystem from these triplets must
        // succeed (every index < dim by construction).
        let system = SparseLinearSystem::<f64>::new(
            view.dim(),
            view.node_count(),
            view.branch_count(),
            triplets,
            view.rhs().to_vec(),
        )
        .expect("sub-view lowers to a well-formed SparseLinearSystem");
        assert_eq!(system.dim(), view.dim());
    }

    #[test]
    fn mna_sub_view_round_trip_through_solver() {
        // Full round trip: MnaSystem -> SubView (ground-suppressed)
        // -> SparseLinearSystem -> RussellRealSolver. This is the
        // on-main lowering pipeline; #15 lands the sub-view, #16
        // (this module) consumes it.
        //
        // Circuit: a single 1Ω resistor between node `n1` and ground.
        // After ground suppression the system is
        //   [ 1   0 ] [v0]   [0]
        //   [ 0   1 ] [v1] = [1]
        // (the conductance block is masked to identity on the ground
        // row, and we inject a 1A test current on the n1 row so the
        // solver path exercises a non-trivial RHS).
        use crate::sub_view::SubViewBuilder;
        use netlist_graph::{CircuitBuilder, ElementKind};

        let mut b = CircuitBuilder::default();
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: 1.0,
            },
            ["n1", "0"],
            None,
        )
        .expect("add resistor");
        let g = b.build().expect("build ok");
        let fs = crate::flatten::flatten(&g).expect("flatten ok");
        let mna = crate::assemble::assemble(&fs, &g, &[]).expect("assemble ok");

        let ground = g.node_by_name("0").expect("ground node present").id();
        let view = SubViewBuilder::from_full(&mna)
            .with_ground_node(ground)
            .suppress_ground(true)
            .build()
            .expect("subview build");

        // Pull triplets out of the dense sub-view matrix.
        let triplets = dense_to_triplets(view.matrix(), view.dim());

        // Override the RHS for the round-trip: pick the non-ground
        // node row and inject 1A so the solution is well-defined.
        // The sub-view's own RHS is all zeros for this no-source
        // circuit.
        let dim = view.dim();
        let mut rhs = vec![0.0; dim as usize];
        // Find a non-ground row: it has a positive diagonal under the
        // suppressed sub-view (the conductance contribution from R1).
        // For the single-resistor circuit there is exactly one such
        // row; the ground row is the identity.
        let n1 = g.node_by_name("n1").expect("n1 present").id();
        let n1_row = n1.index();
        assert!(n1_row < dim);
        rhs[n1_row as usize] = 1.0;

        let system = SparseLinearSystem::<f64>::new(
            view.dim(),
            view.node_count(),
            view.branch_count(),
            triplets,
            rhs,
        )
        .expect("system construction");

        let solver = RussellRealSolver::new();
        let solution = solver
            .solve(&system)
            .expect("round-trip solve through RussellRealSolver");

        assert_eq!(solution.dim(), dim);
        // Ground node forced to 0 by suppression.
        let v_gnd = solution.unknowns()[ground.index() as usize];
        assert!(approx(v_gnd, 0.0, 1e-12), "v_gnd = {v_gnd}");
        // n1 carries 1V across R=1Ω with 1A injection.
        let v_n1 = solution.unknowns()[n1_row as usize];
        assert!(approx(v_n1, 1.0, 1e-12), "v_n1 = {v_n1}");
    }
}
