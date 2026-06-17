//! Pass-2 MNA matrix assembly: stamp linearized models into the full
//! Modified Nodal Analysis matrix (including ground row/column) from a
//! [`FlattenedStructure`].
//!
//! This module covers `tasks.md` item #14 of
//! `circuit-solver/2026-05-21-v1-spec`. It consumes the Pass-1 output
//! produced by [`crate::flatten()`] (item #6) plus, for nonlinear
//! devices, the per-element [`LinearizedModel`] produced by
//! `device-modeling`'s `DeviceModel::linearize` (item #8) and
//! produces a full MNA system: the conductance/augmentation matrix
//! `A` and the right-hand-side vector `b` of dimension
//! `node_count + branch_count`.
//!
//! # Design references
//!
//! - **ADR-0003 — Two-Pass Graph Flattening with Per-Analysis
//!   Sub-Views.** *Pass 2 (matrix assembly) builds the full MNA matrix
//!   from the flattened structure at each solve point; the sub-view
//!   extractor then applies analysis-specific masks (ground
//!   suppression for DC, complex augmentation for AC, companion-model
//!   stamps for transient, ...).* This module honors that contract by
//!   producing the **full** matrix — ground row and column intact —
//!   leaving any masking to `tasks.md` item #15 (sub-view extractor).
//! - **ADR-0005 — Closed-Enum Device Model Dispatch.** Nonlinear
//!   contributions come into the assembler as values of the closed
//!   enum [`LinearizedModel`]; the assembler `match`es on the variant
//!   to learn the stamp dimension (Diode 2×2, BJT 3×3, MOSFET 4×4).
//!   Adding a new device family forces a compile-time update here,
//!   which is the property the closed enum exists to guarantee.
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** The
//!   [`MnaSystem`] type, the [`assemble`] entry point, and
//!   [`MnaAssemblyError`] are all part of the v1 unstable surface.
//! - **`design.md` C4 L2.** *Flattener → `"FlattenedStructure"` →
//!   Assembler → `"FullMNA"` (with ground) → `SubView`.* This module
//!   is the `Assembler` box; [`MnaSystem`] is the `"FullMNA"` edge
//!   label.
//! - **`wiki/concepts/branch-stamping.md`.** The stamp templates used
//!   here (`+y` on the diagonal, `-y` off-diagonal for two-terminal
//!   admittances; `±1` on B/C blocks for voltage-source augmentation;
//!   right-hand-side contribution for current sources) are exactly
//!   the textbook templates documented there.
//! - **Scenario `dc-operating-point#linear-resistive-dc-operating-point`.**
//!   This module is the load-bearing piece that, together with item
//!   #15 (sub-view extraction) and item #16 (russell sparse LU
//!   dispatch), makes that scenario's "linear resistive netlist →
//!   `OperatingPoint`" path executable.
//!
//! # Stamp templates (linear elements)
//!
//! Let `i` and `j` index circuit nodes, `b` index MNA branch rows.
//! The dense MNA matrix `A` has dimension
//! `(node_count + branch_count) × (node_count + branch_count)`;
//! columns/rows `0..node_count` are node rows/columns, and
//! columns/rows `node_count..node_count+branch_count` are branch
//! rows/columns. The RHS vector `b` shares this layout.
//!
//! - **Resistor R between nodes `i` and `j`:** with conductance
//!   `g = 1 / resistance_ohms`, stamp `+g` at `(i, i)` and `(j, j)`,
//!   and `-g` at `(i, j)` and `(j, i)`.
//! - **Inductor L between nodes `i` and `j` (DC short, owns branch `b`):**
//!   the branch row enforces `v_i - v_j = 0` at DC. Stamp `+1` at
//!   `(node_count+b, i)`, `-1` at `(node_count+b, j)`, `+1` at
//!   `(i, node_count+b)`, `-1` at `(j, node_count+b)`. RHS at branch
//!   row is `0`.
//! - **Voltage source V between nodes `i` (positive terminal 0) and
//!   `j` (negative terminal 1), value `E` volts, owns branch `b`:**
//!   stamp the same incidence as for an inductor and set
//!   `RHS[node_count+b] = E`.
//! - **Capacitor C between two nodes (DC open):** no stamp at DC. The
//!   transient companion stamp lands in `tasks.md` items #28+.
//! - **Current source I from terminal 0 (`from`) to terminal 1 (`to`)
//!   with value `S` amperes:** SPICE convention is that `S` amperes
//!   flow into the `from` terminal and out of the `to` terminal —
//!   stamp `RHS[from] += S` and `RHS[to] -= S`. (Equivalently the
//!   current leaves node `from` through the source, so KCL at `from`
//!   has `+S` on the right-hand side.)
//!
//! All linear stamps go into the **full** matrix; ground row and
//! column are written exactly like any other node row/column. Item
//! #15 (sub-view extractor) replaces the ground row/column with
//! identity later.
//!
//! # Stamp templates (nonlinear devices via `LinearizedModel`)
//!
//! `device-modeling`'s `DeviceModel::linearize` returns a
//! family-tagged [`LinearizedModel`] (`Diode`, `BJT`, `MOSFET`) carrying
//! a terminal-local Jacobian and a terminal-local companion-current
//! vector. The assembler maps terminal slots to global node indices
//! via the [`ElementIncidence::nodes`] array stored on the
//! flattened structure (per-device pin order: `[anode, cathode]` for
//! a diode, `[collector, base, emitter]` for a BJT,
//! `[drain, gate, source, bulk]` for a MOSFET — same SPICE convention
//! the `device-modeling` crate documents on its `OperatingPoint`
//! type).
//!
//! For each linearized device:
//!
//! 1. Add `jacobian[i][j]` to `A[node_of(i), node_of(j)]` for all
//!    terminal pairs `(i, j)`.
//! 2. *Subtract* `companion_current[k]` from `RHS[node_of(k)]` for
//!    all terminal slots `k`.
//!
//! The minus sign on the companion current is the load-bearing
//! convention here. Per the linearization contract
//! (`device_modeling::stamp` docstring) `companion_current[k] =
//! I_term(v*) − J[k,:]·v*` is the *residual* terminal current the
//! linearized model would draw at `v = 0`, i.e., the current the
//! device leaks OUT OF node `k` INTO the device terminal `k` once
//! the linear `J·V` part has been moved into `A`. Standard MNA puts
//! conductances on the LHS and external-source current injections
//! on the RHS; the linearized device current sits on the LHS, so
//! moving its constant part to the RHS introduces the minus sign.
//! v1 devices do not carry MNA branch rows.
//!
//! # Element-to-linearization indexing
//!
//! The assembler accepts an optional `linearizations: &[Option<LinearizedModel>]`
//! slice, indexed by `ElementId::index()`. Conventions:
//!
//! - If the slice is empty (or shorter than `element_count`), missing
//!   slots are treated as `None`. This is the convenient default for
//!   the *linear resistive DC operating point* scenario which has no
//!   semiconductor devices.
//! - For elements whose `kind` is [`ElementKind::Semiconductor`], the
//!   matching slot **must** carry a `Some(_)`; a `None` is reported
//!   as [`MnaAssemblyError::MissingLinearizationForDevice`]. This is a
//!   programmer error in the caller (the orchestrator should have
//!   asked `device-modeling` to linearize every device first).
//! - For non-semiconductor elements the slot is ignored (an over-
//!   eager caller can pass `Some(..)` and it will be a no-op — but
//!   the contract is `None` for everything that is not a
//!   semiconductor).
//!
//! # What this module does *not* do
//!
//! - **No sparse representation.** The matrix is a dense
//!   `Vec<f64>` in row-major order. Sparse-LU dispatch lands in
//!   `tasks.md` item #16 (russell). This module's `MnaSystem` is the
//!   intermediate representation between the assembler and the
//!   sub-view extractor (item #15).
//! - **No ground suppression.** Per ADR-0003 the full matrix is
//!   produced; item #15 removes the ground row/column.
//! - **No Newton-Raphson loop.** That is item #17.
//! - **No subcircuit expansion / topology check.** Those run in Pass
//!   1 and earlier.

