//! `StampInterface` trait — the contract between the MNA assembler and
//! the numeric solver (ADR-0002).
//!
//! Per ADR-0002 the MNA stamp surface is a **trait** so that:
//!
//! - the one-shot [`crate::assemble::assemble`] function can stamp through it,
//! - the incremental [`IncrementalMnaBuilder`] can stamp through it,
//! - future sparse-backed stamp targets can implement it without changing the
//!   stamp logic.
//!
//! This module defines:
//!
//! - [`StampValue`] — a single `(row, col, value)` triple for logging/testing.
//! - [`StampError`] — errors specific to the stamp interface (out-of-range,
//!   non-finite, system too large).
//! - [`StampInterface`] — the ADR-0002 contract: `dim()`, `node_count()`,
//!   `branch_count()`, `stamp_matrix()`, `stamp_rhs()`, `finish()`.
//! - [`IncrementalMnaBuilder`] — constructed from a [`FlattenedStructure`],
//!   provides per-kind stamp methods, implements [`StampInterface`], and
//!   produces an [`crate::assemble::MnaSystem`] on [`finish`](IncrementalMnaBuilder::finish).
//!
//! # Why a separate module from `assemble.rs`?
//!
//! `assemble.rs` owns the one-shot walk of the flattened structure. That
//! function is the high-level entry point for DC operating-point assembly.
//! `mna.rs` owns the *abstraction* that the walk (and the Newton-Raphson
//! loop, and the transient companion stamp) all stamp through. Keeping
//! them separate means the trait can be reviewed and ratified (ADR-0002)
//! independently of the stamp implementation details.
//!
//! # Relationship to the reference design
//!
//! The reference design (`project/src/numeric/mna.rs`) defined
//! `AssembledSystem` as the output type of `StampInterface::finish`.
//! In this crate, [`crate::assemble::MnaSystem`] already serves that role
//! and is consumed by `sub_view`. Rather than introduce a duplicate type,
//! `IncrementalMnaBuilder::finish` returns `MnaSystem` directly. The
//! `MnaSystem` accessors (`matrix_mut`, `rhs_mut`, `get_matrix`,
//! `set_matrix`, `get_rhs`, `set_rhs`, `from_raw_parts`) provide the
//! same surface that `AssembledSystem` did in the reference.

use circuit_solver_types::flattened::FlattenedStructure;

use crate::assemble::{MnaAssemblyError, MnaSystem};

// ---------------------------------------------------------------------------
// StampValue
// ---------------------------------------------------------------------------

/// A single matrix-stamp triple `(row, col, value)`.
///
/// Used for logging, testing, and debugging. Not consumed by the stamp
/// loop itself (that goes through [`StampInterface::stamp_matrix`] directly).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StampValue {
    /// Row index in the MNA system.
    pub row: u32,
    /// Column index in the MNA system.
    pub col: u32,
    /// The value to add at `(row, col)`.
    pub value: f64,
}

// ---------------------------------------------------------------------------
// StampError
// ---------------------------------------------------------------------------

/// Errors raised by [`StampInterface`] methods.
///
/// These are the stamp-interface-specific errors. They map 1:1 to a subset
/// of [`MnaAssemblyError`] variants when [`IncrementalMnaBuilder::finish`]
/// converts them, but exist as a separate type so that trait consumers
/// don't need to know about the full assembly error taxonomy.
#[derive(Debug, Clone, PartialEq)]
pub enum StampError {
    /// Row index exceeds `dim - 1`.
    RowOutOfRange {
        /// The out-of-range row index.
        row: u32,
        /// System dimension at the time of the error.
        dim: u32,
    },
    /// Column index exceeds `dim - 1`.
    ColOutOfRange {
        /// The out-of-range column index.
        col: u32,
        /// System dimension at the time of the error.
        dim: u32,
    },
    /// Stamped value is NaN or ±∞.
    NonFiniteStampValue {
        /// Row of the offending stamp.
        row: u32,
        /// Column of the offending stamp.
        col: u32,
        /// The non-finite value.
        value: f64,
    },
    /// System dimension would overflow `u32`.
    SystemTooLarge {
        /// Node count at the point of overflow.
        node_count: u32,
        /// Branch count at the point of overflow.
        branch_count: u32,
    },
}

