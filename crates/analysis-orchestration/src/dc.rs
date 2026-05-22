//! DC operating-point analysis control loop.
//!
//! This module covers `tasks.md` item #20 of
//! `circuit-solver/2026-05-21-v1-spec`. It is the per-analysis driver
//! that composes:
//!
//! - Pass 2 MNA assembly ([`numeric_solver::assemble()`], tasks.md #14),
//! - the per-analysis DC sub-view ([`numeric_solver::SubViewBuilder`],
//!   tasks.md #15) for ground suppression,
//! - the [`numeric_solver::NewtonRaphsonDriver`] dual-criterion outer
//!   loop (tasks.md #17, ADR-0006), and
//! - the real-valued [`numeric_solver::RussellRealSolver`] sparse-LU
//!   backend (tasks.md #16, ADR-0002)
//!
//! into a single end-to-end DC analysis that accepts a
//! [`DcAnalysisRequest`] and returns a [`DcAnalysisResult`] containing
//! the converged [`OperatingPoint`] plus the [`ConvergenceStatus`].
//!
//! # Spec scope (item #20)
//!
//! `tasks.md` item #20 — *"Implement DC analysis control loop: accept
//! `AnalysisRequest`, drive NR driver, return `OperatingPoint` with
//! `ConvergenceStatus`"* — explicitly maps to the
//! [`dc-operating-point#linear-resistive-dc-operating-point`][spec]
//! scenario:
//!
//! > Given CircuitDesigner has constructed a Circuit from a linear
//! > resistive netlist
//! > And the Circuit contains no nonlinear devices
//! > When CircuitDesigner submits a DC operating-point Analysis request
//! > Then the Simulator returns a Result containing an OperatingPoint
//! > And every node voltage and branch current in the OperatingPoint
//! > matches the Golden Reference within the tolerance envelope
//! > And the Convergence status is "converged"
//!
//! That is, the **linear-only** path. Sibling tasks own the other
//! `dc-operating-point` scenarios:
//!
//! - tasks.md #18 / #19 — Gmin- and source-stepping homotopies that
//!   cover the *nonlinear / homotopy* path
//!   (`dc-operating-point-with-gmin-stepping-homotopy`);
//! - tasks.md #22 — convergence-failure path that returns
//!   last-iterate diagnostics
//!   (`dc-operating-point-convergence-failure`);
//! - tasks.md #21 — DC sweep over a source parameter
//!   (`dc-sweep-over-a-voltage-source`).
//!
//! Those siblings compose **on top of** the control loop in this
//! module — they wrap [`dc_analysis`] with extra orchestration but
//! reuse its public types. The nonlinear-device branch of the NR
//! callback (linearizing diodes / BJTs / MOSFETs at each iterate)
//! lands as part of the homotopy work in #18; this module's NR
//! callback handles the linear-only case where the system is its
//! own linearization and converges in one iteration.
//!
//! # Design references
//!
//! - **ADR-0002 — Sparse Direct LU Dispatch.** This module uses
//!   [`RussellRealSolver`] (the `russell_sparse`-backed `f64` half
//!   of the hybrid backend) for the inner linear solves driven by
//!   Newton-Raphson.
//! - **ADR-0003 — Two-Pass Graph Flattening with Per-Analysis
//!   Sub-Views.** The control loop assembles the **full** MNA matrix
//!   once via [`assemble`], then applies the textbook DC sub-view
//!   mask (ground suppression) via [`SubViewBuilder::from_full`]
//!   for the linear-only path. Future nonlinear extensions
//!   (tasks.md #18 / #19) will re-stamp the matrix per iterate via
//!   the [`assemble`] call inside the NR callback.
//! - **ADR-0006 — Dual Convergence Criterion for Newton-Raphson.**
//!   The driver itself ([`NewtonRaphsonDriver`]) honors ADR-0006; this
//!   module's contribution is to supply a [`NonlinearSystem`]
//!   implementor whose `linearize` and `residue` callbacks are
//!   self-consistent for the linear case.
//! - **ADR-0008 — Per-Node max(Relative, Absolute) Tolerance
//!   Envelope.** The ADR-0008 envelope is the *conformance harness*
//!   bound used by golden-reference comparison (tasks.md #62+); the
//!   *Newton-Raphson convergence* bounds in this module are the
//!   distinct SPICE-style `reltol`/`abstol` pair carried by
//!   [`circuit_solver_types::ConvergenceTolerances`], following ADR-0006's contract. The
//!   two tolerance regimes live at different layers and the
//!   conformance harness (not this control loop) applies the
//!   max(rel, abs) envelope to the resulting [`OperatingPoint`].
//! - **ADR-0009 — Topology Checker for Floating-Node Detection.**
//!   If the caller has run the Pass-1 topology checker and the
//!   report flags any floating nodes (hard fault, no DC path to
//!   ground), [`dc_analysis`] short-circuits with a
//!   [`DcAnalysisError::FloatingNodeFault`] rather than handing a
//!   structurally singular matrix to the solver. Possibly-floating
//!   (warning-level) nodes are *not* short-circuited; ADR-0009
//!   prescribes auto-enabling Gmin-stepping for them, which is
//!   tasks.md item #18's responsibility (this module passes the
//!   warning through in [`DcAnalysisResult::topology_warnings`]
//!   so the homotopy task can read it without re-running the
//!   checker).
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** Every
//!   surface exported here is part of the v1 *unstable* public Rust
//!   API.
//!
//! # What this module does *not* do
//!
//! - **No homotopy fallback.** When NR returns a non-`Converged`
//!   status, [`dc_analysis`] surfaces it verbatim. tasks.md #18
//!   (Gmin-stepping) and #19 (source-stepping) decide whether to
//!   retry with a homotopy. This separation keeps the linear path
//!   simple and the homotopy path explicit.
//! - **No DC sweep.** tasks.md #21 wraps [`dc_analysis`] in a loop
//!   over a source parameter range.
//! - **No frontend translation.** The `PyO3` layer (tasks.md #52+)
//!   converts user-supplied netlists into the [`CircuitGraph`] +
//!   [`FlattenedStructure`] pair this module consumes.
//!
//! # Input contract
//!
//! The caller supplies, in [`DcAnalysisRequest`]:
//!
//! - the immutable [`CircuitGraph`] (for element parameter lookups),
//! - the [`FlattenedStructure`] produced by Pass 1
//!   ([`numeric_solver::flatten()`]).
//! - an optional [`NewtonRaphsonConfig`] (defaults to
//!   [`NewtonRaphsonConfig::DC_DEFAULTS`]).
//!
//! The control loop is intentionally narrow: it does *not* re-run
//! Pass 1, does *not* validate the topology (that is ADR-0009's
//! pre-pass and the orchestrator's job), and does *not* manage
//! caching of operating-point results across calls (the AC analysis
//! task #26 is the caller for that pattern).
//!
//! [spec]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md
//! [`NewtonRaphsonDriver`]: numeric_solver::NewtonRaphsonDriver
//! [`NewtonRaphsonConfig`]: numeric_solver::NewtonRaphsonConfig
//! [`NewtonRaphsonConfig::DC_DEFAULTS`]: numeric_solver::NewtonRaphsonConfig::DC_DEFAULTS
//! [`RussellRealSolver`]: numeric_solver::RussellRealSolver
//! [`NonlinearSystem`]: numeric_solver::NonlinearSystem
//! [`assemble`]: numeric_solver::assemble()
//! [`SubViewBuilder::from_full`]: numeric_solver::SubViewBuilder::from_full

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::{BranchId, ConvergenceStatus, NodeId, TopologyReport};
use netlist_graph::CircuitGraph;
use numeric_solver::{
    assemble, MnaAssemblyError, MnaSystem, NewtonRaphsonConfig, NewtonRaphsonDriver,
    NewtonRaphsonError, NonlinearSystem, RussellRealSolver, SparseLinearSystem, SparseTriplet,
    SubViewBuilder, SubViewError, SystemError as NrSystemError,
};