use circuit_solver_types::{BranchId, ElementId, NodeId};
use device_modeling::stamp::{LinearizedModel, BJT_TERMINALS, DIODE_TERMINALS, MOSFET_TERMINALS};
use netlist_graph::{CircuitGraph, ElementKind};

use circuit_solver_types::flattened::{ElementIncidence, FlattenedStructure};

/// A full MNA system: matrix `a` (dense, row-major) and right-hand-side
/// vector `b`, both of dimension `node_count + branch_count`. The
/// ground row/column is present and stamped like any other node;
/// the sub-view extractor (`tasks.md` item #15) is responsible for
/// removing it before the linear solver is invoked.
///
/// # Layout
///
/// Row/column indices `0..node_count` correspond to node-current
/// equations (KCL at each node). Row/column indices
/// `node_count..node_count+branch_count` correspond to MNA branch
/// equations (one per current-carrying element: voltage sources,
/// inductors). The matrix is stored in row-major order: entry at
/// row `r`, column `c` lives at `a[r * dim + c]`.
///
/// # Why dense
///
/// `tasks.md` item #14 produces only the intermediate full-incidence
/// representation. The Pass-2 / sub-view boundary documented in
/// `design.md` keeps this internal stage analysis-agnostic, and the
/// real-valued sparse-LU dispatch lands one task later (#16,
/// russell). Storing dense `f64`s here keeps the assembler simple,
/// the test surface small, and the `MnaSystem` value a plain data
/// owner with no third-party type leaks.
#[derive(Debug, Clone, PartialEq)]
pub struct MnaSystem {
    /// Total node count (including ground at node 0).
    node_count: u32,
    /// Total MNA branch count.
    branch_count: u32,
    /// Square matrix of dimension `dim = node_count + branch_count`,
    /// stored in row-major order: `a[r * dim + c]`.
    a: Vec<f64>,
    /// Right-hand-side vector of dimension `dim`.
    b: Vec<f64>,
}

impl MnaSystem {
    /// Total dimension of the MNA system: `node_count + branch_count`.
    ///
    /// # Panics
    ///
    /// Panics only if `node_count + branch_count` overflows `u32`,
    /// which is structurally impossible because both fields are
    /// individually `u32` and the sum is computed via
    /// [`u32::checked_add`] at construction time.
    #[must_use]
    pub fn dim(&self) -> u32 {
        // checked at construction; recompute for the public accessor.
        self.node_count
            .checked_add(self.branch_count)
            .expect("dim was validated at construction")
    }

    /// Total node count (including ground).
    #[must_use]
    pub fn node_count(&self) -> u32 {
        self.node_count
    }

    /// Total MNA branch count.
    #[must_use]
    pub fn branch_count(&self) -> u32 {
        self.branch_count
    }

    /// Borrow the full matrix `a` as a flat row-major slice of length
    /// `dim * dim`.
    #[must_use]
    pub fn matrix(&self) -> &[f64] {
        &self.a
    }

    /// Borrow the right-hand-side vector `b` of length `dim`.
    #[must_use]
    pub fn rhs(&self) -> &[f64] {
        &self.b
    }

    /// Look up `a[r, c]`. Returns `None` if either index is out of
    /// range.
    #[must_use]
    pub fn matrix_entry(&self, r: u32, c: u32) -> Option<f64> {
        let dim = self.dim();
        if r >= dim || c >= dim {
            return None;
        }
        let idx = (r as usize) * (dim as usize) + (c as usize);
        self.a.get(idx).copied()
    }

    /// Look up `b[r]`. Returns `None` if the index is out of range.
    #[must_use]
    pub fn rhs_entry(&self, r: u32) -> Option<f64> {
        self.b.get(r as usize).copied()
    }

    /// Produce a new [`MnaSystem`] with the given pre-allocated
    /// matrix and RHS, preserving the node/branch counts of
    /// `self`.
    ///
    /// Intended for use by in-crate modules (e.g., `gmin_inserter`)
    /// that need to return a modified copy without re-running the
    /// full assembler.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `a.len() != (dim * dim)` or
    /// `b.len() != dim`, where `dim = node_count + branch_count`.
    /// In release builds the caller's invariant is assumed.
    #[must_use]
    pub(crate) fn clone_with_matrix(&self, a: Vec<f64>, b: Vec<f64>) -> Self {
        let dim = self.dim() as usize;
        debug_assert_eq!(
            a.len(),
            dim * dim,
            "matrix slice length must be dim*dim = {dim}*{dim}"
        );
        debug_assert_eq!(
            b.len(),
            dim,
            "rhs slice length must be dim = {dim}"
        );
        Self {
            node_count: self.node_count,
            branch_count: self.branch_count,
            a,
            b,
        }
    }
}

