//! DC operating-point solver — 3-tier Newton-Raphson fallback loop.
//!
//! This module implements `solve_dc_operating_point`, the project-level
//! DC analysis orchestrator that composes the lower-level `numeric-solver`
//! driver primitives into a 3-tier convergence strategy:
//!
//! 1. **Plain Newton-Raphson** — fast path for well-conditioned circuits.
//! 2. **Gmin-stepping homotopy** — adds diagonal shunt conductances and
//!    walks them to zero, aiding convergence on floating-node circuits.
//! 3. **Source-stepping homotopy** — ramps independent sources from 0 to
//!    full value, the last resort for stiff or highly nonlinear circuits.
//!
//! # Architecture
//!
//! The [`CircuitNonlinearSystem`] struct implements the
//! [`NonlinearSystem`] and [`SourceSteppableSystem`] traits from
//! `numeric-solver`. On each `linearize` or `residue` call it:
//!
//! 1. Clones the base (linear-only) MNA system.
//! 2. Re-evaluates each nonlinear device at the current iterate via
//!    [`DeviceModel::linearize`] (ADR-0005 closed-enum dispatch).
//! 3. Stamps the companion-model Jacobian and current into the clone.
//! 4. Applies source-stepping scaling to the RHS.
//! 5. Applies ground suppression (zero ground row/col, identity diagonal,
//!    zero RHS at ground).
//! 6. Converts the dense matrix to sparse triplets for the linear solver.
//!
//! # Sign convention
//!
//! Per the `device-modeling` crate and `StampInterface`:
//!
//! - The Jacobian `J[i][j]` is **added** to the MNA matrix `A`.
//! - The companion current `I_eq[k]` is **subtracted** from the RHS `b`.
//!
//! This matches the MNA convention `A · x = b` where the LHS collects
//! conductive contributions and the RHS collects source contributions.
//!
//! # ADR compliance
//!
//! - **ADR-0005**: closed-enum dispatch via `match` on `DeviceModel` and
//!   `LinearizedModel` — no `dyn` trait objects.
//! - **ADR-0003**: ground row/column kept intact during assembly;
//!   suppression applied in the sub-view extraction step.
//! - **ADR-0010**: all public types are part of the v1 unstable API.

use circuit_solver_types::{ConvergenceStatus, NodeId};
use device_modeling::stamp::OperatingPoint;
use device_modeling::DeviceModel;
use numeric_solver::gmin_stepping::{
    GminSteppingConfig, GminSteppingDriver, GminSteppingOutcome, HomotopyStatus,
};
use numeric_solver::linear_solver::{LinearSolver, SparseLinearSystem, SparseTriplet};
use numeric_solver::newton_raphson::{
    NewtonRaphsonConfig, NewtonRaphsonDriver, NewtonRaphsonOutcome, NonlinearSystem, SystemError,
};
use numeric_solver::source_stepping::{
    SourceSteppableSystem, SourceSteppingConfig, SourceSteppingDriver, SourceSteppingOutcome,
};

use crate::devices::stamp::stamp_linearized_device;
use crate::numeric::mna::{AssembledSystem, StampInterface};

// ---------------------------------------------------------------------------
// Nonlinear device entry
// ---------------------------------------------------------------------------

/// A nonlinear device that must be re-evaluated at each Newton-Raphson
/// iteration.
///
/// Stores the device model (for re-linearization) and the terminal-to-node
/// mapping. The `DeviceModel` is `Clone + Copy` (ADR-0005: inline payload,
/// no `Box`, no `dyn`), so this struct is cheap to own and clone.
#[derive(Debug, Clone)]
pub struct NonlinearDeviceEntry {
    /// The device model — used to call `DeviceModel::linearize` at each
    /// iterate with the terminal voltages extracted from the solution
    /// vector.
    pub device_model: DeviceModel,
    /// Terminal-to-node mapping in SPICE canonical order:
    ///
    /// - Diode: `[anode, cathode]`
    /// - BJT:   `[collector, base, emitter]`
    /// - MOSFET: `[drain, gate, source, bulk]`
    pub nodes: Vec<NodeId>,
}

// ---------------------------------------------------------------------------
// CircuitNonlinearSystem
// ---------------------------------------------------------------------------

