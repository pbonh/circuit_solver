//! The immutable `CircuitGraph` returned by `CircuitBuilder::build()`.
//!
//! Per ADR-0001 a built graph is an opaque immutable handle. This
//! Rust-side type holds the resolved `Node`s, `Element`s, and the
//! model registry; it exposes only read-only query methods. The
//! application-frontend (`PyO3`) crate wraps `CircuitGraph` in
//! `Py<CircuitGraph>`; attempting to mutate it from Python raises
//! `ImmutableHandleError` (covered by the
//! `python-frontend#immutable-circuit-graph-prevents-post-build-mutation` scenario,
//! enabled by a downstream task).
//!
//! Cloning a `CircuitGraph` is cheap relative to building it but is
//! still a deep copy: this is what gives the
//! `python-frontend#builder-isolation-across-multiple-builds`
//! scenario its independence guarantee. Each call to `build()`
//! produces a fresh `CircuitGraph` that does not share storage with
//! previously-built handles.

use crate::element::{Element, ElementKind, ElementName};
use circuit_solver_types::{ElementId, ModelName, NodeId};
use std::collections::HashMap;

/// A single electrical node in the graph. Per the bounded context's
/// ubiquitous language, the reference node is called `Ground` and is
/// always `NodeId::GROUND`.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    id: NodeId,
    name: String,
    is_ground: bool,
}

impl Node {
    /// Construct. Crate-private.
    pub(crate) fn new(id: NodeId, name: String, is_ground: bool) -> Self {
        Self {
            id,
            name,
            is_ground,
        }
    }

    /// The node's stable identifier.
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }

    /// The user-facing net name (e.g. `"n1"`, `"vdd"`, `"0"`).
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// True iff this node is the ground reference.
    #[must_use]
    pub const fn is_ground(&self) -> bool {
        self.is_ground
    }
}

/// The immutable circuit-graph handle produced by
/// `CircuitBuilder::build()`. Read-only; subsequent mutations on the
/// originating builder do not affect previously-built graphs (per
/// ADR-0001).
#[derive(Debug, Clone)]
pub struct CircuitGraph {
    nodes: Vec<Node>,
    elements: Vec<Element>,
    /// Stable lookup: node-name → `NodeId`.
    node_by_name: HashMap<String, NodeId>,
    /// Stable lookup: element-name → `ElementId`.
    element_by_name: HashMap<ElementName, ElementId>,
    /// Registered device-model names (without their physics — the
    /// device-modeling crate resolves these to `DeviceModel`).
    models: Vec<ModelName>,
}

impl CircuitGraph {
    /// Construct. Crate-private; user code obtains a `CircuitGraph`
    /// only via `CircuitBuilder::build()`.
    pub(crate) fn new(
        nodes: Vec<Node>,
        elements: Vec<Element>,
        node_by_name: HashMap<String, NodeId>,
        element_by_name: HashMap<ElementName, ElementId>,
        models: Vec<ModelName>,
    ) -> Self {
        Self {
            nodes,
            elements,
            node_by_name,
            element_by_name,
            models,
        }
    }

    /// Number of electrical nodes in the graph, ground included.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of elements in the graph (after subcircuit expansion).
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    /// Number of device-model definitions registered with the builder.
    #[must_use]
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// All nodes, in `NodeId` order.
    #[must_use]
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// All elements, in `ElementId` order.
    #[must_use]
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    /// Registered model names.
    #[must_use]
    pub fn models(&self) -> &[ModelName] {
        &self.models
    }

    /// Look up a node by its user-facing net name.
    #[must_use]
    pub fn node_by_name(&self, name: &str) -> Option<&Node> {
        let id = self.node_by_name.get(name)?;
        self.nodes.get(id.index() as usize)
    }

    /// Look up an element by its user-facing name (e.g. `"R1"`).
    #[must_use]
    pub fn element_by_name(&self, name: &str) -> Option<&Element> {
        let id = self.element_by_name.get(&ElementName::new(name))?;
        self.elements.get(id.index() as usize)
    }

    /// Look up an element by its `ElementId`.
    #[must_use]
    pub fn element(&self, id: ElementId) -> Option<&Element> {
        self.elements.get(id.index() as usize)
    }

    /// Look up a node by its `NodeId`.
    #[must_use]
    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id.index() as usize)
    }

    /// True iff the graph contains zero elements *of any kind* — used
    /// only for assertions in tests; the topology-checker task (#4)
    /// owns the real emptiness/connectedness invariants.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// True iff every element is a non-subcircuit kind. The builder's
    /// `expand_subcircuits` is guaranteed to have run before `build()`
    /// returns, so this should always be `true` for a graph produced
    /// by `CircuitBuilder::build()`. The accessor exists primarily for
    /// downstream verification.
    #[must_use]
    pub fn is_fully_expanded(&self) -> bool {
        !self
            .elements
            .iter()
            .any(|e| matches!(e.kind(), ElementKind::SubcircuitInstance { .. }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::ElementName;

    #[test]
    fn empty_graph_reports_zero_counts() {
        let g = CircuitGraph::new(
            Vec::new(),
            Vec::new(),
            HashMap::new(),
            HashMap::new(),
            Vec::new(),
        );
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.element_count(), 0);
        assert_eq!(g.model_count(), 0);
        assert!(g.is_empty());
        assert!(g.is_fully_expanded());
    }

    #[test]
    fn lookup_by_name_round_trips() {
        let mut nbm = HashMap::new();
        nbm.insert("n1".to_string(), NodeId::new(1));
        let mut ebm = HashMap::new();
        ebm.insert(ElementName::new("R1"), ElementId::new(0));
        let nodes = vec![
            Node::new(NodeId::GROUND, "0".to_string(), true),
            Node::new(NodeId::new(1), "n1".to_string(), false),
        ];
        let elems = vec![Element::new(
            ElementId::new(0),
            ElementName::new("R1"),
            ElementKind::Resistor {
                resistance_ohms: 1000.0,
            },
            vec![NodeId::GROUND, NodeId::new(1)],
            None,
        )];
        let g = CircuitGraph::new(nodes, elems, nbm, ebm, Vec::new());
        assert_eq!(g.node_by_name("n1").map(Node::id), Some(NodeId::new(1)));
        assert_eq!(
            g.element_by_name("R1").map(|e| e.terminals().len()),
            Some(2)
        );
    }
}
