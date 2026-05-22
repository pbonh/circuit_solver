//! The `CircuitBuilder` — incremental construction API for
//! `CircuitGraph`.
//!
//! The builder is the *only* way to produce a `CircuitGraph` in this
//! crate. Its API mirrors the surface required by the
//! `python-frontend#incremental-circuit-construction-via-builder-api`
//! scenario:
//!
//! - [`CircuitBuilder::add_element`] — record an element instance.
//! - [`CircuitBuilder::add_wire`] — declare two net names refer to
//!   the same electrical node.
//! - [`CircuitBuilder::add_model`] — register a `ModelName` whose
//!   physics is resolved by the device-modeling context.
//! - [`CircuitBuilder::add_subcircuit`] — register a reusable
//!   subcircuit definition.
//! - [`CircuitBuilder::add_subcircuit_instance`] — record an
//!   instantiation of a previously-registered subcircuit.
//! - [`CircuitBuilder::expand_subcircuits`] — explicit pre-build
//!   expansion of all subcircuit instances. `build()` always invokes
//!   this internally; the standalone entry point exists for
//!   inspection in tests.
//! - [`CircuitBuilder::build`] — produce an immutable `CircuitGraph`.
//!
//! # Net names, wires, and union-find
//!
//! The user-facing addressing scheme is *net names* (strings such as
//! `"n1"`, `"vdd"`, `"0"`). Each net name maps to one *electrical
//! node* in the final graph, but a single node may carry several net
//! names — `add_wire("a", "b")` declares `"a"` and `"b"` are aliases
//! for the same node. The builder maintains a simple disjoint-set
//! union-find over net names; `build()` walks each root and assigns
//! a stable [`NodeId`]. The net named `"0"` (SPICE ground) is
//! pinned to [`NodeId::GROUND`].
//!
//! # Builder isolation (ADR-0001)
//!
//! Every call to `build()` produces a fresh `CircuitGraph` by
//! cloning the builder's internal storage. Subsequent mutations on
//! the builder do not propagate to previously-built handles — this
//! enables the
//! `python-frontend#builder-isolation-across-multiple-builds`
//! scenario.

use crate::element::{Element, ElementKind, ElementName, SubcircuitName};
use crate::error::NetlistGraphError;
use crate::graph::{CircuitGraph, Node};
use crate::subcircuit::SubcircuitDefinition;
use circuit_solver_types::{ElementId, ModelName, NodeId};
use std::collections::HashMap;

/// The SPICE ground net name.
pub const GROUND_NET: &str = "0";

/// A user-facing net name (a string identifying an electrical node
/// in the source netlist before the builder assigns `NodeId`s).
pub type NetName = String;

/// A deferred element declaration: what the user passed to
/// `add_element`, recorded in pre-built form so the builder can
/// replay it during subcircuit expansion against a mangled net
/// namespace. Public so subcircuit bodies can carry it across the
/// crate boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDecl {
    /// The user-supplied element name (e.g. `"R1"`).
    pub name: ElementName,
    /// The element kind discriminator and value attributes.
    pub kind: ElementKind,
    /// The ordered list of net-name terminals.
    pub terminals: Vec<NetName>,
    /// Optional model-name reference.
    pub model: Option<ModelName>,
}

/// Incremental circuit-construction API. Once configured, call
/// [`CircuitBuilder::build`] to obtain an immutable `CircuitGraph`.
#[derive(Debug, Default, Clone)]
pub struct CircuitBuilder {
    /// Top-level element declarations in insertion order.
    elements: Vec<ElementDecl>,
    /// Element names already used (for duplicate detection).
    element_names: std::collections::HashSet<ElementName>,
    /// Registered device-model names, in insertion order.
    models: Vec<ModelName>,
    /// Registered subcircuit definitions, keyed by name.
    subcircuits: HashMap<SubcircuitName, SubcircuitDefinition>,
    /// Subcircuit instances at top level: (instance-name, definition,
    /// port-bindings → net-names).
    subcircuit_instances: Vec<SubcircuitInstance>,
    /// Wires declared via `add_wire`: explicit net-name aliases that
    /// must collapse into a single `NodeId`. Stored as raw pairs so
    /// `build()` can run a fresh union-find pass.
    wires: Vec<(NetName, NetName)>,
}