impl core::fmt::Display for StampError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RowOutOfRange { row, dim } => {
                write!(f, "row {row} out of range for dim={dim}")
            }
            Self::ColOutOfRange { col, dim } => {
                write!(f, "col {col} out of range for dim={dim}")
            }
            Self::NonFiniteStampValue { row, col, value } => {
                write!(f, "non-finite stamp value {value} at ({row}, {col})")
            }
            Self::SystemTooLarge {
                node_count,
                branch_count,
            } => {
                write!(
                    f,
                    "system dimension overflowed u32 (nodes={node_count}, branches={branch_count})"
                )
            }
        }
    }
}

impl std::error::Error for StampError {}

// ---------------------------------------------------------------------------
// StampInterface
// ---------------------------------------------------------------------------

/// The ADR-0002 MNA stamp contract.
///
/// Implementations receive stamp contributions (conductances, RHS entries)
/// from the assembler and produce an assembled system on [`finish`].
///
/// The trait is object-safe (`Self: Sized` is not required) so that
/// downstream code can stamp through `&mut dyn StampInterface<...>` if
/// needed, though the primary consumers use static dispatch through
/// [`IncrementalMnaBuilder`].
///
/// [`finish`]: StampInterface::finish
pub trait StampInterface {
    /// The error type returned by [`finish`](StampInterface::finish).
    type Error: std::error::Error + Send + Sync + 'static;

    /// Total dimension of the MNA system (`node_count + branch_count`).
    fn dim(&self) -> u32;

    /// Number of node equations (including ground).
    fn node_count(&self) -> u32;

    /// Number of MNA branch equations.
    fn branch_count(&self) -> u32;

    /// Add `value` to the matrix entry at `(row, col)`.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::RowOutOfRange`] or [`StampError::ColOutOfRange`]
    /// if indices exceed `dim - 1`, or [`StampError::NonFiniteStampValue`]
    /// if `value` is not finite.
    fn stamp_matrix(&mut self, row: u32, col: u32, value: f64) -> Result<(), StampError>;

    /// Add `value` to the RHS entry at `row`.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::RowOutOfRange`] if `row` exceeds `dim - 1`,
    /// or [`StampError::NonFiniteStampValue`] if `value` is not finite.
    fn stamp_rhs(&mut self, row: u32, value: f64) -> Result<(), StampError>;

    /// Finalize the assembly and return the completed system.
    ///
    /// # Errors
    ///
    /// Returns the implementation's error type if finalization fails.
    fn finish(self) -> Result<MnaSystem, Self::Error>;
}

// ---------------------------------------------------------------------------
// IncrementalMnaBuilder
// ---------------------------------------------------------------------------

/// Incremental MNA builder that implements [`StampInterface`].
///
/// Constructed from a [`FlattenedStructure`] reference, allocates a
/// zero-initialized dense matrix and RHS vector of the correct dimension,
/// and accumulates stamp contributions. On [`finish`](IncrementalMnaBuilder::finish)
/// it produces an [`MnaSystem`].
///
/// This is the builder that the Newton-Raphson loop will use: each
/// iteration creates a fresh builder (or re-uses one via [`reset`]),
/// stamps all device contributions, and calls [`finish`](IncrementalMnaBuilder::finish).
///
/// [`reset`]: IncrementalMnaBuilder::reset
#[derive(Debug)]
pub struct IncrementalMnaBuilder {
    /// Total node count (including ground).
    node_count: u32,
    /// Total MNA branch count.
    branch_count: u32,
    /// `node_count + branch_count`.
    dim: u32,
    /// Dense row-major matrix of dimension `dim × dim`.
    a: Vec<f64>,
    /// RHS vector of dimension `dim`.
    b: Vec<f64>,
}