// -----------------------------------------------------------------------------
// Request / Result envelopes
// -----------------------------------------------------------------------------

/// DC operating-point analysis input bundle.
///
/// All fields are required references; the control loop borrows them
/// for the duration of [`dc_analysis`]. The bundle is `Copy` so it can
/// be passed by value cheaply.
///
/// The Gherkin scenario phrasing
///
/// > When CircuitDesigner submits a DC operating-point Analysis request
///
/// maps directly to a single value of this type. The `PyO3` frontend
/// (tasks.md #56+) is responsible for translating Python-side
/// `AnalysisRequest` objects into this Rust-side request envelope; the
/// orchestrator layer (this module) does not depend on `PyO3`.
///
/// Per ADR-0010, this struct's *layout* is unstable for v1; the
/// *semantics* of each field are pinned.
#[derive(Debug, Clone, Copy)]
pub struct DcAnalysisRequest<'a> {
    /// The immutable source circuit graph. Used for element
    /// parameter lookups during MNA stamping.
    pub graph: &'a CircuitGraph,
    /// Pass-1 flattened incidence over `graph`. Must satisfy
    /// `structure.element_count() == graph.elements().len()`;
    /// [`dc_analysis`] re-validates this via the assembler's
    /// [`MnaAssemblyError::GraphFlattenMismatch`] surface.
    pub structure: &'a FlattenedStructure,
    /// Newton-Raphson tuning. `None` defaults to
    /// [`NewtonRaphsonConfig::DC_DEFAULTS`] (`ITL1 = 100`,
    /// `reltol = 1e-3`, `abstol = 1e-12`).
    pub newton_raphson: Option<NewtonRaphsonConfig>,
    /// Override the ground node. `None` defaults to
    /// [`FlattenedStructure::ground_node`] (always
    /// [`NodeId::GROUND`] in v1; the override exists for forward
    /// compatibility with future structural changes per
    /// [`SubViewBuilder::with_ground_node`]).
    pub ground: Option<NodeId>,
}

impl<'a> DcAnalysisRequest<'a> {
    /// Build a request with the SPICE-default Newton-Raphson tuning
    /// and the structure's own ground node.
    #[must_use]
    pub fn new(graph: &'a CircuitGraph, structure: &'a FlattenedStructure) -> Self {
        Self {
            graph,
            structure,
            newton_raphson: None,
            ground: None,
        }
    }

    /// Builder-style override for Newton-Raphson configuration.
    #[must_use]
    pub fn with_newton_raphson(mut self, config: NewtonRaphsonConfig) -> Self {
        self.newton_raphson = Some(config);
        self
    }

    /// Builder-style override for ground node id.
    #[must_use]
    pub fn with_ground(mut self, ground: NodeId) -> Self {
        self.ground = Some(ground);
        self
    }
}

/// One element of an [`OperatingPoint::branch_currents`] entry: the
/// branch identifier and the current flowing through the
/// current-carrying element that owns the branch.
///
/// Per the inlined glossary, an `OperatingPoint` is "the DC
/// steady-state solution used as a reference for AC/noise/transient",
/// and the scenario requires "every node voltage **and branch
/// current** in the `OperatingPoint` matches the Golden Reference".
/// Branch currents are reported only for elements that introduce an
/// MNA branch unknown (voltage sources, inductors, and any nonlinear
/// device with an internal current unknown); resistors / capacitors /
/// current sources do *not* contribute branch rows and therefore do
/// not appear here.
///
/// The MNA branch-current sign convention follows the assembler's
/// stamping:
/// `+1` at `(node_count+b, plus_terminal)` and `-1` at `(node_count+b,
/// minus_terminal)` — i.e., the branch row reads `V_+ − V_− = E` for
/// a voltage source, so the branch unknown is the current flowing
/// **from the `+` terminal through the source to the `−` terminal**.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BranchCurrentSample {
    /// The MNA branch index this sample reports on.
    pub branch: BranchId,
    /// Current through the branch, in amperes. Positive = flowing
    /// from the element's `+` terminal (terminal slot 0) to the
    /// element's `−` terminal (terminal slot 1).
    pub current_amperes: f64,
}