/// A pending subcircuit instantiation. The instance is recorded here
/// and replayed by [`CircuitBuilder::expand_subcircuits`].
#[derive(Debug, Clone, PartialEq)]
struct SubcircuitInstance {
    /// Instance name (e.g. `"X1"`). Used as the prefix for internal
    /// nets after expansion.
    name: ElementName,
    /// Definition this instance points at.
    definition: SubcircuitName,
    /// Per-port net-name bindings, in the same order as the
    /// definition's port list.
    port_bindings: Vec<NetName>,
}

impl CircuitBuilder {
    /// Construct an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a device-model name. Idempotent: registering the same
    /// `ModelName` twice is a no-op (SPICE-style model redefinition is
    /// the device-modeling crate's concern, not ours).
    pub fn add_model(&mut self, model: ModelName) -> &mut Self {
        if !self.models.contains(&model) {
            self.models.push(model);
        }
        self
    }

    /// Register a subcircuit definition. Returns
    /// `Err(DuplicateSubcircuit)` if a definition under the same name
    /// was already registered.
    ///
    /// # Errors
    ///
    /// Returns `NetlistGraphError::DuplicateSubcircuit` when the
    /// definition's name collides with a previously-registered one.
    pub fn add_subcircuit(
        &mut self,
        definition: SubcircuitDefinition,
    ) -> Result<&mut Self, NetlistGraphError> {
        let name = definition.name().clone();
        if self.subcircuits.contains_key(&name) {
            return Err(NetlistGraphError::DuplicateSubcircuit(name));
        }
        self.subcircuits.insert(name, definition);
        Ok(self)
    }

    /// Declare that two net names refer to the same electrical node.
    /// Order does not matter; multiple `add_wire` calls in any order
    /// produce the same final node assignment.
    pub fn add_wire(&mut self, a: impl Into<NetName>, b: impl Into<NetName>) -> &mut Self {
        self.wires.push((a.into(), b.into()));
        self
    }

    /// Add an element. The `terminals` slice is the ordered list of
    /// net names the element connects to.
    ///
    /// # Errors
    ///
    /// - `DuplicateElementName` if `name` is already registered.
    /// - `TerminalArityMismatch` if a two-terminal kind is supplied a
    ///   non-2-terminal list (other arities are validated by
    ///   downstream tasks: the device-modeling stamp generator owns
    ///   the per-device-kind terminal count).
    pub fn add_element(
        &mut self,
        name: impl Into<ElementName>,
        kind: ElementKind,
        terminals: impl IntoIterator<Item = impl Into<NetName>>,
        model: Option<ModelName>,
    ) -> Result<&mut Self, NetlistGraphError> {
        let name = name.into();
        if self.element_names.contains(&name) {
            return Err(NetlistGraphError::DuplicateElementName(name));
        }
        let terminals: Vec<NetName> = terminals.into_iter().map(Into::into).collect();
        if kind.is_two_terminal() && terminals.len() != 2 {
            return Err(NetlistGraphError::TerminalArityMismatch {
                element: name,
                expected: 2,
                actual: terminals.len(),
            });
        }
        self.element_names.insert(name.clone());
        self.elements.push(ElementDecl {
            name,
            kind,
            terminals,
            model,
        });
        Ok(self)
    }

