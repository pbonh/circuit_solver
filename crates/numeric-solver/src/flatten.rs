//! Pass-1 structure flattening: walk a `CircuitGraph` once and produce
//! a [`FlattenedStructure`] with full incidence including the ground
//! node.
//!
//! This module covers `tasks.md` item #6 of
//! `circuit-solver/2026-05-21-v1-spec`. It is the canonical caller of
//! [`FlattenedStructure::new`] (defined alongside the type in
//! `flattened.rs`, tasks.md item #3) and the hand-off from the
//! [`netlist_graph`] context to the rest of `numeric-solver`.
//!
//! # Design references
//!
//! - **ADR-0003 — Two-Pass Graph Flattening with Per-Analysis
//!   Sub-Views.** "The Numeric Solver Engine reads the `CircuitGraph`
//!   once from the Netlist Graph Builder and constructs the full
//!   incidence structure: node-to-branch mapping, element enumeration,
//!   and ground-reference bookkeeping. This pass is analysis-agnostic
//!   and executes exactly once per `CircuitGraph`."
//!   That is the contract this module honors.
//! - **`design.md` C4 L1**: the Flattener box sits inside the
//!   `numeric-solver` crate (right next to the MNA Assembler), which is
//!   why this code lives here and not in `netlist-graph` — the netlist
//!   crate owns *construction* of the immutable graph; this crate owns
//!   *consumption* of it.
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** The
//!   [`flatten`] function and the [`FlattenError`] enum are part of the
//!   v1 unstable surface; consumers must pin to exact versions.
//!
//! # Algorithm
//!
//! The flattening walk is intentionally trivial: every interesting
//! invariant is enforced by [`FlattenedStructure::new`]. The flattener
//! is therefore responsible only for the *categorization* step — for
//! each `Element` in the graph, decide which kind of
//! [`ElementIncidence`] it produces and (when applicable) allocate a
//! fresh [`BranchId`] for its MNA augmentation row.
//!
//! 1. Compute `node_count` from `graph.node_count()`. The builder
//!    always seeds the ground net, so this is always `>= 1`.
//! 2. Iterate `graph.elements()` in `ElementId` order:
//!    - **Current-carrying** kinds (`VoltageSource`, `Inductor`) get a
//!      freshly-allocated `BranchId` (running counter) and are recorded
//!      via [`ElementIncidence::two_terminal_current_carrying`].
//!    - **Conductive two-terminal** kinds (`Resistor`, `Capacitor`,
//!      `CurrentSource`) get no branch row and are recorded via
//!      [`ElementIncidence::two_terminal_conductive`].
//!    - **Semiconductor** devices get no branch row in v1 — the
//!      Diode/BJT/MOSFET stamps (tasks.md items #9..#13) all live in
//!      the conductance matrix at the linearized step. Multi-terminal
//!      semiconductor incidence is recorded via
//!      [`ElementIncidence::device`].
//!    - **Subcircuit instances** are a hard error: by the immutability
//!      contract in [`netlist_graph::CircuitGraph`], `build()`
//!      always runs `expand_subcircuits` first, so an unexpanded
//!      instance reaching the flattener is a netlist-graph invariant
//!      violation.
//! 3. Hand `(node_count, branch_count, elements)` to
//!    [`FlattenedStructure::new`] for validation; map any structural
//!    error onto [`FlattenError::Structural`].
//!
//! # Branch-row policy
//!
//! Per ADR-0003 we build the *full* incidence — including the ground
//! node — and let the sub-view extractor (tasks.md item #15) handle
//! ground suppression. The flattener therefore makes **no** attempt to
//! skip elements that connect to ground; every `Element` in the source
//! graph produces exactly one `ElementIncidence`, in element-id order.
//!
//! Branch ids are allocated densely from 0 in the order
//! current-carrying elements are encountered. This means the `BranchId`
//! sequence is a deterministic function of element-insertion order in
//! the builder, which makes downstream conformance comparison against
//! ngspice's matrix layout reproducible.