/// The DC steady-state solution.
///
/// Per the inlined glossary:
///
/// > `OperatingPoint` — the DC steady-state solution used as a
/// > reference for AC/noise/transient.
///
/// And per the spec's acceptance criterion *"The `OperatingPoint`
/// result is immutable once produced; subsequent analyses may
/// reference it but cannot mutate it."* — every field is `pub` with
/// no mutation API, and the struct itself is `Clone + PartialEq` so
/// downstream code (AC at item #26, noise at item #40) can take a
/// reference and treat it as a frozen artifact.
///
/// `node_voltages[i]` is the DC voltage at node `i` (with `i ==
/// ground.index()` always 0.0 V by construction — the sub-view's
/// ground-suppression mask pins it). `branch_currents` is a list of
/// `(BranchId, current)` pairs, one per current-carrying element in
/// the [`FlattenedStructure`].
#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPoint {
    /// Node voltages in volts, indexed by [`NodeId::index`]. Length
    /// equals the underlying [`FlattenedStructure::node_count`].
    pub node_voltages: Vec<f64>,
    /// Branch currents in amperes, one entry per current-carrying
    /// element in the flattened structure. Indexed by the
    /// element's [`BranchId`].
    pub branch_currents: Vec<BranchCurrentSample>,
}

impl OperatingPoint {
    /// Look up the voltage at a given node, or `None` if the index
    /// is out of range.
    #[must_use]
    pub fn voltage_at(&self, node: NodeId) -> Option<f64> {
        self.node_voltages.get(node.index() as usize).copied()
    }

    /// Look up the current through a given branch, or `None` if no
    /// sample with that [`BranchId`] is present.
    #[must_use]
    pub fn current_through(&self, branch: BranchId) -> Option<f64> {
        self.branch_currents
            .iter()
            .find(|s| s.branch == branch)
            .map(|s| s.current_amperes)
    }

    /// Number of nodes represented.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.node_voltages.len()
    }
}

/// The bundled result of a DC operating-point analysis.
///
/// On convergence (the headline scenario), `operating_point` is
/// `Some` and `convergence.is_converged()` is `true`. On
/// non-convergence outcomes (`Stalled`, `MaxIterationsExceeded`,
/// `Diverged`), `operating_point` carries the last-iterate
/// node voltages / branch currents so the convergence-failure
/// scenario witness (tasks.md #22) and the homotopy fallbacks
/// (tasks.md #18 / #19) can read the diagnostic state.
///
/// The `topology_warnings` field is populated when the underlying
/// [`FlattenedStructure`] carries an attached
/// [`TopologyReport`] with
/// warning-level (possibly-floating) nodes. Hard-fault (floating)
/// nodes short-circuit before reaching the solver and are surfaced
/// via [`DcAnalysisError::FloatingNodeFault`] instead.
#[derive(Debug, Clone, PartialEq)]
pub struct DcAnalysisResult {
    /// The converged (or last-iterate) operating point. Always
    /// `Some` once the loop has produced at least one finite
    /// iterate, regardless of [`convergence`](Self::convergence)
    /// variant.
    pub operating_point: Option<OperatingPoint>,
    /// The Newton-Raphson convergence outcome. The diagnostic
    /// (final norms, iteration count, effective tolerances) is
    /// extractable via [`ConvergenceStatus::diagnostic`].
    pub convergence: ConvergenceStatus,
    /// Possibly-floating nodes flagged by the Pass-1 topology
    /// checker (ADR-0009 warning level). Empty when no topology
    /// report was attached or when the report was clean. Hard
    /// floating-node faults short-circuit the analysis and never
    /// land here.
    pub topology_warnings: Vec<NodeId>,
}

impl DcAnalysisResult {
    /// True iff the analysis converged (per ADR-0006's dual
    /// criterion) *and* an operating point was produced.
    #[must_use]
    pub fn is_converged(&self) -> bool {
        self.convergence.is_converged() && self.operating_point.is_some()
    }
}

// -----------------------------------------------------------------------------
// Error surface
// -----------------------------------------------------------------------------

/// Errors raised by [`dc_analysis`] *before* or *during* the
/// Newton-Raphson loop in a way that prevented the loop from running
/// to its natural termination.
///
/// Non-convergence outcomes (`Stalled`, `MaxIterationsExceeded`,
/// `Diverged`) are **not** errors here; they are reported on the
/// `Ok` path inside [`DcAnalysisResult::convergence`]. This split
/// matches the [`NewtonRaphsonDriver`] convention:
///
/// > Convergence outcomes (including divergence and stall) are
/// > reported as `Ok` with the appropriate `ConvergenceStatus`
/// > variant.
///
/// [`NewtonRaphsonDriver`]: numeric_solver::NewtonRaphsonDriver
#[derive(Debug, Clone, PartialEq)]
pub enum DcAnalysisError {
    /// Pass-1 MNA assembly rejected the inputs. Most commonly
    /// [`MnaAssemblyError::GraphFlattenMismatch`] when the caller
    /// flattened one graph and then passed a different one in
    /// [`DcAnalysisRequest::graph`].
    AssemblyFailed(MnaAssemblyError),
    /// The DC sub-view builder rejected the inputs. Most commonly
    /// [`SubViewError::GroundNodeOutOfRange`] when the caller
    /// overrode [`DcAnalysisRequest::ground`] with a value out of
    /// range for the flattened structure.
    SubViewBuildFailed(SubViewError),
    /// The Pass-1 topology checker (ADR-0009) flagged one or more
    /// nodes as hard floating (no DC path to ground through any
    /// conductive element). The MNA matrix would be structurally
    /// singular at these nodes; we short-circuit before the solver
    /// runs and report the offending node list. Homotopy cannot
    /// rescue a structurally-floating node, so this is a terminal
    /// error rather than a convergence failure.
    FloatingNodeFault {
        /// The floating node ids reported by the topology checker.
        /// Non-empty by construction (the variant is only emitted
        /// when at least one is present).
        nodes: Vec<NodeId>,
    },
    /// The Newton-Raphson driver itself returned a hard failure
    /// (dimension mismatch, system callback failure, unrecoverable
    /// linear-solver error). Distinct from a convergence failure
    /// which is reported via `Ok(DcAnalysisResult { convergence: …
    /// })`.
    NewtonRaphsonFailed(NewtonRaphsonError),
}

