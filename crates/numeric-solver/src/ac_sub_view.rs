//! AC sub-view extraction: complex-valued MNA augmentation `(G + jωC)`
//! around an operating point.
//!
//! This module covers `tasks.md` item #24 of
//! `circuit-solver/2026-05-21-v1-spec`. It is the complex-valued analog
//! of [`crate::sub_view`] (item #15): it consumes a full real-valued
//! [`MnaSystem`] — the DC operating-point assembly produced by
//! [`crate::assemble::assemble`] (item #14) — and produces a sparse
//! [`SparseLinearSystem<Complex<f64>>`] at a single angular frequency
//! `ω`, ready to hand to the [`FaerComplexSolver`] dispatcher
//! ([`tasks.md` item #23]).
//!
//! The downstream AC analysis control loop (`tasks.md` item #25) drives
//! this module once per frequency point of an AC sweep.
//!
//! # Design references
//!
//! - **Scenario `ac-small-signal#ac-analysis-with-pre-computed-operating-point`.**
//!   The spec requires the simulator to *linearize the circuit at the
//!   `OperatingPoint` and solve a complex-valued MNA system at each
//!   frequency in the Sweep*. The operating-point linearization (`G`
//!   plus device Jacobian stamps) is already present in the
//!   [`MnaSystem`] passed in; this module adds the frequency-dependent
//!   `jωC` and `jωL` contributions and lowers the result to the sparse
//!   triplet form the complex solver consumes.
//!
//! - **ADR-0002 — Sparse Direct LU Dispatch.** This module's output is
//!   the input contract of [`FaerComplexSolver`]: a
//!   [`SparseLinearSystem<Complex<f64>>`] with one set of triplets and
//!   one dense complex RHS per frequency point. No symbolic factor is
//!   cached across frequencies because the matrix changes with `ω`
//!   (see the ADR's "no shared symbolic analysis" note).
//!
//! - **ADR-0003 — Two-Pass Graph Flattening with Per-Analysis
//!   Sub-Views.** *Pass 2 builds the full MNA matrix once; per-analysis
//!   sub-views apply analysis-specific augmentation at solve time
//!   without re-flattening or re-assembling.* This module is the AC
//!   leg of that pattern, parallel to [`crate::sub_view::SubView`]'s DC
//!   leg.
//!
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** The
//!   [`AcSubView`], [`AcSubViewBuilder`], and [`AcSubViewError`]
//!   surfaces are unstable per ADR-0010 — the shape may change between
//!   v1.x.
//!
//! # What "AC sub-view extraction" means here
//!
//! For an AC small-signal analysis at angular frequency `ω = 2πf`, the
//! MNA system is the complex-valued analog of the DC operating-point
//! system:
//!
//! ```text
//! (G + jω·C_node) · V = I
//! ```
//!
//! where `V` and `I` are the complex node-voltage / branch-current
//! phasor vectors. Compared with the DC operating-point matrix:
//!
//! - **Conductance / linearized-device block (`G`).** Unchanged. The
//!   operating-point assembler ([`crate::assemble::assemble`]) has
//!   already stamped the DC conductance and the linearized device
//!   Jacobians; the AC matrix inherits them by promotion to
//!   `Complex<f64>` (imaginary part zero).
//!
//! - **Capacitor stamps.** At DC the assembler stamps nothing for a
//!   capacitor (the device is open). At AC the device contributes the
//!   admittance `y = jωC` between its two terminals, stamped with the
//!   standard two-terminal admittance template:
//!
//!   ```text
//!     A[i, i] += +jωC      A[i, j] += -jωC
//!     A[j, j] += +jωC      A[j, i] += -jωC
//!   ```
//!
//! - **Inductor branch row.** At DC the assembler stamps the branch
//!   row to enforce `v_i − v_j = 0` (a wire). At AC the branch
//!   equation is `v_i − v_j − jωL · i_br = 0`, which means subtracting
//!   `jωL` from the branch-row diagonal:
//!
//!   ```text
//!     A[br, br] += -jωL
//!   ```
//!
//!   The `±1` incidence terms `(br, i)`, `(br, j)`, `(i, br)`,
//!   `(j, br)` are already present from the DC stamp and carry
//!   unchanged into the complex matrix.
//!
//! - **Voltage / current sources.** Independent-source stamps are
//!   left intact: per the spec, the AC small-signal stimulus is
//!   carried by the same DC-value parameters in v1 (there is no
//!   separate AC-magnitude annotation on
//!   [`ElementKind::VoltageSource`] / [`ElementKind::CurrentSource`]
//!   yet). The matrix's `±1` voltage-source incidence rows and the
//!   RHS phasors carry over from the operating-point system by
//!   complex promotion.
//!
//! - **Semiconductor / linearized-device stamps.** Already promoted by
//!   inheritance from the operating-point matrix. AC small signal at
//!   one operating point uses the *same* linearization the DC solver
//!   converged on; this module deliberately does *not* re-linearize.
//!
//! - **Ground suppression.** Identical to the real-valued sub-view:
//!   replace the ground row with the standard basis row `e_g`, zero
//!   the ground column in every non-ground row, and set the ground
//!   RHS entry to `0`. This is what makes the system full-rank.
//!
//! # Output shape
//!
//! Unlike [`crate::sub_view::SubView`], which keeps a dense
//! `Vec<f64>`, this module emits a **sparse** [`SparseLinearSystem`]
//! directly. Rationale: the [`FaerComplexSolver`] consumes
//! [`SparseLinearSystem`] only (triplets + dense RHS), so any
//! intermediate dense complex form would be discarded immediately.
//! Going straight to triplet form also keeps the per-frequency
//! allocation footprint proportional to the number of nonzeros, which
//! matters because AC sweeps can have hundreds of frequency points
//! per decade.
//!
//! # What this module does *not* do
//!
//! - **No solve.** This module produces the input the linear solver
//!   consumes ([`tasks.md` item #23]). It does not call into `faer`.
//! - **No sweep loop.** The frequency-sweep driver lives in
//!   `tasks.md` item #25.
//! - **No re-linearization.** Device linearizations come from the
//!   operating-point system (already stamped by the assembler at the
//!   converged DC solution); this module does not invoke
//!   `device_modeling::DeviceModel::linearize`.
//! - **No homotopy.** The Gmin / source-stepping masks ([`tasks.md`]
//!   items #18 / #19) belong to the DC convergence loop, not the AC
//!   linear solve.
//!
//! [`FaerComplexSolver`]: crate::linear_solver::FaerComplexSolver
//! [`SparseLinearSystem<Complex<f64>>`]: crate::linear_solver::SparseLinearSystem
//! [`SparseLinearSystem`]: crate::linear_solver::SparseLinearSystem
//! [`tasks.md`]: https://github.com/pbonh/circuit_solver/blob/main/openspec/changes/circuit-solver-2026-05-21-v1-spec/tasks.md
//! [`tasks.md` item #23]: https://github.com/pbonh/circuit_solver/blob/main/openspec/changes/circuit-solver-2026-05-21-v1-spec/tasks.md
//! [`tasks.md` item #25]: https://github.com/pbonh/circuit_solver/blob/main/openspec/changes/circuit-solver-2026-05-21-v1-spec/tasks.md
//! [`MnaSystem`]: crate::assemble::MnaSystem
//! [`ElementKind::VoltageSource`]: netlist_graph::ElementKind::VoltageSource
//! [`ElementKind::CurrentSource`]: netlist_graph::ElementKind::CurrentSource

