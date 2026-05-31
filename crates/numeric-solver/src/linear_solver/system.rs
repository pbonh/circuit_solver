//! Input/output value types for the [`LinearSolver`](super::LinearSolver)
//! trait.
//!
//! These types sit on the boundary between the assembly layer
//! ([`crate::assemble`](mod@crate::assemble)) and the sparse-direct LU backends. They are
//! deliberately backend-agnostic: no `faer`, no `russell`, just
//! plain owned `Vec`s of triplets and scalars. The trait
//! implementations convert from these into their own native sparse
//! formats.
//!
//! # Why a separate module
//!
//! Both the real-valued (`russell`, task #16) and the complex-valued
//! (`faer`, task #23) implementations share the same input shape:
//! a list of `(row, col, value)` triplets plus a dense RHS vector
//! and dimension metadata. Pulling those types out of the trait
//! definition keeps `mod.rs` focused on the contract and lets the
//! backend modules consume the types without importing each other.

use core::fmt;

/// A single `(row, col, value)` entry stamped into the sparse MNA
/// matrix.
///
/// `row` and `col` are zero-based sub-view indices (after any
/// ground-suppression and constraint masking applied by `tasks.md`
/// item #15 / the AC sub-view extractor in item #24). Multiple
/// triplets at the same `(row, col)` are **summed** by the backend,
/// matching the standard SPICE stamping convention where every
/// device contributes additively at its terminal positions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SparseTriplet<Scalar> {
    /// Zero-based row index, must be `< dim`.
    pub row: u32,
    /// Zero-based column index, must be `< dim`.
    pub col: u32,
    /// The stamped value. Backends sum duplicate `(row, col)` pairs.
    pub value: Scalar,
}

/// A square sparse linear system `A · x = b` in coordinate (triplet)
/// form, ready to be handed to a [`LinearSolver`](super::LinearSolver)
/// implementation.
///
/// # Layout invariants
///
/// - `dim` is the row/column count of the square matrix `A` and the
///   length of the RHS vector `b`.
/// - Every triplet's `row` and `col` must satisfy `< dim`. The
///   solver validates this before invoking the backend and returns
///   [`LinearSolverError::TripletOutOfRange`] on violation.
/// - `rhs` must have length exactly `dim`. The solver returns
///   [`LinearSolverError::RhsDimensionMismatch`] otherwise.
/// - `node_count + branch_count == dim`. The optional
///   `node_count` / `branch_count` split lets callers slice the
///   returned [`SolutionVector`] back into node-voltage and
///   branch-current segments without re-deriving the layout.
///
/// # Why coordinate form
///
/// MNA stamping naturally produces `(row, col, +value)` entries one
/// at a time as the assembler walks elements. Coordinate form lets
/// the assembler push entries in arbitrary order without bookkeeping;
/// the backend converts to CSC internally. (`faer` accepts
/// `SparseColMat::try_new_from_triplets`, which is exactly this
/// shape; `russell` accepts its own `CooMatrix::put`, also a
/// triplet-stream.)
#[derive(Debug, Clone, PartialEq)]
pub struct SparseLinearSystem<Scalar> {
    dim: u32,
    node_count: u32,
    branch_count: u32,
    triplets: Vec<SparseTriplet<Scalar>>,
    rhs: Vec<Scalar>,
}