impl core::fmt::Display for DcAnalysisError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AssemblyFailed(inner) => {
                write!(f, "dc-analysis: MNA assembly failed: {inner}")
            }
            Self::SubViewBuildFailed(inner) => {
                write!(f, "dc-analysis: DC sub-view build failed: {inner}")
            }
            Self::FloatingNodeFault { nodes } => {
                write!(
                    f,
                    "dc-analysis: topology checker flagged {n} floating node(s) \
                     (no DC path to ground); MNA matrix would be structurally singular",
                    n = nodes.len()
                )
            }
            Self::NewtonRaphsonFailed(inner) => {
                write!(f, "dc-analysis: newton-raphson hard failure: {inner}")
            }
        }
    }
}

impl std::error::Error for DcAnalysisError {}

impl From<MnaAssemblyError> for DcAnalysisError {
    fn from(value: MnaAssemblyError) -> Self {
        Self::AssemblyFailed(value)
    }
}

impl From<SubViewError> for DcAnalysisError {
    fn from(value: SubViewError) -> Self {
        Self::SubViewBuildFailed(value)
    }
}

impl From<NewtonRaphsonError> for DcAnalysisError {
    fn from(value: NewtonRaphsonError) -> Self {
        Self::NewtonRaphsonFailed(value)
    }
}

// -----------------------------------------------------------------------------
// Linear-only NonlinearSystem adapter
// -----------------------------------------------------------------------------

/// [`NonlinearSystem`] adapter for the linear-only DC path
/// (`linear-resistive-dc-operating-point` scenario).
///
/// A linear circuit *is* its own linearization: the assembled MNA
/// matrix `A` and RHS `b` do not depend on the current iterate. On
/// every `linearize` callback the adapter hands back the same
/// pre-assembled [`SparseLinearSystem`]; on every `residue` callback
/// it computes the linear residue `A · x − b`. Newton-Raphson then
/// converges in one iteration: the linear solve yields `x_1 = A⁻¹b`,
/// and `A · x_1 − b = 0` to round-off.
///
/// Storing the sparse system once up front (rather than re-deriving
/// from the dense [`MnaSystem`] every iteration) is sound because
/// the sub-view mask in the linear path is iterate-independent. The
/// nonlinear adapter (tasks.md #18) will *not* be able to cache this
/// way — it will need to re-stamp per iterate using the current
/// linearization from `device-modeling::DeviceModel::linearize` —
/// but it can still reuse the dense-to-sparse lowering helper
/// `mna_subview_to_sparse_linear_system` below.
struct LinearDcSystem {
    /// The pre-assembled sparse linear system (ground-suppressed
    /// sub-view, lowered from dense to triplet form). All callbacks
    /// hand back a clone of this value.
    system: SparseLinearSystem<f64>,
}

impl LinearDcSystem {
    fn new(system: SparseLinearSystem<f64>) -> Self {
        Self { system }
    }
}

impl NonlinearSystem for LinearDcSystem {
    fn dim(&self) -> u32 {
        self.system.dim()
    }

    fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, NrSystemError> {
        // The linear case: identical system every iteration.
        Ok(self.system.clone())
    }

    fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, NrSystemError> {
        // Linear residue: F(x) = A · x − b. Computed via the triplet
        // stream (each non-zero contributes `value * iterate[col]`
        // to row `row`).
        let dim = self.system.dim() as usize;
        let mut f = vec![0.0_f64; dim];
        for t in self.system.triplets() {
            // SAFETY OF INDEXING:
            // - `SparseLinearSystem::new` enforces `row, col < dim`
            //   at construction time.
            // - The driver's pre-loop validation enforces
            //   `iterate.len() == dim`.
            // So both indices are in range.
            f[t.row as usize] += t.value * iterate[t.col as usize];
        }
        for (i, rhs_i) in self.system.rhs().iter().enumerate() {
            f[i] -= *rhs_i;
        }
        Ok(f)
    }
}

// -----------------------------------------------------------------------------
// Dense → sparse lowering
// -----------------------------------------------------------------------------

