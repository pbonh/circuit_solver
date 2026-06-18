/// Opaque node identifier wrapping a `usize` index.
///
/// Index 0 is always the ground node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        NodeId(value)
    }
}

impl From<NodeId> for usize {
    fn from(id: NodeId) -> Self {
        id.0
    }
}

/// Circuit topology graph.
///
/// Nodes represent electrical nodes (wires/nets).  Node 0 is always the
/// ground node and is seeded automatically when the graph is constructed.
/// Edges represent element connections (added by higher-level code).
#[derive(Debug, Default)]
pub struct CircuitGraph {
    /// Number of nodes, including the implicit ground node.
    node_count: usize,
}

impl CircuitGraph {
    /// Create a new `CircuitGraph` with the ground node (index 0) pre-seeded.
    pub fn new() -> Self {
        CircuitGraph { node_count: 1 }
    }

    /// Return the [`NodeId`] of the ground node (always index 0).
    pub fn ground() -> NodeId {
        NodeId(0)
    }

    /// Add a new node and return its [`NodeId`].
    pub fn add_node(&mut self) -> NodeId {
        let id = NodeId(self.node_count);
        self.node_count += 1;
        id
    }

    /// Total number of nodes in the graph (including ground).
    pub fn node_count(&self) -> usize {
        self.node_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ground_node_is_index_zero() {
        let g = CircuitGraph::new();
        assert_eq!(usize::from(CircuitGraph::ground()), 0);
        // Ground is the only pre-seeded node.
        assert_eq!(g.node_count(), 1);
    }

    #[test]
    fn add_node_increments_index() {
        let mut g = CircuitGraph::new();
        let n1 = g.add_node();
        let n2 = g.add_node();
        assert_eq!(usize::from(n1), 1);
        assert_eq!(usize::from(n2), 2);
        assert_eq!(g.node_count(), 3);
    }

    #[test]
    fn node_id_roundtrip() {
        let id: NodeId = NodeId::from(42usize);
        let raw: usize = id.into();
        assert_eq!(raw, 42);
    }

    #[test]
    fn default_circuit_graph_has_no_nodes() {
        // Default is intentionally zero-initialised (no ground node).
        // Use CircuitGraph::new() for a properly seeded graph.
        let g = CircuitGraph::default();
        assert_eq!(g.node_count(), 0);
    }
}
