//! Pass-1 flattened structure for MNA assembly.
//!
//! `FlattenedStructure` is the output of the Pass-1 graph flattening step
//! (`tasks.md` item #6) and the input to the Pass-2 MNA matrix assembly
//! step (`tasks.md` item #14). It carries the full incidence mapping of
//! a `CircuitGraph` in a layout that the assembler can stamp directly
//! without re-walking the original graph.
//!
//! This module covers `tasks.md` item #3: defining the *struct itself*
//! — its fields, invariants, constructor API, and accessor surface.
//! Item #6 fills in the algorithm that walks a `CircuitGraph` and
//! produces a `FlattenedStructure`; item #4 (the topology checker)
//! attaches a `TopologyReport` to the same struct.
//!
//! # Design references
//!
//! - ADR-0003 (two-pass graph flattening): Pass 1 produces *one*
//!   `FlattenedStructure` per `CircuitGraph`, with full incidence
//!   including the ground node. Pass 2 then builds the full MNA matrix
//!   from this structure; sub-view extraction applies per-analysis
//!   masks (ground suppression, complex augmentation, companion
//!   stamps) without re-flattening.
//! - ADR-0009 (topology checker): the checker attaches a
//!   `TopologyReport` to the `FlattenedStructure` so the orchestrator
//!   can auto-enable Gmin-stepping for warning nodes before any solve
//!   is attempted.
//! - `design.md` ASR-3: the structure is *computed once and cached*;
//!   analysis switches reuse it.
//! - Scenario `dc-operating-point#linear-resistive-dc-operating-point`:
//!   the eventual consumer that uses the flattened incidence to stamp
//!   a linear conductance matrix.
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.
//!
//! # Indexing invariants
//!
//! - `NodeId` indices and `BranchId` indices live in **independent**
//!   namespaces. Both start at 0 (the residual risk explicitly carried
//!   forward from `t_391b08fc`/`t_1a3758b0`); `NodeId::GROUND` is
//!   `NodeId::new(0)`, while `BranchId::new(0)` is the first MNA
//!   augmentation row regardless of what node id 0 means.
//! - The ground reference is a `NodeId`, by construction equal to
//!   `NodeId::GROUND`. It is exposed via `ground_node()` so future code
//!   does not hard-code `NodeId::GROUND` against this structure.
//! - `node_count()` includes the ground node. The MNA assembler builds
//!   the **full** matrix per ADR-0003; sub-view extraction (tasks.md
//!   item #15) is where ground suppression happens.

use circuit_solver_types::{BranchId, ElementId, NodeId};

/// Incidence stamp for a single circuit element after Pass 1
/// flattening.
///
/// `ElementIncidence` records *which* nodes (and, if applicable, which
/// branch row) a given element connects to. The actual *numeric* stamp
/// (conductance values, companion-model coefficients) is computed in
/// Pass 2 by walking the per-element `DeviceModel` (tasks.md item #14);
/// this struct only records *topology*.
///
/// # Variants
///
/// - **Two-terminal conductive** (`Resistor`, `Capacitor`, current
///   sources): two endpoint nodes, no branch row.
/// - **Two-terminal current-carrying** (voltage sources, inductors):
///   two endpoint nodes plus one MNA branch row for the unknown
///   current.
/// - **Three- and four-terminal devices** (diodes are two-terminal,
///   BJTs three-terminal, MOSFETs four-terminal): a small array of
///   port nodes plus, when the device introduces an internal
///   current-unknown (e.g. a MOSFET in its augmented form), an
///   optional branch row.
///
/// `ElementIncidence` does **not** model the device itself — that is
/// the `DeviceModel` enum (tasks.md item #7 / ADR-0005). The two pair
/// up via shared `ElementId` indexing in the parent
/// `FlattenedStructure`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementIncidence {
    /// The element this stamp belongs to.
    pub element: ElementId,
    /// The circuit nodes the element connects to, in element-defined
    /// pin order (e.g. `[anode, cathode]` for a diode;
    /// `[collector, base, emitter]` for a BJT). The slice is short
    /// (≤4 in v1) so a `Vec<NodeId>` is acceptable; the consumer must
    /// honor each model's pin convention.
    pub nodes: Vec<NodeId>,
    /// `Some(branch)` if this element contributes a current-carrying
    /// augmentation row to the MNA system (voltage source, inductor,
    /// or any model with an internal current unknown). `None` if the
    /// element's contribution stays in the conductance matrix
    /// (resistor, capacitor, current source).
    pub branch: Option<BranchId>,
}