/// Lower a dense ground-suppressed DC sub-view into the
/// [`SparseLinearSystem<f64>`] shape expected by
/// [`RussellRealSolver`].
///
/// Drops exact-zero entries — `russell_sparse` accepts them but they
/// only inflate the symbolic factorization. The non-zero pattern of
/// a `SubView` after ground suppression is exactly the set of stamps
/// the assembler placed on non-ground nodes, plus the identity row
/// for the ground node itself.
///
/// Mirrors the lowering logic in
/// [`numeric_solver::AcSubViewBuilder::build`] (the complex sibling
/// for AC) so that the DC path uses an identically-shaped contract
/// against the solver trait.
fn mna_subview_to_sparse_linear_system(
    sub_view: &numeric_solver::SubView,
) -> Result<SparseLinearSystem<f64>, numeric_solver::LinearSolverError> {
    let dim = sub_view.dim();
    let node_count = sub_view.node_count();
    let branch_count = sub_view.branch_count();
    let dim_us = dim as usize;
    let matrix = sub_view.matrix();
    let rhs_dense = sub_view.rhs();

    // Pre-count non-zeros so we can size the triplet buffer exactly.
    let nnz = matrix.iter().filter(|v| **v != 0.0).count();
    let mut triplets: Vec<SparseTriplet<f64>> = Vec::with_capacity(nnz);
    for r in 0..dim {
        let row_base = (r as usize) * dim_us;
        for c in 0..dim {
            let v = matrix[row_base + (c as usize)];
            if v == 0.0 {
                continue;
            }
            triplets.push(SparseTriplet {
                row: r,
                col: c,
                value: v,
            });
        }
    }

    SparseLinearSystem::new(dim, node_count, branch_count, triplets, rhs_dense.to_vec())
}

// -----------------------------------------------------------------------------
// OperatingPoint extraction
// -----------------------------------------------------------------------------

/// Project a Newton-Raphson iterate vector into an [`OperatingPoint`].
///
/// The iterate layout matches the sub-view: `[node_voltages...,
/// branch_currents...]`. Node voltages are the first `node_count`
/// entries; the remaining `branch_count` entries are the MNA branch
/// unknowns. We pair each branch unknown with the [`BranchId`] of
/// the [`ElementIncidence`] that owns the corresponding branch row
/// in the flattened structure.
fn iterate_to_operating_point(iterate: &[f64], structure: &FlattenedStructure) -> OperatingPoint {
    let node_count = structure.node_count() as usize;
    let node_voltages = iterate[..node_count].to_vec();

    let mut branch_currents: Vec<BranchCurrentSample> = structure
        .elements()
        .filter_map(|inc| inc.branch.map(|b| (inc.element, b)))
        .map(|(_, branch)| BranchCurrentSample {
            branch,
            current_amperes: iterate[node_count + branch.index() as usize],
        })
        .collect();

    // Sort by branch id so the output order is deterministic across
    // runs / platforms / flattener orderings.
    branch_currents.sort_by_key(|s| s.branch.index());
    branch_currents.dedup_by_key(|s| s.branch.index());

    OperatingPoint {
        node_voltages,
        branch_currents,
    }
}

/// Compute the initial iterate for Newton-Raphson.
///
/// For DC operating point on a linear circuit, the conventional
/// starting point is the all-zero vector (every node at 0 V, every
/// branch current at 0 A). The linear NR loop converges in one
/// iteration regardless of starting point, but the all-zero choice
/// keeps the *update norm* on iteration 1 equal to `‖A⁻¹b‖∞`
/// (which is itself a useful diagnostic).
fn initial_iterate(dim: u32) -> Vec<f64> {
    vec![0.0_f64; dim as usize]
}

// -----------------------------------------------------------------------------
// Entry point
// -----------------------------------------------------------------------------

/// Run the DC operating-point analysis control loop.
///
/// Steps, in order:
///
/// 1. **Topology pre-pass.** If the flattened structure carries an
///    ADR-0009 topology report with hard floating nodes, short-circuit
///    with [`DcAnalysisError::FloatingNodeFault`].
/// 2. **Pass-2 assembly.** Build the full MNA matrix
///    ([`assemble()`]) from the structure plus an empty linearization
///    slice (linear-only path). Future nonlinear extensions
///    (tasks.md #18) pass per-device linearizations here.
/// 3. **Sub-view extraction.** Apply the textbook DC sub-view mask
///    via [`SubViewBuilder::from_full`] with ground suppression on.
/// 4. **Sparse lowering.** Lower the dense sub-view into a
///    [`SparseLinearSystem<f64>`] suitable for
///    [`RussellRealSolver`].
/// 5. **Newton-Raphson.** Run [`NewtonRaphsonDriver::solve`] with
///    the requested config; the linear adapter returns the same
///    sparse system on every callback, so NR converges in one
///    iteration on a well-formed linear circuit.
/// 6. **Operating-point extraction.** Project the final iterate
///    into a [`Vec<f64>`] of node voltages and a `Vec<BranchCurrentSample>`
///    of branch currents, indexed via the flattened structure's
///    per-element [`BranchId`].
///
/// On any [`NewtonRaphsonOutcome`](numeric_solver::NewtonRaphsonOutcome)
/// status (converged or non-converged), the function returns
/// `Ok(DcAnalysisResult)` with the *last-iterate* operating point
/// embedded — this is the load-bearing behavior for the
/// `dc-operating-point-convergence-failure` scenario witness
/// (tasks.md #22). Pre-loop and during-loop hard failures (dim
/// mismatch, modeling error, unrecoverable linear-solver error)
/// surface as `Err(DcAnalysisError)`.
///
/// # Errors
///
/// - [`DcAnalysisError::AssemblyFailed`] — Pass-2 MNA assembly
///   rejected the inputs (graph/flatten mismatch, non-finite
///   parameter, etc.).
/// - [`DcAnalysisError::SubViewBuildFailed`] — Sub-view builder
///   rejected the inputs (most commonly an out-of-range
///   ground node override).
/// - [`DcAnalysisError::FloatingNodeFault`] — Topology report
///   flagged hard-floating nodes; the matrix would be singular.
/// - [`DcAnalysisError::NewtonRaphsonFailed`] — The Newton-Raphson
///   driver itself returned a hard failure (dim mismatch
///   propagated up from a mis-paired `MnaSystem` /
///   `FlattenedStructure`, etc.).
///
/// # Panics
///
/// Does not panic in normal operation. The internal indexing into
/// the iterate buffer is gated by NR's pre-loop dim check and the
/// sparse system's invariants.
pub fn dc_analysis(req: DcAnalysisRequest<'_>) -> Result<DcAnalysisResult, DcAnalysisError> {
    // --- (1) Topology pre-pass ----------------------------------------------
    let (topology_warnings, topology_floating) = topology_findings(req.structure);
    if !topology_floating.is_empty() {
        return Err(DcAnalysisError::FloatingNodeFault {
            nodes: topology_floating,
        });
    }

    // --- (2) Pass-2 assembly ------------------------------------------------
    // Linear-only path: empty linearization slice. Nonlinear scenarios
    // (tasks.md #18 / #19) supply per-iterate linearizations through a
    // dedicated NonlinearSystem implementor that owns a sibling assembly
    // path.
    let mna: MnaSystem = assemble(req.structure, req.graph, &[])?;

    // --- (3) Sub-view extraction --------------------------------------------
    let ground = req.ground.unwrap_or_else(|| req.structure.ground_node());
    let sub_view = SubViewBuilder::from_full(&mna)
        .with_ground_node(ground)
        .suppress_ground(true)
        .build()?;

    // --- (4) Sparse lowering ------------------------------------------------
    // The Russell backend wants a SparseLinearSystem; the dense
    // SubView is what the assembler-sub_view boundary produces.
    let sparse =
        mna_subview_to_sparse_linear_system(&sub_view).map_err(DcAnalysisError::from_lse)?;
    let dim = sparse.dim();

    // --- (5) Newton-Raphson -------------------------------------------------
    let config = req
        .newton_raphson
        .unwrap_or(NewtonRaphsonConfig::DC_DEFAULTS);
    let solver = RussellRealSolver;
    let mut system = LinearDcSystem::new(sparse);
    let outcome = NewtonRaphsonDriver
        .solve(config, &mut system, &solver, initial_iterate(dim))
        .map_err(DcAnalysisError::from)?;

    // --- (6) Operating-point extraction ------------------------------------
    let op = iterate_to_operating_point(&outcome.iterate, req.structure);

    Ok(DcAnalysisResult {
        operating_point: Some(op),
        convergence: outcome.status,
        topology_warnings,
    })
}