impl IncrementalMnaBuilder {
    /// Create a new builder sized for the given [`FlattenedStructure`].
    ///
    /// Allocates zero-initialized matrix and RHS. No stamps are applied;
    /// the caller stamps through [`StampInterface`] methods or the
    /// per-kind convenience methods, then calls [`finish`](Self::finish).
    ///
    /// # Errors
    ///
    /// Returns [`StampError::SystemTooLarge`] if `node_count + branch_count`
    /// overflows `u32`.
    pub fn new(flat: &FlattenedStructure) -> Result<Self, StampError> {
        Self::with_dimensions(flat.node_count(), flat.branch_count())
    }

    /// Create a builder with explicit dimensions (no `FlattenedStructure`).
    ///
    /// Useful for tests and for the transient solver which may assemble
    /// companion stamps into a differently-sized system.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::SystemTooLarge`] if `node_count + branch_count`
    /// overflows `u32`.
    pub fn with_dimensions(node_count: u32, branch_count: u32) -> Result<Self, StampError> {
        let dim = node_count
            .checked_add(branch_count)
            .ok_or(StampError::SystemTooLarge {
                node_count,
                branch_count,
            })?;
        let dim_usize = dim as usize;
        Ok(Self {
            node_count,
            branch_count,
            dim,
            a: vec![0.0_f64; dim_usize * dim_usize],
            b: vec![0.0_f64; dim_usize],
        })
    }

    /// Reset the builder to zero-initialized state, keeping the same dimensions.
    ///
    /// This allows reusing the allocated buffers across Newton-Raphson
    /// iterations without a fresh allocation.
    pub fn reset(&mut self) {
        self.a.fill(0.0);
        self.b.fill(0.0);
    }

    /// Add `value` to `a[row * dim + col]`.
    ///
    /// This is the core stamp primitive. Bounds and finiteness are checked.
    fn add_to_matrix(&mut self, row: u32, col: u32, value: f64) -> Result<(), StampError> {
        if row >= self.dim {
            return Err(StampError::RowOutOfRange {
                row,
                dim: self.dim,
            });
        }
        if col >= self.dim {
            return Err(StampError::ColOutOfRange {
                col,
                dim: self.dim,
            });
        }
        if !value.is_finite() {
            return Err(StampError::NonFiniteStampValue { row, col, value });
        }
        self.a[row as usize * self.dim as usize + col as usize] += value;
        Ok(())
    }