impl<Scalar> SparseLinearSystem<Scalar> {
    /// Construct from raw parts. The constructor enforces every
    /// structural invariant documented on the type up-front so the
    /// downstream solver code can run pre-check loops on already-
    /// validated data.
    ///
    /// # Errors
    ///
    /// - [`LinearSolverError::DimensionPartitionMismatch`] if
    ///   `node_count + branch_count != dim` (with `u32` overflow
    ///   treated as a mismatch).
    /// - [`LinearSolverError::RhsDimensionMismatch`] if
    ///   `rhs.len() != dim`.
    /// - [`LinearSolverError::TripletOutOfRange`] if any triplet's
    ///   `row` or `col` is `>= dim`.
    ///
    /// Non-finite scalar checks live in the backend (they require
    /// knowing how to ask the scalar whether it is finite, which is
    /// type-dependent: `f64::is_finite` versus
    /// `Complex<f64>` real/imag finiteness).
    pub fn new(
        dim: u32,
        node_count: u32,
        branch_count: u32,
        triplets: Vec<SparseTriplet<Scalar>>,
        rhs: Vec<Scalar>,
    ) -> Result<Self, LinearSolverError> {
        // Partition sanity.
        match node_count.checked_add(branch_count) {
            Some(sum) if sum == dim => {}
            _ => {
                return Err(LinearSolverError::DimensionPartitionMismatch {
                    dim,
                    node_count,
                    branch_count,
                });
            }
        }

        // RHS length.
        if rhs.len() != dim as usize {
            return Err(LinearSolverError::RhsDimensionMismatch {
                dim,
                rhs_len: rhs.len(),
            });
        }

        // Triplet index ranges.
        for t in &triplets {
            if t.row >= dim || t.col >= dim {
                return Err(LinearSolverError::TripletOutOfRange {
                    row: t.row,
                    col: t.col,
                    dim,
                });
            }
        }

        Ok(Self {
            dim,
            node_count,
            branch_count,
            triplets,
            rhs,
        })
    }

    /// Total dimension of `A` and `b`.
    #[must_use]
    pub fn dim(&self) -> u32 {
        self.dim
    }

    /// The number of node-equation rows in the sub-view layout.
    /// `dim == node_count + branch_count`.
    #[must_use]
    pub fn node_count(&self) -> u32 {
        self.node_count
    }

    /// The number of MNA branch-equation rows in the sub-view layout.
    #[must_use]
    pub fn branch_count(&self) -> u32 {
        self.branch_count
    }

    /// Borrow the triplet stream.
    #[must_use]
    pub fn triplets(&self) -> &[SparseTriplet<Scalar>] {
        &self.triplets
    }

    /// Borrow the dense RHS vector.
    #[must_use]
    pub fn rhs(&self) -> &[Scalar] {
        &self.rhs
    }
}

/// The dense solution vector `x` returned by a successful
/// [`LinearSolver::solve`](super::LinearSolver::solve) call.
///
/// The vector has length `dim = node_count + branch_count`. The
/// `0..node_count` segment holds the sub-view node unknowns
/// (typically voltages, for both the real and complex variants);
/// `node_count..node_count + branch_count` holds the MNA branch
/// unknowns (typically currents through voltage sources and
/// inductors).
#[derive(Debug, Clone, PartialEq)]
pub struct SolutionVector<Scalar> {
    node_count: u32,
    branch_count: u32,
    unknowns: Vec<Scalar>,
}

impl<Scalar> SolutionVector<Scalar> {
    /// Construct from parts. Used by backend implementations once
    /// they have a dense column.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if
    /// `unknowns.len() != node_count + branch_count`. Release builds
    /// trust the backend (the only callers are inside this crate)
    /// to construct correctly.
    pub(crate) fn from_parts(node_count: u32, branch_count: u32, unknowns: Vec<Scalar>) -> Self {
        debug_assert_eq!(
            unknowns.len(),
            (node_count as usize) + (branch_count as usize),
            "SolutionVector dimension mismatch",
        );
        Self {
            node_count,
            branch_count,
            unknowns,
        }
    }

    /// Total dimension of the solution vector.
    #[must_use]
    pub fn dim(&self) -> u32 {
        self.node_count + self.branch_count
    }

    /// Node-equation count in the layout.
    #[must_use]
    pub fn node_count(&self) -> u32 {
        self.node_count
    }

    /// MNA branch-equation count in the layout.
    #[must_use]
    pub fn branch_count(&self) -> u32 {
        self.branch_count
    }

    /// Full slice of unknowns in layout order
    /// `[node_0, …, node_{node_count-1}, branch_0, …,
    /// branch_{branch_count-1}]`.
    #[must_use]
    pub fn unknowns(&self) -> &[Scalar] {
        &self.unknowns
    }