    /// Record an instance of a previously-registered subcircuit.
    /// `port_bindings` are net names in the parent scope, supplied in
    /// the same order as the definition's port list.
    ///
    /// # Errors
    ///
    /// - `DuplicateElementName` if `name` is already used by another
    ///   element or instance at top-level scope.
    /// - `UnknownSubcircuit` if `definition` is not registered.
    /// - `SubcircuitPortArityMismatch` if `port_bindings.len()` does
    ///   not match the definition's port arity.
    pub fn add_subcircuit_instance(
        &mut self,
        name: impl Into<ElementName>,
        definition: impl Into<SubcircuitName>,
        port_bindings: impl IntoIterator<Item = impl Into<NetName>>,
    ) -> Result<&mut Self, NetlistGraphError> {
        let name = name.into();
        let definition = definition.into();
        if self.element_names.contains(&name) {
            return Err(NetlistGraphError::DuplicateElementName(name));
        }
        let Some(def) = self.subcircuits.get(&definition) else {
            return Err(NetlistGraphError::UnknownSubcircuit(definition));
        };
        let port_bindings: Vec<NetName> = port_bindings.into_iter().map(Into::into).collect();
        if port_bindings.len() != def.ports().len() {
            return Err(NetlistGraphError::SubcircuitPortArityMismatch {
                subcircuit: definition,
                expected: def.ports().len(),
                actual: port_bindings.len(),
            });
        }
        self.element_names.insert(name.clone());
        self.subcircuit_instances.push(SubcircuitInstance {
            name,
            definition,
            port_bindings,
        });
        Ok(self)
    }

    /// Explicitly expand all top-level subcircuit instances into a
    /// flat list of element declarations. `build()` calls this
    /// internally, but the standalone entry point is useful for
    /// inspecting intermediate expansion state in tests.
    ///
    /// # Errors
    ///
    /// - `UnknownSubcircuit` / `SubcircuitPortArityMismatch` —
    ///   propagated from `add_subcircuit_instance` invariants if the
    ///   builder somehow recorded a malformed instance (defensive).
    /// - `SubcircuitCycle` — if a definition transitively
    ///   instantiates itself.
    pub fn expand_subcircuits(&mut self) -> Result<&mut Self, NetlistGraphError> {
        // Materialize the recorded instances at the *top* level (the
        // builder's own `elements` Vec). Each replays into top-level
        // element declarations with the instance name as the mangled
        // prefix for internal nets.
        let instances = std::mem::take(&mut self.subcircuit_instances);
        for instance in instances {
            // The instance name was reserved at add-subcircuit-instance
            // time; release it so the recursive expansion of the body
            // can introduce element names that include the instance
            // prefix without colliding with the reservation itself.
            self.element_names.remove(&instance.name);
            self.expand_instance(&instance, &mut Vec::new())?;
        }
        Ok(self)
    }