use crate::flattened::{ElementIncidence, FlattenedStructure, FlattenedStructureError};
use circuit_solver_types::{BranchId, ElementId, NodeId};
use netlist_graph::{CircuitGraph, ElementKind};

/// Errors raised by [`flatten`].
///
/// The flattener is structurally trivial — almost all error cases are
/// invariant violations of either the input `CircuitGraph` (which the
/// builder should have prevented) or of the
/// [`FlattenedStructure::new`] constructor (which validates the
/// produced incidence bundle).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlattenError {
    /// A `SubcircuitInstance` element survived `build()` — the
    /// netlist-graph contract requires subcircuit expansion before a
    /// `CircuitGraph` is handed to the flattener.
    UnexpandedSubcircuit {
        /// The offending element's id.
        element: ElementId,
    },
    /// The `CircuitGraph` exposed more than `u32::MAX` elements — a
    /// structural impossibility but mapped to an explicit variant so
    /// the caller does not see a panic.
    TooManyElements,
    /// The `CircuitGraph` exposed more than `u32::MAX` nodes — same
    /// rationale as [`Self::TooManyElements`].
    TooManyNodes,
    /// [`FlattenedStructure::new`] rejected the assembled bundle. The
    /// flattener never produces these in well-formed input, so seeing
    /// one indicates either a builder regression or a flattener bug.
    Structural(FlattenedStructureError),
}

impl core::fmt::Display for FlattenError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnexpandedSubcircuit { element } => write!(
                f,
                "{element} is an unexpanded SubcircuitInstance — \
                 CircuitGraph must be subcircuit-expanded before \
                 flattening (ADR-0001 / netlist-graph invariant)"
            ),
            Self::TooManyElements => {
                write!(f, "CircuitGraph contains more than u32::MAX elements")
            }
            Self::TooManyNodes => {
                write!(f, "CircuitGraph contains more than u32::MAX nodes")
            }
            Self::Structural(inner) => write!(f, "flattened structure rejected: {inner}"),
        }
    }
}

impl std::error::Error for FlattenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Structural(inner) => Some(inner),
            _ => None,
        }
    }
}

impl From<FlattenedStructureError> for FlattenError {
    fn from(value: FlattenedStructureError) -> Self {
        Self::Structural(value)
    }
}

/// True iff this element kind contributes an MNA branch (current-
/// carrying augmentation row) in v1.
///
/// v1 policy: voltage sources and inductors carry their current as an
/// unknown; resistors, capacitors, and current sources stay in the
/// conductance matrix; semiconductors stamp into the conductance
/// matrix at the linearized step (tasks.md items #9..#13).
fn kind_needs_branch(kind: &ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::VoltageSource { .. } | ElementKind::Inductor { .. }
    )
}