    /// Slice of node-equation unknowns only.
    #[must_use]
    pub fn node_unknowns(&self) -> &[Scalar] {
        &self.unknowns[..(self.node_count as usize)]
    }

    /// Slice of MNA branch-equation unknowns only.
    #[must_use]
    pub fn branch_unknowns(&self) -> &[Scalar] {
        &self.unknowns[(self.node_count as usize)..]
    }

    /// Consume the wrapper and return the raw `Vec<Scalar>`.
    #[must_use]
    pub fn into_unknowns(self) -> Vec<Scalar> {
        self.unknowns
    }
}

/// Errors returned by [`LinearSolver::solve`](super::LinearSolver::solve)
/// implementations.
///
/// The variants are intentionally backend-agnostic: any error from
/// `faer` or `russell` must be projected into one of these. This
/// keeps the analysis orchestrator's match-on-error sites stable
/// even as backends evolve.
#[derive(Debug, Clone, PartialEq)]
pub enum LinearSolverError {
    /// `node_count + branch_count` does not equal `dim` (or overflows
    /// `u32`). Indicates a caller bug in lowering
    /// [`crate::assemble::MnaSystem`] into a [`SparseLinearSystem`].
    DimensionPartitionMismatch {
        /// Reported dim.
        dim: u32,
        /// Reported node count.
        node_count: u32,
        /// Reported branch count.
        branch_count: u32,
    },
    /// RHS length does not match `dim`.
    RhsDimensionMismatch {
        /// The dim of the matrix.
        dim: u32,
        /// The length of the RHS vector actually supplied.
        rhs_len: usize,
    },
    /// A triplet pointed at `(row, col)` outside the matrix bounds.
    TripletOutOfRange {
        /// Offending row index.
        row: u32,
        /// Offending column index.
        col: u32,
        /// Dim against which the index was compared.
        dim: u32,
    },
    /// A triplet or RHS entry carried a non-finite numerical value
    /// (NaN, ±∞ on the real or imaginary axis). Backends refuse to
    /// run on poisoned matrices.
    NonFiniteEntry {
        /// Human-readable location: `"triplet[i]"`, `"rhs[i]"`.
        location: String,
    },
    /// The matrix is structurally singular (the LU algorithm reached
    /// a row with no live pivot). Distinct from a finite but
    /// arbitrarily ill-conditioned matrix.
    SingularMatrix {
        /// Best-effort column index reported by the backend at
        /// which the singularity was detected, when available.
        column_hint: Option<u32>,
    },
    /// The backend failed for a reason unrelated to the matrix
    /// values: allocation failure, internal capacity overflow,
    /// platform error. Carries a stringified description because
    /// the underlying enum is backend-specific.
    BackendFailure {
        /// Short backend tag (e.g. `"faer"`).
        backend: &'static str,
        /// Stringified backend error.
        description: String,
    },
}

impl fmt::Display for LinearSolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionPartitionMismatch {
                dim,
                node_count,
                branch_count,
            } => write!(
                f,
                "linear-solver: dim partition mismatch: dim={dim}, \
                 node_count={node_count}, branch_count={branch_count}",
            ),
            Self::RhsDimensionMismatch { dim, rhs_len } => {
                write!(f, "linear-solver: rhs length {rhs_len} != dim {dim}")
            }
            Self::TripletOutOfRange { row, col, dim } => write!(
                f,
                "linear-solver: triplet (row={row}, col={col}) out of range for dim={dim}",
            ),
            Self::NonFiniteEntry { location } => {
                write!(f, "linear-solver: non-finite entry at {location}")
            }
            Self::SingularMatrix { column_hint } => match column_hint {
                Some(c) => write!(f, "linear-solver: singular matrix at column {c}"),
                None => write!(f, "linear-solver: singular matrix"),
            },
            Self::BackendFailure {
                backend,
                description,
            } => write!(f, "linear-solver: {backend} backend failure: {description}"),
        }
    }
}

impl std::error::Error for LinearSolverError {}
