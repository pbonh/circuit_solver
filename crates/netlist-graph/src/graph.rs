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
use core::fmt;
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

    /// Return a clone of this graph with one named
    /// [`ElementKind::VoltageSource`]'s `voltage_volts` value
    /// replaced by `new_value`. Every other field — node table,
    /// element table for all other elements, terminal incidences,
    /// model registry, lookup maps — is preserved identically.
    ///
    /// This is the narrow surface used by the DC-sweep control loop
    /// in `analysis-orchestration` (tasks.md #21, scenario
    /// `dc-operating-point#dc-sweep-over-a-voltage-source`): per
    /// sweep point the orchestrator needs a graph that differs from
    /// the user-supplied graph *only* in one voltage source's value.
    /// Re-flattening is unnecessary (topology is unchanged), so
    /// downstream callers can reuse the original
    /// [`FlattenedStructure`](circuit_solver_types::flattened::FlattenedStructure)
    /// across all returned graphs.
    ///
    /// Per ADR-0001 a built graph is an *immutable handle*; this
    /// method preserves that contract by returning a brand-new
    /// `CircuitGraph` rather than mutating `self`. The returned graph
    /// is independent of `self` (no shared storage) and the
    /// `name`-keyed lookup tables continue to resolve identically.
    ///
    /// # Errors
    ///
    /// - [`VoltageSourceOverrideError::UnknownElement`] — no element
    ///   with the given `name` exists in this graph.
    /// - [`VoltageSourceOverrideError::WrongElementKind`] — an
    ///   element with that name exists but is not a
    ///   [`ElementKind::VoltageSource`].
    /// - [`VoltageSourceOverrideError::NonFiniteValue`] — the
    ///   supplied `new_value` is `NaN` or `±∞`. Matches the
    ///   assembler's finite-value precondition for source stamping
    ///   (`numeric_solver::assemble`), so the substitution surfaces
    ///   the bug here rather than deeper in the solver.
    ///
    /// # Panics
    ///
    /// Does not panic in normal operation. The internal `.expect` on
    /// the element lookup is gated by the invariant that
    /// `self.element_by_name` only maps to in-range
    /// `ElementId`s — a violation would indicate corruption of the
    /// graph's internal lookup tables, which the public constructor
    /// (`CircuitBuilder::build`) prevents.
    pub fn with_voltage_source_value(
        &self,
        name: &str,
        new_value: f64,
    ) -> Result<Self, VoltageSourceOverrideError> {
        if !new_value.is_finite() {
            return Err(VoltageSourceOverrideError::NonFiniteValue {
                element: name.to_string(),
                value: new_value,
            });
        }
        let id = match self.element_by_name.get(&ElementName::new(name)) {
            Some(id) => *id,
            None => {
                return Err(VoltageSourceOverrideError::UnknownElement {
                    element: name.to_string(),
                })
            }
        };
        let idx = id.index() as usize;
        let existing = self
            .elements
            .get(idx)
            .expect("element_by_name resolved a valid index");
        match existing.kind() {
            ElementKind::VoltageSource { .. } => {}
            other => {
                return Err(VoltageSourceOverrideError::WrongElementKind {
                    element: name.to_string(),
                    actual_tag: other.tag(),
                })
            }
        }

        // Clone all elements, substituting the one we are overriding.
        let mut new_elements = self.elements.clone();
        new_elements[idx] = Element::new(
            existing.id(),
            existing.name().clone(),
            ElementKind::VoltageSource {
                voltage_volts: new_value,
            },
            existing.terminals().to_vec(),
            existing.model().cloned(),
        );

        Ok(Self {
            nodes: self.nodes.clone(),
            elements: new_elements,
            node_by_name: self.node_by_name.clone(),
            element_by_name: self.element_by_name.clone(),
            models: self.models.clone(),
        })
    }
}

/// Errors raised by
/// [`CircuitGraph::with_voltage_source_value`].
///
/// Per ADR-0010 this type is part of the v1 *unstable* public Rust
/// API surface; variants may be added in a future change.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum VoltageSourceOverrideError {
    /// No element with the supplied name exists in the graph.
    UnknownElement {
        /// The user-supplied element name that was not found.
        element: String,
    },
    /// An element with the supplied name exists, but its kind is
    /// not [`ElementKind::VoltageSource`]. The DC-sweep contract
    /// pins the swept source to a voltage source per scenario
    /// `dc-operating-point#dc-sweep-over-a-voltage-source`;
    /// other-kind sweeps (current source, parameter sweeps) are
    /// out of scope for v1.
    WrongElementKind {
        /// The element's user-supplied name.
        element: String,
        /// Short tag of the element's actual kind
        /// ([`ElementKind::tag`]), e.g. `"R"`, `"I"`, `"DEV"`.
        actual_tag: &'static str,
    },
    /// The supplied replacement value is `NaN` or `±∞`. Mirrors
    /// the assembler's finite-value precondition for source
    /// stamping.
    NonFiniteValue {
        /// The element name whose value would have been replaced.
        element: String,
        /// The non-finite value the caller supplied.
        value: f64,
    },
}