impl ElementIncidence {
    /// Construct an incidence record for a two-terminal conductive
    /// element (resistor, capacitor, current source).
    ///
    /// The element contributes only conductance-matrix entries — no
    /// MNA branch row.
    #[must_use]
    pub fn two_terminal_conductive(element: ElementId, a: NodeId, b: NodeId) -> Self {
        Self {
            element,
            nodes: vec![a, b],
            branch: None,
        }
    }

    /// Construct an incidence record for a two-terminal
    /// current-carrying element (voltage source, inductor).
    ///
    /// `branch` is the MNA branch row reserved for this element's
    /// current unknown.
    #[must_use]
    pub fn two_terminal_current_carrying(
        element: ElementId,
        a: NodeId,
        b: NodeId,
        branch: BranchId,
    ) -> Self {
        Self {
            element,
            nodes: vec![a, b],
            branch: Some(branch),
        }
    }

    /// Construct an incidence record for a multi-terminal device
    /// (diode, BJT, MOSFET).
    ///
    /// `branch` is `None` for device models that stamp only into the
    /// conductance matrix at the linearized step (the v1 Diode/BJT/
    /// MOSFET stamps per tasks.md items #9–#13), and `Some(_)` if a
    /// future model variant introduces an internal current unknown.
    #[must_use]
    pub fn device(element: ElementId, nodes: Vec<NodeId>, branch: Option<BranchId>) -> Self {
        Self {
            element,
            nodes,
            branch,
        }
    }

    /// True iff this element contributes an MNA branch row.
    #[must_use]
    pub fn has_branch(&self) -> bool {
        self.branch.is_some()
    }
}

/// Pass-1 flattened netlist topology, ready for Pass-2 MNA assembly.
///
/// `FlattenedStructure` is the canonical hand-off between
/// `netlist-graph` (which produces it via Pass 1, tasks.md item #6)
/// and `numeric-solver` (which consumes it via Pass 2 assembly,
/// tasks.md item #14, and sub-view extraction, item #15). It also
/// carries the topology checker's report (tasks.md item #4 /
/// ADR-0009), populated when the checker has run.
///
/// # Fields and accessors
///
/// All fields are exposed via accessor methods rather than `pub`
/// fields so the internal representation can evolve (e.g. swapping
/// `Vec` for a CSR-style packed incidence) without breaking
/// downstream crates. Construction goes through
/// [`FlattenedStructure::new`] with a builder-style argument bundle
/// that the Pass-1 implementation in `netlist-graph` fills in.
///
/// # Indexing
///
/// - Node indices and branch indices are **independent**. A
///   `NodeId(7)` is unrelated to a `BranchId(7)`.
/// - `NodeId::GROUND` (== `NodeId::new(0)`) is the ground reference.
///   `node_count()` includes ground.
/// - Element indices are dense `0..element_count()` — every element in
///   the original `CircuitGraph` produces exactly one
///   `ElementIncidence`, in the order they were added.
/// - The `node_to_branches` map is keyed by node and stores the
///   branches *incident to that node*. It is populated by
///   [`FlattenedStructure::new`] from the per-element incidence
///   records; callers do not maintain it directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlattenedStructure {
    /// Total node count, including the ground node at index 0.
    node_count: u32,
    /// Total MNA branch (current-carrying augmentation row) count.
    branch_count: u32,
    /// Per-element incidence records, indexed by `ElementId::index()`.
    elements: Vec<ElementIncidence>,
    /// `node_to_branches[node_index]` is the list of `BranchId`s
    /// incident to that node, in element-insertion order. The ground
    /// node has its own bucket (the assembler builds the full MNA
    /// matrix per ADR-0003; ground suppression happens later in the
    /// sub-view extractor, tasks.md item #15).
    node_to_branches: Vec<Vec<BranchId>>,
    /// Topology report attached by the topology checker (tasks.md
    /// item #4, ADR-0009). `None` if the checker has not yet been run.
    topology_report: Option<TopologyReport>,
}