// `a`, `b`, `i`, `j`, `y` are the textbook single-letter names for
// MNA matrix / RHS vector / terminal pair / two-terminal admittance
// (per `wiki/concepts/branch-stamping.md`). Renaming for the
// lint's sake would obscure the parallel with the assembler.
#![allow(clippy::many_single_char_names)]
// Loop counters over `dim` (a `u32` carried by the underlying
// [`MnaSystem`]) and converted-back-to-`u32` triplet indices fit by
// construction: `r * dim + c < dim * dim <= u32::MAX * u32::MAX`,
// and the conversion is only ever from `idx / dim_us` and
// `idx % dim_us`, both bounded by `dim` which is itself `u32`.
#![allow(clippy::cast_possible_truncation)]

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::NodeId;
use netlist_graph::{CircuitGraph, ElementKind};
use num_complex::Complex;

use crate::assemble::MnaSystem;
use crate::linear_solver::{LinearSolverError, SparseLinearSystem, SparseTriplet, C64};

/// Per-frequency AC sub-view: a complex-valued, sparse
/// [`SparseLinearSystem`] derived from a real-valued operating-point
/// [`MnaSystem`] by augmenting with `jωC` and `jωL` contributions and
/// applying ground suppression.
///
/// An `AcSubView` is the value [`AcSubViewBuilder::build`] returns.
/// It owns the triplet list and RHS vector that the
/// [`crate::linear_solver::FaerComplexSolver`] consumes via
/// [`AcSubView::into_system`] (or [`AcSubView::system`] for borrowing
/// inspection in tests).
///
/// # Layout
///
/// The dimension matches the underlying [`MnaSystem`]: rows/columns
/// `0..node_count` are node-KCL equations, rows/columns
/// `node_count..node_count + branch_count` are MNA branch equations.
/// Ground suppression is a *mask* (identity row at the ground index),
/// not a slice — node indexing is therefore stable across the entire
/// solver pipeline. Identical convention to the real-valued
/// [`crate::sub_view::SubView`].
#[derive(Debug, Clone)]
pub struct AcSubView {
    system: SparseLinearSystem<C64>,
}

impl AcSubView {
    /// Borrow the underlying [`SparseLinearSystem`] without consuming
    /// the view.
    #[must_use]
    pub fn system(&self) -> &SparseLinearSystem<C64> {
        &self.system
    }

    /// Consume the view and return ownership of the underlying
    /// [`SparseLinearSystem`]. This is the normal hand-off into
    /// [`crate::linear_solver::LinearSolver::solve`].
    #[must_use]
    pub fn into_system(self) -> SparseLinearSystem<C64> {
        self.system
    }

    /// Total dimension of the sub-view: `node_count + branch_count`.
    #[must_use]
    pub fn dim(&self) -> u32 {
        self.system.dim()
    }

    /// Node-equation count in the layout (including ground, which is
    /// suppressed to identity when ground suppression is enabled).
    #[must_use]
    pub fn node_count(&self) -> u32 {
        self.system.node_count()
    }

    /// MNA branch-equation count in the layout.
    #[must_use]
    pub fn branch_count(&self) -> u32 {
        self.system.branch_count()
    }
}