    /// Add `value` to `b[row]`.
    ///
    /// Bounds and finiteness are checked.
    fn add_to_rhs(&mut self, row: u32, value: f64) -> Result<(), StampError> {
        if row >= self.dim {
            return Err(StampError::RowOutOfRange {
                row,
                dim: self.dim,
            });
        }
        if !value.is_finite() {
            return Err(StampError::NonFiniteStampValue {
                row,
                col: 0, // no col for RHS
                value,
            });
        }
        self.b[row as usize] += value;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Per-kind convenience stamp methods
    // -----------------------------------------------------------------------

    /// Stamp a two-terminal conductive element (resistor, diode companion)
    /// with conductance `g` between nodes `i` and `j`.
    ///
    /// Adds `+g` at `(i,i)` and `(j,j)`, `-g` at `(i,j)` and `(j,i)`.
    ///
    /// # Errors
    ///
    /// Propagates [`StampError`] from individual stamp operations.
    pub fn stamp_conductive(&mut self, i: u32, j: u32, g: f64) -> Result<(), StampError> {
        self.add_to_matrix(i, i, g)?;
        self.add_to_matrix(j, j, g)?;
        self.add_to_matrix(i, j, -g)?;
        self.add_to_matrix(j, i, -g)?;
        Ok(())
    }

    /// Stamp a voltage source (or inductor at DC) with value `e` between
    /// positive node `i` and negative node `j`, owning branch row `b`.
    ///
    /// Branch row index in the full system is `node_count + b`.
    ///
    /// Stamps:
    /// - `+1` at `(br, i)`, `-1` at `(br, j)`,
    /// - `+1` at `(i, br)`, `-1` at `(j, br)`,
    /// - `RHS[br] += e`.
    ///
    /// # Errors
    ///
    /// Propagates [`StampError`] from individual stamp operations.
    pub fn stamp_voltage_source(
        &mut self,
        i: u32,
        j: u32,
        b: u32,
        e: f64,
    ) -> Result<(), StampError> {
        let br = self.node_count + b;
        self.add_to_matrix(br, i, 1.0)?;
        self.add_to_matrix(br, j, -1.0)?;
        self.add_to_matrix(i, br, 1.0)?;
        self.add_to_matrix(j, br, -1.0)?;
        self.add_to_rhs(br, e)?;
        Ok(())
    }

    /// Stamp an independent current source of value `s` amperes flowing
    /// into node `from_node` and out of node `to_node`.
    ///
    /// Stamps `RHS[from_node] += s`, `RHS[to_node] -= s`.
    ///
    /// # Errors
    ///
    /// Propagates [`StampError`] from individual stamp operations.
    pub fn stamp_current_source(
        &mut self,
        from_node: u32,
        to_node: u32,
        s: f64,
    ) -> Result<(), StampError> {
        self.add_to_rhs(from_node, s)?;
        self.add_to_rhs(to_node, -s)?;
        Ok(())
    }

    /// Stamp a semiconductor linearization.
    ///
    /// The `nodes` slice maps terminal slots to global node indices
    /// (per the `FlattenedStructure` incidence). The `jac` matrix is
    /// terminal-local (indexed `[term_i][term_j]`). The `companion`
    /// vector is terminal-local (indexed `[term_k]`).
    ///
    /// Stamps:
    /// - `A[nodes[i], nodes[j]] += jac[i][j]` for all terminal pairs.
    /// - `RHS[nodes[k]] -= companion[k]` for all terminal slots.
    ///
    /// # Errors
    ///
    /// Propagates [`StampError`] from individual stamp operations.
    pub fn stamp_linearization(
        &mut self,
        nodes: &[u32],
        jac: &[&[f64]],
        companion: &[f64],
    ) -> Result<(), StampError> {
        let n = nodes.len();
        debug_assert_eq!(jac.len(), n);
        debug_assert_eq!(companion.len(), n);

        for (i, &ni) in nodes.iter().enumerate() {
            for (j, &nj) in nodes.iter().enumerate() {
                self.add_to_matrix(ni, nj, jac[i][j])?;
            }
            self.add_to_rhs(ni, -companion[i])?;
        }
        Ok(())
    }
}

impl StampInterface for IncrementalMnaBuilder {
    type Error = MnaAssemblyError;

    fn dim(&self) -> u32 {
        self.dim
    }

    fn node_count(&self) -> u32 {
        self.node_count
    }

    fn branch_count(&self) -> u32 {
        self.branch_count
    }

    fn stamp_matrix(&mut self, row: u32, col: u32, value: f64) -> Result<(), StampError> {
        self.add_to_matrix(row, col, value)
    }

    fn stamp_rhs(&mut self, row: u32, value: f64) -> Result<(), StampError> {
        self.add_to_rhs(row, value)
    }