/// Result of the topology checker (tasks.md item #4, ADR-0009).
///
/// Pass 1's topology checker traverses the flattened incidence and
/// classifies every node by its DC connectivity to ground:
///
/// - `floating` — no DC path to ground through any conductive element
///   (resistor, voltage source, or inductor's DC short). This is a
///   hard fault: the MNA matrix will be structurally singular.
/// - `warning` — DC path to ground exists only through "possibly
///   conductive" elements (diodes, MOSFETs). The orchestrator
///   auto-enables Gmin-stepping for these.
///
/// Per ADR-0009 the checker is conservative: it flags possibly-
/// conductive paths as warnings rather than errors, so a valid
/// nonlinear circuit is not rejected pre-solve.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TopologyReport {
    /// Nodes with no DC path to ground; hard fault.
    pub floating: Vec<NodeId>,
    /// Nodes grounded only through possibly-conductive devices;
    /// orchestrator should auto-enable Gmin-stepping per ADR-0009.
    pub warning: Vec<NodeId>,
}

impl TopologyReport {
    /// True iff the report records at least one floating node.
    #[must_use]
    pub fn has_floating(&self) -> bool {
        !self.floating.is_empty()
    }

    /// True iff the report records at least one warning (possibly
    /// floating) node, but no hard-fault floating node.
    #[must_use]
    pub fn has_warning(&self) -> bool {
        !self.warning.is_empty()
    }

    /// True iff the report is clean (no floating, no warning).
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.floating.is_empty() && self.warning.is_empty()
    }
}

/// Errors raised when constructing a `FlattenedStructure` from
/// inconsistent inputs.
///
/// The Pass-1 implementation (tasks.md item #6) is responsible for
/// constructing a consistent bundle; these errors exist so that
/// constructor misuse fails loudly rather than silently corrupting
/// downstream assembly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlattenedStructureError {
    /// `node_count` was zero — at minimum the ground node must exist.
    EmptyNodeSet,
    /// An `ElementIncidence` referred to a `NodeId` whose index is
    /// out of range for `node_count`.
    NodeOutOfRange {
        /// Element whose record was rejected.
        element: ElementId,
        /// The offending node id.
        node: NodeId,
        /// The node count the structure was constructed with.
        node_count: u32,
    },
    /// An `ElementIncidence` referred to a `BranchId` whose index is
    /// out of range for `branch_count`.
    BranchOutOfRange {
        /// Element whose record was rejected.
        element: ElementId,
        /// The offending branch id.
        branch: BranchId,
        /// The branch count the structure was constructed with.
        branch_count: u32,
    },
    /// An `ElementIncidence`'s embedded `ElementId` did not match its
    /// position in the input slice.
    ElementIndexMismatch {
        /// The position the record occupied in the input slice.
        expected: ElementId,
        /// The `ElementId` actually stored in the record.
        found: ElementId,
    },
    /// Two `ElementIncidence` records claimed the same `BranchId`.
    /// Each MNA branch row must correspond to exactly one
    /// current-carrying element.
    DuplicateBranchOwner {
        /// The branch that was claimed twice.
        branch: BranchId,
        /// The first element seen claiming the branch.
        first: ElementId,
        /// The second element seen claiming the branch.
        second: ElementId,
    },
}