/// Errors raised by [`AcSubViewBuilder::build`].
///
/// Most variants mirror those of [`crate::sub_view::SubViewError`] so
/// callers driving DC and AC sub-views in the same orchestrator can
/// share error-handling shape. AC-specific failures (frequency,
/// reactive-parameter problems) get their own variants so the
/// orchestrator can distinguish "DC was bad" from "AC stamp was bad".
#[derive(Debug, Clone, PartialEq)]
pub enum AcSubViewError {
    /// The supplied angular frequency was non-finite (NaN or ±∞).
    /// Stamping `jω · reactive_param` would poison the matrix.
    NonFiniteOmega {
        /// The offending value in radians/second.
        omega_radians_per_second: f64,
    },
    /// A capacitor's `capacitance_farads` parameter was non-finite at
    /// the AC stamping pass. The operating-point assembler does check
    /// finiteness on capacitors (DC-open path); this is a defense-in-
    /// depth check at the AC layer in case a future caller skips the
    /// real-valued assembler.
    NonFiniteCapacitance {
        /// The offending value.
        capacitance_farads: f64,
    },
    /// An inductor's `inductance_henries` parameter was non-finite at
    /// the AC stamping pass. Same defense-in-depth rationale as
    /// [`Self::NonFiniteCapacitance`].
    NonFiniteInductance {
        /// The offending value.
        inductance_henries: f64,
    },
    /// The [`FlattenedStructure`] and [`CircuitGraph`] disagree on
    /// element count — the same root cause as
    /// [`crate::assemble::MnaAssemblyError::GraphFlattenMismatch`]. The
    /// AC builder revalidates because it walks both structures while
    /// looking up reactive-element values; surfacing the mismatch
    /// here pinpoints AC misuse rather than DC misuse.
    GraphFlattenMismatch {
        /// Element count reported by the flattened structure.
        flat_count: u32,
        /// Element count reported by the circuit graph.
        graph_count: usize,
    },
    /// A [`FlattenedStructure`] reactive element had a terminal count
    /// other than two. The flattener guarantees this invariant; the
    /// AC builder revalidates locally so the error pinpoints AC
    /// misuse.
    WrongTerminalCountForReactive {
        /// Short tag (`"C"` or `"L"`) of the offending element.
        kind: &'static str,
        /// The terminal count that was actually present.
        actual: usize,
    },
    /// A [`FlattenedStructure`] inductor element reached the AC
    /// builder without an MNA branch row — Pass 1 must allocate one.
    /// Indicates a Pass-1 regression. Same root cause as
    /// [`crate::assemble::MnaAssemblyError::MissingBranchForCurrentCarrying`].
    MissingBranchForInductor,
    /// A reactive-element node index exceeded the [`MnaSystem`]'s
    /// `node_count`. Either the caller paired a `FlattenedStructure`
    /// with the wrong `MnaSystem` or a Pass-1 invariant was violated.
    NodeIndexOutOfRange {
        /// The offending node id.
        node: NodeId,
        /// The system's node count.
        node_count: u32,
    },
    /// A reactive-element branch index exceeded the [`MnaSystem`]'s
    /// `branch_count`. Same root cause as
    /// [`Self::NodeIndexOutOfRange`].
    BranchIndexOutOfRange {
        /// The offending branch index.
        branch: u32,
        /// The system's branch count.
        branch_count: u32,
    },
    /// The ground node index recorded by [`AcSubViewBuilder::with_ground_node`]
    /// is out of range for the [`MnaSystem`]'s `node_count`. Indicates
    /// the caller paired a `FlattenedStructure` and an `MnaSystem`
    /// from different graphs.
    GroundNodeOutOfRange {
        /// The offending index.
        ground: NodeId,
        /// The system's node count.
        node_count: u32,
    },
    /// The operating-point [`MnaSystem`] carried a non-finite entry
    /// (NaN or ±∞). The real-valued assembler refuses to stamp such
    /// values; this check defends against synthesized test inputs.
    NonFiniteOperatingPointEntry {
        /// Row index of the offender.
        row: u32,
        /// Column index of the offender. `None` for RHS rows (in which
        /// case `row` indexes the RHS vector).
        col: Option<u32>,
    },
    /// The triplet-list assembly tripped a downstream invariant in
    /// [`SparseLinearSystem::new`]. Surfacing the inner error keeps
    /// the variant set stable for the analysis orchestrator.
    SystemConstructionFailed {
        /// The wrapped solver-input error.
        inner: LinearSolverError,
    },
}

impl core::fmt::Display for AcSubViewError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteOmega {
                omega_radians_per_second,
            } => write!(
                f,
                "ac-sub-view: angular frequency {omega_radians_per_second} rad/s is non-finite"
            ),
            Self::NonFiniteCapacitance { capacitance_farads } => write!(
                f,
                "ac-sub-view: capacitor value {capacitance_farads} F is non-finite"
            ),
            Self::NonFiniteInductance { inductance_henries } => write!(
                f,
                "ac-sub-view: inductor value {inductance_henries} H is non-finite"
            ),
            Self::GraphFlattenMismatch {
                flat_count,
                graph_count,
            } => write!(
                f,
                "ac-sub-view: FlattenedStructure has {flat_count} elements but CircuitGraph has {graph_count}"
            ),
            Self::WrongTerminalCountForReactive { kind, actual } => write!(
                f,
                "ac-sub-view: {kind} element recorded {actual} terminals; reactive devices require 2"
            ),
            Self::MissingBranchForInductor => write!(
                f,
                "ac-sub-view: inductor reached AC builder with no MNA branch row \
                 (Pass-1 invariant violation)"
            ),
            Self::NodeIndexOutOfRange { node, node_count } => write!(
                f,
                "ac-sub-view: {node} is out of range for node_count={node_count}"
            ),
            Self::BranchIndexOutOfRange {
                branch,
                branch_count,
            } => write!(
                f,
                "ac-sub-view: branch index {branch} is out of range for branch_count={branch_count}"
            ),
            Self::GroundNodeOutOfRange { ground, node_count } => write!(
                f,
                "ac-sub-view: ground {ground} is out of range for node_count={node_count}; \
                 caller paired a FlattenedStructure with the wrong MnaSystem"
            ),
            Self::NonFiniteOperatingPointEntry { row, col } => match col {
                Some(c) => write!(
                    f,
                    "ac-sub-view: operating-point matrix has a non-finite entry at ({row}, {c})"
                ),
                None => write!(
                    f,
                    "ac-sub-view: operating-point RHS has a non-finite entry at row {row}"
                ),
            },
            Self::SystemConstructionFailed { inner } => write!(
                f,
                "ac-sub-view: SparseLinearSystem::new rejected the assembled triplets: {inner}"
            ),
        }
    }
}

impl std::error::Error for AcSubViewError {}

/// Per-frequency AC sub-view builder.
///
/// Construction is cheap (no allocation): the builder borrows the
/// underlying [`MnaSystem`], [`FlattenedStructure`], and
/// [`CircuitGraph`]. [`AcSubViewBuilder::build`] is what actually
/// allocates: it copies the operating-point matrix into a dense
/// complex working buffer, applies the AC augmentation and ground
/// suppression, lowers the dense buffer into a sparse triplet list,
/// and hands the result to [`SparseLinearSystem::new`].
///
/// Re-using a single `MnaSystem` / `FlattenedStructure` / `CircuitGraph`
/// trio across many `AcSubViewBuilder::build` calls (one per
/// frequency in a sweep) is the intended pattern. Each call produces
/// a fresh [`AcSubView`]; the underlying borrowed inputs are
/// untouched.
///
/// # Default configuration
///
/// - Ground node: [`NodeId::GROUND`] (matches the Pass-1 convention).
/// - Ground suppression: **enabled** (the v1 AC scenarios all expect
///   a full-rank system).
/// - No frequency is set until [`AcSubViewBuilder::at_frequency`] (or
///   [`AcSubViewBuilder::at_omega`]) is called; build at the default
///   `ω = 0.0` returns the real operating-point matrix promoted to
///   complex, which is the DC limit and useful for tests but unlikely
///   to be what an analysis orchestrator wants in production.
#[derive(Debug, Clone)]
pub struct AcSubViewBuilder<'a> {
    system: &'a MnaSystem,
    structure: &'a FlattenedStructure,
    graph: &'a CircuitGraph,
    ground: NodeId,
    suppress_ground: bool,
    omega_radians_per_second: f64,
}

