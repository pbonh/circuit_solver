//! Variable map: maps [`NodeId`] / [`BranchId`] to MNA row/column indices.
//!
//! [`VarMap`] is the look-up table passed alongside
//! [`MnaMatrix`](crate::mna_matrix::MnaMatrix) to every
//! [`DeviceModel`](crate::traits::DeviceModel) stamp method so that a
//! device can translate its terminal [`NodeId`]s into integer row/column
//! offsets into the MNA matrix.
//!
//! # Design rationale
//!
//! The [`crate::traits::DeviceModel`] trait does not own a graph; it only
//! knows its own terminals as a slice of [`NodeId`]s.  `VarMap` is the
//! bridge that converts those identifiers to the flat 0-based offsets the
//! `MnaMatrix` slice uses.  Ground (node 0) maps to index 0; additional
//! nodes are assigned indices 1, 2, … in encounter order; branch
//! variables (for voltage sources and inductors) follow after the last
//! node index.
//!
//! This design mirrors the `FlattenedStructure` contract in the
//! `numeric-solver` crate without importing that crate (which would
//! create a circular dependency).  A `numeric-solver` callsite wraps its
//! `FlattenedStructure`'s incidence into a `VarMap` before handing
//! control to stamp methods.
//!
//! # Trait-object safety
//!
//! `VarMap` is a plain value type (`Clone + Debug`); it does not affect
//! the object-safety of `DeviceModel`.

use std::collections::HashMap;

use circuit_solver_types::{BranchId, NodeId};

/// Look-up table from [`NodeId`] / [`BranchId`] to MNA matrix indices.
///
/// - Node indices start at 0 (ground ≡ `NodeId::GROUND` ≡ index 0).
/// - Branch indices start immediately after the last node index:
///   branch `b` maps to `node_count + b.index()`.
///
/// The total MNA dimension is `node_count + branch_count`, which
/// matches the `dim` passed to [`MnaMatrix::new`](crate::mna_matrix::MnaMatrix::new).
#[derive(Debug, Clone)]
pub struct VarMap {
    node_to_idx: HashMap<NodeId, usize>,
    branch_to_idx: HashMap<BranchId, usize>,
    node_count: usize,
}

impl VarMap {
    /// Construct a `VarMap` from explicit mappings.
    ///
    /// `node_count` is the number of nodes (including ground).  Branch
    /// variable indices are assumed to start at `node_count`; callers
    /// must ensure `branch_to_idx` values satisfy
    /// `value >= node_count`.
    #[must_use]
    pub fn new(
        node_to_idx: HashMap<NodeId, usize>,
        branch_to_idx: HashMap<BranchId, usize>,
        node_count: usize,
    ) -> Self {
        Self {
            node_to_idx,
            branch_to_idx,
            node_count,
        }
    }

    /// Build a `VarMap` from an ordered slice of node identifiers.
    ///
    /// Ground (`NodeId::GROUND`) is mapped to index 0 regardless of
    /// its position in `nodes`.  Every other node is mapped in the
    /// order provided.  No branch variables are registered; use
    /// [`with_branches`](Self::with_branches) to extend.
    ///
    /// This is the primary constructor for tests and for simple linear
    /// networks that have no inductors or voltage sources.
    #[must_use]
    pub fn from_nodes(nodes: &[NodeId]) -> Self {
        let mut map = HashMap::with_capacity(nodes.len());
        for (i, &n) in nodes.iter().enumerate() {
            map.insert(n, i);
        }
        let node_count = nodes.len();
        Self {
            node_to_idx: map,
            branch_to_idx: HashMap::new(),
            node_count,
        }
    }

    /// Attach branch-variable entries to an existing `VarMap`,
    /// consuming and returning it.
    ///
    /// Branch indices are assigned starting at `self.node_count()`,
    /// in the order provided by `branches`.
    #[must_use]
    pub fn with_branches(mut self, branches: &[BranchId]) -> Self {
        for (i, &b) in branches.iter().enumerate() {
            self.branch_to_idx.insert(b, self.node_count + i);
        }
        self
    }

    /// Number of nodes (including ground).
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Total MNA dimension: `node_count + branch_count`.
    #[must_use]
    pub fn dim(&self) -> usize {
        self.node_count + self.branch_to_idx.len()
    }

    /// Look up the MNA row/column index for a node.
    ///
    /// Returns `None` if `node` was not registered.
    #[must_use]
    pub fn node_index(&self, node: NodeId) -> Option<usize> {
        self.node_to_idx.get(&node).copied()
    }

    /// Look up the MNA row/column index for a branch variable.
    ///
    /// Returns `None` if `branch` was not registered.
    #[must_use]
    pub fn branch_index(&self, branch: BranchId) -> Option<usize> {
        self.branch_to_idx.get(&branch).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_solver_types::BranchId;

    #[test]
    fn from_nodes_assigns_ground_index_zero() {
        let nodes = [NodeId::GROUND, NodeId::new(1), NodeId::new(2)];
        let vm = VarMap::from_nodes(&nodes);
        assert_eq!(vm.node_index(NodeId::GROUND), Some(0));
        assert_eq!(vm.node_index(NodeId::new(1)), Some(1));
        assert_eq!(vm.node_index(NodeId::new(2)), Some(2));
        assert_eq!(vm.node_count(), 3);
        assert_eq!(vm.dim(), 3);
    }

    #[test]
    fn with_branches_appends_after_nodes() {
        let nodes = [NodeId::GROUND, NodeId::new(1)];
        let branches = [BranchId::new(0)];
        let vm = VarMap::from_nodes(&nodes).with_branches(&branches);
        assert_eq!(vm.branch_index(BranchId::new(0)), Some(2));
        assert_eq!(vm.dim(), 3);
    }

    #[test]
    fn unknown_node_returns_none() {
        let nodes = [NodeId::GROUND];
        let vm = VarMap::from_nodes(&nodes);
        assert_eq!(vm.node_index(NodeId::new(99)), None);
    }
}