impl core::fmt::Display for FlattenedStructureError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyNodeSet => write!(
                f,
                "FlattenedStructure requires at least one node (the ground reference)"
            ),
            Self::NodeOutOfRange {
                element,
                node,
                node_count,
            } => write!(
                f,
                "element {element} references {node}, which is out of range for node_count={node_count}"
            ),
            Self::BranchOutOfRange {
                element,
                branch,
                branch_count,
            } => write!(
                f,
                "element {element} references {branch}, which is out of range for branch_count={branch_count}"
            ),
            Self::ElementIndexMismatch { expected, found } => write!(
                f,
                "element record at position {expected} has stored id {found}"
            ),
            Self::DuplicateBranchOwner {
                branch,
                first,
                second,
            } => write!(
                f,
                "{branch} claimed by both {first} and {second}; each MNA branch row must have exactly one owner"
            ),
        }
    }
}

impl std::error::Error for FlattenedStructureError {}

impl FlattenedStructure {
    /// Construct a flattened structure from a consistent incidence
    /// bundle.
    ///
    /// The Pass-1 implementation in `netlist-graph` (tasks.md item #6)
    /// is the canonical caller: it walks the `CircuitGraph` and
    /// produces the `(node_count, branch_count, elements)` triple.
    /// This constructor validates the triple, computes the
    /// `node_to_branches` reverse index, and returns the assembled
    /// structure.
    ///
    /// # Invariants checked
    ///
    /// - `node_count >= 1` (at least the ground node).
    /// - Every `NodeId` referenced by an element is in
    ///   `0..node_count`.
    /// - Every `BranchId` referenced by an element is in
    ///   `0..branch_count`.
    /// - The `ElementId` stored in each `ElementIncidence` matches
    ///   that record's position in `elements`.
    /// - Each `BranchId` is owned by at most one element (no two
    ///   elements may share an MNA branch row).
    ///
    /// # Errors
    ///
    /// Returns the corresponding [`FlattenedStructureError`] variant
    /// if any invariant is violated.
    ///
    /// # Panics
    ///
    /// Panics only if the input `elements.len()` exceeds `u32::MAX`,
    /// which is structurally impossible: every `ElementId` is itself a
    /// `u32` and `elements` is indexed by `ElementId`.
    pub fn new(
        node_count: u32,
        branch_count: u32,
        elements: Vec<ElementIncidence>,
    ) -> Result<Self, FlattenedStructureError> {
        if node_count == 0 {
            return Err(FlattenedStructureError::EmptyNodeSet);
        }

        let mut node_to_branches: Vec<Vec<BranchId>> =
            (0..node_count).map(|_| Vec::new()).collect();
        let mut branch_owner: Vec<Option<ElementId>> = (0..branch_count).map(|_| None).collect();

        for (idx, inc) in elements.iter().enumerate() {
            let expected = ElementId::new(u32::try_from(idx).expect("element index fits in u32"));
            if inc.element != expected {
                return Err(FlattenedStructureError::ElementIndexMismatch {
                    expected,
                    found: inc.element,
                });
            }

            for &node in &inc.nodes {
                if node.index() >= node_count {
                    return Err(FlattenedStructureError::NodeOutOfRange {
                        element: inc.element,
                        node,
                        node_count,
                    });
                }
            }

            if let Some(branch) = inc.branch {
                let bi = branch.index();
                if bi >= branch_count {
                    return Err(FlattenedStructureError::BranchOutOfRange {
                        element: inc.element,
                        branch,
                        branch_count,
                    });
                }
                let slot = &mut branch_owner[bi as usize];
                if let Some(first) = *slot {
                    return Err(FlattenedStructureError::DuplicateBranchOwner {
                        branch,
                        first,
                        second: inc.element,
                    });
                }
                *slot = Some(inc.element);

                for &node in &inc.nodes {
                    node_to_branches[node.index() as usize].push(branch);
                }
            }
        }

        Ok(Self {
            node_count,
            branch_count,
            elements,
            node_to_branches,
            topology_report: None,
        })
    }