/// A circuit-level nonlinear system suitable for Newton-Raphson iteration.
///
/// Wraps an MNA assembly with:
/// - A **base** `AssembledSystem` containing only linear device stamps
///   (resistors, capacitors, etc.) and zero RHS.
/// - A **source RHS** vector containing only independent source
///   contributions (voltage source branch equations, current sources).
/// - A list of **nonlinear devices** that must be re-evaluated and
///   re-stamped at each iterate.
/// - A **source alpha** factor for source stepping (1.0 = full sources).
/// - A **ground node index** for ground suppression in the sub-view.
///
/// # Ground suppression
///
/// The base system retains the full ground row/column (ADR-0003).
/// During `linearize` and `residue`, the ground row/column is zeroed,
/// the diagonal set to 1.0, and the RHS at the ground row set to 0.
/// This converts the full system into a sub-view where ground is pinned
/// to 0 V, which is the form the linear solver expects.
pub struct CircuitNonlinearSystem {
    /// Base MNA system with linear device stamps only. The RHS contains
    /// only non-source contributions (resistive stamps produce zero RHS).
    base: AssembledSystem,
    /// Independent source contributions to the RHS. During linearization,
    /// `source_alpha * source_rhs` is added to the base RHS.
    source_rhs: Vec<f64>,
    /// Nonlinear devices that must be re-evaluated at each iterate.
    nonlinear_devices: Vec<NonlinearDeviceEntry>,
    /// Source-stepping factor α ∈ [0, 1]. 1.0 = full sources (default).
    source_alpha: f64,
    /// Ground node index (conventionally 0 per ADR-0003).
    ground_node: u32,
}

impl CircuitNonlinearSystem {
    /// Construct a new nonlinear circuit system.
    ///
    /// # Arguments
    ///
    /// - `base` — assembled MNA system with linear device stamps only.
    ///   The base RHS should contain only non-source contributions.
    /// - `source_rhs` — independent source contributions to the RHS,
    ///   of length `base.dim()`. Scaled by `source_alpha` during
    ///   linearization.
    /// - `nonlinear_devices` — devices to re-evaluate at each iterate.
    /// - `ground_node` — index of the ground node (typically 0).
    ///
    /// # Panics
    ///
    /// Panics if `source_rhs.len() != base.dim() as usize`.
    pub fn new(
        base: AssembledSystem,
        source_rhs: Vec<f64>,
        nonlinear_devices: Vec<NonlinearDeviceEntry>,
        ground_node: u32,
    ) -> Self {
        assert_eq!(
            source_rhs.len(),
            base.dim() as usize,
            "source_rhs length must equal base system dimension"
        );
        Self {
            base,
            source_rhs,
            nonlinear_devices,
            source_alpha: 1.0,
            ground_node,
        }
    }

    /// Build the full linearized system at a given iterate.
    ///
    /// This is the core of the `linearize` and `residue` computations:
    /// 1. Clone the base system.
    /// 2. Stamp each nonlinear device's companion model.
    /// 3. Apply source-stepping to the RHS.
    /// 4. Apply ground suppression.
    ///
    /// Returns the modified `AssembledSystem` (dense form).
    fn build_linearized_system(&self, iterate: &[f64]) -> Result<AssembledSystem, SystemError> {
        let mut sys = self.base.clone();

        // Stamp nonlinear devices.
        for entry in &self.nonlinear_devices {
            let op = build_operating_point(&entry.device_model, &entry.nodes, iterate);
            let lin = entry
                .device_model
                .linearize(&op)
                .map_err(|e| SystemError::new(format!("device linearization failed: {e}")))?;

            stamp_linearized_device(&mut sys, &lin, &entry.nodes)
                .map_err(|e| SystemError::new(format!("device stamp failed: {e}")))?;
        }

        // Apply source stepping: add source_alpha * source_rhs to the RHS.
        let rhs = sys.rhs_mut();
        for (i, &s) in self.source_rhs.iter().enumerate() {
            rhs[i] += self.source_alpha * s;
        }

        // Apply ground suppression.
        apply_ground_suppression(&mut sys, self.ground_node);

        Ok(sys)
    }
}

// ---------------------------------------------------------------------------
// NonlinearSystem implementation
// ---------------------------------------------------------------------------

impl NonlinearSystem for CircuitNonlinearSystem {
    fn dim(&self) -> u32 {
        self.base.dim()
    }

    fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
        let sys = self.build_linearized_system(iterate)?;
        dense_to_sparse(&sys)
    }

    fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
        // Re-linearize at the residue argument (companion-form identity
        // guarantees this equals the true nonlinear residue at that point).
        let sys = self.build_linearized_system(iterate)?;

        // Compute F = A · x − b using dense matrix-vector multiply.
        let dim = sys.dim() as usize;
        let a = sys.matrix();
        let b = sys.rhs();
        let mut residue = vec![0.0; dim];
        for i in 0..dim {
            let row_offset = i * dim;
            let mut sum = 0.0_f64;
            for j in 0..dim {
                sum += a[row_offset + j] * iterate[j];
            }
            residue[i] = sum - b[i];
        }
        Ok(residue)
    }
}

// ---------------------------------------------------------------------------
// SourceSteppableSystem implementation
// ---------------------------------------------------------------------------

impl SourceSteppableSystem for CircuitNonlinearSystem {
    fn set_source_alpha(&mut self, alpha: f64) {
        self.source_alpha = alpha;
    }
}

// ---------------------------------------------------------------------------
// Helper: build OperatingPoint from iterate + node mapping
// ---------------------------------------------------------------------------

/// Extract terminal voltages from the iterate vector and construct an
/// [`OperatingPoint`] matching the device family.
///
/// # Panics
///
/// Panics if the node indices are out of bounds or the device family
/// doesn't match the expected terminal count.
fn build_operating_point(
    device_model: &DeviceModel,
    nodes: &[NodeId],
    iterate: &[f64],
) -> OperatingPoint {
    let voltages: Vec<f64> = nodes
        .iter()
        .map(|n| {
            let idx = n.index() as usize;
            iterate.get(idx).copied().unwrap_or(0.0)
        })
        .collect();

    match device_model {
        DeviceModel::Diode(_) => {
            assert_eq!(
                voltages.len(),
                2,
                "Diode requires exactly 2 terminal voltages"
            );
            let mut arr = [0.0_f64; 2];
            arr.copy_from_slice(&voltages[..2]);
            OperatingPoint::Diode(arr)
        }
        DeviceModel::BJT(_) => {
            assert_eq!(
                voltages.len(),
                3,
                "BJT requires exactly 3 terminal voltages"
            );
            let mut arr = [0.0_f64; 3];
            arr.copy_from_slice(&voltages[..3]);
            OperatingPoint::BJT(arr)
        }
        DeviceModel::MOSFET(_) => {
            assert_eq!(
                voltages.len(),
                4,
                "MOSFET requires exactly 4 terminal voltages"
            );
            let mut arr = [0.0_f64; 4];
            arr.copy_from_slice(&voltages[..4]);
            OperatingPoint::MOSFET(arr)
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: ground suppression
// ---------------------------------------------------------------------------

/// Apply ground suppression to an assembled system in-place.
///
/// Zeros the ground row and column, sets the ground diagonal to 1.0,
/// and zeros the RHS at the ground row. This pins the ground node to
/// 0 V, converting the full system into a solvable sub-view.
fn apply_ground_suppression(sys: &mut AssembledSystem, ground: u32) {
    let dim = sys.dim() as usize;
    let g = ground as usize;

    // Zero the ground row and column in the matrix.
    let a = sys.matrix_mut();
    for c in 0..dim {
        a[g * dim + c] = 0.0; // zero row g
        a[c * dim + g] = 0.0; // zero col g
    }
    // Set ground diagonal to 1.0 (identity row).
    a[g * dim + g] = 1.0;

    // Zero the RHS at the ground row.
    sys.rhs_mut()[g] = 0.0;
}

// ---------------------------------------------------------------------------
// Helper: dense → sparse conversion
// ---------------------------------------------------------------------------

/// Convert an `AssembledSystem` (dense row-major matrix) into a
/// `SparseLinearSystem<f64>` (coordinate triplet form).
///
/// Only non-zero matrix entries are emitted as triplets. The partition
/// (node_count, branch_count) is preserved from the dense system.
fn dense_to_sparse(sys: &AssembledSystem) -> Result<SparseLinearSystem<f64>, SystemError> {
    let dim = sys.dim();
    let node_count = sys.node_count();
    let branch_count = sys.branch_count();
    let a = sys.matrix();
    let b = sys.rhs();

    let mut triplets = Vec::new();
    let dim_usize = dim as usize;

    for r in 0..dim_usize {
        for c in 0..dim_usize {
            let val = a[r * dim_usize + c];
            if val != 0.0 {
                triplets.push(SparseTriplet {
                    row: r as u32,
                    col: c as u32,
                    value: val,
                });
            }
        }
    }

    SparseLinearSystem::new(dim, node_count, branch_count, triplets, b.to_vec())
        .map_err(|e| SystemError::new(format!("sparse system construction failed: {e}")))
}

// ---------------------------------------------------------------------------
// StampInterface impl for AssembledSystem
// ---------------------------------------------------------------------------

/// Allow `stamp_linearized_device` to stamp directly into an
/// `AssembledSystem` for re-stamping nonlinear devices.
impl StampInterface for AssembledSystem {
    type Error = StampAsSystemError;

    fn dim(&self) -> u32 {
        AssembledSystem::dim(self)
    }

    fn node_count(&self) -> u32 {
        AssembledSystem::node_count(self)
    }

    fn branch_count(&self) -> u32 {
        AssembledSystem::branch_count(self)
    }

    fn stamp_matrix(&mut self, row: u32, col: u32, value: f64) -> Result<(), Self::Error> {
        let dim = self.dim();
        if row >= dim {
            return Err(StampAsSystemError::RowOutOfRange { row, dim });
        }
        if col >= dim {
            return Err(StampAsSystemError::ColOutOfRange { col, dim });
        }
        if !value.is_finite() {
            return Err(StampAsSystemError::NonFiniteValue { row, col, value });
        }
        self.set_matrix(row, col, self.get_matrix(row, col) + value);
        Ok(())
    }

    fn stamp_rhs(&mut self, row: u32, value: f64) -> Result<(), Self::Error> {
        let dim = self.dim();
        if row >= dim {
            return Err(StampAsSystemError::RowOutOfRange { row, dim });
        }
        if !value.is_finite() {
            return Err(StampAsSystemError::NonFiniteValue { row, col: 0, value });
        }
        self.set_rhs(row, self.get_rhs(row) + value);
        Ok(())
    }

    fn finish(self) -> Result<AssembledSystem, Self::Error> {
        Ok(self)
    }
}

/// Error type for stamping into an `AssembledSystem`.
#[derive(Debug, Clone, PartialEq)]
pub enum StampAsSystemError {
    /// Row index out of range.
    RowOutOfRange {
        /// The offending row.
        row: u32,
        /// The valid upper bound (dim).
        dim: u32,
    },
    /// Column index out of range.
    ColOutOfRange {
        /// The offending column.
        col: u32,
        /// The valid upper bound (dim).
        dim: u32,
    },
    /// Non-finite value encountered during stamping.
    NonFiniteValue {
        /// Row being stamped.
        row: u32,
        /// Column being stamped (0 for RHS stamps).
        col: u32,
        /// The offending value.
        value: f64,
    },
}

impl core::fmt::Display for StampAsSystemError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RowOutOfRange { row, dim } => {
                write!(f, "row index {row} out of range 0..{dim}")
            }
            Self::ColOutOfRange { col, dim } => {
                write!(f, "column index {col} out of range 0..{dim}")
            }
            Self::NonFiniteValue { row, col, value } => {
                write!(f, "non-finite stamp value {value} at row {row}, col {col}")
            }
        }
    }
}