impl<'a> AcSubViewBuilder<'a> {
    /// Start a new builder from the operating-point [`MnaSystem`], the
    /// [`FlattenedStructure`] used to assemble it, and the source
    /// [`CircuitGraph`]. Defaults: ground at [`NodeId::GROUND`], ground
    /// suppression on, `ω = 0` (DC limit — call
    /// [`AcSubViewBuilder::at_frequency`] before [`AcSubViewBuilder::build`]
    /// for any non-trivial AC operating point).
    #[must_use]
    pub fn from_operating_point(
        system: &'a MnaSystem,
        structure: &'a FlattenedStructure,
        graph: &'a CircuitGraph,
    ) -> Self {
        Self {
            system,
            structure,
            graph,
            ground: NodeId::GROUND,
            suppress_ground: true,
            omega_radians_per_second: 0.0,
        }
    }

    /// Override the ground node. Defaults to [`NodeId::GROUND`].
    /// Mirrors [`crate::sub_view::SubViewBuilder::with_ground_node`]
    /// so DC and AC sub-views stay symmetric.
    #[must_use]
    pub fn with_ground_node(mut self, ground: NodeId) -> Self {
        self.ground = ground;
        self
    }

    /// Enable or disable ground suppression. The default is `true`
    /// (the v1 AC scenarios all assume ground-suppressed solves). The
    /// `false` path exists for debugging and parity with
    /// [`crate::sub_view::SubViewBuilder::suppress_ground`].
    #[must_use]
    pub fn suppress_ground(mut self, suppress: bool) -> Self {
        self.suppress_ground = suppress;
        self
    }

    /// Set the AC frequency in hertz. Internally converts to the
    /// angular frequency `ω = 2πf` used by the reactive stamps.
    ///
    /// Equivalent to
    /// `self.at_omega(2.0 * std::f64::consts::PI * frequency_hz)`.
    #[must_use]
    pub fn at_frequency(self, frequency_hz: f64) -> Self {
        let omega = 2.0 * core::f64::consts::PI * frequency_hz;
        self.at_omega(omega)
    }

    /// Set the angular frequency `ω` in radians/second directly.
    /// Useful when the caller already has `ω` from a logarithmic sweep
    /// generator.
    #[must_use]
    pub fn at_omega(mut self, omega_radians_per_second: f64) -> Self {
        self.omega_radians_per_second = omega_radians_per_second;
        self
    }