    /// Attach a topology report (tasks.md item #4 / ADR-0009).
    ///
    /// The topology checker runs after Pass 1 and decorates the
    /// already-built `FlattenedStructure` with its findings. The
    /// orchestrator consumes the report to decide whether to enable
    /// Gmin-stepping pre-solve.
    pub fn set_topology_report(&mut self, report: TopologyReport) {
        self.topology_report = Some(report);
    }

    /// The ground reference node. Always `NodeId::GROUND` by
    /// construction; the accessor exists so downstream code does not
    /// hard-code the constant against this structure.
    #[must_use]
    pub fn ground_node(&self) -> NodeId {
        NodeId::GROUND
    }

    /// Total node count, including the ground node.
    #[must_use]
    pub fn node_count(&self) -> u32 {
        self.node_count
    }

    /// Total MNA branch (current-carrying augmentation row) count.
    #[must_use]
    pub fn branch_count(&self) -> u32 {
        self.branch_count
    }

    /// Total element count.
    ///
    /// # Panics
    ///
    /// Panics only if the underlying `Vec<ElementIncidence>` somehow
    /// holds more than `u32::MAX` entries, which is structurally
    /// impossible because every `ElementId` is itself a `u32`.
    #[must_use]
    pub fn element_count(&self) -> u32 {
        u32::try_from(self.elements.len()).expect("element count fits in u32")
    }

    /// Iterate over all element incidence records in element-id order.
    pub fn elements(&self) -> impl Iterator<Item = &ElementIncidence> {
        self.elements.iter()
    }

    /// Look up the incidence record for a specific element.
    #[must_use]
    pub fn element(&self, id: ElementId) -> Option<&ElementIncidence> {
        self.elements.get(id.index() as usize)
    }

    /// The MNA branches incident to a given node.
    ///
    /// Returns an empty slice if the node has no branch incidences (a
    /// purely-conductive subgraph attached to it), or if the node is
    /// out of range.
    #[must_use]
    pub fn branches_at(&self, node: NodeId) -> &[BranchId] {
        self.node_to_branches
            .get(node.index() as usize)
            .map_or(&[][..], Vec::as_slice)
    }

    /// Iterate over all `(NodeId, &[BranchId])` pairs in node-id
    /// order, including the ground node.
    ///
    /// # Panics
    ///
    /// Panics only if `node_count` exceeds `u32::MAX`, which is
    /// structurally impossible because the field is itself a `u32`.
    pub fn node_to_branches(&self) -> impl Iterator<Item = (NodeId, &[BranchId])> {
        self.node_to_branches
            .iter()
            .enumerate()
            .map(|(idx, branches)| {
                (
                    NodeId::new(u32::try_from(idx).expect("node index fits in u32")),
                    branches.as_slice(),
                )
            })
    }

    /// Borrow the topology report, if the topology checker has run.
    #[must_use]
    pub fn topology_report(&self) -> Option<&TopologyReport> {
        self.topology_report.as_ref()
    }

    /// True iff the topology checker has been run on this structure.
    #[must_use]
    pub fn has_topology_report(&self) -> bool {
        self.topology_report.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- helpers ----------------------------------------------------

    fn elem(idx: u32) -> ElementId {
        ElementId::new(idx)
    }

    fn node(idx: u32) -> NodeId {
        NodeId::new(idx)
    }

    fn branch(idx: u32) -> BranchId {
        BranchId::new(idx)
    }

    // ---------- construction invariants ------------------------------------

    #[test]
    fn empty_node_set_is_rejected() {
        let err = FlattenedStructure::new(0, 0, vec![]).unwrap_err();
        assert_eq!(err, FlattenedStructureError::EmptyNodeSet);
    }

    #[test]
    fn ground_only_circuit_is_valid() {
        let fs = FlattenedStructure::new(1, 0, vec![]).expect("ground-only is legal");
        assert_eq!(fs.node_count(), 1);
        assert_eq!(fs.branch_count(), 0);
        assert_eq!(fs.element_count(), 0);
        assert_eq!(fs.ground_node(), NodeId::GROUND);
        assert!(fs.branches_at(NodeId::GROUND).is_empty());
    }

    #[test]
    fn ground_node_is_always_node_zero() {
        // Even with many real nodes, ground stays at index 0.
        let fs = FlattenedStructure::new(8, 0, vec![]).expect("legal");
        assert_eq!(fs.ground_node(), NodeId::new(0));
    }

    #[test]
    fn node_out_of_range_is_rejected() {
        let bad = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(5));
        let err = FlattenedStructure::new(2, 0, vec![bad]).unwrap_err();
        assert_eq!(
            err,
            FlattenedStructureError::NodeOutOfRange {
                element: elem(0),
                node: node(5),
                node_count: 2,
            }
        );
    }

