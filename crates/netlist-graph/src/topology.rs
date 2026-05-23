//! Topology checker — Pass-1 floating-node detection (tasks.md item #4, ADR-0009).
//!
//! This module owns the pre-solve graph-connectivity check that
//! [ADR-0009](../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0009-topology-checker-floating-node-detection.md)
//! mandates: every node in a [`FlattenedStructure`] is classified
//! against its DC path to the ground reference, and the result is
//! packaged into a [`TopologyReport`] that the analysis orchestrator
//! consumes before any matrix assembly happens.
//!
//! # Why
//!
//! Floating nodes are the most common cause of DC convergence failure
//! in SPICE simulators. A node with no DC path to ground produces a
//! structurally singular MNA matrix that Newton-Raphson cannot
//! recover from. ADR-0009's chosen approach is **Option C**: detect
//! the condition in Pass 1, before any matrix is built, by traversing
//! the flattened incidence graph.
//!
//! # Three-tier element classification
//!
//! ADR-0009 partitions circuit elements by their *DC* conductivity:
//!
//! - **Always conductive**: resistors, voltage sources (an ideal
//!   source is a low-impedance connection), inductors (DC short).
//! - **Possibly conductive**: diodes, BJTs, MOSFETs. Whether they
//!   conduct at the operating point depends on the bias the solver
//!   discovers — the checker cannot know in advance.
//! - **Never conductive at DC**: capacitors (open at DC), independent
//!   current sources (a current source is an open circuit between
//!   its terminals at DC because it sources/sinks a fixed current
//!   regardless of voltage; from a *connectivity* standpoint it does
//!   not bind its endpoint nodes together).
//!
//! The caller (the Pass-1 builder in tasks.md item #6) supplies one
//! [`ConductivityClass`] per element in `ElementId` order. The
//! checker treats this slice as the source of truth — it does **not**
//! inspect element kinds itself, because
//! [`ElementIncidence`](circuit_solver_types::flattened::ElementIncidence)
//! does not currently carry enough information to distinguish (e.g.) a
//! resistor from a capacitor (both are two-terminal, branchless).
//!
//! # Classification rules (per ADR-0009)
//!
//! 1. **Grounded** — the node sits in the connected component of
//!    `NodeId::GROUND` via *Always*-conductive edges only.
//! 2. **Possibly grounded (warning)** — the node sits in the
//!    connected component of `NodeId::GROUND` via *Always ∪
//!    Possibly*-conductive edges, but **not** through Always edges
//!    alone. The orchestrator should auto-enable Gmin-stepping for
//!    these per ADR-0009's "false-positive mitigation".
//! 3. **Floating** — the node has no DC path to ground through any
//!    Always- or Possibly-conductive element. This is a hard fault.
//!
//! `NodeId::GROUND` itself is, by definition, *grounded* — it is
//! never reported as floating or warning.
//!
//! # Algorithm and complexity
//!
//! Two union-find passes over the element incidence:
//!
//! - Pass A unions every node pair touched by an *Always*-conductive
//!   element. After this pass, every node sharing ground's component
//!   is hard-grounded.
//! - Pass B unions every node pair touched by a *Possibly*-conductive
//!   element on top of Pass A's state. After this pass, every node
//!   sharing ground's component (but not already grounded by Pass A)
//!   is `warning`. Anything still outside is `floating`.
//!
//! Edges contributed by *Never*-conductive elements are not added at
//! all — they cannot carry DC current.
//!
//! Total cost is `O((N + E) α(N))` where `N` is `node_count`, `E` is
//! the total pin count across all elements, and `α` is the inverse
//! Ackermann factor (essentially constant). This matches ADR-0009's
//! "O(N) traversal" budget.
//!
//! # Multi-terminal device wiring (3+ pin elements)
//!
//! For a device with `k ≥ 2` pins (e.g. a BJT with 3 pins, a MOSFET
//! with 4), the checker unions every pin pair. Topologically this
//! treats the device as a clique over its terminals — the most
//! permissive interpretation, consistent with ADR-0009's bias toward
//! warning rather than false-positive error on nonlinear devices.
//!
//! For a two-terminal current-carrying element (voltage source,
//! inductor) the branch row is **not** an additional node — it is an
//! MNA augmentation column and lives in its own namespace per
//! ADR-0003. The checker only unions the two endpoint nodes.
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