impl std::error::Error for StampAsSystemError {}

// ---------------------------------------------------------------------------
// DC operating-point result
// ---------------------------------------------------------------------------

/// Result of a DC operating-point solve attempt.
///
/// Carries the final iterate, the convergence status, and which tier
/// of the 3-tier fallback succeeded (or failed).
#[derive(Debug, Clone)]
pub struct DcOperatingPointResult {
    /// The final Newton-Raphson iterate. On convergence this is the
    /// accepted operating point; on failure it is the last finite
    /// iterate produced.
    pub iterate: Vec<f64>,
    /// Convergence status from the final NR run or homotopy step.
    pub status: ConvergenceStatus,
    /// Which fallback tier produced this result.
    pub method: DcMethod,
}

/// Which tier of the 3-tier DC fallback produced the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcMethod {
    /// Plain Newton-Raphson converged on the first attempt.
    PlainNewton,
    /// Gmin-stepping homotopy converged.
    GminStepping {
        /// Number of homotopy steps taken.
        steps: u32,
    },
    /// Source-stepping homotopy converged.
    SourceStepping {
        /// Number of accepted α values (including 0.0 and 1.0).
        homotopy_steps: u32,
    },
    /// All three tiers failed. The iterate and status reflect the
    /// last attempt (source stepping).
    Failed,
}

// ---------------------------------------------------------------------------
// solve_dc_operating_point
// ---------------------------------------------------------------------------