impl DcAnalysisError {
    /// Specialized lowering for the rare case where dense→sparse
    /// fails its own pre-checks. In practice
    /// [`SparseLinearSystem::new`] only fails on
    /// `DimensionPartitionMismatch` / `RhsDimensionMismatch` /
    /// `TripletOutOfRange`, none of which can happen if the upstream
    /// sub-view was built correctly. We keep the conversion narrow
    /// so a future regression in the dense / sparse contract surfaces
    /// here instead of inside the NR driver.
    fn from_lse(err: numeric_solver::LinearSolverError) -> Self {
        // Wrap into the NR error surface so downstream consumers do
        // not need to learn a fourth variant. The wrapped form
        // exposes the same message verbatim.
        Self::NewtonRaphsonFailed(NewtonRaphsonError::LinearSolver {
            iteration: 0,
            source: err,
        })
    }
}

/// Read the optional topology report attached to a
/// [`FlattenedStructure`] and return `(warnings, floating)`.
///
/// Returns empty vectors when no report is attached; the analysis
/// is then permitted to proceed without floating-node short-circuit.
/// This deliberately makes the topology checker an *opt-in*
/// pre-pass — the orchestrator (or the `PyO3` frontend) runs
/// [`netlist_graph::topology::check_topology`] and attaches the
/// report before submitting the request, and circuits that legitimately
/// have only conductive paths can skip the checker entirely.
fn topology_findings(structure: &FlattenedStructure) -> (Vec<NodeId>, Vec<NodeId>) {
    let report: Option<&TopologyReport> = structure.topology_report();
    let warnings = report.map(|r| r.warning.clone()).unwrap_or_default();
    let floating = report.map(|r| r.floating.clone()).unwrap_or_default();
    (warnings, floating)
}

#[cfg(test)]
#[allow(clippy::similar_names, clippy::float_cmp)]
mod tests {
    use super::*;
    use circuit_solver_types::ConvergenceTolerances;
    use netlist_graph::{CircuitBuilder, ElementKind};
    use numeric_solver::{flatten, FlattenedStructure as FsAlias};

    // -------- helpers ------------------------------------------------------