use circuit_solver_types::flattened::{FlattenedStructure, TopologyReport};
use circuit_solver_types::NodeId;

/// DC conductivity class of a single circuit element, per ADR-0009.
///
/// The Pass-1 builder (tasks.md item #6) supplies one of these per
/// element in `ElementId` order; the topology checker consumes the
/// slice via [`check_topology`].
///
/// See the module-level documentation for the exact rules; a brief
/// summary:
///
/// - [`ConductivityClass::Always`] for elements that conduct at DC
///   regardless of bias (resistors, voltage sources, inductors).
/// - [`ConductivityClass::Possibly`] for elements whose conductance
///   depends on the bias the solver finds (diodes, BJTs, MOSFETs).
/// - [`ConductivityClass::Never`] for elements that are open circuits
///   at DC (capacitors, independent current sources).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConductivityClass {
    /// Always conductive at DC — contributes a hard-grounded edge.
    Always,
    /// Conductive only when the device is biased on at the operating
    /// point. Contributes a *warning* edge: the orchestrator should
    /// auto-enable Gmin-stepping if a node is grounded only through
    /// such edges.
    Possibly,
    /// Open circuit at DC — does not contribute to graph connectivity
    /// at the operating point.
    Never,
}

/// Error returned by [`check_topology`] when the inputs are
/// inconsistent.
///
/// The checker performs a single up-front validation step: the length
/// of the `classes` slice must equal the `FlattenedStructure`'s
/// element count, because the slice is indexed by `ElementId::index()`
/// 1:1. Any other invariants (node-id range, branch ownership) are
/// already enforced by [`FlattenedStructure::new`] and are not
/// re-checked here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyCheckError {
    /// The caller-supplied conductivity-class slice has a different
    /// length than `flattened.element_count()`.
    ClassLengthMismatch {
        /// The expected length (`flattened.element_count()`).
        expected: u32,
        /// The actual length of the caller's slice.
        got: u32,
    },
}

impl core::fmt::Display for TopologyCheckError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ClassLengthMismatch { expected, got } => write!(
                f,
                "topology checker expected {expected} conductivity classes \
                 (one per element), got {got}"
            ),
        }
    }
}

impl std::error::Error for TopologyCheckError {}