impl fmt::Display for VoltageSourceOverrideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownElement { element } => {
                write!(f, "unknown element: {element}")
            }
            Self::WrongElementKind {
                element,
                actual_tag,
            } => write!(
                f,
                "element {element} is a {actual_tag}, not a VoltageSource (V)"
            ),
            Self::NonFiniteValue { element, value } => {
                write!(f, "non-finite voltage value {value} for element {element}")
            }
        }
    }
}

impl std::error::Error for VoltageSourceOverrideError {}

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

    // ----- with_voltage_source_value -----------------------------------

    /// Build a tiny graph with one voltage source `V1` and one
    /// resistor `R1`, by hand (the builder lives in `builder.rs`;
    /// the unit tests here exercise `CircuitGraph` in isolation).
    fn small_graph_with_vsource(volts: f64) -> CircuitGraph {
        let mut nbm = HashMap::new();
        nbm.insert("0".to_string(), NodeId::GROUND);
        nbm.insert("n1".to_string(), NodeId::new(1));
        let mut ebm = HashMap::new();
        ebm.insert(ElementName::new("V1"), ElementId::new(0));
        ebm.insert(ElementName::new("R1"), ElementId::new(1));
        let nodes = vec![
            Node::new(NodeId::GROUND, "0".to_string(), true),
            Node::new(NodeId::new(1), "n1".to_string(), false),
        ];
        let elems = vec![
            Element::new(
                ElementId::new(0),
                ElementName::new("V1"),
                ElementKind::VoltageSource {
                    voltage_volts: volts,
                },
                vec![NodeId::new(1), NodeId::GROUND],
                None,
            ),
            Element::new(
                ElementId::new(1),
                ElementName::new("R1"),
                ElementKind::Resistor {
                    resistance_ohms: 1_000.0,
                },
                vec![NodeId::new(1), NodeId::GROUND],
                None,
            ),
        ];
        CircuitGraph::new(nodes, elems, nbm, ebm, Vec::new())
    }

    #[test]
    fn with_voltage_source_value_replaces_only_named_source() {
        let g = small_graph_with_vsource(1.0);
        let g2 = g
            .with_voltage_source_value("V1", 4.25)
            .expect("override ok");
        // V1 was replaced.
        match g2.element_by_name("V1").unwrap().kind() {
            ElementKind::VoltageSource { voltage_volts } => {
                assert!((voltage_volts - 4.25).abs() < 1e-15);
            }
            other => panic!("expected VoltageSource, got {other:?}"),
        }
        // R1 was not touched.
        match g2.element_by_name("R1").unwrap().kind() {
            ElementKind::Resistor { resistance_ohms } => {
                assert!((resistance_ohms - 1_000.0).abs() < 1e-15);
            }
            other => panic!("expected Resistor, got {other:?}"),
        }
        // Returned graph is independent: mutating the override
        // result's logical state does not bleed back to `g`.
        match g.element_by_name("V1").unwrap().kind() {
            ElementKind::VoltageSource { voltage_volts } => {
                assert!((voltage_volts - 1.0).abs() < 1e-15);
            }
            other => panic!("expected VoltageSource, got {other:?}"),
        }
    }

    #[test]
    fn with_voltage_source_value_preserves_lookups() {
        let g = small_graph_with_vsource(1.0);
        let g2 = g.with_voltage_source_value("V1", 5.0).expect("ok");
        assert_eq!(g.node_count(), g2.node_count());
        assert_eq!(g.element_count(), g2.element_count());
        assert!(g2.node_by_name("n1").is_some());
        assert!(g2.element_by_name("V1").is_some());
        assert!(g2.element_by_name("R1").is_some());
    }

    #[test]
    fn with_voltage_source_value_unknown_element_errors() {
        let g = small_graph_with_vsource(1.0);
        let err = g
            .with_voltage_source_value("V99", 1.0)
            .expect_err("expected UnknownElement");
        assert_eq!(
            err,
            VoltageSourceOverrideError::UnknownElement {
                element: "V99".to_string()
            }
        );
    }

    #[test]
    fn with_voltage_source_value_wrong_kind_errors() {
        let g = small_graph_with_vsource(1.0);
        let err = g
            .with_voltage_source_value("R1", 1.0)
            .expect_err("expected WrongElementKind");
        match err {
            VoltageSourceOverrideError::WrongElementKind {
                element,
                actual_tag,
            } => {
                assert_eq!(element, "R1");
                assert_eq!(actual_tag, "R");
            }
            other => panic!("expected WrongElementKind, got {other:?}"),
        }
    }

    #[test]
    fn with_voltage_source_value_non_finite_errors() {
        let g = small_graph_with_vsource(1.0);
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let err = g
                .with_voltage_source_value("V1", bad)
                .expect_err("expected NonFiniteValue");
            match err {
                VoltageSourceOverrideError::NonFiniteValue { element, .. } => {
                    assert_eq!(element, "V1");
                }
                other => panic!("expected NonFiniteValue, got {other:?}"),
            }
        }
    }
}
