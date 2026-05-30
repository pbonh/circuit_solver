//! Subcircuit definitions and the deferred-element representation the
//! builder records before `expand_subcircuits` flattens them into the
//! parent `CircuitGraph`.
//!
//! Per the `netlist-graph` bounded context's ubiquitous language a
//! **Subcircuit** is a reusable circuit module with named ports.
//! Instantiation expands into the parent graph: external port names
//! (e.g. `"in"`, `"out"`) bind to caller-supplied net names, internal
//! nets get a mangled prefix to avoid name collisions across siblings.

use crate::builder::{ElementDecl, NetName};
use crate::element::SubcircuitName;

/// A subcircuit definition: a name, an ordered port list, and a list
/// of element-declarations that comprise its body. Bodies are stored
/// in *unbuilt* form (as [`ElementDecl`]) so they can be replayed
/// against the parent net namespace during expansion.
#[derive(Debug, Clone, PartialEq)]
pub struct SubcircuitDefinition {
    name: SubcircuitName,
    ports: Vec<NetName>,
    body: Vec<ElementDecl>,
}

impl SubcircuitDefinition {
    /// Construct a definition from its name, ordered port list, and
    /// body of element declarations.
    #[must_use]
    pub fn new(name: SubcircuitName, ports: Vec<NetName>, body: Vec<ElementDecl>) -> Self {
        Self { name, ports, body }
    }

    /// The subcircuit's name.
    #[must_use]
    pub fn name(&self) -> &SubcircuitName {
        &self.name
    }

    /// The ordered list of external port net-names.
    #[must_use]
    pub fn ports(&self) -> &[NetName] {
        &self.ports
    }

    /// The body of element declarations to be expanded.
    #[must_use]
    pub fn body(&self) -> &[ElementDecl] {
        &self.body
    }
}