    fn finish(self) -> Result<MnaSystem, Self::Error> {
        MnaSystem::from_raw_parts(self.node_count, self.branch_count, self.a, self.b)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_solver_types::flattened::ElementIncidence;
    use circuit_solver_types::{BranchId, ElementId, NodeId};

    // -------------------------------------------------------------------
    // Helper: build a minimal FlattenedStructure for testing
    // -------------------------------------------------------------------

    /// 3-node (0=ground, 1, 2), 1-branch system with one resistor.
    fn simple_resistor_flat() -> FlattenedStructure {
        let r = ElementIncidence::two_terminal_conductive(
            ElementId::new(0),
            NodeId::new(1),
            NodeId::new(2),
        );
        FlattenedStructure::new(3, 1, vec![r]).expect("build simple resistor flat")
    }

    // -------------------------------------------------------------------
    // StampValue
    // -------------------------------------------------------------------

    #[test]
    fn stamp_value_fields() {
        let sv = StampValue {
            row: 1,
            col: 2,
            value: 3.5,
        };
        assert_eq!(sv.row, 1);
        assert_eq!(sv.col, 2);
        assert!((sv.value - 3.5).abs() < f64::EPSILON);
    }

    // -------------------------------------------------------------------
    // StampError Display
    // -------------------------------------------------------------------

    #[test]
    fn stamp_error_display_row_out_of_range() {
        let e = StampError::RowOutOfRange { row: 5, dim: 3 };
        assert_eq!(e.to_string(), "row 5 out of range for dim=3");
    }

    #[test]
    fn stamp_error_display_col_out_of_range() {
        let e = StampError::ColOutOfRange { col: 10, dim: 4 };
        assert_eq!(e.to_string(), "col 10 out of range for dim=4");
    }

    #[test]
    fn stamp_error_display_non_finite() {
        let e = StampError::NonFiniteStampValue {
            row: 0,
            col: 1,
            value: f64::NAN,
        };
        assert!(e.to_string().contains("non-finite"));
    }

    #[test]
    fn stamp_error_display_system_too_large() {
        let e = StampError::SystemTooLarge {
            node_count: u32::MAX,
            branch_count: 1,
        };
        assert!(e.to_string().contains("overflowed"));
    }

    // -------------------------------------------------------------------
    // IncrementalMnaBuilder::new
    // -------------------------------------------------------------------

    #[test]
    fn builder_new_from_flattened() {
        let flat = simple_resistor_flat();
        let builder = IncrementalMnaBuilder::new(&flat).expect("builder");
        assert_eq!(builder.dim(), 4); // 3 nodes + 1 branch
        assert_eq!(builder.node_count(), 3);
        assert_eq!(builder.branch_count(), 1);
    }

    #[test]
    fn builder_new_system_too_large() {
        let err = IncrementalMnaBuilder::with_dimensions(u32::MAX - 1, 2)
            .expect_err("should overflow");
        assert_eq!(
            err,
            StampError::SystemTooLarge {
                node_count: u32::MAX - 1,
                branch_count: 2,
            }
        );
    }

    // -------------------------------------------------------------------
    // StampInterface: stamp_matrix / stamp_rhs / finish
    // -------------------------------------------------------------------

    #[test]
    fn trait_stamp_matrix_and_finish() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");

        // Stamp a 1S conductance between node 1 and node 2
        builder.stamp_conductive(1, 2, 1.0).expect("stamp");

        let sys: MnaSystem = builder.finish().expect("finish");
        assert_eq!(sys.dim(), 4);
        assert_eq!(sys.node_count(), 3);
        assert_eq!(sys.branch_count(), 1);

        // Verify the conductance stamp: +1 at (1,1), (2,2); -1 at (1,2), (2,1)
        assert!((sys.get_matrix(1, 1) - 1.0).abs() < 1e-15);
        assert!((sys.get_matrix(2, 2) - 1.0).abs() < 1e-15);
        assert!((sys.get_matrix(1, 2) - (-1.0)).abs() < 1e-15);
        assert!((sys.get_matrix(2, 1) - (-1.0)).abs() < 1e-15);
        // Ground row/col should be zero (no stamps there)
        assert!(sys.get_matrix(0, 0).abs() < 1e-15);
    }

    #[test]
    fn trait_stamp_rhs_and_finish() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");

        // Current source of 2A into node 1, out of node 2
        builder.stamp_current_source(1, 2, 2.0).expect("stamp");

        let sys: MnaSystem = builder.finish().expect("finish");
        assert!((sys.get_rhs(1) - 2.0).abs() < 1e-15);
        assert!((sys.get_rhs(2) - (-2.0)).abs() < 1e-15);
    }

    #[test]
    fn trait_stamp_voltage_source() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");

        // Voltage source 5V between node 1 (+) and node 2 (-), branch 0
        builder.stamp_voltage_source(1, 2, 0, 5.0).expect("stamp");

        let sys: MnaSystem = builder.finish().expect("finish");
        let br = 3; // node_count(3) + branch_index(0) = 3