/// Run the topology checker against a flattened structure.
///
/// `classes[i]` is the [`ConductivityClass`] for the element whose
/// `ElementId` is `ElementId::new(i as u32)`. The slice length must
/// equal `flattened.element_count()` — see [`TopologyCheckError`].
///
/// Returns a [`TopologyReport`] suitable to attach via
/// [`FlattenedStructure::set_topology_report`]. The returned report
/// contains:
///
/// - `floating`: nodes with no DC path to ground through any
///   `Always`- or `Possibly`-conductive element.
/// - `warning`: nodes grounded only through `Possibly`-conductive
///   elements (and therefore eligible for Gmin-stepping per
///   ADR-0009).
///
/// `NodeId::GROUND` is never reported (it is trivially grounded).
///
/// # Errors
///
/// Returns [`TopologyCheckError::ClassLengthMismatch`] if the
/// `classes` slice does not have exactly `flattened.element_count()`
/// entries.
///
/// # Examples
///
/// A pure voltage-divider (V + 2 R, all *Always* conductive) reports
/// a clean topology:
///
/// ```
/// use circuit_solver_types::{BranchId, ElementId, NodeId};
/// use circuit_solver_types::flattened::{ElementIncidence, FlattenedStructure};
/// use netlist_graph::topology::{check_topology, ConductivityClass};
///
/// let v1 = ElementIncidence::two_terminal_current_carrying(
///     ElementId::new(0), NodeId::new(1), NodeId::GROUND, BranchId::new(0));
/// let r1 = ElementIncidence::two_terminal_conductive(
///     ElementId::new(1), NodeId::new(1), NodeId::new(2));
/// let r2 = ElementIncidence::two_terminal_conductive(
///     ElementId::new(2), NodeId::new(2), NodeId::GROUND);
///
/// let fs = FlattenedStructure::new(3, 1, vec![v1, r1, r2]).unwrap();
/// let classes = [
///     ConductivityClass::Always, // V1
///     ConductivityClass::Always, // R1
///     ConductivityClass::Always, // R2
/// ];
/// let report = check_topology(&fs, &classes).unwrap();
/// assert!(report.is_clean());
/// ```
pub fn check_topology(
    flattened: &FlattenedStructure,
    classes: &[ConductivityClass],
) -> Result<TopologyReport, TopologyCheckError> {
    let expected = flattened.element_count();
    let got = u32::try_from(classes.len()).unwrap_or(u32::MAX);
    if got != expected {
        return Err(TopologyCheckError::ClassLengthMismatch { expected, got });
    }

    let node_count = flattened.node_count();

    // Pass A: union by Always-conductive elements only.
    let mut always_uf = UnionFind::new(node_count);
    for inc in flattened.elements() {
        let class = classes[inc.element.index() as usize];
        if class == ConductivityClass::Always {
            union_clique(&mut always_uf, &inc.nodes);
        }
    }

    // Pass B: start fresh, union by Always ∪ Possibly.
    let mut combined_uf = UnionFind::new(node_count);
    for inc in flattened.elements() {
        let class = classes[inc.element.index() as usize];
        if class == ConductivityClass::Always || class == ConductivityClass::Possibly {
            union_clique(&mut combined_uf, &inc.nodes);
        }
    }

    let ground = NodeId::GROUND.index();
    let mut floating = Vec::new();
    let mut warning = Vec::new();

    // Node 0 (== ground) is skipped from reporting: it is grounded by
    // definition. Iterate 1..node_count and classify each node.
    for raw in 1..node_count {
        let grounded_always = always_uf.same_component(raw, ground);
        let grounded_combined = combined_uf.same_component(raw, ground);

        match (grounded_always, grounded_combined) {
            (true, _) => {
                // Hard-grounded via Always edges — clean.
            }
            (false, true) => {
                // Reachable only through Possibly edges — warning.
                warning.push(NodeId::new(raw));
            }
            (false, false) => {
                // No path through any conductive element — floating.
                floating.push(NodeId::new(raw));
            }
        }
    }

    Ok(TopologyReport { floating, warning })
}

/// Helper: union every pair of nodes in a multi-terminal element's
/// pin list.
///
/// For a 2-pin element this is a single union; for a 3-pin device a
/// 3-clique (3 unions); for a 4-pin device a 4-clique (6 unions).
/// All v1 device families top out at 4 pins, so the inner cost is
/// bounded by a constant.
///
/// Out-of-range node ids are silently skipped — `FlattenedStructure`
/// validates ranges at construction time, so this is a defensive
/// no-op rather than a load-bearing check.
fn union_clique(uf: &mut UnionFind, nodes: &[NodeId]) {
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let a = nodes[i].index();
            let b = nodes[j].index();
            uf.union(a, b);
        }
    }
}

// ----------------------------------------------------------------------
// Union-find with path compression + union-by-rank.
//
// Internal implementation detail. Kept in this module to avoid an
// inter-crate exposure of a generic data structure; the checker is the
// only intended consumer.