/// Errors raised by [`assemble`].
#[derive(Debug, Clone, PartialEq)]
pub enum MnaAssemblyError {
    /// The provided [`FlattenedStructure`] disagrees with the source
    /// [`CircuitGraph`] on element count. Indicates that the caller
    /// flattened one graph and then passed a different graph to
    /// [`assemble`].
    GraphFlattenMismatch {
        /// Element count reported by the flattened structure.
        flat_count: u32,
        /// Element count reported by the circuit graph.
        graph_count: usize,
    },
    /// A linear element's [`ElementKind`] carries a non-finite
    /// numerical parameter (NaN, ±∞). Stamping a non-finite value
    /// would poison the entire matrix.
    NonFiniteParameter {
        /// The element whose parameter was non-finite.
        element: ElementId,
        /// Short tag for the kind (e.g. `"R"`, `"V"`).
        kind: &'static str,
    },
    /// A resistor was specified with zero or negative resistance.
    /// Zero ohms is a wire (use `Inductor` for the v1 DC short); a
    /// negative resistance is unphysical at this layer of the solver.
    NonPositiveResistance {
        /// The offending resistor.
        element: ElementId,
        /// The bad value.
        resistance_ohms: f64,
    },
    /// A [`FlattenedStructure`] element terminal slot pointed at a
    /// node that exceeds the structure's `node_count`. Indicates an
    /// upstream invariant violation in Pass 1.
    NodeIndexOutOfRange {
        /// The element whose terminal was out of range.
        element: ElementId,
        /// The offending node id.
        node: NodeId,
        /// The structure's node count.
        node_count: u32,
    },
    /// A [`FlattenedStructure`] element claimed a branch index that
    /// exceeds the structure's `branch_count`. Same root cause as
    /// [`Self::NodeIndexOutOfRange`].
    BranchIndexOutOfRange {
        /// The element whose branch was out of range.
        element: ElementId,
        /// The offending branch id.
        branch: BranchId,
        /// The structure's branch count.
        branch_count: u32,
    },
    /// A two-terminal linear element had a terminal count other than
    /// two in its [`ElementIncidence`]. This is a Pass-1 invariant
    /// violation.
    WrongTerminalCountForKind {
        /// The element with the wrong terminal count.
        element: ElementId,
        /// The element's kind tag.
        kind: &'static str,
        /// The terminal count that was actually present.
        actual: usize,
        /// The terminal count the kind requires.
        expected: usize,
    },
    /// A [`ElementKind::VoltageSource`] or [`ElementKind::Inductor`]
    /// reached the assembler without an MNA branch row. Pass 1 is
    /// contractually required to allocate one; reaching this state
    /// indicates a Pass-1 regression.
    MissingBranchForCurrentCarrying {
        /// The current-carrying element with no branch row.
        element: ElementId,
        /// The element's kind tag.
        kind: &'static str,
    },
    /// A [`ElementKind::Semiconductor`] element reached the assembler
    /// without a corresponding `Some(LinearizedModel)` in the
    /// `linearizations` slice. The orchestrator must produce a
    /// linearization for every semiconductor before each MNA assembly
    /// pass.
    MissingLinearizationForDevice {
        /// The semiconductor element with no linearization.
        element: ElementId,
    },
    /// A [`LinearizedModel`] variant disagreed with the device's
    /// terminal count in the [`FlattenedStructure`]. For instance, a
    /// [`LinearizedModel::Diode`] (2 terminals) was supplied for an
    /// element whose flattened incidence carries 3 nodes. This is a
    /// caller bug: the orchestrator paired the wrong linearization
    /// with the wrong element.
    LinearizationFamilyMismatch {
        /// The offending element.
        element: ElementId,
        /// Family tag the linearization claims (e.g. `"Diode"`).
        linearization_family: &'static str,
        /// Terminal count carried by the linearization.
        linearization_terminals: usize,
        /// Terminal count the flattened structure recorded.
        flatten_terminals: usize,
    },
    /// An [`ElementKind::SubcircuitInstance`] reached the assembler.
    /// Subcircuit expansion happens in `CircuitBuilder::build()`, and
    /// [`crate::flatten()`] already rejects unexpanded instances; if one
    /// makes it here it is a regression.
    UnexpandedSubcircuit {
        /// The offending element.
        element: ElementId,
    },
    /// An [`ElementKind`] variant the assembler doesn't yet know how
    /// to stamp reached this code path. `ElementKind` is
    /// `#[non_exhaustive]`, so adding a variant in `netlist-graph`
    /// without a matching stamp here is the canonical cause. Failing
    /// loudly is preferable to silently dropping the element on the
    /// floor.
    UnknownElementKind {
        /// The offending element.
        element: ElementId,
        /// The element-kind tag (e.g. `"R"`, `"DEV"`).
        kind: &'static str,
    },
    /// The combined `node_count + branch_count` overflows `u32`.
    /// Structurally impossible (both fields are `u32`) but reported
    /// explicitly so the assembler never panics.
    SystemTooLarge {
        /// Node count at the point of overflow.
        node_count: u32,
        /// Branch count at the point of overflow.
        branch_count: u32,
    },
}

impl core::fmt::Display for MnaAssemblyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GraphFlattenMismatch {
                flat_count,
                graph_count,
            } => write!(
                f,
                "FlattenedStructure has {flat_count} elements but CircuitGraph has {graph_count}"
            ),
            Self::NonFiniteParameter { element, kind } => {
                write!(f, "{element} ({kind}) carries a non-finite parameter")
            }
            Self::NonPositiveResistance {
                element,
                resistance_ohms,
            } => write!(
                f,
                "{element} has non-positive resistance {resistance_ohms} Ω; \
                 zero ohms is a wire (model as Inductor at DC), and negative \
                 resistance is unphysical at the MNA stamp level"
            ),
            Self::NodeIndexOutOfRange {
                element,
                node,
                node_count,
            } => write!(
                f,
                "{element} references {node}, out of range for node_count={node_count}"
            ),
            Self::BranchIndexOutOfRange {
                element,
                branch,
                branch_count,
            } => write!(
                f,
                "{element} references {branch}, out of range for branch_count={branch_count}"
            ),
            Self::WrongTerminalCountForKind {
                element,
                kind,
                actual,
                expected,
            } => write!(
                f,
                "{element} ({kind}) recorded {actual} terminals; kind requires {expected}"
            ),
            Self::MissingBranchForCurrentCarrying { element, kind } => write!(
                f,
                "{element} ({kind}) is current-carrying but Pass 1 did not allocate an MNA branch row"
            ),
            Self::MissingLinearizationForDevice { element } => write!(
                f,
                "{element} is a semiconductor but no LinearizedModel was supplied"
            ),
            Self::LinearizationFamilyMismatch {
                element,
                linearization_family,
                linearization_terminals,
                flatten_terminals,
            } => write!(
                f,
                "{element}: linearization {linearization_family} expects \
                 {linearization_terminals} terminals but flattened structure \
                 carries {flatten_terminals}"
            ),
            Self::UnexpandedSubcircuit { element } => write!(
                f,
                "{element} is an unexpanded SubcircuitInstance — \
                 CircuitBuilder::build() must expand before Pass 1 (regression)"
            ),
            Self::UnknownElementKind { element, kind } => write!(
                f,
                "{element} ({kind}) is an ElementKind variant the assembler does not yet stamp; \
                 a stamp arm must be added in this module (netlist-graph::ElementKind is non_exhaustive)"
            ),
            Self::SystemTooLarge {
                node_count,
                branch_count,
            } => write!(
                f,
                "MNA system dimension overflowed u32 (nodes={node_count}, branches={branch_count})"
            ),
        }
    }
}

impl std::error::Error for MnaAssemblyError {}