    /// Recursive expansion helper. `stack` is the chain of definitions
    /// currently being expanded — used to detect cycles.
    fn expand_instance(
        &mut self,
        instance: &SubcircuitInstance,
        stack: &mut Vec<SubcircuitName>,
    ) -> Result<(), NetlistGraphError> {
        let Some(def) = self.subcircuits.get(&instance.definition).cloned() else {
            return Err(NetlistGraphError::UnknownSubcircuit(
                instance.definition.clone(),
            ));
        };
        if stack.contains(&instance.definition) {
            let mut chain = stack.clone();
            chain.push(instance.definition.clone());
            return Err(NetlistGraphError::SubcircuitCycle(chain));
        }
        stack.push(instance.definition.clone());

        // Build the port→parent-net mapping. Internal nets get a
        // mangled prefix derived from the instance name.
        let port_map: HashMap<&str, &str> = def
            .ports()
            .iter()
            .zip(instance.port_bindings.iter())
            .map(|(p, b)| (p.as_str(), b.as_str()))
            .collect();
        let prefix = format!("{}.", instance.name.as_str());

        let body_decls: Vec<ElementDecl> = def.body().to_vec();
        for decl in body_decls {
            let new_name = ElementName::new(format!("{}{}", prefix, decl.name.as_str()));
            if self.element_names.contains(&new_name) {
                return Err(NetlistGraphError::DuplicateElementName(new_name));
            }
            let new_terminals: Vec<NetName> = decl
                .terminals
                .iter()
                .map(|t| {
                    port_map
                        .get(t.as_str())
                        .map_or_else(|| format!("{prefix}{t}"), |bound| (*bound).to_string())
                })
                .collect();

            if let ElementKind::SubcircuitInstance { definition } = &decl.kind {
                // Nested instance: synthesize a SubcircuitInstance
                // record and recurse.
                let nested = SubcircuitInstance {
                    name: new_name,
                    definition: definition.clone(),
                    port_bindings: new_terminals,
                };
                // Validate arity before recursing.
                let Some(nested_def) = self.subcircuits.get(&nested.definition) else {
                    return Err(NetlistGraphError::UnknownSubcircuit(
                        nested.definition.clone(),
                    ));
                };
                if nested.port_bindings.len() != nested_def.ports().len() {
                    return Err(NetlistGraphError::SubcircuitPortArityMismatch {
                        subcircuit: nested.definition.clone(),
                        expected: nested_def.ports().len(),
                        actual: nested.port_bindings.len(),
                    });
                }
                self.expand_instance(&nested, stack)?;
            } else {
                self.element_names.insert(new_name.clone());
                self.elements.push(ElementDecl {
                    name: new_name,
                    kind: decl.kind.clone(),
                    terminals: new_terminals,
                    model: decl.model.clone(),
                });
            }
        }

        stack.pop();
        Ok(())
    }

    /// Finalize the build: expand subcircuits, run union-find over
    /// wires + element terminals to assign stable `NodeId`s, and
    /// return an immutable `CircuitGraph`. The originating builder
    /// remains usable; subsequent mutations do not affect the returned
    /// graph (ADR-0001 / `builder-isolation-across-multiple-builds`).
    ///
    /// # Errors
    ///
    /// Propagates any error from `expand_subcircuits`.
    ///
    /// # Panics
    ///
    /// Panics if the assembled graph would exceed `u32::MAX` nodes or
    /// elements. The `NodeId` / `ElementId` newtypes wrap `u32`, so
    /// this is a hard ceiling of the data model rather than the
    /// builder. Real circuits are nowhere near this scale.
    pub fn build(&mut self) -> Result<CircuitGraph, NetlistGraphError> {
        // Operate on a snapshot so the builder remains unchanged for
        // the user's next `build()` call (apart from the one-time
        // subcircuit expansion, which is monotonic and idempotent
        // because expansion consumes `subcircuit_instances`).
        self.expand_subcircuits()?;

        let mut uf = NetUnionFind::default();

        // Seed: ground net is always present.
        uf.find_or_insert(GROUND_NET);

        // Walk every net mentioned by elements and wires so each gets
        // a representative.
        for decl in &self.elements {
            for net in &decl.terminals {
                uf.find_or_insert(net);
            }
        }
        for (a, b) in &self.wires {
            uf.find_or_insert(a);
            uf.find_or_insert(b);
            uf.union(a, b);
        }

        // Assign NodeIds. Ground always gets NodeId::GROUND.
        let ground_root = uf.find(GROUND_NET).expect("ground seeded");
        let mut root_to_id: HashMap<NetName, NodeId> = HashMap::new();
        let mut nodes: Vec<Node> = Vec::new();
        // Pre-assign ground first.
        root_to_id.insert(ground_root.clone(), NodeId::GROUND);
        nodes.push(Node::new(NodeId::GROUND, GROUND_NET.to_string(), true));

        // Now assign indices to the remaining roots. We need a
        // deterministic order — walk the net names in insertion order
        // and pick representatives.
        let net_insertion_order = uf.insertion_order();
        for net in &net_insertion_order {
            let root = uf.find(net).expect("net was inserted");
            if !root_to_id.contains_key(&root) {
                let id = NodeId::new(u32::try_from(nodes.len()).expect("node count fits u32"));
                root_to_id.insert(root.clone(), id);
                // Use the root's net name as the canonical node name.
                nodes.push(Node::new(id, root, false));
            }
        }

        // Build node_by_name map: every alias points at its root's NodeId.
        let mut node_by_name: HashMap<String, NodeId> = HashMap::new();
        for net in &net_insertion_order {
            let root = uf.find(net).expect("net was inserted");
            let id = root_to_id[&root];
            node_by_name.insert(net.clone(), id);
        }
        // Make sure ground is reachable under its canonical name too.
        node_by_name.insert(GROUND_NET.to_string(), NodeId::GROUND);

        // Resolve elements.
        let mut elements: Vec<Element> = Vec::with_capacity(self.elements.len());
        let mut element_by_name: HashMap<ElementName, ElementId> =
            HashMap::with_capacity(self.elements.len());
        for (idx, decl) in self.elements.iter().enumerate() {
            let element_id = ElementId::new(u32::try_from(idx).expect("element count fits u32"));
            let terminals: Vec<NodeId> = decl
                .terminals
                .iter()
                .map(|net| {
                    let root = uf.find(net).expect("terminal net was inserted");
                    root_to_id[&root]
                })
                .collect();
            elements.push(Element::new(
                element_id,
                decl.name.clone(),
                decl.kind.clone(),
                terminals,
                decl.model.clone(),
            ));
            element_by_name.insert(decl.name.clone(), element_id);
        }

        Ok(CircuitGraph::new(
            nodes,
            elements,
            node_by_name,
            element_by_name,
            self.models.clone(),
        ))
    }