/// Pass-1 structure flattening: read the [`CircuitGraph`] once and
/// produce a [`FlattenedStructure`] with full incidence including the
/// ground node.
///
/// This is the canonical caller of [`FlattenedStructure::new`].
///
/// # Errors
///
/// Returns:
///
/// - [`FlattenError::UnexpandedSubcircuit`] if any element is a
///   [`ElementKind::SubcircuitInstance`]. The builder is contractually
///   required to call `expand_subcircuits` before `build()`; reaching
///   the flattener with an unexpanded instance is a netlist-graph
///   regression.
/// - [`FlattenError::TooManyElements`] / [`FlattenError::TooManyNodes`]
///   if the source graph exposes more than `u32::MAX` of either. These
///   are structurally impossible (every id is a `u32`) but reported
///   explicitly so we never panic.
/// - [`FlattenError::Structural`] wrapping a
///   [`FlattenedStructureError`] if the assembled bundle fails the
///   constructor's invariant checks (an internal bug — the flattener
///   is expected to produce only consistent bundles).
pub fn flatten(graph: &CircuitGraph) -> Result<FlattenedStructure, FlattenError> {
    let node_count: u32 =
        u32::try_from(graph.node_count()).map_err(|_| FlattenError::TooManyNodes)?;

    // Reserve one slot per source element. Branch counter advances
    // lazily as we encounter current-carrying kinds.
    let source_elements = graph.elements();
    let _: u32 = u32::try_from(source_elements.len()).map_err(|_| FlattenError::TooManyElements)?;

    let mut incidences: Vec<ElementIncidence> = Vec::with_capacity(source_elements.len());
    let mut next_branch: u32 = 0;

    for (idx, element) in source_elements.iter().enumerate() {
        // The graph's element-id ordering must match its slice ordering
        // — this is a CircuitGraph invariant — so we use the slice
        // index as the canonical `ElementId`. We do not trust
        // `element.id()` blindly; if the two disagree, the
        // FlattenedStructure constructor will reject it as
        // `ElementIndexMismatch`, which is the right surface.
        let element_id =
            ElementId::new(u32::try_from(idx).map_err(|_| FlattenError::TooManyElements)?);

        let kind = element.kind();

        // Refuse unexpanded subcircuits up front: this is a contract
        // violation of `CircuitBuilder::build()`, not a flattener
        // policy choice.
        if let ElementKind::SubcircuitInstance { .. } = kind {
            return Err(FlattenError::UnexpandedSubcircuit {
                element: element_id,
            });
        }

        let terminals: Vec<NodeId> = element.terminals().to_vec();

        let incidence = if kind_needs_branch(kind) {
            // Allocate a fresh MNA branch row for this current
            // unknown. Two-terminal current-carrying elements
            // (voltage sources, inductors) are the only v1 producers
            // of branches.
            let branch = BranchId::new(next_branch);
            next_branch = next_branch
                .checked_add(1)
                .ok_or(FlattenError::TooManyElements)?;
            // v1's two current-carrying kinds are strictly two-terminal,
            // but defensively use the device constructor for any future
            // kind that returns true from `kind_needs_branch` with more
            // than two terminals.
            if terminals.len() == 2 {
                ElementIncidence::two_terminal_current_carrying(
                    element_id,
                    terminals[0],
                    terminals[1],
                    branch,
                )
            } else {
                ElementIncidence::device(element_id, terminals, Some(branch))
            }
        } else if matches!(kind, ElementKind::Semiconductor) {
            // Semiconductors carry no MNA branch in v1 — diode/BJT/
            // MOSFET stamps all live in the conductance matrix at
            // the linearized step.
            ElementIncidence::device(element_id, terminals, None)
        } else if kind.is_two_terminal() {
            // Resistors, capacitors, current sources: pure conductance
            // contributors. Two terminals, no branch.
            //
            // Two-terminal capacitors are conductive at DC (open
            // circuit, stamped via companion model for transient);
            // they still have no MNA branch row of their own in v1.
            ElementIncidence::two_terminal_conductive(element_id, terminals[0], terminals[1])
        } else {
            // Future multi-terminal non-branch kind — keep the
            // categorization conservative and dispatch through the
            // generic device constructor. The constructor invariants
            // will catch any mismatch downstream.
            ElementIncidence::device(element_id, terminals, None)
        };

        incidences.push(incidence);
    }

    FlattenedStructure::new(node_count, next_branch, incidences).map_err(FlattenError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use netlist_graph::{CircuitBuilder, GROUND_NET};

    // ---------- helpers ----------------------------------------------------

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

    // ---------- ground-only -------------------------------------------------

    #[test]
    fn empty_graph_with_only_ground_flattens_cleanly() {
        // The builder always seeds ground, so even an empty builder
        // produces a graph with one node and zero elements.
        let g = CircuitBuilder::default().build().expect("empty build ok");
        let fs = flatten(&g).expect("ground-only flattens");
        assert_eq!(fs.node_count(), 1);
        assert_eq!(fs.branch_count(), 0);
        assert_eq!(fs.element_count(), 0);
        assert_eq!(fs.ground_node(), NodeId::GROUND);
        assert!(fs.branches_at(NodeId::GROUND).is_empty());
        // No topology report attached — that is the topology checker's
        // job (tasks.md item #4 / ADR-0009).
        assert!(!fs.has_topology_report());
    }

    // ---------- linear resistive DC headline scenario ----------------------
    //
    // The dc-operating-point#linear-resistive-dc-operating-point
    // scenario exercises a voltage divider:
    //
    //     V1 (10 V) from n1 to gnd
    //     R1 (1 kΩ) between n1 and n2
    //     R2 (1 kΩ) between n2 and gnd
    //
    // After flattening:
    //   - 3 nodes (ground + n1 + n2)
    //   - 1 branch (V1's current unknown)
    //   - 3 elements (V1, R1, R2 in insertion order)

    #[test]
    fn linear_resistive_voltage_divider_flattens_to_expected_shape() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", GROUND_NET, 10.0);
        add_resistor(&mut b, "R1", "n1", "n2", 1000.0);
        add_resistor(&mut b, "R2", "n2", GROUND_NET, 1000.0);
        let g = b.build().expect("build voltage divider");

        let fs = flatten(&g).expect("flatten voltage divider");

        // Shape checks: 3 nodes including ground, 1 branch for V1, 3 elements.
        assert_eq!(fs.node_count(), 3);
        assert_eq!(fs.branch_count(), 1);
        assert_eq!(fs.element_count(), 3);

        // Ground bookkeeping: ground is always node 0.
        assert_eq!(fs.ground_node(), NodeId::GROUND);

        // Element 0 is V1 — current-carrying, branch 0.
        let v1 = fs.element(ElementId::new(0)).expect("V1 present");
        assert!(v1.has_branch(), "V1 must own an MNA branch row");
        assert_eq!(v1.branch, Some(BranchId::new(0)));
        assert_eq!(v1.nodes.len(), 2);

        // Element 1 is R1 — purely conductive, no branch.
        let r1 = fs.element(ElementId::new(1)).expect("R1 present");
        assert!(!r1.has_branch(), "R1 must not own a branch row");
        assert_eq!(r1.nodes.len(), 2);

        // Element 2 is R2 — purely conductive, no branch.
        let r2 = fs.element(ElementId::new(2)).expect("R2 present");
        assert!(!r2.has_branch(), "R2 must not own a branch row");

        // Per ADR-0003 the ground node is in the incidence map (full
        // matrix; ground suppression is a sub-view concern). Branch 0
        // touches both ground and n1 because V1 connects them.
        assert_eq!(fs.branches_at(NodeId::GROUND), &[BranchId::new(0)]);
        let n1 = g.node_by_name("n1").expect("n1 resolved").id();
        assert_eq!(fs.branches_at(n1), &[BranchId::new(0)]);
        let n2 = g.node_by_name("n2").expect("n2 resolved").id();
        assert!(
            fs.branches_at(n2).is_empty(),
            "n2 is touched only by purely conductive elements"
        );
    }

    // ---------- ground node is full-incidence (ADR-0003) -------------------

    #[test]
    fn ground_node_appears_in_branch_map_when_branch_touches_it() {
        // Single voltage source from n1 to ground. The ground row of
        // node_to_branches MUST contain branch 0 — ADR-0003.
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", GROUND_NET, 5.0);
        let g = b.build().expect("ok");

        let fs = flatten(&g).expect("flatten ok");

        assert_eq!(fs.branch_count(), 1);
        assert!(
            fs.branches_at(NodeId::GROUND).contains(&BranchId::new(0)),
            "ground must carry its incident branch — ADR-0003 says the full \
             matrix is built; ground suppression is a sub-view concern"
        );
    }

    // ---------- conductive-only circuits have zero branches ----------------

    #[test]
    fn pure_resistor_circuit_has_no_branches() {
        // Two resistors in series — no voltage source, no inductor,
        // therefore no MNA branch rows.
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "n2", 1000.0);
        add_resistor(&mut b, "R2", "n2", GROUND_NET, 1000.0);
        let g = b.build().expect("ok");

        let fs = flatten(&g).expect("flatten ok");
        assert_eq!(fs.branch_count(), 0);
        assert_eq!(fs.element_count(), 2);
        assert!(fs.branches_at(NodeId::GROUND).is_empty());
    }

    // ---------- inductor as second branch owner ---------------------------

    #[test]
    fn inductor_allocates_a_branch_alongside_voltage_source() {
        // V1 + R1 + L1 — exercises the "more than one branch row"
        // path, and verifies branch ids are dense from 0.
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", GROUND_NET, 5.0);
        add_resistor(&mut b, "R1", "n1", "n2", 100.0);
        add_inductor(&mut b, "L1", "n2", GROUND_NET, 1e-3);
        let g = b.build().expect("ok");

        let fs = flatten(&g).expect("flatten ok");
        assert_eq!(
            fs.branch_count(),
            2,
            "voltage source + inductor each own one MNA branch row"
        );
        assert_eq!(fs.element_count(), 3);
        // V1 gets branch 0 (first encountered).
        assert_eq!(
            fs.element(ElementId::new(0)).unwrap().branch,
            Some(BranchId::new(0))
        );
        // R1 gets no branch.
        assert_eq!(fs.element(ElementId::new(1)).unwrap().branch, None);
        // L1 gets branch 1 (second encountered).
        assert_eq!(
            fs.element(ElementId::new(2)).unwrap().branch,
            Some(BranchId::new(1))
        );
    }

    // ---------- current source is conductive (no branch) ------------------

    #[test]
    fn current_source_is_purely_conductive_no_branch() {
        // CurrentSource stamps directly into the RHS vector — no MNA
        // branch row required.
        let mut b = CircuitBuilder::default();
        add_current_source(&mut b, "I1", "n1", GROUND_NET, 1e-3);
        add_resistor(&mut b, "R1", "n1", GROUND_NET, 1000.0);
        let g = b.build().expect("ok");

        let fs = flatten(&g).expect("flatten ok");
        assert_eq!(fs.branch_count(), 0);
        assert_eq!(fs.element_count(), 2);
        let i1 = fs.element(ElementId::new(0)).expect("I1 present");
        assert!(!i1.has_branch(), "current source carries no MNA branch row");
    }

    // ---------- capacitor is conductive at this layer ---------------------

    #[test]
    fn capacitor_has_no_mna_branch_at_flatten_time() {
        // At Pass 1 a capacitor is just a two-terminal element with no
        // companion model yet. The companion stamp (open at DC,
        // Norton-equivalent for transient) is added by Pass 2 / the
        // integration method, not here.
        let mut b = CircuitBuilder::default();
        add_capacitor(&mut b, "C1", "n1", GROUND_NET, 1e-9);
        let g = b.build().expect("ok");

        let fs = flatten(&g).expect("flatten ok");
        assert_eq!(fs.branch_count(), 0);
        let c1 = fs.element(ElementId::new(0)).expect("C1 present");
        assert!(!c1.has_branch());
        assert_eq!(c1.nodes.len(), 2);
    }

    // ---------- semiconductor incidence -----------------------------------

    #[test]
    fn semiconductor_records_all_terminals_no_branch() {
        // Three-terminal Semiconductor (e.g. a BJT instance). v1
        // stamps live in the conductance matrix at the linearized
        // step, so no MNA branch row is required.
        let mut b = CircuitBuilder::default();
        b.add_model(circuit_solver_types::ModelName::new("Q2N2222"));
        b.add_element(
            "Q1",
            ElementKind::Semiconductor,
            ["nc", "nb", GROUND_NET],
            Some(circuit_solver_types::ModelName::new("Q2N2222")),
        )
        .expect("add Q1");
        let g = b.build().expect("ok");

        let fs = flatten(&g).expect("flatten ok");
        assert_eq!(fs.element_count(), 1);
        let q1 = fs.element(ElementId::new(0)).expect("Q1 present");
        assert!(
            !q1.has_branch(),
            "v1 semiconductor stamps are conductance-only"
        );
        assert_eq!(
            q1.nodes.len(),
            3,
            "BJT incidence records all three terminals"
        );
        assert_eq!(fs.branch_count(), 0);
    }

    // ---------- element-id ordering matches insertion ---------------------

    #[test]
    fn element_id_ordering_matches_insertion_order() {
        // Confirms the deterministic-branch-ordering property the
        // golden-reference conformance harness will rely on.
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "a", GROUND_NET, 1.0);
        add_voltage_source(&mut b, "V1", "a", GROUND_NET, 2.0);
        add_resistor(&mut b, "R2", "a", GROUND_NET, 3.0);
        add_voltage_source(&mut b, "V2", "a", GROUND_NET, 4.0);
        let g = b.build().expect("ok");

        let fs = flatten(&g).expect("flatten ok");
        // V1 was the second element added → ElementId(1) → branch 0.
        assert_eq!(
            fs.element(ElementId::new(1)).unwrap().branch,
            Some(BranchId::new(0))
        );
        // V2 was the fourth element added → ElementId(3) → branch 1.
        assert_eq!(
            fs.element(ElementId::new(3)).unwrap().branch,
            Some(BranchId::new(1))
        );
        assert_eq!(fs.branch_count(), 2);
    }

    // ---------- structural invariants exposed through the API -------------

    #[test]
    fn node_and_branch_indexing_remain_independent() {
        // Carries forward the residual risk from t_391b08fc / item #3:
        // BranchId and NodeId must remain semantically distinct even
        // when their numeric indices coincide. After Pass 1, branch 0
        // and ground node both have index 0.
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", GROUND_NET, 5.0);
        let g = b.build().expect("ok");
        let fs = flatten(&g).expect("flatten ok");

        assert_eq!(fs.ground_node().index(), 0);
        let v1 = fs.element(ElementId::new(0)).unwrap();
        assert_eq!(v1.branch.unwrap().index(), 0);
        // Display strings expose the type tag so a human reader can
        // tell them apart.
        assert_eq!(format!("{}", fs.ground_node()), "node:GND");
        assert_eq!(format!("{}", v1.branch.unwrap()), "branch:0");
    }

    // ---------- topology report not attached by Pass 1 --------------------

    #[test]
    fn flatten_does_not_attach_topology_report() {
        // The topology checker (tasks.md item #4 / ADR-0009) is a
        // separate pass that runs after Pass 1. Pass 1 itself never
        // attaches a report.
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", GROUND_NET, 1.0);
        let g = b.build().expect("ok");
        let fs = flatten(&g).expect("flatten ok");
        assert!(!fs.has_topology_report());
        assert!(fs.topology_report().is_none());
    }

    // ---------- error display ---------------------------------------------

    #[test]
    fn flatten_error_display_is_actionable() {
        // We construct the error variants by hand to exercise Display
        // without having to engineer a netlist-graph regression.
        let err = FlattenError::UnexpandedSubcircuit {
            element: ElementId::new(7),
        };
        let s = format!("{err}");
        assert!(s.contains("elem:7"));
        assert!(s.contains("Subcircuit"));

        let err = FlattenError::TooManyElements;
        assert!(format!("{err}").contains("u32::MAX"));

        let err = FlattenError::TooManyNodes;
        assert!(format!("{err}").contains("u32::MAX"));

        let inner = FlattenedStructureError::EmptyNodeSet;
        let err = FlattenError::from(inner.clone());
        assert_eq!(err, FlattenError::Structural(inner));
        assert!(format!("{err}").starts_with("flattened structure rejected:"));
    }
}