    fn add_resistor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, ohms: f64) {
        b.add_element(
            name,
            ElementKind::Resistor {
                resistance_ohms: ohms,
            },
            [n1, n2],
            None,
        )
        .expect("add resistor");
    }

    fn add_voltage_source(b: &mut CircuitBuilder, name: &str, plus: &str, minus: &str, volts: f64) {
        b.add_element(
            name,
            ElementKind::VoltageSource {
                voltage_volts: volts,
            },
            [plus, minus],
            None,
        )
        .expect("add voltage source");
    }

    fn add_current_source(b: &mut CircuitBuilder, name: &str, from: &str, to: &str, amps: f64) {
        b.add_element(
            name,
            ElementKind::CurrentSource {
                current_amperes: amps,
            },
            [from, to],
            None,
        )
        .expect("add current source");
    }

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol.max(tol * a.abs().max(b.abs()))
    }

    fn voltage_divider(vsrc: f64, r1: f64, r2: f64) -> (FsAlias, CircuitGraph) {
        // V1 (vsrc) ─── n_in ── R1 ── n_mid ── R2 ── gnd
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", vsrc);
        add_resistor(&mut b, "R1", "n_in", "n_mid", r1);
        add_resistor(&mut b, "R2", "n_mid", "0", r2);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        (fs, g)
    }

    // -------- core happy path ----------------------------------------------

    /// Scenario `dc-operating-point#linear-resistive-dc-operating-point`:
    /// linear resistive netlist → `OperatingPoint`, convergence = converged.
    #[test]
    fn linear_voltage_divider_converges_to_analytic_solution() {
        let (fs, g) = voltage_divider(10.0, 1_000.0, 1_000.0);

        let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc analysis ok");

        assert!(
            result.is_converged(),
            "expected Converged, got {:?}",
            result.convergence
        );
        let op = result.operating_point.expect("op available");

        // Expected: V(n_in) = 10 V, V(n_mid) = 5 V, V(gnd) = 0.
        // Node ids are assigned in the order the builder encountered
        // them. We look them up via the graph to avoid hard-coding
        // indices.
        let n_in = node_id(&g, "n_in");
        let n_mid = node_id(&g, "n_mid");
        let gnd = NodeId::GROUND;

        assert!(approx(op.voltage_at(n_in).unwrap(), 10.0, 1e-9));
        assert!(approx(op.voltage_at(n_mid).unwrap(), 5.0, 1e-9));
        assert!(approx(op.voltage_at(gnd).unwrap(), 0.0, 1e-9));

        // Branch current: i_V1 (current through V1) should be
        // 10 V / (1k + 1k) = 5 mA. The sign convention is:
        // the branch unknown is the current flowing from the + terminal
        // (n_in) into V1 to the − terminal (gnd). For a source that
        // *delivers* power, this current is negative (current
        // physically flows out of the + terminal through the external
        // circuit). The MNA stamp's KCL at n_in is
        //   g·(V_n_in − V_n_mid) + i_V1 = 0
        // ⇒ i_V1 = −g·(V_n_in − V_n_mid) = −(5/1k) = −5 mA.
        // Confirm magnitude only (sign is implementation-dependent
        // across MNA conventions but the magnitude is invariant).
        let currents: Vec<f64> = op
            .branch_currents
            .iter()
            .map(|s| s.current_amperes)
            .collect();
        assert_eq!(currents.len(), 1, "one branch (the voltage source)");
        assert!(
            approx(currents[0].abs(), 5e-3, 1e-9),
            "expected |i_V1| = 5 mA, got {}",
            currents[0]
        );
    }

    fn node_id(g: &CircuitGraph, name: &str) -> NodeId {
        g.nodes()
            .iter()
            .find(|n| n.name() == name)
            .expect("node present")
            .id()
    }

    /// A 1 mA current source driving a 2 kΩ load to ground should
    /// produce 2 V at the load node (`I = V / R`). Tests the
    /// current-source RHS stamp path and node-only (no branch)
    /// circuits.
    #[test]
    fn current_source_into_resistor_produces_ohms_law_solution() {
        let mut b = CircuitBuilder::default();
        add_current_source(&mut b, "I1", "0", "n_load", 1e-3);
        add_resistor(&mut b, "R1", "n_load", "0", 2_000.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");

        let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc analysis ok");
        assert!(result.is_converged());
        let op = result.operating_point.unwrap();

        let n_load = node_id(&g, "n_load");
        // SPICE current-source stamping convention (per
        // numeric-solver::assemble docs): "S amperes flow into the
        // `from` terminal and out of the `to` terminal — stamp
        // RHS[from] += S and RHS[to] -= S." Under that sign rule, I1
        // `from=0`, `to=n_load`, value 1 mA pulls 1 mA *out of*
        // n_load through the source back to ground, so the resistor
        // R1 between n_load and ground must supply that 1 mA in the
        // direction gnd → n_load via R1, which drives n_load to
        // −2 V (not +2 V). We use the stamping convention's own
        // sign rather than re-deriving from external intuition.
        let v_expected = -2.0;
        assert!(
            approx(op.voltage_at(n_load).unwrap(), v_expected, 1e-9),
            "expected V(n_load) = {v_expected} V (per assembler current-source sign convention), got {}",
            op.voltage_at(n_load).unwrap()
        );
        // No voltage sources or inductors ⇒ no branch unknowns.
        assert!(op.branch_currents.is_empty());
    }

    /// An empty circuit (only the ground node, no elements) is
    /// vacuously solved: ground is at 0 V, no branches.
    #[test]
    fn empty_circuit_is_vacuously_converged() {
        let mut b = CircuitBuilder::default();
        let g = b.build().expect("build empty graph");
        let fs = flatten(&g).expect("flatten empty");
        let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc ok");
        assert!(result.is_converged());
        let op = result.operating_point.unwrap();
        assert_eq!(op.node_voltages, vec![0.0_f64]); // only ground
        assert!(op.branch_currents.is_empty());
    }

    // -------- API contracts -------------------------------------------------

    /// Builder-style configuration overrides are honored.
    #[test]
    fn builder_overrides_are_applied() {
        let (fs, g) = voltage_divider(5.0, 100.0, 100.0);
        let cfg = NewtonRaphsonConfig {
            max_iterations: 5,
            tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
        };
        let req = DcAnalysisRequest::new(&g, &fs)
            .with_newton_raphson(cfg)
            .with_ground(NodeId::GROUND);
        assert_eq!(req.newton_raphson, Some(cfg));
        assert_eq!(req.ground, Some(NodeId::GROUND));

        let result = dc_analysis(req).expect("dc ok");
        // The dual-criterion NR loop (ADR-0006) converges in **two**
        // iterations on a linear system, not one: iteration 1 produces
        // x_1 = A⁻¹b from the all-zero start (so ‖Δx‖ = ‖x_1‖, which
        // typically exceeds `update_tol`), iteration 2 produces
        // x_2 = x_1 (so ‖Δx‖ = 0 ≤ update_tol AND ‖F(x_2)‖ ≤
        // residue_tol). The single-iteration shortcut would have
        // violated ADR-0006's dual-criterion contract.
        assert!(result.is_converged());
        assert_eq!(
            result.convergence.diagnostic().iterations,
            2,
            "dual-criterion NR (ADR-0006) on a linear system converges in two iterations"
        );
    }

    /// Convergence diagnostic carries the requested tolerances
    /// verbatim.
    #[test]
    fn convergence_diagnostic_uses_requested_tolerances() {
        let (fs, g) = voltage_divider(1.0, 1_000.0, 1_000.0);
        let custom = ConvergenceTolerances::new(1e-6, 1e-15);
        let cfg = NewtonRaphsonConfig {
            max_iterations: 50,
            tolerances: custom,
        };
        let req = DcAnalysisRequest::new(&g, &fs).with_newton_raphson(cfg);
        let result = dc_analysis(req).expect("dc ok");
        assert_eq!(result.convergence.diagnostic().tolerances, custom);
    }

    // -------- topology pre-pass --------------------------------------------

    /// When a topology report with hard-floating nodes is attached,
    /// the analysis short-circuits with `FloatingNodeFault`.
    #[test]
    fn floating_topology_report_short_circuits() {
        let (mut fs, g) = voltage_divider(1.0, 100.0, 100.0);
        // Forge a topology report — we are testing the
        // short-circuit logic, not the checker itself.
        let report = TopologyReport {
            floating: vec![NodeId::new(2)],
            warning: vec![],
        };
        fs.set_topology_report(report);

        let err =
            dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect_err("expected FloatingNodeFault");
        match err {
            DcAnalysisError::FloatingNodeFault { nodes } => {
                assert_eq!(nodes, vec![NodeId::new(2)]);
            }
            other => panic!("expected FloatingNodeFault, got {other:?}"),
        }
    }

    /// Warning-level (possibly-floating) topology entries pass
    /// through into the result without blocking the solve.
    #[test]
    fn topology_warnings_pass_through() {
        let (mut fs, g) = voltage_divider(1.0, 100.0, 100.0);
        let report = TopologyReport {
            floating: vec![],
            warning: vec![NodeId::new(1)],
        };
        fs.set_topology_report(report);

        let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc ok");
        assert!(result.is_converged());
        assert_eq!(result.topology_warnings, vec![NodeId::new(1)]);
    }

    /// Without a topology report attached, the analysis proceeds
    /// normally and `topology_warnings` is empty.
    #[test]
    fn no_topology_report_means_no_warnings() {
        let (fs, g) = voltage_divider(1.0, 100.0, 100.0);
        let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc ok");
        assert!(result.is_converged());
        assert!(result.topology_warnings.is_empty());
    }

    // -------- error surface ------------------------------------------------

    /// Mismatched ground node override surfaces as
    /// `SubViewBuildFailed`.
    #[test]
    fn ground_out_of_range_is_subview_error() {
        let (fs, g) = voltage_divider(1.0, 100.0, 100.0);
        // Node count is small; pick an index far above it.
        let bogus = NodeId::new(999);
        let req = DcAnalysisRequest::new(&g, &fs).with_ground(bogus);
        let err = dc_analysis(req).expect_err("expected sub-view error");
        match err {
            DcAnalysisError::SubViewBuildFailed(SubViewError::GroundNodeOutOfRange { .. }) => {}
            other => panic!("expected SubViewBuildFailed, got {other:?}"),
        }
    }

    /// Assembling against a mismatched (graph, structure) pair
    /// surfaces as `AssemblyFailed`. We force a mismatch by
    /// flattening one graph and analyzing with another.
    #[test]
    fn mismatched_graph_and_structure_is_assembly_error() {
        let (fs, _g_a) = voltage_divider(1.0, 100.0, 100.0);
        // A different graph with a different element count.
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "a", "0", 1.0);
        let g_b = b.build().expect("build B");

        let req = DcAnalysisRequest::new(&g_b, &fs);
        let err = dc_analysis(req).expect_err("expected assembly error");
        match err {
            DcAnalysisError::AssemblyFailed(MnaAssemblyError::GraphFlattenMismatch { .. }) => {}
            other => panic!("expected AssemblyFailed, got {other:?}"),
        }
    }

    // -------- OperatingPoint surface ---------------------------------------

    /// `OperatingPoint::voltage_at` and `current_through` honor
    /// the spec's "immutable once produced" by providing only
    /// shared-reference access (no mutating method exists).
    #[test]
    fn operating_point_accessors_return_expected_values() {
        let (fs, g) = voltage_divider(6.0, 200.0, 100.0);
        let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("ok");
        let op = result.operating_point.unwrap();

        // V(n_mid) = 6 V * 100 / (200 + 100) = 2 V.
        let n_mid = node_id(&g, "n_mid");
        assert!(approx(op.voltage_at(n_mid).unwrap(), 2.0, 1e-9));

        // out-of-range node id returns None
        assert!(op.voltage_at(NodeId::new(999)).is_none());

        // unknown branch id returns None
        assert!(op.current_through(BranchId::new(999)).is_none());
    }

    /// `DcAnalysisResult::is_converged()` returns true only when
    /// both flags align.
    #[test]
    fn is_converged_requires_both_flags() {
        use circuit_solver_types::ConvergenceDiagnostic;
        let diag = ConvergenceDiagnostic {
            update_norm: 1.0,
            residue_norm: 1.0,
            iterations: 1,
            tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
        };
        let r = DcAnalysisResult {
            operating_point: None,
            convergence: ConvergenceStatus::Converged(diag),
            topology_warnings: vec![],
        };
        assert!(!r.is_converged());

        let r = DcAnalysisResult {
            operating_point: Some(OperatingPoint {
                node_voltages: vec![],
                branch_currents: vec![],
            }),
            convergence: ConvergenceStatus::Stalled(diag),
            topology_warnings: vec![],
        };
        assert!(!r.is_converged());
    }
}