    /// Number of top-level (post-expansion) element declarations
    /// recorded so far. Useful for tests that drive the expansion API
    /// directly.
    #[must_use]
    pub fn element_decl_count(&self) -> usize {
        self.elements.len()
    }
}

/// Disjoint-set union-find over net names. `String`-keyed because the
/// user-facing identifiers are strings; performance is not critical
/// here (the hot path is the solver, not the builder).
#[derive(Debug, Default)]
struct NetUnionFind {
    parent: HashMap<NetName, NetName>,
    insertion_order: Vec<NetName>,
}

impl NetUnionFind {
    fn find_or_insert(&mut self, net: &str) {
        if !self.parent.contains_key(net) {
            self.parent.insert(net.to_string(), net.to_string());
            self.insertion_order.push(net.to_string());
        }
    }

    fn find(&self, net: &str) -> Option<NetName> {
        let mut cur = self.parent.get(net)?.clone();
        loop {
            let parent = self.parent.get(&cur)?.clone();
            if parent == cur {
                return Some(cur);
            }
            cur = parent;
        }
    }

    fn union(&mut self, a: &str, b: &str) {
        let ra = self.find(a).expect("union: a was inserted");
        let rb = self.find(b).expect("union: b was inserted");
        if ra == rb {
            return;
        }
        // Pin ground (`"0"`) as the root regardless of insertion order
        // — otherwise an `add_wire("0", "vdd")` followed by element
        // terminals naming `"vdd"` could elect `"vdd"` as the root
        // when we ask for `find("0")` later.
        if ra == GROUND_NET {
            self.parent.insert(rb, ra);
        } else if rb == GROUND_NET {
            self.parent.insert(ra, rb);
        } else {
            // Pick the lexicographically smaller as the root for
            // determinism. (Insertion-order would also work but
            // lexicographic is independent of the call sequence.)
            if ra < rb {
                self.parent.insert(rb, ra);
            } else {
                self.parent.insert(ra, rb);
            }
        }
    }