/// Run the 3-tier DC operating-point solver.
///
/// # Tier 1: Plain Newton-Raphson
///
/// Attempts to solve the nonlinear system with a direct NR iteration.
/// If NR converges (`ConvergenceStatus::Converged`), returns immediately.
///
/// # Tier 2: Gmin-stepping homotopy
///
/// If plain NR fails, adds diagonal shunt conductances (Gmin) to each
/// non-ground node and walks them geometrically to zero. Each step
/// uses the previous converged iterate as the starting point.
///
/// # Tier 3: Source-stepping homotopy
///
/// If Gmin stepping fails, ramps independent sources from 0 to full
/// value (α = 0 → 1) in steps, using the previous converged iterate
/// as the starting point for each α value.
///
/// # Arguments
///
/// - `system` — the nonlinear circuit system (borrowed mutably; not
///   consumed).
/// - `solver` — the linear solver backend.
/// - `initial_iterate` — the starting voltage/current vector.
///
/// # Returns
///
/// A [`DcOperatingPointResult`] with the final iterate, convergence
/// status, and the method that produced the result.
pub fn solve_dc_operating_point<S, L>(
    system: &mut S,
    solver: &L,
    initial_iterate: Vec<f64>,
) -> DcOperatingPointResult
where
    S: SourceSteppableSystem,
    L: LinearSolver<f64>,
{
    let nr_config = NewtonRaphsonConfig::DC_DEFAULTS;

    // ─── Tier 1: Plain Newton-Raphson ──────────────────────────
    let nr_outcome = NewtonRaphsonDriver.solve(nr_config, system, solver, initial_iterate.clone());

    match nr_outcome {
        Ok(NewtonRaphsonOutcome {
            ref iterate,
            status: ConvergenceStatus::Converged(_),
        }) => {
            DcOperatingPointResult {
                iterate: iterate.clone(),
                status: ConvergenceStatus::Converged(
                    *nr_outcome.as_ref().unwrap().status.diagnostic(),
                ),
                method: DcMethod::PlainNewton,
            }
        }
        Ok(NewtonRaphsonOutcome { iterate, status }) => {
            // NR didn't converge; fall through to Tier 2.
            // Use the last finite iterate as warm start.
            let warm_start = iterate;
            run_tier2_gmin_stepping(system, solver, warm_start, status)
        }
        Err(_) => {
            // Hard NR error (dim mismatch, modeling error, etc.).
            // Fall through to Tier 2 with the original initial iterate.
            run_tier2_gmin_stepping(system, solver, initial_iterate, ConvergenceStatus::Diverged(
                circuit_solver_types::ConvergenceDiagnostic {
                    update_norm: f64::INFINITY,
                    residue_norm: f64::INFINITY,
                    iterations: 0,
                    tolerances: nr_config.tolerances,
                },
            ))
        }
    }
}

/// Tier 2: Gmin-stepping homotopy.
fn run_tier2_gmin_stepping<S, L>(
    system: &mut S,
    solver: &L,
    warm_start: Vec<f64>,
    fallback_status: ConvergenceStatus,
) -> DcOperatingPointResult
where
    S: SourceSteppableSystem,
    L: LinearSolver<f64>,
{
    let gmin_config = GminSteppingConfig::DC_DEFAULTS;

    let gmin_outcome = GminSteppingDriver.solve(gmin_config, system, solver, warm_start.clone());

    match gmin_outcome {
        Ok(GminSteppingOutcome {
            iterate,
            status: HomotopyStatus::ConvergedViaHomotopy { steps, final_diagnostic },
        }) => {
            DcOperatingPointResult {
                iterate,
                status: ConvergenceStatus::Converged(final_diagnostic),
                method: DcMethod::GminStepping { steps },
            }
        }
        Ok(GminSteppingOutcome {
            iterate: gmin_iterate,
            status: HomotopyStatus::StepFailed { inner_status, .. },
        }) => {
            // Gmin stepping failed; fall through to Tier 3.
            run_tier3_source_stepping(
                system,
                solver,
                gmin_iterate,
                inner_status,
            )
        }
        Err(_) => {
            // Hard error in Gmin stepping.
            run_tier3_source_stepping(
                system,
                solver,
                warm_start,
                fallback_status,
            )
        }
    }
}