    /// Apply the configured augmentation and ground suppression to a
    /// fresh complex copy of the underlying operating-point system,
    /// and return the resulting [`AcSubView`].
    ///
    /// Order of operations:
    ///
    /// 1. Promote the operating-point matrix and RHS to
    ///    `Complex<f64>` (imaginary part zero).
    /// 2. For each [`ElementKind::Capacitor`] in the graph, stamp
    ///    `+jωC` on the diagonal pair and `-jωC` on the off-diagonal
    ///    pair (two-terminal admittance template).
    /// 3. For each [`ElementKind::Inductor`] in the graph, subtract
    ///    `jωL` from the branch-row diagonal (the `±1` incidence is
    ///    already in the operating-point matrix).
    /// 4. Apply ground suppression (matrix + RHS).
    /// 5. Lower the dense complex matrix to triplet form (dropping
    ///    exact zeros) and hand the result to
    ///    [`SparseLinearSystem::new`].
    ///
    /// # Errors
    ///
    /// - [`AcSubViewError::NonFiniteOmega`] — non-finite `ω`.
    /// - [`AcSubViewError::NonFiniteCapacitance`] /
    ///   [`AcSubViewError::NonFiniteInductance`] — bad reactive
    ///   parameter encountered during the AC walk.
    /// - [`AcSubViewError::GraphFlattenMismatch`] — `structure` and
    ///   `graph` disagree on element count.
    /// - [`AcSubViewError::WrongTerminalCountForReactive`] /
    ///   [`AcSubViewError::MissingBranchForInductor`] /
    ///   [`AcSubViewError::NodeIndexOutOfRange`] /
    ///   [`AcSubViewError::BranchIndexOutOfRange`] — Pass-1
    ///   invariant violations.
    /// - [`AcSubViewError::GroundNodeOutOfRange`] — ground index
    ///   exceeds `node_count`.
    /// - [`AcSubViewError::NonFiniteOperatingPointEntry`] — the
    ///   operating-point matrix or RHS carried a NaN or ±∞.
    /// - [`AcSubViewError::SystemConstructionFailed`] — wraps the
    ///   underlying [`LinearSolverError`] if
    ///   [`SparseLinearSystem::new`] rejects the triplet list.
    #[allow(clippy::too_many_lines)]
    pub fn build(self) -> Result<AcSubView, AcSubViewError> {
        // --- Up-front validation -------------------------------------------
        if !self.omega_radians_per_second.is_finite() {
            return Err(AcSubViewError::NonFiniteOmega {
                omega_radians_per_second: self.omega_radians_per_second,
            });
        }
        if self.structure.element_count() as usize != self.graph.elements().len() {
            return Err(AcSubViewError::GraphFlattenMismatch {
                flat_count: self.structure.element_count(),
                graph_count: self.graph.elements().len(),
            });
        }
        if self.ground.index() >= self.system.node_count() {
            return Err(AcSubViewError::GroundNodeOutOfRange {
                ground: self.ground,
                node_count: self.system.node_count(),
            });
        }

        let dim = self.system.dim();
        let dim_us = dim as usize;
        let node_count = self.system.node_count();
        let branch_count = self.system.branch_count();

        // --- 1. Promote operating-point matrix to complex ------------------
        // Dense row-major buffer. We do this dense-first to make
        // accumulation of reactive stamps and ground-suppression
        // overwrites simple; the final triplet lowering walks the
        // dense buffer once and emits nonzero entries only.
        let real_matrix = self.system.matrix();
        let real_rhs = self.system.rhs();
        let mut a: Vec<C64> = Vec::with_capacity(dim_us * dim_us);
        for (idx, &value) in real_matrix.iter().enumerate() {
            if !value.is_finite() {
                let row = (idx / dim_us) as u32;
                let col = (idx % dim_us) as u32;
                return Err(AcSubViewError::NonFiniteOperatingPointEntry {
                    row,
                    col: Some(col),
                });
            }
            a.push(Complex::new(value, 0.0));
        }
        let mut b: Vec<C64> = Vec::with_capacity(dim_us);
        for (row, &value) in real_rhs.iter().enumerate() {
            if !value.is_finite() {
                return Err(AcSubViewError::NonFiniteOperatingPointEntry {
                    row: row as u32,
                    col: None,
                });
            }
            b.push(Complex::new(value, 0.0));
        }

        // --- 2. and 3. AC augmentation walk --------------------------------
        let omega = self.omega_radians_per_second;
        for incidence in self.structure.elements() {
            let element = self.graph.element(incidence.element).ok_or(
                AcSubViewError::GraphFlattenMismatch {
                    flat_count: self.structure.element_count(),
                    graph_count: self.graph.elements().len(),
                },
            )?;
            match element.kind() {
                ElementKind::Capacitor { capacitance_farads } => {
                    let cap = *capacitance_farads;
                    if !cap.is_finite() {
                        return Err(AcSubViewError::NonFiniteCapacitance {
                            capacitance_farads: cap,
                        });
                    }
                    if incidence.nodes.len() != 2 {
                        return Err(AcSubViewError::WrongTerminalCountForReactive {
                            kind: "C",
                            actual: incidence.nodes.len(),
                        });
                    }
                    let i = incidence.nodes[0];
                    let j = incidence.nodes[1];
                    if i.index() >= node_count {
                        return Err(AcSubViewError::NodeIndexOutOfRange {
                            node: i,
                            node_count,
                        });
                    }
                    if j.index() >= node_count {
                        return Err(AcSubViewError::NodeIndexOutOfRange {
                            node: j,
                            node_count,
                        });
                    }
                    // Admittance y = jωC, stamped two-terminal:
                    //   +y at (i,i) and (j,j);  -y at (i,j) and (j,i).
                    let y = Complex::new(0.0, omega * cap);
                    let ii = (i.index() as usize) * dim_us + (i.index() as usize);
                    let jj = (j.index() as usize) * dim_us + (j.index() as usize);
                    let ij = (i.index() as usize) * dim_us + (j.index() as usize);
                    let ji = (j.index() as usize) * dim_us + (i.index() as usize);
                    a[ii] += y;
                    a[jj] += y;
                    a[ij] -= y;
                    a[ji] -= y;
                }
                ElementKind::Inductor { inductance_henries } => {
                    let ind = *inductance_henries;
                    if !ind.is_finite() {
                        return Err(AcSubViewError::NonFiniteInductance {
                            inductance_henries: ind,
                        });
                    }
                    if incidence.nodes.len() != 2 {
                        return Err(AcSubViewError::WrongTerminalCountForReactive {
                            kind: "L",
                            actual: incidence.nodes.len(),
                        });
                    }
                    let branch = incidence
                        .branch
                        .ok_or(AcSubViewError::MissingBranchForInductor)?;
                    if branch.index() >= branch_count {
                        return Err(AcSubViewError::BranchIndexOutOfRange {
                            branch: branch.index(),
                            branch_count,
                        });
                    }
                    // Branch row: v_i − v_j − jωL · i_br = 0.
                    // The ±1 incidence and the DC zero diagonal are
                    // already stamped; we add `-jωL` to A[br, br].
                    let br = (node_count + branch.index()) as usize;
                    let brbr = br * dim_us + br;
                    a[brbr] -= Complex::new(0.0, omega * ind);
                }
                // All other element kinds (R, V, I, Semiconductor,
                // SubcircuitInstance, future variants) are either
                // already stamped by the real-valued assembler — and
                // therefore inherited unchanged — or rejected at
                // assembly time. We deliberately do not re-check
                // them here so that this module's failure modes stay
                // AC-specific.
                _ => {}
            }
        }

        // --- 4. Ground suppression -----------------------------------------
        if self.suppress_ground {
            let g = self.ground.index() as usize;
            // Row g → e_g.
            let row_start = g * dim_us;
            for c in 0..dim_us {
                a[row_start + c] = C64::default();
            }
            a[row_start + g] = Complex::new(1.0, 0.0);
            // Column g → 0 for every non-ground row.
            for r in 0..dim_us {
                if r == g {
                    continue;
                }
                a[r * dim_us + g] = C64::default();
            }
            // RHS at ground row → 0.
            b[g] = C64::default();
        }

        // --- 5. Lower dense buffer to triplets ------------------------------
        // Drop exact-zero entries: faer's `try_new_from_triplets` will
        // happily accept them but they only inflate the symbolic
        // factorization. Note that AC stamps can produce a numerically
        // zero entry (e.g. ω = 0 makes every jωC vanish); we want the
        // sparse pattern to be the union of the operating-point
        // pattern and the reactive pattern, but with exact zeros
        // dropped per the SPICE-canonical "stamp the nonzeros"
        // convention.
        let nnz_estimate = real_matrix.iter().filter(|v| **v != 0.0).count();
        let mut triplets: Vec<SparseTriplet<C64>> = Vec::with_capacity(nnz_estimate);
        for r in 0..dim_us {
            for c in 0..dim_us {
                let v = a[r * dim_us + c];
                if v.re == 0.0 && v.im == 0.0 {
                    continue;
                }
                triplets.push(SparseTriplet {
                    row: r as u32,
                    col: c as u32,
                    value: v,
                });
            }
        }

        let system = SparseLinearSystem::new(dim, node_count, branch_count, triplets, b)
            .map_err(|inner| AcSubViewError::SystemConstructionFailed { inner })?;

        Ok(AcSubView { system })
    }
}

#[cfg(test)]
#[allow(
    // Tests use single-character matrix / loop names (`a`, `b`, `r`, `c`,
    // `i`, `j`, `g`, `y`) deliberately — they line up with the MNA stamp
    // notation used in the assembler tests and the wiki concept page
    // `branch-stamping.md`.
    clippy::similar_names,
    clippy::needless_range_loop,
)]
mod tests {
    use super::*;
    use crate::assemble::assemble;
    use crate::flatten::flatten;
    use crate::linear_solver::{FaerComplexSolver, LinearSolver};
    use netlist_graph::CircuitBuilder;