    fn insertion_order(&self) -> Vec<NetName> {
        self.insertion_order.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The headline Gherkin scenario: incremental construction via the
    /// builder API.
    ///
    /// ```gherkin
    /// Given PythonDeveloper imports the circuit_solver module
    /// When PythonDeveloper creates a CircuitBuilder and adds a
    ///   resistor "R1" between nodes "n1" and "n2" with value 1 kΩ
    /// And PythonDeveloper adds a voltage source "V1" between nodes
    ///   "n2" and "0" with value 5 V
    /// And PythonDeveloper calls builder.build()
    /// Then the returned object is an immutable CircuitGraph
    /// And the CircuitGraph contains two elements and three nodes
    /// ```
    #[test]
    fn incremental_construction_via_builder_api() {
        let mut b = CircuitBuilder::new();
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: 1_000.0,
            },
            ["n1", "n2"],
            None,
        )
        .expect("add R1");
        b.add_element(
            "V1",
            ElementKind::VoltageSource { voltage_volts: 5.0 },
            ["n2", "0"],
            None,
        )
        .expect("add V1");
        let g = b.build().expect("build");
        // "The CircuitGraph contains two elements and three nodes."
        assert_eq!(g.element_count(), 2, "expected 2 elements (R1, V1)");
        assert_eq!(g.node_count(), 3, "expected 3 nodes (ground/0, n1, n2)");
        // The graph is fully expanded (no subcircuit instances).
        assert!(g.is_fully_expanded());
        // Lookup by user-facing name works.
        assert!(g.node_by_name("0").unwrap().is_ground());
        assert!(g.node_by_name("n1").is_some());
        assert!(g.node_by_name("n2").is_some());
        assert!(g.element_by_name("R1").is_some());
        assert!(g.element_by_name("V1").is_some());
        // R1 and V1 share node n2.
        let r1 = g.element_by_name("R1").unwrap();
        let v1 = g.element_by_name("V1").unwrap();
        let n2 = g.node_by_name("n2").unwrap().id();
        assert!(r1.terminals().contains(&n2));
        assert!(v1.terminals().contains(&n2));
        // V1's second terminal is ground.
        assert!(v1.terminals().contains(&NodeId::GROUND));
    }

    /// Companion `python-frontend#builder-isolation-across-multiple-builds`
    /// scenario. While this scenario lights up in a downstream task,
    /// the *invariant* is owned by `CircuitBuilder::build` and is
    /// asserted here to prevent regression as future tasks add
    /// builder state.
    #[test]
    fn builder_isolation_across_multiple_builds() {
        let mut b = CircuitBuilder::new();
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: 1_000.0,
            },
            ["n1", "0"],
            None,
        )
        .expect("add R1");
        let graph_a = b.build().expect("build a");
        b.add_element(
            "R2",
            ElementKind::Resistor {
                resistance_ohms: 2_000.0,
            },
            ["n2", "0"],
            None,
        )
        .expect("add R2");
        let graph_b = b.build().expect("build b");
        assert_eq!(graph_a.element_count(), 1, "graph_a unchanged by R2");
        assert_eq!(graph_b.element_count(), 2, "graph_b has both elements");
    }

    #[test]
    fn add_wire_unions_nets() {
        let mut b = CircuitBuilder::new();
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: 1.0,
            },
            ["a", "b"],
            None,
        )
        .expect("R1");
        b.add_element(
            "R2",
            ElementKind::Resistor {
                resistance_ohms: 1.0,
            },
            ["c", "0"],
            None,
        )
        .expect("R2");
        b.add_wire("b", "c");
        let g = b.build().expect("build");
        // After wiring b and c we have nodes: ground/0, a, b(=c). 3.
        assert_eq!(g.node_count(), 3);
        // The two resistors share that merged node.
        let r1 = g.element_by_name("R1").unwrap();
        let r2 = g.element_by_name("R2").unwrap();
        let shared = r1.terminals()[1];
        assert_eq!(shared, r2.terminals()[0]);
    }

    #[test]
    fn add_model_is_idempotent_and_recorded() {
        let mut b = CircuitBuilder::new();
        b.add_model(ModelName::new("nmos_lvt"));
        b.add_model(ModelName::new("nmos_lvt")); // duplicate ignored.
        b.add_model(ModelName::new("d1n4148"));
        let g = b.build().expect("build");
        assert_eq!(g.model_count(), 2);
    }

    #[test]
    fn duplicate_element_name_is_rejected() {
        let mut b = CircuitBuilder::new();
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: 1.0,
            },
            ["a", "0"],
            None,
        )
        .expect("first R1");
        let err = b
            .add_element(
                "R1",
                ElementKind::Resistor {
                    resistance_ohms: 2.0,
                },
                ["b", "0"],
                None,
            )
            .expect_err("duplicate must fail");
        assert!(matches!(err, NetlistGraphError::DuplicateElementName(_)));
    }

    #[test]
    fn two_terminal_arity_mismatch_is_rejected() {
        let mut b = CircuitBuilder::new();
        let err = b
            .add_element(
                "R1",
                ElementKind::Resistor {
                    resistance_ohms: 1.0,
                },
                ["a", "b", "c"],
                None,
            )
            .expect_err("3-terminal resistor must fail");
        assert!(matches!(
            err,
            NetlistGraphError::TerminalArityMismatch { .. }
        ));
    }

    /// Subcircuit expansion flattens an inverter-like 2-element body
    /// into the parent graph. We assert the resulting graph has the
    /// element count we expect plus correctly-mangled internal nets.
    #[test]
    fn subcircuit_expansion_flattens_body() {
        let mut b = CircuitBuilder::new();
        // Define DIVIDER with ports (in, out, gnd):
        //   R1: in→mid  R2: mid→gnd, with `out` wired to `mid`.
        let body = vec![
            ElementDecl {
                name: ElementName::new("R1"),
                kind: ElementKind::Resistor {
                    resistance_ohms: 1_000.0,
                },
                terminals: vec!["in".to_string(), "out".to_string()],
                model: None,
            },
            ElementDecl {
                name: ElementName::new("R2"),
                kind: ElementKind::Resistor {
                    resistance_ohms: 1_000.0,
                },
                terminals: vec!["out".to_string(), "gnd".to_string()],
                model: None,
            },
        ];
        b.add_subcircuit(SubcircuitDefinition::new(
            SubcircuitName::new("DIVIDER"),
            vec!["in".to_string(), "out".to_string(), "gnd".to_string()],
            body,
        ))
        .expect("define subckt");

        b.add_element(
            "V1",
            ElementKind::VoltageSource { voltage_volts: 5.0 },
            ["vin", "0"],
            None,
        )
        .expect("V1");
        b.add_subcircuit_instance("X1", "DIVIDER", ["vin", "vmid", "0"])
            .expect("instantiate X1");

        let g = b.build().expect("build");
        assert!(g.is_fully_expanded());
        // V1 + X1's R1 + X1's R2 = 3 elements.
        assert_eq!(g.element_count(), 3);
        // Expanded names are prefixed: X1.R1, X1.R2.
        assert!(g.element_by_name("X1.R1").is_some());
        assert!(g.element_by_name("X1.R2").is_some());
        // The subcircuit's "in" port is bound to the parent net "vin",
        // so X1.R1 terminal 0 must equal V1 terminal 0.
        let v1 = g.element_by_name("V1").unwrap();
        let x1_r1 = g.element_by_name("X1.R1").unwrap();
        assert_eq!(v1.terminals()[0], x1_r1.terminals()[0]);
        // The subcircuit's "gnd" port is bound to "0" → ground.
        let x1_r2 = g.element_by_name("X1.R2").unwrap();
        assert_eq!(x1_r2.terminals()[1], NodeId::GROUND);
    }

    #[test]
    fn unknown_subcircuit_instance_is_rejected() {
        let mut b = CircuitBuilder::new();
        let err = b
            .add_subcircuit_instance("X1", "MISSING", ["a", "b"])
            .expect_err("unknown subckt must fail");
        assert!(matches!(err, NetlistGraphError::UnknownSubcircuit(_)));
    }

    #[test]
    fn subcircuit_port_arity_mismatch_is_rejected() {
        let mut b = CircuitBuilder::new();
        b.add_subcircuit(SubcircuitDefinition::new(
            SubcircuitName::new("INV"),
            vec!["in".to_string(), "out".to_string()],
            Vec::new(),
        ))
        .expect("def");
        let err = b
            .add_subcircuit_instance("X1", "INV", ["a", "b", "c"])
            .expect_err("arity mismatch must fail");
        assert!(matches!(
            err,
            NetlistGraphError::SubcircuitPortArityMismatch { .. }
        ));
    }

    #[test]
    fn duplicate_subcircuit_definition_is_rejected() {
        let mut b = CircuitBuilder::new();
        b.add_subcircuit(SubcircuitDefinition::new(
            SubcircuitName::new("INV"),
            vec!["in".to_string(), "out".to_string()],
            Vec::new(),
        ))
        .expect("first def");
        let err = b
            .add_subcircuit(SubcircuitDefinition::new(
                SubcircuitName::new("INV"),
                vec!["a".to_string(), "b".to_string()],
                Vec::new(),
            ))
            .expect_err("duplicate must fail");
        assert!(matches!(err, NetlistGraphError::DuplicateSubcircuit(_)));
    }

    /// A subcircuit that instantiates itself triggers the cycle
    /// detector. The wiki context explicitly states "Subcircuit
    /// expansion is acyclic"; this is the dynamic enforcement.
    #[test]
    fn subcircuit_cycle_is_detected() {
        let mut b = CircuitBuilder::new();
        // SELF refers to SELF by instantiating itself in its own body.
        let body = vec![ElementDecl {
            name: ElementName::new("X_inner"),
            kind: ElementKind::SubcircuitInstance {
                definition: SubcircuitName::new("SELF"),
            },
            terminals: vec!["p".to_string(), "q".to_string()],
            model: None,
        }];
        b.add_subcircuit(SubcircuitDefinition::new(
            SubcircuitName::new("SELF"),
            vec!["p".to_string(), "q".to_string()],
            body,
        ))
        .expect("def SELF");
        b.add_subcircuit_instance("X1", "SELF", ["a", "b"])
            .expect("instantiate");
        let err = b.build().expect_err("cycle must fail");
        assert!(matches!(err, NetlistGraphError::SubcircuitCycle(_)));
    }

    #[test]
    fn nested_subcircuits_expand_with_mangled_names() {
        let mut b = CircuitBuilder::new();
        // INNER(p, q) := R1 p→q
        b.add_subcircuit(SubcircuitDefinition::new(
            SubcircuitName::new("INNER"),
            vec!["p".to_string(), "q".to_string()],
            vec![ElementDecl {
                name: ElementName::new("R1"),
                kind: ElementKind::Resistor {
                    resistance_ohms: 1.0,
                },
                terminals: vec!["p".to_string(), "q".to_string()],
                model: None,
            }],
        ))
        .expect("def INNER");
        // OUTER(a, b) := XI INNER(a, b)
        b.add_subcircuit(SubcircuitDefinition::new(
            SubcircuitName::new("OUTER"),
            vec!["a".to_string(), "b".to_string()],
            vec![ElementDecl {
                name: ElementName::new("XI"),
                kind: ElementKind::SubcircuitInstance {
                    definition: SubcircuitName::new("INNER"),
                },
                terminals: vec!["a".to_string(), "b".to_string()],
                model: None,
            }],
        ))
        .expect("def OUTER");
        b.add_subcircuit_instance("XO", "OUTER", ["x", "0"])
            .expect("instantiate");
        let g = b.build().expect("build");
        assert!(g.is_fully_expanded());
        // The single resistor reaches the top with name "XO.XI.R1".
        assert!(g.element_by_name("XO.XI.R1").is_some());
        assert_eq!(g.element_count(), 1);
    }
}