    #[test]
    fn branch_out_of_range_is_rejected() {
        let bad =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(0), node(1), branch(3));
        let err = FlattenedStructure::new(2, 1, vec![bad]).unwrap_err();
        assert_eq!(
            err,
            FlattenedStructureError::BranchOutOfRange {
                element: elem(0),
                branch: branch(3),
                branch_count: 1,
            }
        );
    }

    #[test]
    fn element_index_mismatch_is_rejected() {
        // The record claims to be ElementId(2) but sits at position 0.
        let bad = ElementIncidence::two_terminal_conductive(elem(2), node(0), node(1));
        let err = FlattenedStructure::new(2, 0, vec![bad]).unwrap_err();
        assert_eq!(
            err,
            FlattenedStructureError::ElementIndexMismatch {
                expected: elem(0),
                found: elem(2),
            }
        );
    }

    #[test]
    fn duplicate_branch_owner_is_rejected() {
        // Two voltage sources both claiming branch 0 — illegal.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(0), node(1), branch(0));
        let v2 =
            ElementIncidence::two_terminal_current_carrying(elem(1), node(0), node(1), branch(0));
        let err = FlattenedStructure::new(2, 1, vec![v1, v2]).unwrap_err();
        assert_eq!(
            err,
            FlattenedStructureError::DuplicateBranchOwner {
                branch: branch(0),
                first: elem(0),
                second: elem(1),
            }
        );
    }

    // ---------- accessor surface ------------------------------------------

    #[test]
    fn element_enumeration_preserves_order() {
        // R1: nodes 0-1; R2: nodes 1-2; V1: nodes 2-0 + branch 0.
        let r1 = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(1));
        let r2 = ElementIncidence::two_terminal_conductive(elem(1), node(1), node(2));
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(2), node(2), node(0), branch(0));

        let fs =
            FlattenedStructure::new(3, 1, vec![r1.clone(), r2.clone(), v1.clone()]).expect("ok");

        let collected: Vec<&ElementIncidence> = fs.elements().collect();
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0], &r1);
        assert_eq!(collected[1], &r2);
        assert_eq!(collected[2], &v1);

        assert_eq!(fs.element(elem(0)), Some(&r1));
        assert_eq!(fs.element(elem(2)), Some(&v1));
        assert_eq!(fs.element(elem(7)), None);
    }

    #[test]
    fn node_to_branch_map_records_branch_incidence() {
        // V1 between node 1 and ground (node 0), branch 0.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(1), node(0), branch(0));
        // R1 between node 1 and node 2 — does NOT contribute a branch.
        let r1 = ElementIncidence::two_terminal_conductive(elem(1), node(1), node(2));
        let fs = FlattenedStructure::new(3, 1, vec![v1, r1]).expect("ok");

        // Branch 0 is incident to both node 0 (ground) and node 1.
        assert_eq!(fs.branches_at(node(0)), &[branch(0)]);
        assert_eq!(fs.branches_at(node(1)), &[branch(0)]);
        // Node 2 has no branch incidence — only the conductive resistor.
        assert_eq!(fs.branches_at(node(2)), &[] as &[BranchId]);
    }

    #[test]
    fn ground_reference_appears_in_branch_map_when_branch_touches_it() {
        // Ground-referenced voltage source is the canonical case; the
        // `dc-operating-point` linear scenario depends on this.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(1), node(0), branch(0));
        let fs = FlattenedStructure::new(2, 1, vec![v1]).expect("ok");
        // The ground node MUST appear in the map — ADR-0003 says we build the
        // FULL MNA matrix in Pass 2; ground suppression is a sub-view concern.
        assert!(fs.branches_at(NodeId::GROUND).contains(&branch(0)));
    }

    #[test]
    fn conductive_only_circuit_has_no_branches() {
        // A pure-resistor circuit (no voltage sources, no inductors)
        // has zero branches — the resistor stamps live entirely in
        // the conductance matrix.
        let r1 = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(1));
        let r2 = ElementIncidence::two_terminal_conductive(elem(1), node(1), node(0));
        let fs = FlattenedStructure::new(2, 0, vec![r1, r2]).expect("ok");

        assert_eq!(fs.branch_count(), 0);
        assert!(fs.branches_at(node(0)).is_empty());
        assert!(fs.branches_at(node(1)).is_empty());
        assert_eq!(fs.element_count(), 2);
    }

    #[test]
    fn branches_at_out_of_range_node_is_empty() {
        let fs = FlattenedStructure::new(2, 0, vec![]).expect("ok");
        assert!(fs.branches_at(NodeId::new(99)).is_empty());
    }

    #[test]
    fn node_id_and_branch_id_indices_are_independent() {
        // Carries forward the residual risk from t_391b08fc/t_1a3758b0:
        // "BranchId and NodeId indexing must remain distinct in Pass 1
        // FlattenedStructure (tasks.md #6)". A NodeId(0) (== ground)
        // co-exists with a BranchId(0) without any aliasing.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(0), node(1), branch(0));
        let fs = FlattenedStructure::new(2, 1, vec![v1]).expect("ok");

        assert_eq!(fs.ground_node().index(), 0);
        assert_eq!(fs.element(elem(0)).unwrap().branch.unwrap().index(), 0);
        // Same numeric index, different semantic spaces.
        assert_ne!(
            fs.ground_node().index().to_string(),
            "branch:0",
            "sanity: Display does the separating job"
        );
        assert_eq!(format!("{}", fs.ground_node()), "node:GND");
        assert_eq!(format!("{}", branch(0)), "branch:0");
    }

    #[test]
    fn node_to_branches_iter_yields_every_node_including_ground() {
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(1), node(0), branch(0));
        let fs = FlattenedStructure::new(3, 1, vec![v1]).expect("ok");

        let pairs: Vec<(NodeId, Vec<BranchId>)> = fs
            .node_to_branches()
            .map(|(n, b)| (n, b.to_vec()))
            .collect();

        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0].0, NodeId::GROUND);
        assert_eq!(pairs[0].1, vec![branch(0)]);
        assert_eq!(pairs[1].0, NodeId::new(1));
        assert_eq!(pairs[1].1, vec![branch(0)]);
        assert_eq!(pairs[2].0, NodeId::new(2));
        assert!(pairs[2].1.is_empty());
    }

    // ---------- element-incidence convenience constructors ----------------

    #[test]
    fn element_incidence_two_terminal_conductive_has_no_branch() {
        let r = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(1));
        assert!(!r.has_branch());
        assert_eq!(r.nodes, vec![node(0), node(1)]);
    }

    #[test]
    fn element_incidence_two_terminal_current_carrying_has_branch() {
        let v =
            ElementIncidence::two_terminal_current_carrying(elem(3), node(2), node(0), branch(1));
        assert!(v.has_branch());
        assert_eq!(v.branch, Some(branch(1)));
        assert_eq!(v.nodes, vec![node(2), node(0)]);
    }

    #[test]
    fn element_incidence_device_supports_three_and_four_terminal() {
        // BJT (three terminals).
        let q = ElementIncidence::device(elem(0), vec![node(1), node(2), node(0)], None);
        assert_eq!(q.nodes.len(), 3);
        assert!(!q.has_branch());

        // MOSFET (four terminals).
        let m = ElementIncidence::device(elem(1), vec![node(1), node(2), node(0), node(0)], None);
        assert_eq!(m.nodes.len(), 4);
        assert!(!m.has_branch());
    }

    // ---------- topology report attachment --------------------------------

    #[test]
    fn topology_report_is_initially_absent() {
        let fs = FlattenedStructure::new(2, 0, vec![]).expect("ok");
        assert!(!fs.has_topology_report());
        assert!(fs.topology_report().is_none());
    }

    #[test]
    fn topology_report_attaches_and_is_queryable() {
        let mut fs = FlattenedStructure::new(3, 0, vec![]).expect("ok");
        let report = TopologyReport {
            floating: vec![node(2)],
            warning: vec![],
        };
        fs.set_topology_report(report.clone());

        assert!(fs.has_topology_report());
        assert_eq!(fs.topology_report(), Some(&report));
        assert!(report.has_floating());
        assert!(!report.has_warning());
        assert!(!report.is_clean());
    }

    #[test]
    fn clean_topology_report_distinguishes_from_warning_only() {
        let clean = TopologyReport::default();
        assert!(clean.is_clean());
        assert!(!clean.has_floating());
        assert!(!clean.has_warning());

        let warning_only = TopologyReport {
            floating: vec![],
            warning: vec![node(5)],
        };
        assert!(!warning_only.is_clean());
        assert!(!warning_only.has_floating());
        assert!(warning_only.has_warning());
    }

    // ---------- the linear-resistive DC scenario sanity check -------------

    #[test]
    fn linear_resistive_voltage_divider_topology_flattens() {
        // The minimal circuit the dc-operating-point#linear-resistive-
        // dc-operating-point scenario exercises:
        //
        //   V1 (10 V) ──> n1
        //      │
        //      R1 (1 kΩ) between n1 and n2
        //      R2 (1 kΩ) between n2 and gnd
        //      V1 negative terminal at gnd
        //
        // Node layout: 0=gnd, 1=n1, 2=n2.
        // Branch layout: 0=V1's current.
        // Element layout: 0=V1, 1=R1, 2=R2.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(1), node(0), branch(0));
        let r1 = ElementIncidence::two_terminal_conductive(elem(1), node(1), node(2));
        let r2 = ElementIncidence::two_terminal_conductive(elem(2), node(2), node(0));

        let fs = FlattenedStructure::new(3, 1, vec![v1, r1, r2]).expect("ok");

        // Three nodes including ground, one branch, three elements.
        assert_eq!(fs.node_count(), 3);
        assert_eq!(fs.branch_count(), 1);
        assert_eq!(fs.element_count(), 3);

        // V1's current touches ground and n1 — not n2.
        assert_eq!(fs.branches_at(NodeId::GROUND), &[branch(0)]);
        assert_eq!(fs.branches_at(node(1)), &[branch(0)]);
        assert!(fs.branches_at(node(2)).is_empty());

        // No topology report attached yet — the checker (tasks.md #4)
        // is a separate task.
        assert!(!fs.has_topology_report());
    }

    #[test]
    fn error_display_strings_are_actionable() {
        let err = FlattenedStructure::new(0, 0, vec![]).unwrap_err();
        assert_eq!(
            format!("{err}"),
            "FlattenedStructure requires at least one node (the ground reference)"
        );

        let bad = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(5));
        let err = FlattenedStructure::new(2, 0, vec![bad]).unwrap_err();
        let s = format!("{err}");
        assert!(s.contains("elem:0"));
        assert!(s.contains("node:5"));
        assert!(s.contains("node_count=2"));
    }
}