    // ---------------- helpers -----------------------------------------------

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

    fn add_capacitor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, farads: f64) {
        b.add_element(
            name,
            ElementKind::Capacitor {
                capacitance_farads: farads,
            },
            [n1, n2],
            None,
        )
        .expect("add capacitor");
    }

    fn add_inductor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, henries: f64) {
        b.add_element(
            name,
            ElementKind::Inductor {
                inductance_henries: henries,
            },
            [n1, n2],
            None,
        )
        .expect("add inductor");
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

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol.max(tol * a.abs().max(b.abs()))
    }

    fn approx_c(a: C64, b: C64, tol: f64) -> bool {
        approx(a.re, b.re, tol) && approx(a.im, b.im, tol)
    }

    /// Pretty-print a sparse system as a dense complex matrix for
    /// assertions. Returns `[r * dim + c]` flat row-major.
    fn dense_complex(sys: &SparseLinearSystem<C64>) -> Vec<C64> {
        let dim = sys.dim() as usize;
        let mut out = vec![C64::default(); dim * dim];
        for t in sys.triplets() {
            out[(t.row as usize) * dim + (t.col as usize)] += t.value;
        }
        out
    }

    /// Build the simplest first-order RC low-pass: V1 → R1 → C1 → gnd.
    /// Node layout: 0 = gnd, 1 = `n_in`, 2 = `n_out`.
    fn rc_lowpass(
        vsrc: f64,
        r_ohms: f64,
        c_farads: f64,
    ) -> (FlattenedStructure, CircuitGraph, MnaSystem) {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", vsrc);
        add_resistor(&mut b, "R1", "n_in", "n_out", r_ohms);
        add_capacitor(&mut b, "C1", "n_out", "0", c_farads);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        (fs, g, sys)
    }

    /// Build an RL series loop: V1 → R1 → L1 → gnd.
    /// Layout: 0 = gnd, 1 = `n_in`, 2 = `n_mid`; V1 owns branch 0, L1
    /// owns branch 1.
    fn rl_series(
        vsrc: f64,
        r_ohms: f64,
        l_henries: f64,
    ) -> (FlattenedStructure, CircuitGraph, MnaSystem) {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", vsrc);
        add_resistor(&mut b, "R1", "n_in", "n_mid", r_ohms);
        add_inductor(&mut b, "L1", "n_mid", "0", l_henries);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        (fs, g, sys)
    }

    // ---------------- capacitor stamping -----------------------------------

    #[test]
    fn capacitor_adds_jwc_two_terminal_pattern_at_nonzero_omega() {
        // R + C in parallel between n1 and gnd, no source. We disable
        // ground suppression so we can read all four entries of the
        // C stamp directly.
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1000.0);
        add_capacitor(&mut b, "C1", "n1", "0", 1e-6); // 1 µF
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");

        // ω = 2π·1 kHz. jωC = j·2π·1e3·1e-6 = j·6.2831853e-3 S.
        let f_hz = 1000.0;
        let omega = 2.0 * core::f64::consts::PI * f_hz;
        let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_frequency(f_hz)
            .suppress_ground(false)
            .build()
            .expect("ac sub-view ok");

        let dense = dense_complex(view.system());
        let dim = view.dim() as usize;
        let n1 = 1usize; // gnd = 0, n1 = 1
        let g_idx = 0usize;

        // Real conductance: 2 entries with +1/R at (n1, n1) and
        // (gnd, gnd), and -1/R at (n1, gnd) / (gnd, n1).
        let g_r = 1.0 / 1000.0;
        let y_c = Complex::new(0.0, omega * 1e-6);

        // Expected stamp at (n1, n1): G + jωC.
        let expected_n1n1 = Complex::new(g_r, 0.0) + y_c;
        assert!(approx_c(dense[n1 * dim + n1], expected_n1n1, 1e-12));
        // (gnd, gnd) likewise.
        assert!(approx_c(
            dense[g_idx * dim + g_idx],
            Complex::new(g_r, 0.0) + y_c,
            1e-12
        ));
        // Off-diagonal pair: -(G + jωC).
        let expected_off = -(Complex::new(g_r, 0.0) + y_c);
        assert!(approx_c(dense[n1 * dim + g_idx], expected_off, 1e-12));
        assert!(approx_c(dense[g_idx * dim + n1], expected_off, 1e-12));
    }

    #[test]
    fn capacitor_stamp_vanishes_at_omega_zero() {
        // At ω = 0 jωC = 0; matrix should match the DC matrix
        // (modulo complex promotion).
        let (fs, g, sys) = rc_lowpass(1.0, 1000.0, 1e-6);
        let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_omega(0.0)
            .suppress_ground(false)
            .build()
            .expect("ac sub-view ok");
        let dense = dense_complex(view.system());
        let dim = view.dim() as usize;
        for r in 0..dim {
            for c in 0..dim {
                let want = sys.matrix_entry(r as u32, c as u32).unwrap();
                let got = dense[r * dim + c];
                assert!(
                    approx_c(got, Complex::new(want, 0.0), 1e-12),
                    "({r},{c}): got {got:?}, want {want}+0i"
                );
            }
        }
    }

    #[test]
    fn capacitor_imag_part_scales_linearly_with_frequency() {
        // Same RC: at 1 kHz the imag part of (n_out, gnd) entry is
        // -jωC = -j·6.28e-3; at 10 kHz it is -j·6.28e-2. Pure
        // proportionality.
        let (fs, g, sys) = rc_lowpass(1.0, 1000.0, 1e-6);
        // Read the cap stamp at the (n_out, gnd) off-diagonal pair.
        // Layout: gnd=0, n_in=1, n_out=2; V1 branch row = 3.
        // The capacitor is between n_out (2) and gnd (0).
        let n_out = 2usize;
        let gnd = 0usize;
        let read_im_off = |omega: f64| -> f64 {
            let v = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
                .at_omega(omega)
                .suppress_ground(false)
                .build()
                .expect("ac sub-view ok");
            let dense = dense_complex(v.system());
            let dim = v.dim() as usize;
            dense[n_out * dim + gnd].im
        };
        let im_at_1khz = read_im_off(2.0 * core::f64::consts::PI * 1.0e3);
        let im_at_10khz = read_im_off(2.0 * core::f64::consts::PI * 1.0e4);
        // Off-diagonal carries -jωC, so the imag parts are negative
        // and at a ratio of 10×.
        assert!(
            im_at_1khz < 0.0,
            "expected negative imag off-diagonal, got {im_at_1khz}"
        );
        assert!(approx(im_at_10khz / im_at_1khz, 10.0, 1e-9));
    }

    // ---------------- inductor stamping ------------------------------------

    #[test]
    fn inductor_subtracts_jwl_from_branch_diagonal() {
        let (fs, g, sys) = rl_series(1.0, 100.0, 10e-3); // 100 Ω, 10 mH

        // L1 is the *second* current-carrying element, so it owns
        // branch row 1 (V1 owns branch row 0). Branch rows live at
        // node_count..node_count+branch_count, so dim = 3 nodes + 2
        // branches = 5; L1 branch is at index 3 + 1 = 4.
        let f_hz = 2_000.0;
        let omega = 2.0 * core::f64::consts::PI * f_hz;
        let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_frequency(f_hz)
            .suppress_ground(false)
            .build()
            .expect("ac sub-view ok");
        let dense = dense_complex(view.system());
        let dim = view.dim() as usize;
        assert_eq!(dim, 5);

        // The branch row diagonal for L1: DC stamps 0 there, so AC
        // value is exactly -jωL.
        let l_br = 4usize;
        let want = Complex::new(0.0, -omega * 10e-3);
        assert!(approx_c(dense[l_br * dim + l_br], want, 1e-12));

        // The branch incidence (±1 at (br, node) and (node, br)) is
        // preserved from DC and carries real-only.
        // L1 connects n_mid (idx 2) and gnd (idx 0).
        let n_mid = 2usize;
        let gnd = 0usize;
        assert!(approx_c(
            dense[l_br * dim + n_mid],
            Complex::new(1.0, 0.0),
            1e-12
        ));
        assert!(approx_c(
            dense[l_br * dim + gnd],
            Complex::new(-1.0, 0.0),
            1e-12
        ));
        assert!(approx_c(
            dense[n_mid * dim + l_br],
            Complex::new(1.0, 0.0),
            1e-12
        ));
        assert!(approx_c(
            dense[gnd * dim + l_br],
            Complex::new(-1.0, 0.0),
            1e-12
        ));
    }

    #[test]
    fn inductor_branch_diagonal_zero_at_omega_zero() {
        let (fs, g, sys) = rl_series(1.0, 100.0, 10e-3);
        let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_omega(0.0)
            .suppress_ground(false)
            .build()
            .expect("ac sub-view ok");
        let dense = dense_complex(view.system());
        let dim = view.dim() as usize;
        // L1 branch diag at (4, 4) is 0+0i: ω=0 reduces to DC short.
        assert!(approx_c(dense[4 * dim + 4], C64::default(), 1e-12));
    }

    // ---------------- ground suppression -----------------------------------

    #[test]
    fn ground_suppression_replaces_ground_row_with_identity_and_zeros_column() {
        let (fs, g, sys) = rc_lowpass(1.0, 1000.0, 1e-6);
        let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_frequency(1_000.0)
            .build()
            .expect("ac sub-view ok");
        let dense = dense_complex(view.system());
        let dim = view.dim() as usize;
        // Ground row: e_0.
        for c in 0..dim {
            let want = if c == 0 {
                Complex::new(1.0, 0.0)
            } else {
                C64::default()
            };
            assert!(
                approx_c(dense[c], want, 1e-12),
                "row 0 col {c}: got {:?}, want {want:?}",
                dense[c],
            );
        }
        // Ground column: 0 in every non-ground row.
        for r in 1..dim {
            assert!(approx_c(dense[r * dim], C64::default(), 1e-12));
        }
        // RHS at ground row: 0.
        assert!(approx_c(view.system().rhs()[0], C64::default(), 1e-12));
    }

    // ---------------- promotion of real entries ----------------------------

    #[test]
    fn real_entries_promote_with_zero_imag_part() {
        let (fs, g, sys) = rl_series(1.0, 100.0, 10e-3);
        // ω = 0 → no augmentation. Then suppress ground = false so
        // every original real entry shows up unchanged.
        let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_omega(0.0)
            .suppress_ground(false)
            .build()
            .expect("ac sub-view ok");
        let dense = dense_complex(view.system());
        let dim = view.dim() as usize;
        for r in 0..dim {
            for c in 0..dim {
                let v = dense[r * dim + c];
                assert!(approx(v.im, 0.0, 1e-12), "({r},{c}): imag {} != 0", v.im);
            }
        }
    }

    // ---------------- end-to-end faer roundtrip ----------------------------

    #[test]
    fn rc_lowpass_solution_matches_analytic_transfer_function() {
        // RC low-pass: H(jω) = 1 / (1 + jωRC).
        // V1 = 1 V at the input. At ω = 1 / (RC), |H| = 1/√2 (≈
        // -3.01 dB) and ∠H = -45°.
        let r = 1_000.0_f64; // 1 kΩ
        let c = 1.0e-6_f64; // 1 µF, so RC = 1 ms → cutoff = 1/(2π·RC) ≈ 159 Hz.
        let (fs, graph, sys) = rc_lowpass(1.0, r, c);
        // ω = 1 / (RC) — corner frequency.
        let omega = 1.0 / (r * c);

        let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &graph)
            .at_omega(omega)
            .build()
            .expect("ac sub-view ok");
        let solver = FaerComplexSolver;
        let sol = solver
            .solve(view.system())
            .expect("faer complex LU solves the AC sub-view");

        // Layout: gnd=0, n_in=1, n_out=2; V1 branch row = 3.
        let v_n_out = sol.unknowns()[2];
        // Analytic: V_out = 1.0 / (1 + jωRC) = 1.0 / (1 + j).
        let h_analytic = Complex::new(1.0, 0.0) / Complex::new(1.0, 1.0);
        assert!(
            approx_c(v_n_out, h_analytic, 1e-9),
            "n_out got {v_n_out:?}, analytic {h_analytic:?}"
        );

        // Magnitude in dB: 20·log10|H| = -3.0103 dB at ω = 1/(RC).
        let mag_db = 20.0_f64 * (v_n_out.norm()).log10();
        assert!(
            approx(mag_db, -3.0103, 1e-3),
            "magnitude got {mag_db} dB, want -3.0103 dB"
        );
        // Phase in degrees: ∠H = -45°.
        let phase_deg = v_n_out.arg().to_degrees();
        assert!(
            approx(phase_deg, -45.0, 1e-6),
            "phase got {phase_deg}°, want -45°"
        );
    }

    #[test]
    fn rl_series_solution_matches_analytic_impedance_divider() {
        // RL series from V1 → R1 → L1 → gnd. The current is
        // I = V1 / (R + jωL); the node voltage at n_mid is
        // V_n_mid = I · jωL = V1 · jωL / (R + jωL).
        let r = 100.0_f64;
        let l = 10e-3_f64;
        let (fs, graph, sys) = rl_series(1.0, r, l);
        let omega = 2.0 * core::f64::consts::PI * 2_000.0;

        let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &graph)
            .at_omega(omega)
            .build()
            .expect("ac sub-view ok");
        let solver = FaerComplexSolver;
        let sol = solver
            .solve(view.system())
            .expect("faer complex LU solves the AC sub-view");
        let v_n_mid = sol.unknowns()[2];
        let jwl = Complex::new(0.0, omega * l);
        let analytic = jwl / (Complex::new(r, 0.0) + jwl);
        assert!(
            approx_c(v_n_mid, analytic, 1e-9),
            "n_mid got {v_n_mid:?}, analytic {analytic:?}"
        );
    }

    #[test]
    fn solving_at_many_frequencies_reuses_one_operating_point_system() {
        // The intended caller pattern: assemble once, build many.
        let (fs, graph, sys) = rc_lowpass(1.0, 1_000.0, 1e-6);
        let solver = FaerComplexSolver;
        let freqs_hz = [1.0_f64, 10.0, 100.0, 1_000.0, 10_000.0, 100_000.0];
        let mut last_mag = f64::INFINITY;
        for &f_hz in &freqs_hz {
            let omega = 2.0 * core::f64::consts::PI * f_hz;
            let view = AcSubViewBuilder::from_operating_point(&sys, &fs, &graph)
                .at_omega(omega)
                .build()
                .expect("ac sub-view ok");
            let sol = solver.solve(view.system()).expect("solve ok");
            let v_n_out = sol.unknowns()[2];
            let mag = v_n_out.norm();
            // RC low-pass: magnitude monotonically decreases with f.
            assert!(
                mag < last_mag,
                "magnitude not monotonically decreasing at {f_hz} Hz: {mag} ≥ {last_mag}"
            );
            last_mag = mag;
        }
    }

    // ---------------- error paths ------------------------------------------

    #[test]
    fn rejects_non_finite_omega() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1e-6);
        let err = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_omega(f64::NAN)
            .build()
            .unwrap_err();
        assert!(matches!(err, AcSubViewError::NonFiniteOmega { .. }));
        let err = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_omega(f64::INFINITY)
            .build()
            .unwrap_err();
        assert!(matches!(err, AcSubViewError::NonFiniteOmega { .. }));
    }

    #[test]
    fn rejects_ground_node_out_of_range() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1e-6);
        let err = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .with_ground_node(NodeId::new(99))
            .at_frequency(1.0)
            .build()
            .unwrap_err();
        assert!(matches!(err, AcSubViewError::GroundNodeOutOfRange { .. }));
    }

    #[test]
    fn rejects_graph_flatten_mismatch_on_element_count() {
        // Flatten one graph with N elements and pair its structure
        // with an MnaSystem whose graph has N+1 elements; the
        // mismatch must be reported as a structured error rather
        // than panicking through index arithmetic.
        let (fs_small, _g_small, _sys_small) = rc_lowpass(1.0, 1_000.0, 1e-6);
        // Build a second graph with one *extra* element so element
        // counts differ.
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_mid", 100.0);
        add_inductor(&mut b, "L1", "n_mid", "0", 10e-3);
        add_resistor(&mut b, "R2", "n_mid", "0", 100.0); // extra
        let g_large = b.build().expect("build ok");
        let fs_large = flatten(&g_large).expect("flatten ok");
        let sys_large = assemble(&fs_large, &g_large, &[]).expect("assemble ok");
        let err = AcSubViewBuilder::from_operating_point(&sys_large, &fs_small, &g_large)
            .at_frequency(1.0)
            .build()
            .unwrap_err();
        assert!(matches!(err, AcSubViewError::GraphFlattenMismatch { .. }));
    }

    #[test]
    fn at_frequency_and_at_omega_agree() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1e-6);
        let f_hz = 1_234.5_f64;
        let omega = 2.0 * core::f64::consts::PI * f_hz;
        let v_a = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_frequency(f_hz)
            .build()
            .expect("ok a");
        let v_b = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_omega(omega)
            .build()
            .expect("ok b");
        // The triplet streams may differ in storage order but the
        // dense projection must match exactly (no floating-point
        // operations distinguish the two paths since 2π·f is
        // computed identically).
        let dense_a = dense_complex(v_a.system());
        let dense_b = dense_complex(v_b.system());
        assert_eq!(dense_a, dense_b);
    }

    // ---------------- non-mutation of inputs -------------------------------

    #[test]
    fn building_ac_subview_does_not_mutate_inputs() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1e-6);
        let original_matrix = sys.matrix().to_vec();
        let original_rhs = sys.rhs().to_vec();
        let _ = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_frequency(1_000.0)
            .build()
            .expect("ok");
        assert_eq!(sys.matrix(), original_matrix.as_slice());
        assert_eq!(sys.rhs(), original_rhs.as_slice());
    }

    // ---------------- empty / trivial graphs --------------------------------

    #[test]
    fn empty_ground_only_graph_yields_one_by_one_identity_complex_system() {
        let g = CircuitBuilder::default().build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        let v = AcSubViewBuilder::from_operating_point(&sys, &fs, &g)
            .at_frequency(1_000.0)
            .build()
            .expect("ok");
        assert_eq!(v.dim(), 1);
        let dense = dense_complex(v.system());
        assert_eq!(dense, vec![Complex::new(1.0, 0.0)]);
        assert_eq!(v.system().rhs(), &[C64::default()]);
    }
}