/// Tier 3: Source-stepping homotopy.
fn run_tier3_source_stepping<S, L>(
    system: &mut S,
    solver: &L,
    warm_start: Vec<f64>,
    fallback_status: ConvergenceStatus,
) -> DcOperatingPointResult
where
    S: SourceSteppableSystem,
    L: LinearSolver<f64>,
{
    let source_config = SourceSteppingConfig::dc_defaults();

    let source_outcome =
        SourceSteppingDriver.solve(&source_config, system, solver, warm_start.clone());

    match source_outcome {
        Ok(SourceSteppingOutcome {
            ref iterate,
            status: ConvergenceStatus::Converged(_),
            homotopy_steps,
            ..
        }) => {
            DcOperatingPointResult {
                iterate: iterate.clone(),
                status: source_outcome.as_ref().unwrap().status,
                method: DcMethod::SourceStepping { homotopy_steps },
            }
        }
        Ok(SourceSteppingOutcome { iterate, status, .. }) => {
            // Source stepping failed too — all tiers exhausted.
            DcOperatingPointResult {
                iterate,
                status,
                method: DcMethod::Failed,
            }
        }
        Err(_) => {
            // Hard error in source stepping — all tiers exhausted.
            DcOperatingPointResult {
                iterate: warm_start,
                status: fallback_status,
                method: DcMethod::Failed,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::model::DiodeParams;
    use crate::numeric::mna::IncrementalMnaBuilder;
    use circuit_solver_types::flattened::FlattenedStructure;
    use numeric_solver::linear_solver::RussellRealSolver;

    /// Build a FlattenedStructure with `n` nodes (including ground), 0 branches.
    fn make_flat(node_count: u32) -> FlattenedStructure {
        FlattenedStructure::new(node_count, 0, vec![]).expect("flat should be valid")
    }

    // ------------------------------------------------------------------
    // Ground suppression test
    // ------------------------------------------------------------------

    #[test]
    fn ground_suppression_zeros_row_col_and_sets_diagonal() {
        // Build a 3-node system (ground + 2 signal nodes) with some
        // values in the ground row/column.
        let flat = make_flat(3);
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");

        // Stamp some conductance to populate the matrix.
        builder.stamp_conductive(0, 1, 1.0).expect("stamp should work");
        builder.stamp_conductive(1, 2, 2.0).expect("stamp should work");

        let mut sys = builder.finish().expect("finish should succeed");

        // Verify ground row/col are non-zero before suppression.
        assert_ne!(sys.get_matrix(0, 1), 0.0, "pre-condition: A[0,1] should be non-zero");
        assert_ne!(sys.get_matrix(1, 0), 0.0, "pre-condition: A[1,0] should be non-zero");

        apply_ground_suppression(&mut sys, 0);

        // Ground row: all zeros except diagonal = 1.0.
        assert_eq!(sys.get_matrix(0, 0), 1.0, "ground diagonal should be 1.0");
        assert_eq!(sys.get_matrix(0, 1), 0.0, "ground row should be zero");
        assert_eq!(sys.get_matrix(0, 2), 0.0, "ground row should be zero");

        // Ground column: all zeros except diagonal = 1.0.
        assert_eq!(sys.get_matrix(1, 0), 0.0, "ground col should be zero");
        assert_eq!(sys.get_matrix(2, 0), 0.0, "ground col should be zero");

        // Ground RHS should be zero.
        assert_eq!(sys.get_rhs(0), 0.0, "ground RHS should be zero");

        // Non-ground entries should be preserved.
        assert_eq!(sys.get_matrix(1, 1), 3.0, "A[1,1] = 1.0 + 2.0");
        assert_eq!(sys.get_matrix(1, 2), -2.0, "A[1,2] = -2.0");
        assert_eq!(sys.get_matrix(2, 1), -2.0, "A[2,1] = -2.0");
        assert_eq!(sys.get_matrix(2, 2), 2.0, "A[2,2] = 2.0");
    }

    // ------------------------------------------------------------------
    // Dense-to-sparse conversion test
    // ------------------------------------------------------------------

    #[test]
    fn dense_to_sparse_produces_correct_triplets() {
        let flat = make_flat(2);
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");
        builder.stamp_conductive(0, 1, 5.0).expect("stamp should work");
        let sys = builder.finish().expect("finish should succeed");

        let sparse = dense_to_sparse(&sys).expect("conversion should succeed");
        assert_eq!(sparse.dim(), 2);
        assert_eq!(sparse.node_count(), 2);
        assert_eq!(sparse.branch_count(), 0);

        // Should have 4 non-zero entries: (0,0,5), (0,1,-5), (1,0,-5), (1,1,5).
        assert_eq!(sparse.triplets().len(), 4);

        // Verify specific triplets exist (order not guaranteed, check values).
        let trip = sparse.triplets();
        let find = |r, c| trip.iter().find(|t| t.row == r && t.col == c).map(|t| t.value);
        assert_eq!(find(0, 0), Some(5.0));
        assert_eq!(find(0, 1), Some(-5.0));
        assert_eq!(find(1, 0), Some(-5.0));
        assert_eq!(find(1, 1), Some(5.0));
    }

    // ------------------------------------------------------------------
    // StampInterface impl for AssembledSystem test
    // ------------------------------------------------------------------

    #[test]
    fn assembled_system_stamp_interface_adds_values() {
        let flat = make_flat(3);
        let builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");
        let mut sys = builder.finish().expect("finish should succeed");

        // Stamp a value via the StampInterface impl.
        <AssembledSystem as StampInterface>::stamp_matrix(&mut sys, 1, 1, 10.0)
            .expect("stamp should work");
        assert_eq!(sys.get_matrix(1, 1), 10.0);

        // Stamp RHS.
        <AssembledSystem as StampInterface>::stamp_rhs(&mut sys, 1, 5.0)
            .expect("stamp rhs should work");
        assert_eq!(sys.get_rhs(1), 5.0);
    }

    // ------------------------------------------------------------------
    // NonlinearSystem dim test
    // ------------------------------------------------------------------

    #[test]
    fn circuit_nonlinear_system_dim_matches_base() {
        let flat = make_flat(3);
        let builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");
        let base = builder.finish().expect("finish should succeed");
        let dim = base.dim();

        let sys = CircuitNonlinearSystem::new(base, vec![0.0; dim as usize], vec![], 0);
        assert_eq!(sys.dim(), dim);
    }

    // ------------------------------------------------------------------
    // Linearize produces valid sparse system
    // ------------------------------------------------------------------

    #[test]
    fn linearize_produces_valid_sparse_system() {
        let flat = make_flat(3);
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");
        builder.stamp_conductive(1, 2, 1.0).expect("stamp should work");
        let base = builder.finish().expect("finish should succeed");
        let dim = base.dim();

        let mut sys = CircuitNonlinearSystem::new(base, vec![0.0; dim as usize], vec![], 0);

        let iterate = vec![0.0; dim as usize];
        let sparse = sys.linearize(&iterate).expect("linearize should succeed");

        assert_eq!(sparse.dim(), dim);
        assert_eq!(sparse.node_count(), 3);
        assert_eq!(sparse.branch_count(), 0);
    }

    // ------------------------------------------------------------------
    // Linearize with a diode produces non-zero stamp
    // ------------------------------------------------------------------

    #[test]
    fn linearize_with_diode_produces_nonzero_stamp() {
        let flat = make_flat(3);
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");
        // Add a resistor between nodes 1 and 2.
        builder.stamp_conductive(1, 2, 0.01).expect("stamp should work");
        let base = builder.finish().expect("finish should succeed");
        let dim = base.dim();

        // Add a diode between node 1 (anode) and ground (cathode).
        let diode = NonlinearDeviceEntry {
            device_model: DeviceModel::Diode(DiodeParams::default()),
            nodes: vec![NodeId::new(1), NodeId::new(0)],
        };

        let mut sys = CircuitNonlinearSystem::new(
            base,
            vec![0.0; dim as usize],
            vec![diode],
            0,
        );

        // Forward bias: V_anode = 0.7, V_cathode = 0, V_2 = 0.
        let iterate = vec![0.0, 0.7, 0.0];
        let sparse = sys.linearize(&iterate).expect("linearize should succeed");

        // The diode at 0.7 V forward bias should produce non-zero
        // conductance (gd >> 0).
        let trip = sparse.triplets();
        let a11 = trip.iter().find(|t| t.row == 1 && t.col == 1).map(|t| t.value);
        assert!(a11.unwrap_or(0.0) > 0.01, "A[1,1] should include diode gd > 0.01, got {:?}", a11);
    }

    // ------------------------------------------------------------------
    // Source stepping scaling test
    // ------------------------------------------------------------------

    #[test]
    fn source_alpha_scales_rhs() {
        let flat = make_flat(2);
        let builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");
        let base = builder.finish().expect("finish should succeed");
        let dim = base.dim();

        // Source RHS: 5.0 V source at node 1.
        let source_rhs = vec![0.0, 5.0];

        let mut sys = CircuitNonlinearSystem::new(base, source_rhs.clone(), vec![], 0);

        // With alpha = 0.5, the RHS at node 1 should be 2.5.
        sys.set_source_alpha(0.5);
        let iterate = vec![0.0; dim as usize];
        let sparse = sys.linearize(&iterate).expect("linearize should succeed");

        // Ground RHS should be 0 (suppressed).
        assert_eq!(sparse.rhs()[0], 0.0);
        // Node 1 RHS should be 0.5 * 5.0 = 2.5.
        let tol = 1e-12;
        assert!(
            (sparse.rhs()[1] - 2.5).abs() < tol,
            "rhs[1] should be 2.5, got {}",
            sparse.rhs()[1]
        );
    }

    // ------------------------------------------------------------------
    // Residue computation test
    // ------------------------------------------------------------------

    #[test]
    fn residue_at_zero_iterate_is_minus_rhs() {
        // At x = 0: F(0) = A·0 − b = −b
        let flat = make_flat(2);
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");
        builder.stamp_conductive(0, 1, 1.0).expect("stamp should work");
        let base = builder.finish().expect("finish should succeed");
        let dim = base.dim();

        let source_rhs = vec![0.0, 3.0]; // 3A current source at node 1
        let mut sys = CircuitNonlinearSystem::new(base, source_rhs, vec![], 0);

        let iterate = vec![0.0; dim as usize];
        let residue = sys.residue(&iterate).expect("residue should succeed");

        // Ground row: residue = 1*0 − 0 = 0 (after suppression: A*gnd = identity, b*gnd = 0)
        // Actually with ground suppression: A[0,0]=1, A[0,1]=0, b[0]=0
        // residue[0] = 1*0 + 0*0 - 0 = 0
        assert_eq!(residue[0], 0.0, "ground residue should be zero");

        // Node 1: after suppression A[1,0]=0, A[1,1]=1, b[1]=3.0
        // residue[1] = 0*0 + 1*0 - 3.0 = -3.0
        let tol = 1e-12;
        assert!(
            (residue[1] - (-3.0)).abs() < tol,
            "residue[1] should be -3.0, got {}",
            residue[1]
        );
    }

    // ------------------------------------------------------------------
    // solve_dc_operating_point: trivial linear circuit converges via
    // plain Newton-Raphson
    // ------------------------------------------------------------------

    #[test]
    fn solve_dc_converges_linear_circuit_via_plain_newton() {
        // Simple circuit: 1Ω resistor from node 1 to ground, 1A current
        // source into node 1. Expected: V1 = 1.0 V.
        let flat = make_flat(2);
        let mut builder = IncrementalMnaBuilder::new(&flat).expect("builder should succeed");
        builder.stamp_conductive(0, 1, 1.0).expect("stamp should work"); // G = 1/1Ω = 1 S
        let base = builder.finish().expect("finish should succeed");
        let dim = base.dim();

        let source_rhs = vec![0.0, 1.0]; // 1A into node 1

        let mut sys = CircuitNonlinearSystem::new(base, source_rhs, vec![], 0);
        let solver = RussellRealSolver;

        let result = solve_dc_operating_point(&mut sys, &solver, vec![0.0; dim as usize]);

        assert!(
            matches!(result.method, DcMethod::PlainNewton),
            "linear circuit should converge via plain Newton, got {:?}",
            result.method
        );
        assert!(
            matches!(result.status, ConvergenceStatus::Converged(_)),
            "should converge, got {:?}",
            result.status
        );

        // Ground node voltage should be 0.
        let tol = 1e-9;
        assert!(
            result.iterate[0].abs() < tol,
            "ground voltage should be ~0, got {}",
            result.iterate[0]
        );
        // Node 1 voltage should be 1.0 V (I = 1A, G = 1S → V = I/G = 1.0).
        assert!(
            (result.iterate[1] - 1.0).abs() < tol,
            "V1 should be 1.0V, got {}",
            result.iterate[1]
        );
    }

    // ------------------------------------------------------------------
    // build_operating_point test
    // ------------------------------------------------------------------

    #[test]
    fn build_operating_point_extracts_correct_voltages() {
        let diode = DeviceModel::Diode(DiodeParams::default());
        let nodes = vec![NodeId::new(1), NodeId::new(0)];
        let iterate = vec![0.0, 0.7, 0.0];

        let op = build_operating_point(&diode, &nodes, &iterate);
        match op {
            OperatingPoint::Diode([va, vc]) => {
                let tol = 1e-15;
                assert!((va - 0.7).abs() < tol, "V_anode should be 0.7, got {va}");
                assert!((vc - 0.0).abs() < tol, "V_cathode should be 0.0, got {vc}");
            }
            other => panic!("expected Diode operating point, got {other:?}"),
        }
    }
}