/// Pass-2 MNA matrix assembly.
///
/// Walks the [`FlattenedStructure`] in element-id order, looking up
/// each element's value in `graph`, and stamps its contribution into
/// the full MNA matrix (including the ground row/column).
/// Semiconductor devices' contributions come from `linearizations`:
/// `linearizations[id.index()]` must be `Some(LinearizedModel)` for
/// every [`ElementKind::Semiconductor`] in the graph. For elements
/// that don't need a linearization, the slot is ignored (and may be
/// `None` or absent if the slice is shorter than `element_count`).
///
/// # Errors
///
/// Returns the relevant [`MnaAssemblyError`] variant if:
///
/// - the structure and graph disagree on element count
///   ([`Self::GraphFlattenMismatch`]),
/// - a linear element carries a non-finite or non-positive value
///   ([`Self::NonFiniteParameter`], [`Self::NonPositiveResistance`]),
/// - the flattened incidence is internally inconsistent
///   ([`Self::NodeIndexOutOfRange`], [`Self::BranchIndexOutOfRange`],
///   [`Self::WrongTerminalCountForKind`],
///   [`Self::MissingBranchForCurrentCarrying`]),
/// - a semiconductor lacks a linearization
///   ([`Self::MissingLinearizationForDevice`]) or has a wrong-family
///   one ([`Self::LinearizationFamilyMismatch`]),
/// - an unexpanded subcircuit slipped through
///   ([`Self::UnexpandedSubcircuit`]),
/// - the system dimension overflows `u32`
///   ([`Self::SystemTooLarge`]).
///
/// [`Self::GraphFlattenMismatch`]: MnaAssemblyError::GraphFlattenMismatch
/// [`Self::NonFiniteParameter`]: MnaAssemblyError::NonFiniteParameter
/// [`Self::NonPositiveResistance`]: MnaAssemblyError::NonPositiveResistance
/// [`Self::NodeIndexOutOfRange`]: MnaAssemblyError::NodeIndexOutOfRange
/// [`Self::BranchIndexOutOfRange`]: MnaAssemblyError::BranchIndexOutOfRange
/// [`Self::WrongTerminalCountForKind`]: MnaAssemblyError::WrongTerminalCountForKind
/// [`Self::MissingBranchForCurrentCarrying`]: MnaAssemblyError::MissingBranchForCurrentCarrying
/// [`Self::MissingLinearizationForDevice`]: MnaAssemblyError::MissingLinearizationForDevice
/// [`Self::LinearizationFamilyMismatch`]: MnaAssemblyError::LinearizationFamilyMismatch
/// [`Self::UnexpandedSubcircuit`]: MnaAssemblyError::UnexpandedSubcircuit
/// [`Self::SystemTooLarge`]: MnaAssemblyError::SystemTooLarge
pub fn assemble(
    flat: &FlattenedStructure,
    graph: &CircuitGraph,
    linearizations: &[Option<LinearizedModel>],
) -> Result<MnaSystem, MnaAssemblyError> {
    let node_count = flat.node_count();
    let branch_count = flat.branch_count();
    let dim = node_count
        .checked_add(branch_count)
        .ok_or(MnaAssemblyError::SystemTooLarge {
            node_count,
            branch_count,
        })?;

    if flat.element_count() as usize != graph.elements().len() {
        return Err(MnaAssemblyError::GraphFlattenMismatch {
            flat_count: flat.element_count(),
            graph_count: graph.elements().len(),
        });
    }

    let dim_usize = dim as usize;
    let mut a = vec![0.0_f64; dim_usize.saturating_mul(dim_usize)];
    let mut b = vec![0.0_f64; dim_usize];

    for incidence in flat.elements() {
        stamp_element(
            &mut a,
            &mut b,
            dim,
            node_count,
            branch_count,
            flat,
            graph,
            linearizations,
            incidence,
        )?;
    }

    Ok(MnaSystem {
        node_count,
        branch_count,
        a,
        b,
    })
}