        assert!((sys.get_matrix(br, 1) - 1.0).abs() < 1e-15);
        assert!((sys.get_matrix(br, 2) - (-1.0)).abs() < 1e-15);
        assert!((sys.get_matrix(1, br) - 1.0).abs() < 1e-15);
        assert!((sys.get_matrix(2, br) - (-1.0)).abs() < 1e-15);
        assert!((sys.get_rhs(br) - 5.0).abs() < 1e-15);
    }

    // -------------------------------------------------------------------
    // Out-of-range and non-finite errors
    // -------------------------------------------------------------------

    #[test]
    fn stamp_matrix_row_out_of_range() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");
        let err = builder.stamp_matrix(10, 0, 1.0).expect_err("out of range");
        assert_eq!(err, StampError::RowOutOfRange { row: 10, dim: 4 });
    }

    #[test]
    fn stamp_matrix_col_out_of_range() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");
        let err = builder.stamp_matrix(0, 10, 1.0).expect_err("out of range");
        assert_eq!(err, StampError::ColOutOfRange { col: 10, dim: 4 });
    }

    #[test]
    fn stamp_matrix_non_finite() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");
        let err = builder
            .stamp_matrix(1, 1, f64::INFINITY)
            .expect_err("non-finite");
        assert!(matches!(err, StampError::NonFiniteStampValue { .. }));
    }

    #[test]
    fn stamp_rhs_row_out_of_range() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");
        let err = builder.stamp_rhs(10, 1.0).expect_err("out of range");
        assert_eq!(err, StampError::RowOutOfRange { row: 10, dim: 4 });
    }

    #[test]
    fn stamp_rhs_non_finite() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");
        let err = builder.stamp_rhs(1, f64::NAN).expect_err("non-finite");
        assert!(matches!(err, StampError::NonFiniteStampValue { .. }));
    }

    // -------------------------------------------------------------------
    // reset()
    // -------------------------------------------------------------------

    #[test]
    fn builder_reset_clears_stamps() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");
        builder.stamp_conductive(1, 2, 1.0).expect("stamp");
        builder.stamp_rhs(1, 5.0).expect("rhs");

        builder.reset();

        // After reset, all entries should be zero
        let sys: MnaSystem = builder.finish().expect("finish");
        for r in 0..sys.dim() {
            for c in 0..sys.dim() {
                assert!(
                    sys.get_matrix(r, c).abs() < 1e-15,
                    "matrix ({r},{c}) should be zero after reset"
                );
            }
            assert!(
                sys.get_rhs(r).abs() < 1e-15,
                "rhs [{r}] should be zero after reset"
            );
        }
    }

    // -------------------------------------------------------------------
    // stamp_linearization (2-terminal diode-like)
    // -------------------------------------------------------------------

    #[test]
    fn stamp_linearization_diode_like() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");

        // Simulate a diode linearization between node 1 (anode) and node 2 (cathode)
        let nodes = [1u32, 2u32];
        let jac: [&[f64]; 2] = [&[0.01, -0.01][..], &[-0.01, 0.01][..]];
        let companion = [0.001, -0.001];

        builder
            .stamp_linearization(&nodes, &jac, &companion)
            .expect("linearization stamp");

        let sys: MnaSystem = builder.finish().expect("finish");

        // Jacobian contributions
        assert!((sys.get_matrix(1, 1) - 0.01).abs() < 1e-15);
        assert!((sys.get_matrix(1, 2) - (-0.01)).abs() < 1e-15);
        assert!((sys.get_matrix(2, 1) - (-0.01)).abs() < 1e-15);
        assert!((sys.get_matrix(2, 2) - 0.01).abs() < 1e-15);

        // Companion current: RHS[node_k] -= companion[k]
        assert!((sys.get_rhs(1) - (-0.001)).abs() < 1e-15);
        assert!((sys.get_rhs(2) - (0.001)).abs() < 1e-15);
    }

    // -------------------------------------------------------------------
    // Full DC operating point: voltage divider
    // -------------------------------------------------------------------

    #[test]
    fn voltage_divider_via_builder() {
        // Build a FlattenedStructure for a voltage divider:
        //   V1 (5V, + at node 1, - at ground), branch 0
        //   R1 (1kΩ, node 1 to node 2)
        //   R2 (1kΩ, node 2 to ground)
        // Expected: V(node1) = 5V, V(node2) = 2.5V
        let v1 = ElementIncidence::two_terminal_current_carrying(
            ElementId::new(0),
            NodeId::new(1),
            NodeId::GROUND,
            BranchId::new(0),
        );
        let r1 = ElementIncidence::two_terminal_conductive(
            ElementId::new(1),
            NodeId::new(1),
            NodeId::new(2),
        );
        let r2 = ElementIncidence::two_terminal_conductive(
            ElementId::new(2),
            NodeId::new(2),
            NodeId::GROUND,
        );

        let flat = FlattenedStructure::new(3, 1, vec![v1, r1, r2])
            .expect("build voltage divider flat");
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");

        // Stamp V1: 5V between node 1 (+) and ground (0), branch 0
        builder.stamp_voltage_source(1, 0, 0, 5.0).expect("V1");

        // Stamp R1: 1kΩ → g = 0.001 S, between node 1 and node 2
        builder.stamp_conductive(1, 2, 0.001).expect("R1");

        // Stamp R2: 1kΩ → g = 0.001 S, between node 2 and ground
        builder.stamp_conductive(2, 0, 0.001).expect("R2");

        let sys: MnaSystem = builder.finish().expect("finish");

        // Dimension: 3 nodes + 1 branch = 4
        assert_eq!(sys.dim(), 4);

        // Verify key matrix entries
        // G(1,1) = g_R1 = 0.001
        // G(2,2) = g_R1 + g_R2 = 0.002
        // G(1,2) = G(2,1) = -g_R1 = -0.001
        // G(2,0) = G(0,2) = -g_R2 = -0.001
        // G(0,0) = g_R2 = 0.001
        // Branch row (row 3): +1 at (3,1), -1 at (3,0), +1 at (1,3), -1 at (0,3)
        // RHS[3] = 5
        assert!((sys.get_matrix(1, 1) - 0.001).abs() < 1e-15);
        assert!((sys.get_matrix(2, 2) - 0.002).abs() < 1e-15);
        assert!((sys.get_matrix(1, 2) - (-0.001)).abs() < 1e-15);
        assert!((sys.get_matrix(2, 1) - (-0.001)).abs() < 1e-15);
        assert!((sys.get_matrix(0, 0) - 0.001).abs() < 1e-15);
        assert!((sys.get_matrix(2, 0) - (-0.001)).abs() < 1e-15);
        assert!((sys.get_matrix(0, 2) - (-0.001)).abs() < 1e-15);
        assert!((sys.get_matrix(3, 1) - 1.0).abs() < 1e-15);
        assert!((sys.get_matrix(3, 0) - (-1.0)).abs() < 1e-15);
        assert!((sys.get_matrix(1, 3) - 1.0).abs() < 1e-15);
        assert!((sys.get_matrix(0, 3) - (-1.0)).abs() < 1e-15);
        assert!((sys.get_rhs(3) - 5.0).abs() < 1e-15);
    }

    // -------------------------------------------------------------------
    // with_dimensions
    // -------------------------------------------------------------------

    #[test]
    fn builder_with_dimensions() {
        let builder = IncrementalMnaBuilder::with_dimensions(5, 2).expect("builder");
        assert_eq!(builder.dim(), 7);
        assert_eq!(builder.node_count(), 5);
        assert_eq!(builder.branch_count(), 2);
    }

    // -------------------------------------------------------------------
    // StampInterface object safety (use as &mut dyn)
    // -------------------------------------------------------------------

    #[test]
    fn stamp_interface_dyn_dispatch() {
        let flat = simple_resistor_flat();
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder");

        // Use through a trait object to verify object safety
        let iface: &mut dyn StampInterface<Error = MnaAssemblyError> = &mut builder;
        iface.stamp_matrix(1, 1, 0.5).expect("stamp via dyn");
        iface.stamp_rhs(1, 1.0).expect("rhs via dyn");

        assert_eq!(iface.dim(), 4);
        assert_eq!(iface.node_count(), 3);
        assert_eq!(iface.branch_count(), 1);
    }
}