/// Disjoint-set union-find over a fixed integer universe `0..n`.
///
/// Uses path compression (in `find_mut`) and union by rank. Operations
/// are effectively constant time for the workloads the topology
/// checker sees (`n` bounded by `u32::MAX`, in practice the netlist
/// node count).
struct UnionFind {
    parent: Vec<u32>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: u32) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0u8; n as usize];
        Self { parent, rank }
    }

    fn find_mut(&mut self, mut x: u32) -> u32 {
        // Iterative path compression: walk to root, then re-walk
        // pointing every node directly at the root.
        let mut root = x;
        while self.parent[root as usize] != root {
            root = self.parent[root as usize];
        }
        while self.parent[x as usize] != root {
            let next = self.parent[x as usize];
            self.parent[x as usize] = root;
            x = next;
        }
        root
    }

    fn union(&mut self, a: u32, b: u32) {
        let n = u32::try_from(self.parent.len()).unwrap_or(u32::MAX);
        if a >= n || b >= n {
            return;
        }
        let ra = self.find_mut(a);
        let rb = self.find_mut(b);
        if ra == rb {
            return;
        }
        let rank_a = self.rank[ra as usize];
        let rank_b = self.rank[rb as usize];
        match rank_a.cmp(&rank_b) {
            core::cmp::Ordering::Less => self.parent[ra as usize] = rb,
            core::cmp::Ordering::Greater => self.parent[rb as usize] = ra,
            core::cmp::Ordering::Equal => {
                self.parent[rb as usize] = ra;
                self.rank[ra as usize] = rank_a.saturating_add(1);
            }
        }
    }

    fn same_component(&mut self, a: u32, b: u32) -> bool {
        let n = u32::try_from(self.parent.len()).unwrap_or(u32::MAX);
        if a >= n || b >= n {
            return false;
        }
        self.find_mut(a) == self.find_mut(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_solver_types::flattened::ElementIncidence;
    use circuit_solver_types::{BranchId, ElementId};

    // ---------- helpers ----------------------------------------------------

    fn elem(i: u32) -> ElementId {
        ElementId::new(i)
    }
    fn node(i: u32) -> NodeId {
        NodeId::new(i)
    }
    fn branch(i: u32) -> BranchId {
        BranchId::new(i)
    }

    // ---------- input validation -------------------------------------------

    #[test]
    fn class_length_mismatch_is_rejected() {
        // 2-node ground-only circuit (no elements).
        let fs = FlattenedStructure::new(2, 0, vec![]).expect("ok");
        let err = check_topology(&fs, &[ConductivityClass::Always]).unwrap_err();
        assert_eq!(
            err,
            TopologyCheckError::ClassLengthMismatch {
                expected: 0,
                got: 1,
            }
        );
    }

    #[test]
    fn class_length_match_with_zero_elements_is_accepted() {
        // 3 nodes, no elements — nodes 1 and 2 are floating.
        let fs = FlattenedStructure::new(3, 0, vec![]).expect("ok");
        let report = check_topology(&fs, &[]).expect("zero-classes for zero-elements ok");
        // Ground is never reported; both other nodes are floating
        // because there are no edges at all.
        assert_eq!(report.floating, vec![node(1), node(2)]);
        assert!(report.warning.is_empty());
        assert!(!report.is_clean());
    }

    #[test]
    fn error_display_is_actionable() {
        let fs = FlattenedStructure::new(2, 0, vec![]).expect("ok");
        let err = check_topology(&fs, &[ConductivityClass::Always]).unwrap_err();
        let s = format!("{err}");
        assert!(s.contains("expected 0"));
        assert!(s.contains("got 1"));
    }

    // ---------- ground-only and trivial cases ------------------------------

    #[test]
    fn ground_only_circuit_is_clean() {
        // The minimum-legal FlattenedStructure: just ground.
        let fs = FlattenedStructure::new(1, 0, vec![]).expect("ok");
        let report = check_topology(&fs, &[]).expect("ok");
        assert!(report.is_clean());
        assert!(report.floating.is_empty());
        assert!(report.warning.is_empty());
    }

    #[test]
    fn ground_node_is_never_reported_even_when_isolated() {
        // Ground stays out of both `floating` and `warning` lists by
        // construction, even when no element touches it. Other nodes
        // float.
        let fs = FlattenedStructure::new(4, 0, vec![]).expect("ok");
        let report = check_topology(&fs, &[]).expect("ok");
        for n in &report.floating {
            assert_ne!(*n, NodeId::GROUND);
        }
        for n in &report.warning {
            assert_ne!(*n, NodeId::GROUND);
        }
    }

    // ---------- hard-grounded via Always edges -----------------------------

    #[test]
    fn linear_resistive_voltage_divider_is_clean() {
        // The scenario the integrator merged: V1 + R1 + R2.
        // Every element is Always-conductive → every node is hard-grounded.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(1), node(0), branch(0));
        let r1 = ElementIncidence::two_terminal_conductive(elem(1), node(1), node(2));
        let r2 = ElementIncidence::two_terminal_conductive(elem(2), node(2), node(0));

        let fs = FlattenedStructure::new(3, 1, vec![v1, r1, r2]).expect("ok");
        let classes = [
            ConductivityClass::Always,
            ConductivityClass::Always,
            ConductivityClass::Always,
        ];
        let report = check_topology(&fs, &classes).expect("ok");

        assert!(
            report.is_clean(),
            "voltage divider should be clean, got {report:?}"
        );
    }

    #[test]
    fn voltage_source_alone_grounds_its_endpoint() {
        // Just V1 from node 1 to ground. Node 1 is hard-grounded.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(1), node(0), branch(0));
        let fs = FlattenedStructure::new(2, 1, vec![v1]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Always]).expect("ok");
        assert!(report.is_clean());
    }

    #[test]
    fn resistor_chain_propagates_grounding() {
        // R1: gnd—n1, R2: n1—n2, R3: n2—n3. All Always-conductive.
        // Every interior node is hard-grounded through the chain.
        let r1 = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(1));
        let r2 = ElementIncidence::two_terminal_conductive(elem(1), node(1), node(2));
        let r3 = ElementIncidence::two_terminal_conductive(elem(2), node(2), node(3));
        let fs = FlattenedStructure::new(4, 0, vec![r1, r2, r3]).expect("ok");
        let report = check_topology(
            &fs,
            &[
                ConductivityClass::Always,
                ConductivityClass::Always,
                ConductivityClass::Always,
            ],
        )
        .expect("ok");
        assert!(report.is_clean());
    }

    // ---------- floating-node detection ------------------------------------

    #[test]
    fn isolated_node_is_floating() {
        // V1: gnd—n1. Node 2 has no edges at all → floating.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(1), node(0), branch(0));
        let fs = FlattenedStructure::new(3, 1, vec![v1]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Always]).expect("ok");
        assert_eq!(report.floating, vec![node(2)]);
        assert!(report.warning.is_empty());
        assert!(report.has_floating());
    }

    #[test]
    fn capacitor_only_path_to_ground_is_floating() {
        // ADR-0009 false-positive mitigation absent: a capacitor is
        // Never-conductive at DC. A node grounded *only* through a
        // capacitor must be reported as floating, not warning.
        let c1 = ElementIncidence::two_terminal_conductive(elem(0), node(1), node(0));
        let fs = FlattenedStructure::new(2, 0, vec![c1]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Never]).expect("ok");
        assert_eq!(report.floating, vec![node(1)]);
        assert!(report.warning.is_empty());
    }

    #[test]
    fn current_source_only_path_to_ground_is_floating() {
        // Independent current source: open at DC for connectivity
        // purposes. Its endpoint nodes are not bound by it.
        let i1 = ElementIncidence::two_terminal_conductive(elem(0), node(1), node(0));
        let fs = FlattenedStructure::new(2, 0, vec![i1]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Never]).expect("ok");
        assert_eq!(report.floating, vec![node(1)]);
    }

    #[test]
    fn disconnected_subgraph_is_floating() {
        // Two islands: (gnd-n1 via R1) and (n2-n3 via R2). Nodes 2
        // and 3 form their own island, untouched by ground.
        let r1 = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(1));
        let r2 = ElementIncidence::two_terminal_conductive(elem(1), node(2), node(3));
        let fs = FlattenedStructure::new(4, 0, vec![r1, r2]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Always, ConductivityClass::Always])
            .expect("ok");

        // Node 1 is hard-grounded.
        assert!(!report.floating.contains(&node(1)));
        // Nodes 2 and 3 sit in their own island — both floating.
        let mut floating_set = report.floating.clone();
        floating_set.sort_unstable_by_key(|n| n.index());
        assert_eq!(floating_set, vec![node(2), node(3)]);
        assert!(report.warning.is_empty());
    }

    // ---------- warning (possibly-grounded) classification ----------------

    #[test]
    fn possibly_conductive_only_path_is_warning() {
        // The diode-only path: D1 between node 1 and ground, no
        // Always-conductive element. ADR-0009 mandates this is a
        // warning (eligible for Gmin-stepping), not a hard fault.
        let d1 = ElementIncidence::device(elem(0), vec![node(1), node(0)], None);
        let fs = FlattenedStructure::new(2, 0, vec![d1]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Possibly]).expect("ok");

        assert!(!report.has_floating());
        assert!(report.has_warning());
        assert_eq!(report.warning, vec![node(1)]);
    }

    #[test]
    fn always_path_overrides_possibly_path() {
        // Node 1 reaches ground both through D1 (possibly) AND R1
        // (always). The Always path wins → clean.
        let d1 = ElementIncidence::device(elem(0), vec![node(1), node(0)], None);
        let r1 = ElementIncidence::two_terminal_conductive(elem(1), node(1), node(0));
        let fs = FlattenedStructure::new(2, 0, vec![d1, r1]).expect("ok");
        let report = check_topology(
            &fs,
            &[ConductivityClass::Possibly, ConductivityClass::Always],
        )
        .expect("ok");
        assert!(report.is_clean());
    }

    #[test]
    fn warning_propagates_through_possibly_chain() {
        // gnd —D1— n1 —D2— n2. Both Possibly. Both n1 and n2 are warning.
        let d1 = ElementIncidence::device(elem(0), vec![node(0), node(1)], None);
        let d2 = ElementIncidence::device(elem(1), vec![node(1), node(2)], None);
        let fs = FlattenedStructure::new(3, 0, vec![d1, d2]).expect("ok");
        let report = check_topology(
            &fs,
            &[ConductivityClass::Possibly, ConductivityClass::Possibly],
        )
        .expect("ok");
        let mut warning = report.warning.clone();
        warning.sort_unstable_by_key(|n| n.index());
        assert_eq!(warning, vec![node(1), node(2)]);
        assert!(report.floating.is_empty());
    }

    #[test]
    fn always_then_possibly_chain_classifies_correctly() {
        // gnd —R1— n1 —D1— n2.
        // n1: hard-grounded (Always edge to ground).
        // n2: only reachable through D1 (Possibly), then Always to
        //     ground — so the *combined* path includes a Possibly
        //     hop, but the *Always-only* component does not contain
        //     n2. n2 must be reported as warning.
        let r1 = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(1));
        let d1 = ElementIncidence::device(elem(1), vec![node(1), node(2)], None);
        let fs = FlattenedStructure::new(3, 0, vec![r1, d1]).expect("ok");
        let report = check_topology(
            &fs,
            &[ConductivityClass::Always, ConductivityClass::Possibly],
        )
        .expect("ok");

        assert!(!report.floating.contains(&node(1)));
        assert!(!report.warning.contains(&node(1)));
        assert_eq!(report.warning, vec![node(2)]);
        assert!(report.floating.is_empty());
    }

    // ---------- multi-terminal devices -------------------------------------

    #[test]
    fn three_terminal_bjt_unions_pin_clique() {
        // BJT Q1 collector=n1, base=gnd, emitter=n2 (Possibly).
        // All three pins are unioned: n1 and n2 reach ground through
        // the device → both are warning.
        let q1 = ElementIncidence::device(elem(0), vec![node(1), node(0), node(2)], None);
        let fs = FlattenedStructure::new(3, 0, vec![q1]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Possibly]).expect("ok");

        let mut warning = report.warning.clone();
        warning.sort_unstable_by_key(|n| n.index());
        assert_eq!(warning, vec![node(1), node(2)]);
        assert!(report.floating.is_empty());
    }

    #[test]
    fn four_terminal_mosfet_unions_pin_clique() {
        // MOSFET M1 [drain=n1, gate=n2, source=gnd, body=gnd]. With
        // Possibly conductivity, all four pins union into ground's
        // component → both n1 and n2 are warning.
        let m1 = ElementIncidence::device(elem(0), vec![node(1), node(2), node(0), node(0)], None);
        let fs = FlattenedStructure::new(3, 0, vec![m1]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Possibly]).expect("ok");

        let mut warning = report.warning.clone();
        warning.sort_unstable_by_key(|n| n.index());
        assert_eq!(warning, vec![node(1), node(2)]);
    }

    // ---------- mixed scenarios --------------------------------------------

    #[test]
    fn mixed_report_lists_floating_and_warning_separately() {
        // Compound circuit:
        //   R1: gnd—n1 (Always)        → n1 hard-grounded
        //   D1: n1—n2 (Possibly)       → n2 warning
        //   C1: gnd—n3 (Never)         → n3 floating
        //   (n4 has no edges)          → n4 floating
        let r1 = ElementIncidence::two_terminal_conductive(elem(0), node(0), node(1));
        let d1 = ElementIncidence::device(elem(1), vec![node(1), node(2)], None);
        let c1 = ElementIncidence::two_terminal_conductive(elem(2), node(0), node(3));
        let fs = FlattenedStructure::new(5, 0, vec![r1, d1, c1]).expect("ok");
        let report = check_topology(
            &fs,
            &[
                ConductivityClass::Always,
                ConductivityClass::Possibly,
                ConductivityClass::Never,
            ],
        )
        .expect("ok");

        assert_eq!(report.warning, vec![node(2)]);
        let mut floating = report.floating.clone();
        floating.sort_unstable_by_key(|n| n.index());
        assert_eq!(floating, vec![node(3), node(4)]);
        assert!(report.has_floating());
        assert!(report.has_warning());
        assert!(!report.is_clean());
    }

    #[test]
    fn floating_nodes_are_reported_in_node_id_order() {
        // The checker walks nodes in index order; consumers should
        // expect a deterministic ordering for diagnostics.
        let fs = FlattenedStructure::new(5, 0, vec![]).expect("ok");
        let report = check_topology(&fs, &[]).expect("ok");
        assert_eq!(
            report.floating,
            vec![node(1), node(2), node(3), node(4)],
            "floating list must be sorted by NodeId for deterministic diagnostics"
        );
    }

    #[test]
    fn report_attaches_to_flattened_structure() {
        // Round-trip: run the checker, attach via the existing
        // setter, verify queryability.
        let v1 =
            ElementIncidence::two_terminal_current_carrying(elem(0), node(1), node(0), branch(0));
        let mut fs = FlattenedStructure::new(2, 1, vec![v1]).expect("ok");
        let report = check_topology(&fs, &[ConductivityClass::Always]).expect("ok");
        fs.set_topology_report(report.clone());

        assert!(fs.has_topology_report());
        assert_eq!(fs.topology_report(), Some(&report));
        assert!(report.is_clean());
    }

    // ---------- union-find sanity (internal) -------------------------------

    #[test]
    fn unionfind_basic_operations() {
        let mut uf = UnionFind::new(5);
        assert!(!uf.same_component(0, 1));
        uf.union(0, 1);
        assert!(uf.same_component(0, 1));
        uf.union(2, 3);
        uf.union(3, 4);
        assert!(uf.same_component(2, 4));
        assert!(!uf.same_component(1, 4));
        uf.union(1, 2);
        assert!(uf.same_component(0, 4));
    }

    #[test]
    fn unionfind_out_of_range_is_silent_noop() {
        let mut uf = UnionFind::new(3);
        // Should not panic.
        uf.union(0, 99);
        uf.union(99, 100);
        assert!(!uf.same_component(0, 99));
    }
}