#[allow(clippy::too_many_arguments)]
fn stamp_element(
    a: &mut [f64],
    b: &mut [f64],
    dim: u32,
    node_count: u32,
    branch_count: u32,
    flat: &FlattenedStructure,
    graph: &CircuitGraph,
    linearizations: &[Option<LinearizedModel>],
    incidence: &ElementIncidence,
) -> Result<(), MnaAssemblyError> {
    let element_id = incidence.element;
    let graph_element =
        graph
            .element(element_id)
            .ok_or(MnaAssemblyError::GraphFlattenMismatch {
                flat_count: flat.element_count(),
                graph_count: graph.elements().len(),
            })?;
    let kind = graph_element.kind();

    // Validate flat-side node indices up front for any element kind.
    for &node in &incidence.nodes {
        if node.index() >= node_count {
            return Err(MnaAssemblyError::NodeIndexOutOfRange {
                element: element_id,
                node,
                node_count,
            });
        }
    }
    if let Some(branch) = incidence.branch {
        if branch.index() >= branch_count {
            return Err(MnaAssemblyError::BranchIndexOutOfRange {
                element: element_id,
                branch,
                branch_count,
            });
        }
    }

    match kind {
        ElementKind::Resistor { resistance_ohms } => {
            stamp_resistor(a, dim, element_id, incidence, *resistance_ohms)?;
        }
        ElementKind::Capacitor { capacitance_farads } => {
            // DC open: no stamp. We still check finiteness so that
            // a NaN capacitance does not silently slip into a
            // later transient pass.
            if !capacitance_farads.is_finite() {
                return Err(MnaAssemblyError::NonFiniteParameter {
                    element: element_id,
                    kind: "C",
                });
            }
        }
        ElementKind::Inductor { inductance_henries } => {
            stamp_inductor_dc(
                a,
                dim,
                node_count,
                element_id,
                incidence,
                *inductance_henries,
            )?;
        }
        ElementKind::VoltageSource { voltage_volts } => {
            stamp_voltage_source(a, b, dim, node_count, element_id, incidence, *voltage_volts)?;
        }
        ElementKind::CurrentSource { current_amperes } => {
            stamp_current_source(b, element_id, incidence, *current_amperes)?;
        }
        ElementKind::Semiconductor => {
            let lin = linearization_for(linearizations, element_id).ok_or(
                MnaAssemblyError::MissingLinearizationForDevice {
                    element: element_id,
                },
            )?;
            stamp_linearization(a, b, dim, element_id, incidence, lin)?;
        }
        ElementKind::SubcircuitInstance { .. } => {
            return Err(MnaAssemblyError::UnexpandedSubcircuit {
                element: element_id,
            });
        }
        // `ElementKind` is `#[non_exhaustive]` (per the
        // `netlist-graph` crate's element module): future variants
        // must add a stamp here. Failing loudly is preferable to
        // silently dropping the element on the floor.
        other => {
            return Err(MnaAssemblyError::UnknownElementKind {
                element: element_id,
                kind: other.tag(),
            });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Per-kind stamp helpers
// ---------------------------------------------------------------------

/// Index into the row-major dense matrix.
#[inline]
fn ar(a: &mut [f64], dim: u32, r: u32, c: u32) -> &mut f64 {
    let idx = (r as usize) * (dim as usize) + (c as usize);
    &mut a[idx]
}

fn linearization_for(
    linearizations: &[Option<LinearizedModel>],
    element: ElementId,
) -> Option<&LinearizedModel> {
    let i = element.index() as usize;
    linearizations.get(i).and_then(|opt| opt.as_ref())
}

fn require_two_terminals(
    incidence: &ElementIncidence,
    element: ElementId,
    kind_tag: &'static str,
) -> Result<(NodeId, NodeId), MnaAssemblyError> {
    if incidence.nodes.len() != 2 {
        return Err(MnaAssemblyError::WrongTerminalCountForKind {
            element,
            kind: kind_tag,
            actual: incidence.nodes.len(),
            expected: 2,
        });
    }
    Ok((incidence.nodes[0], incidence.nodes[1]))
}

fn stamp_resistor(
    a: &mut [f64],
    dim: u32,
    element: ElementId,
    incidence: &ElementIncidence,
    resistance_ohms: f64,
) -> Result<(), MnaAssemblyError> {
    if !resistance_ohms.is_finite() {
        return Err(MnaAssemblyError::NonFiniteParameter { element, kind: "R" });
    }
    if resistance_ohms <= 0.0 {
        return Err(MnaAssemblyError::NonPositiveResistance {
            element,
            resistance_ohms,
        });
    }
    let (i, j) = require_two_terminals(incidence, element, "R")?;
    let g = 1.0 / resistance_ohms;
    // +g on the two diagonal entries
    *ar(a, dim, i.index(), i.index()) += g;
    *ar(a, dim, j.index(), j.index()) += g;
    // -g on the two off-diagonal entries
    *ar(a, dim, i.index(), j.index()) -= g;
    *ar(a, dim, j.index(), i.index()) -= g;
    Ok(())
}

fn stamp_inductor_dc(
    a: &mut [f64],
    dim: u32,
    node_count: u32,
    element: ElementId,
    incidence: &ElementIncidence,
    inductance_henries: f64,
) -> Result<(), MnaAssemblyError> {
    if !inductance_henries.is_finite() {
        return Err(MnaAssemblyError::NonFiniteParameter { element, kind: "L" });
    }
    let (i, j) = require_two_terminals(incidence, element, "L")?;
    let branch = incidence
        .branch
        .ok_or(MnaAssemblyError::MissingBranchForCurrentCarrying { element, kind: "L" })?;
    // Branch row index in the full system.
    let br = node_count + branch.index();
    // Incidence rows/columns: +1 at (br, i), -1 at (br, j); +1 at (i, br), -1 at (j, br).
    *ar(a, dim, br, i.index()) += 1.0;
    *ar(a, dim, br, j.index()) -= 1.0;
    *ar(a, dim, i.index(), br) += 1.0;
    *ar(a, dim, j.index(), br) -= 1.0;
    // DC: branch enforces v_i - v_j = 0, so no RHS contribution.
    Ok(())
}

fn stamp_voltage_source(
    a: &mut [f64],
    b: &mut [f64],
    dim: u32,
    node_count: u32,
    element: ElementId,
    incidence: &ElementIncidence,
    voltage_volts: f64,
) -> Result<(), MnaAssemblyError> {
    if !voltage_volts.is_finite() {
        return Err(MnaAssemblyError::NonFiniteParameter { element, kind: "V" });
    }
    let (plus, minus) = require_two_terminals(incidence, element, "V")?;
    let branch = incidence
        .branch
        .ok_or(MnaAssemblyError::MissingBranchForCurrentCarrying { element, kind: "V" })?;
    let br = node_count + branch.index();
    // Same incidence stamp as the inductor: ±1 on the B/C blocks.
    *ar(a, dim, br, plus.index()) += 1.0;
    *ar(a, dim, br, minus.index()) -= 1.0;
    *ar(a, dim, plus.index(), br) += 1.0;
    *ar(a, dim, minus.index(), br) -= 1.0;
    // The branch row enforces v_plus - v_minus = E.
    b[br as usize] += voltage_volts;
    Ok(())
}

fn stamp_current_source(
    b: &mut [f64],
    element: ElementId,
    incidence: &ElementIncidence,
    current_amperes: f64,
) -> Result<(), MnaAssemblyError> {
    if !current_amperes.is_finite() {
        return Err(MnaAssemblyError::NonFiniteParameter { element, kind: "I" });
    }
    let (from, to) = require_two_terminals(incidence, element, "I")?;
    // SPICE convention: positive current flows from the `from` terminal
    // *into* the device and out of the `to` terminal. KCL at `from`
    // therefore gains a +S on the RHS (current leaves the node into
    // the source), and KCL at `to` gains a -S.
    b[from.index() as usize] += current_amperes;
    b[to.index() as usize] -= current_amperes;
    Ok(())
}

fn stamp_linearization(
    a: &mut [f64],
    b: &mut [f64],
    dim: u32,
    element: ElementId,
    incidence: &ElementIncidence,
    lin: &LinearizedModel,
) -> Result<(), MnaAssemblyError> {
    let flatten_terminals = incidence.nodes.len();
    match lin {
        LinearizedModel::Diode(d) => {
            check_terminal_count(element, "Diode", DIODE_TERMINALS, flatten_terminals)?;
            stamp_dense_block(
                a,
                b,
                dim,
                &incidence.nodes,
                &d.jacobian,
                &d.companion_current,
            );
        }
        LinearizedModel::BJT(t) => {
            check_terminal_count(element, "BJT", BJT_TERMINALS, flatten_terminals)?;
            stamp_dense_block(
                a,
                b,
                dim,
                &incidence.nodes,
                &t.jacobian,
                &t.companion_current,
            );
        }
        LinearizedModel::MOSFET(m) => {
            check_terminal_count(element, "MOSFET", MOSFET_TERMINALS, flatten_terminals)?;
            stamp_dense_block(
                a,
                b,
                dim,
                &incidence.nodes,
                &m.jacobian,
                &m.companion_current,
            );
        }
    }
    Ok(())
}

fn check_terminal_count(
    element: ElementId,
    family: &'static str,
    expected: usize,
    actual: usize,
) -> Result<(), MnaAssemblyError> {
    if expected != actual {
        return Err(MnaAssemblyError::LinearizationFamilyMismatch {
            element,
            linearization_family: family,
            linearization_terminals: expected,
            flatten_terminals: actual,
        });
    }
    Ok(())
}

/// Fold an `N×N` terminal-local Jacobian and `N`-vector companion
/// current into the global system, mapping terminal slot `k` to
/// `incidence_nodes[k]`.
///
/// Generic over the slice length so that all three device families
/// (Diode 2×2, BJT 3×3, MOSFET 4×4) share one body.
fn stamp_dense_block<const N: usize>(
    a: &mut [f64],
    b: &mut [f64],
    dim: u32,
    incidence_nodes: &[NodeId],
    jacobian: &[[f64; N]; N],
    companion_current: &[f64; N],
) {
    // Caller has already validated incidence_nodes.len() == N via
    // check_terminal_count; the explicit length panic here is a
    // last-line defense.
    assert_eq!(
        incidence_nodes.len(),
        N,
        "terminal count was not validated upstream"
    );
    for i in 0..N {
        let row = incidence_nodes[i].index();
        // Companion current is the residual current the linearized
        // model would draw at `v = 0`, in the convention used by
        // `device-modeling`:
        //
        //   companion_current[k] = I_term(v*) - J[k,:]·v*
        //
        // where `I_term(v*)` is the current the device draws OUT of
        // node `k` INTO terminal `k` at the iterate `v*`. The MNA
        // form is `A·V = b` with `A` carrying conductances (positive
        // contributions = current leaving a node per unit voltage)
        // and `b` carrying external-source current INJECTED INTO the
        // node. Moving the linearized device current `J·V +
        // companion_current` from the LHS to the RHS gives `b -=
        // companion_current` (i.e., the linearized model's residual
        // current is *subtracted* from the source injection at this
        // node).
        b[row as usize] -= companion_current[i];
        for j in 0..N {
            let col = incidence_nodes[j].index();
            *ar(a, dim, row, col) += jacobian[i][j];
        }
    }
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flatten::flatten;
    use circuit_solver_types::{BranchId, ElementId, NodeId};
    use device_modeling::stamp::{
        BJTLinearization, DiodeLinearization, LinearizedModel, MOSFETLinearization,
    };
    use netlist_graph::{CircuitBuilder, ElementKind};

    // ---------------- helpers ------------------------------------------------

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

    fn add_inductor(b: &mut CircuitBuilder, name: &str, a: &str, c: &str, henries: f64) {
        b.add_element(
            name,
            ElementKind::Inductor {
                inductance_henries: henries,
            },
            [a, c],
            None,
        )
        .expect("add inductor");
    }

    fn add_capacitor(b: &mut CircuitBuilder, name: &str, a: &str, c: &str, farads: f64) {
        b.add_element(
            name,
            ElementKind::Capacitor {
                capacitance_farads: farads,
            },
            [a, c],
            None,
        )
        .expect("add capacitor");
    }

    fn add_semiconductor(b: &mut CircuitBuilder, name: &str, terminals: &[&str], model: &str) {
        b.add_element(
            name,
            ElementKind::Semiconductor,
            terminals.to_vec(),
            Some(model.into()),
        )
        .expect("add semiconductor");
    }

    /// Approximate equality for `f64` matrix entries.
    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12_f64.max(1e-12 * a.abs().max(b.abs()))
    }

    // ---------------- empty / ground-only -----------------------------------

    #[test]
    fn ground_only_graph_produces_1x1_zero_system() {
        let g = CircuitBuilder::default().build().expect("empty ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        assert_eq!(sys.node_count(), 1);
        assert_eq!(sys.branch_count(), 0);
        assert_eq!(sys.dim(), 1);
        assert!(sys.matrix().iter().all(|&x| x == 0.0));
        assert!(sys.rhs().iter().all(|&x| x == 0.0));
    }

    // ---------------- linear-resistive scenario: the Gherkin ---------------

    /// Two-node ladder: `V1` from `n1` to ground (1 V), `R1` from
    /// `n1` to ground (1 kΩ), `R2` from `n1` to ground (1 kΩ). At DC
    /// the analytic solution is `v_n1 = 1 V` and the source current
    /// `i_V1 = -2 mA` (current flows out of the V+ terminal into the
    /// resistors → that is the negative MNA branch convention).
    ///
    /// This test does not solve the system; it verifies that the
    /// stamped matrix matches the textbook MNA formulation, which is
    /// the contract Pass 2 owes to item #16 (russell sparse LU).
    #[test]
    fn linear_resistive_two_node_stamp_matches_textbook() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", "0", 1.0);
        add_resistor(&mut b, "R1", "n1", "0", 1000.0);
        add_resistor(&mut b, "R2", "n1", "0", 1000.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");

        // node_count = 2 (gnd=0, n1=1); branch_count = 1 (V1).
        assert_eq!(sys.node_count(), 2);
        assert_eq!(sys.branch_count(), 1);
        assert_eq!(sys.dim(), 3);

        // Conductance block:
        //  [ 2/R   -2/R   ; ... ]    at node 0 (gnd)
        //  [ -2/R   2/R   ; ... ]    at node 1 (n1)
        let g_per_resistor = 1.0_f64 / 1000.0;
        let two_g = 2.0 * g_per_resistor;
        assert!(approx(sys.matrix_entry(0, 0).unwrap(), two_g));
        assert!(approx(sys.matrix_entry(1, 1).unwrap(), two_g));
        assert!(approx(sys.matrix_entry(0, 1).unwrap(), -two_g));
        assert!(approx(sys.matrix_entry(1, 0).unwrap(), -two_g));

        // Voltage-source branch row/col: V1 between n1(+) and 0(-) via branch 0.
        // Branch row index in the full system: 2.
        // (br, n1) = +1, (br, 0) = -1; (n1, br) = +1, (0, br) = -1.
        assert!(approx(sys.matrix_entry(2, 1).unwrap(), 1.0));
        assert!(approx(sys.matrix_entry(2, 0).unwrap(), -1.0));
        assert!(approx(sys.matrix_entry(1, 2).unwrap(), 1.0));
        assert!(approx(sys.matrix_entry(0, 2).unwrap(), -1.0));

        // RHS: only the branch row carries the source voltage.
        assert!(approx(sys.rhs_entry(0).unwrap(), 0.0));
        assert!(approx(sys.rhs_entry(1).unwrap(), 0.0));
        assert!(approx(sys.rhs_entry(2).unwrap(), 1.0));
    }

    /// The full MNA matrix has the ground row/column **present** —
    /// ADR-0003 dictates that sub-view extraction (item #15) removes
    /// it later, not Pass 2.
    #[test]
    fn ground_row_and_column_are_present_per_adr_0003() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 100.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");

        // node_count = 2 (gnd + n1), branch_count = 0
        assert_eq!(sys.dim(), 2);
        // Ground row/col MUST carry the conductance entries: stamping
        // a resistor against ground writes G[0][0] = +g, G[0][1] = -g.
        let g_val = 1.0 / 100.0;
        assert!(approx(sys.matrix_entry(0, 0).unwrap(), g_val));
        assert!(approx(sys.matrix_entry(0, 1).unwrap(), -g_val));
        assert!(approx(sys.matrix_entry(1, 0).unwrap(), -g_val));
        assert!(approx(sys.matrix_entry(1, 1).unwrap(), g_val));
    }

    // ---------------- current-source stamp ----------------------------------

    #[test]
    fn current_source_writes_only_to_rhs_with_spice_convention() {
        let mut b = CircuitBuilder::default();
        // Need at least one resistor to give the node a finite stamp;
        // we are checking the RHS sign convention only.
        add_resistor(&mut b, "R1", "n1", "0", 1000.0);
        add_current_source(&mut b, "I1", "n1", "0", 3.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");

        // I1 from `n1` to `0`: +3 at RHS[n1], -3 at RHS[gnd].
        assert!(approx(sys.rhs_entry(0).unwrap(), -3.0));
        assert!(approx(sys.rhs_entry(1).unwrap(), 3.0));
    }

    // ---------------- inductor (DC short) ----------------------------------

    #[test]
    fn inductor_at_dc_acts_as_branch_with_zero_rhs() {
        let mut b = CircuitBuilder::default();
        add_inductor(&mut b, "L1", "n1", "0", 1e-6);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");

        // dim = node_count(2) + branch_count(1) = 3.
        assert_eq!(sys.dim(), 3);
        // Same incidence as a voltage source — but RHS is zero (DC
        // short forces v_n1 - v_0 = 0).
        assert!(approx(sys.matrix_entry(2, 1).unwrap(), 1.0));
        assert!(approx(sys.matrix_entry(2, 0).unwrap(), -1.0));
        assert!(approx(sys.matrix_entry(1, 2).unwrap(), 1.0));
        assert!(approx(sys.matrix_entry(0, 2).unwrap(), -1.0));
        assert!(approx(sys.rhs_entry(2).unwrap(), 0.0));
    }

    // ---------------- capacitor (DC open) ----------------------------------

    #[test]
    fn capacitor_at_dc_is_a_no_op_stamp() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1.0);
        add_capacitor(&mut b, "C1", "n1", "0", 1e-9);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");

        // dim = 2 (gnd + n1), no branches.
        assert_eq!(sys.dim(), 2);
        // Only the resistor contributes; the capacitor adds nothing
        // at DC.
        let g_val = 1.0_f64;
        assert!(approx(sys.matrix_entry(1, 1).unwrap(), g_val));
        assert!(approx(sys.matrix_entry(0, 0).unwrap(), g_val));
        assert!(approx(sys.matrix_entry(0, 1).unwrap(), -g_val));
        assert!(approx(sys.matrix_entry(1, 0).unwrap(), -g_val));
    }

    // ---------------- linearization stamps ---------------------------------

    /// A diode at the iterate `[0.7, 0.0]` with a non-trivial 2×2
    /// Jacobian and 2-vector companion current: the assembler maps
    /// terminal slot 0 → anode node and terminal slot 1 → cathode
    /// node, then folds the entries in.
    #[test]
    fn diode_linearization_stamps_into_anode_cathode_block() {
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "D1", &["n_a", "n_c"], "D");
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");

        // Anode and cathode node ids (graph assigns in order: 0=gnd, 1=n_a, 2=n_c).
        let anode = NodeId::new(1);
        let cathode = NodeId::new(2);

        let lin = LinearizedModel::Diode(DiodeLinearization {
            jacobian: [[1.0, -1.0], [-1.0, 1.0]],
            companion_current: [-0.5, 0.5],
        });
        let lins = vec![Some(lin)];
        let sys = assemble(&fs, &g, &lins).expect("assemble ok");

        // dim = 3 (gnd + n_a + n_c), no branches.
        assert_eq!(sys.dim(), 3);
        // Jacobian goes into the (anode, cathode) sub-block.
        assert!(approx(
            sys.matrix_entry(anode.index(), anode.index()).unwrap(),
            1.0
        ));
        assert!(approx(
            sys.matrix_entry(anode.index(), cathode.index()).unwrap(),
            -1.0
        ));
        assert!(approx(
            sys.matrix_entry(cathode.index(), anode.index()).unwrap(),
            -1.0
        ));
        assert!(approx(
            sys.matrix_entry(cathode.index(), cathode.index()).unwrap(),
            1.0
        ));
        // Companion current is *subtracted* from the RHS at each
        // terminal node — see `stamp_dense_block`'s docstring for
        // the sign convention (companion_current[k] = I_term(v*) −
        // J·v*; this is the residual current the linearized model
        // would draw at `v=0`, which moves to `-` on the RHS when
        // we rearrange `A·V = b` from `LHS - sources = 0`).
        assert!(approx(sys.rhs_entry(anode.index()).unwrap(), 0.5));
        assert!(approx(sys.rhs_entry(cathode.index()).unwrap(), -0.5));
        // Ground row/col is unaffected by this stamp.
        assert!(approx(sys.matrix_entry(0, 0).unwrap(), 0.0));
        assert!(approx(sys.rhs_entry(0).unwrap(), 0.0));
    }

    #[test]
    fn bjt_linearization_stamps_into_3x3_block() {
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "Q1", &["nc", "nb", "ne"], "QN");
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        // graph node order: 0=gnd, 1=nc, 2=nb, 3=ne.
        let lin = LinearizedModel::BJT(BJTLinearization {
            jacobian: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
            companion_current: [0.1, 0.2, 0.3],
        });
        let sys = assemble(&fs, &g, &[Some(lin)]).expect("assemble ok");
        assert!(approx(sys.matrix_entry(1, 1).unwrap(), 1.0));
        assert!(approx(sys.matrix_entry(1, 2).unwrap(), 2.0));
        assert!(approx(sys.matrix_entry(3, 1).unwrap(), 7.0));
        assert!(approx(sys.matrix_entry(3, 3).unwrap(), 9.0));
        // Companion current is subtracted on the RHS (see
        // `stamp_dense_block` docstring).
        assert!(approx(sys.rhs_entry(1).unwrap(), -0.1));
        assert!(approx(sys.rhs_entry(3).unwrap(), -0.3));
    }

    #[test]
    fn mosfet_linearization_stamps_into_4x4_block() {
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "M1", &["nd", "ng", "ns", "nb"], "NMOS");
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        // node order: 0=gnd, 1=nd, 2=ng, 3=ns, 4=nb.
        let mut jac = [[0.0_f64; 4]; 4];
        // Drop a distinctive entry in every cell so we can verify
        // the global index mapping is correct.
        for (i, row) in jac.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let v = u16::try_from(i * 10 + j).expect("i*10+j fits in u16");
                *cell = f64::from(v);
            }
        }
        let lin = LinearizedModel::MOSFET(MOSFETLinearization {
            jacobian: jac,
            companion_current: [0.01, 0.02, 0.03, 0.04],
        });
        let sys = assemble(&fs, &g, &[Some(lin)]).expect("assemble ok");
        assert!(approx(sys.matrix_entry(1, 1).unwrap(), 0.0));
        assert!(approx(sys.matrix_entry(1, 4).unwrap(), 3.0));
        assert!(approx(sys.matrix_entry(2, 3).unwrap(), 12.0));
        assert!(approx(sys.matrix_entry(4, 4).unwrap(), 33.0));
        // Companion current is subtracted on the RHS (see
        // `stamp_dense_block` docstring).
        assert!(approx(sys.rhs_entry(1).unwrap(), -0.01));
        assert!(approx(sys.rhs_entry(4).unwrap(), -0.04));
    }

    #[test]
    fn diode_linearization_can_be_zero() {
        // The task-#8 placeholder is the zero linearization. Verify
        // it stamps to a no-op so a circuit that only contains a
        // device with a zero linearization assembles to a zero matrix.
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "D1", &["n_a", "n_c"], "D");
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(
            &fs,
            &g,
            &[Some(LinearizedModel::Diode(DiodeLinearization::zero()))],
        )
        .expect("assemble ok");
        assert!(sys.matrix().iter().all(|&x| x == 0.0));
        assert!(sys.rhs().iter().all(|&x| x == 0.0));
    }

    // ---------------- error surface ----------------------------------------

    #[test]
    fn semiconductor_without_linearization_is_rejected() {
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "D1", &["n_a", "n_c"], "D");
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let err = assemble(&fs, &g, &[]).unwrap_err();
        assert_eq!(
            err,
            MnaAssemblyError::MissingLinearizationForDevice {
                element: ElementId::new(0),
            }
        );
    }

    #[test]
    fn diode_linearization_with_wrong_terminal_count_is_rejected() {
        // Build a BJT-shaped element (3 terminals) but feed a Diode
        // linearization (2 terminals) — the assembler must reject.
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "Q1", &["nc", "nb", "ne"], "QN");
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let lin = LinearizedModel::Diode(DiodeLinearization::zero());
        let err = assemble(&fs, &g, &[Some(lin)]).unwrap_err();
        assert_eq!(
            err,
            MnaAssemblyError::LinearizationFamilyMismatch {
                element: ElementId::new(0),
                linearization_family: "Diode",
                linearization_terminals: 2,
                flatten_terminals: 3,
            }
        );
    }

    #[test]
    fn non_positive_resistance_is_rejected() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 0.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let err = assemble(&fs, &g, &[]).unwrap_err();
        assert_eq!(
            err,
            MnaAssemblyError::NonPositiveResistance {
                element: ElementId::new(0),
                resistance_ohms: 0.0,
            }
        );
    }

    #[test]
    fn negative_resistance_is_rejected() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", -10.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let err = assemble(&fs, &g, &[]).unwrap_err();
        assert_eq!(
            err,
            MnaAssemblyError::NonPositiveResistance {
                element: ElementId::new(0),
                resistance_ohms: -10.0,
            }
        );
    }

    #[test]
    fn nan_voltage_source_is_rejected() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", "0", f64::NAN);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let err = assemble(&fs, &g, &[]).unwrap_err();
        assert_eq!(
            err,
            MnaAssemblyError::NonFiniteParameter {
                element: ElementId::new(0),
                kind: "V",
            }
        );
    }

    #[test]
    fn nan_resistor_is_rejected() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", f64::NAN);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let err = assemble(&fs, &g, &[]).unwrap_err();
        assert_eq!(
            err,
            MnaAssemblyError::NonFiniteParameter {
                element: ElementId::new(0),
                kind: "R",
            }
        );
    }

    #[test]
    fn inf_current_source_is_rejected() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1.0);
        add_current_source(&mut b, "I1", "n1", "0", f64::INFINITY);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let err = assemble(&fs, &g, &[]).unwrap_err();
        assert_eq!(
            err,
            MnaAssemblyError::NonFiniteParameter {
                element: ElementId::new(1),
                kind: "I",
            }
        );
    }

    // ---------------- composition / additivity ------------------------------

    #[test]
    fn multiple_resistors_in_parallel_sum_conductances() {
        // Three resistors in parallel between n1 and ground:
        // 1Ω, 1Ω, 1Ω → total 3 S between (n1, n1) and (gnd, gnd),
        // -3 S off-diagonal.
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1.0);
        add_resistor(&mut b, "R2", "n1", "0", 1.0);
        add_resistor(&mut b, "R3", "n1", "0", 1.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        assert!(approx(sys.matrix_entry(1, 1).unwrap(), 3.0));
        assert!(approx(sys.matrix_entry(0, 0).unwrap(), 3.0));
        assert!(approx(sys.matrix_entry(1, 0).unwrap(), -3.0));
        assert!(approx(sys.matrix_entry(0, 1).unwrap(), -3.0));
    }

    #[test]
    fn assembly_is_deterministic_for_same_input() {
        // Build the same circuit twice and verify byte-identical
        // matrices — a property the integrator / golden-reference
        // pipeline (ADR-0008) implicitly depends on.
        let build_once = || {
            let mut b = CircuitBuilder::default();
            add_resistor(&mut b, "R1", "n1", "n2", 100.0);
            add_resistor(&mut b, "R2", "n2", "0", 200.0);
            add_voltage_source(&mut b, "V1", "n1", "0", 5.0);
            b.build().expect("build ok")
        };
        let g1 = build_once();
        let g2 = build_once();
        let fs1 = flatten(&g1).expect("ok");
        let fs2 = flatten(&g2).expect("ok");
        let s1 = assemble(&fs1, &g1, &[]).expect("ok");
        let s2 = assemble(&fs2, &g2, &[]).expect("ok");
        assert_eq!(s1, s2);
    }

    #[test]
    fn three_node_ladder_matches_textbook_template() {
        // V1 from n1 to gnd, R1 = 100Ω between n1 and n2,
        // R2 = 200Ω between n2 and gnd.
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", "0", 5.0);
        add_resistor(&mut b, "R1", "n1", "n2", 100.0);
        add_resistor(&mut b, "R2", "n2", "0", 200.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");

        // node_count = 3 (gnd=0, n1=1, n2=2), branch_count = 1 (V1).
        assert_eq!(sys.dim(), 4);

        let g1 = 1.0 / 100.0; // R1 conductance
        let g2 = 1.0 / 200.0; // R2 conductance

        // KCL at gnd: only R2 contributes (between gnd and n2).
        assert!(approx(sys.matrix_entry(0, 0).unwrap(), g2));
        assert!(approx(sys.matrix_entry(0, 2).unwrap(), -g2));
        // KCL at n1: R1 contributes between n1 and n2.
        assert!(approx(sys.matrix_entry(1, 1).unwrap(), g1));
        assert!(approx(sys.matrix_entry(1, 2).unwrap(), -g1));
        // KCL at n2: R1 + R2 incidence.
        assert!(approx(sys.matrix_entry(2, 2).unwrap(), g1 + g2));
        assert!(approx(sys.matrix_entry(2, 1).unwrap(), -g1));
        assert!(approx(sys.matrix_entry(2, 0).unwrap(), -g2));

        // Voltage-source branch row.
        assert!(approx(sys.matrix_entry(3, 1).unwrap(), 1.0));
        assert!(approx(sys.matrix_entry(3, 0).unwrap(), -1.0));
        assert!(approx(sys.matrix_entry(1, 3).unwrap(), 1.0));
        assert!(approx(sys.matrix_entry(0, 3).unwrap(), -1.0));
        // RHS: only branch row carries source.
        assert!(approx(sys.rhs_entry(3).unwrap(), 5.0));
        assert!(approx(sys.rhs_entry(0).unwrap(), 0.0));
        assert!(approx(sys.rhs_entry(1).unwrap(), 0.0));
        assert!(approx(sys.rhs_entry(2).unwrap(), 0.0));
    }

    #[test]
    fn linearization_slice_shorter_than_element_count_treats_missing_as_none() {
        // A linear circuit + a short (empty) linearizations slice
        // must succeed because no element is a semiconductor.
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1000.0);
        add_resistor(&mut b, "R2", "n2", "0", 1000.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        assert!(assemble(&fs, &g, &[]).is_ok());
        assert!(assemble(&fs, &g, &[None]).is_ok());
        assert!(assemble(&fs, &g, &[None, None]).is_ok());
    }

    #[test]
    fn graph_flatten_mismatch_is_rejected() {
        // Flatten one graph, then build a different one — the
        // assembler must refuse.
        let mut b1 = CircuitBuilder::default();
        add_resistor(&mut b1, "R1", "n1", "0", 1.0);
        let g1 = b1.build().expect("build ok");
        let fs = flatten(&g1).expect("flatten ok");

        let mut b2 = CircuitBuilder::default();
        // No elements at all.
        let g2 = b2.build().expect("build ok");
        let err = assemble(&fs, &g2, &[]).unwrap_err();
        assert!(matches!(err, MnaAssemblyError::GraphFlattenMismatch { .. }));
    }

    // ---------------- accessor surface --------------------------------------

    #[test]
    fn matrix_entry_out_of_range_returns_none() {
        let g = CircuitBuilder::default().build().expect("ok");
        let fs = flatten(&g).expect("ok");
        let sys = assemble(&fs, &g, &[]).expect("ok");
        assert!(sys.matrix_entry(99, 0).is_none());
        assert!(sys.matrix_entry(0, 99).is_none());
        assert!(sys.rhs_entry(99).is_none());
    }

    #[test]
    fn dim_matches_node_count_plus_branch_count() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "n2", 100.0);
        add_resistor(&mut b, "R2", "n2", "0", 200.0);
        add_voltage_source(&mut b, "V1", "n1", "0", 5.0);
        add_inductor(&mut b, "L1", "n2", "0", 1e-3);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        assert_eq!(sys.dim(), sys.node_count() + sys.branch_count());
        assert_eq!(sys.matrix().len(), (sys.dim() as usize).pow(2));
        assert_eq!(sys.rhs().len(), sys.dim() as usize);
    }

    // ---------------- branch indexing -- unused param suppression -----------

    #[test]
    fn branch_id_param_compiles() {
        // BranchId is referenced in error variants; this test exists
        // to ensure the type stays linked when adapting the imports.
        let _b = BranchId::new(0);
    }
}
