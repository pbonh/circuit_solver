//! Variable map: bidirectional mapping from symbolic net/branch names to MNA
//! matrix row/column indices.
//!
//! Index 0 is always reserved for the ground node.  Net names are assigned
//! indices in encounter order after ground.  Branch-current variables (one
//! per voltage source or inductor) are appended after the last net index.
//!
//! Intended usage pattern:
//! 1. Call [`VarMap::add_node`] for every net in the netlist.
//! 2. Call [`VarMap::add_branch`] for every V-source / inductor.
//! 3. Use [`VarMap::node_index`] / [`VarMap::var_name`] during matrix stamping
//!    and result extraction.
//!
//! Adding nodes after branches is allowed but appends the node *before* the
//! first branch variable, shifting subsequent branch indices.  For stable
//! indices, add all nodes before any branches.

use std::collections::HashMap;

/// Bidirectional map from symbolic names to MNA matrix indices.
///
/// Index layout:
/// - `0`                — ground node (always)
/// - `1 ..= node_count-1` — nets in encounter order
/// - `node_count ..`    — branch-current variables in encounter order
#[derive(Debug, Default)]
pub struct VarMap {
    /// name → index
    name_to_idx: HashMap<String, usize>,
    /// index → name  (dense Vec; position == index)
    idx_to_name: Vec<String>,
    /// Number of node variables (including ground at index 0).
    node_count: usize,
}

impl VarMap {
    /// Create a new `VarMap` with the ground node pre-seeded at index 0.
    pub fn new() -> Self {
        let ground = "0".to_string();
        let mut name_to_idx = HashMap::new();
        name_to_idx.insert(ground.clone(), 0usize);
        VarMap {
            name_to_idx,
            idx_to_name: vec![ground],
            node_count: 1,
        }
    }

    /// Register a net node by name.  If already present the existing index is
    /// returned unchanged (idempotent).
    ///
    /// If branch variables have already been added, they are shifted up by one
    /// so that all nodes remain contiguous before all branches.  For stable
    /// indices, add all nodes before any branches.
    ///
    /// Returns the index assigned to this net.
    pub fn add_node(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.name_to_idx.get(name) {
            return idx;
        }
        let new_idx = self.node_count;
        let total = self.idx_to_name.len();
        if total > self.node_count {
            // Branches already exist — shift them to maintain the invariant
            // that all nodes precede all branches.
            for i in self.node_count..total {
                let bname = self.idx_to_name[i].clone();
                *self.name_to_idx.get_mut(&bname).unwrap() += 1;
            }
            self.idx_to_name.insert(new_idx, name.to_string());
        } else {
            self.idx_to_name.push(name.to_string());
        }
        self.name_to_idx.insert(name.to_string(), new_idx);
        self.node_count += 1;
        new_idx
    }

    /// Register a branch-current variable by name (e.g. `"V1"` for voltage
    /// source V1, `"L1"` for inductor L1).  Branch variables are always
    /// appended after all node indices.
    ///
    /// If already present, the existing index is returned unchanged
    /// (idempotent).
    ///
    /// Returns the index assigned to this branch variable.
    pub fn add_branch(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.name_to_idx.get(name) {
            return idx;
        }
        let new_idx = self.idx_to_name.len();
        self.idx_to_name.push(name.to_string());
        self.name_to_idx.insert(name.to_string(), new_idx);
        new_idx
    }

    /// Look up the MNA index for a symbolic name (net or branch variable).
    pub fn node_index(&self, name: &str) -> Option<usize> {
        self.name_to_idx.get(name).copied()
    }

    /// Look up the symbolic name for an MNA index.
    pub fn var_name(&self, index: usize) -> Option<&str> {
        self.idx_to_name.get(index).map(String::as_str)
    }

    /// Total number of variables (nodes + branch currents).
    pub fn len(&self) -> usize {
        self.idx_to_name.len()
    }

    /// Returns `true` if no variables beyond ground have been registered.
    pub fn is_empty(&self) -> bool {
        self.idx_to_name.len() <= 1
    }

    /// Number of node variables (including ground at index 0).
    pub fn node_count(&self) -> usize {
        self.node_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 3-node circuit (ground + N1 + N2) with one V-source branch variable.
    ///
    /// Expected indices:
    ///   0 → "0"  (ground)
    ///   1 → "N1"
    ///   2 → "N2"
    ///   3 → "V1" (branch current)
    #[test]
    fn three_node_one_vsource_index_mapping() {
        let mut vm = VarMap::new();

        let idx_n1 = vm.add_node("N1");
        let idx_n2 = vm.add_node("N2");
        let idx_v1 = vm.add_branch("V1");

        assert_eq!(idx_n1, 1, "N1 should be index 1");
        assert_eq!(idx_n2, 2, "N2 should be index 2");
        assert_eq!(idx_v1, 3, "V1 branch should be index 3");

        // Forward lookups
        assert_eq!(vm.node_index("0"), Some(0));
        assert_eq!(vm.node_index("N1"), Some(1));
        assert_eq!(vm.node_index("N2"), Some(2));
        assert_eq!(vm.node_index("V1"), Some(3));

        // Reverse lookups
        assert_eq!(vm.var_name(0), Some("0"));
        assert_eq!(vm.var_name(1), Some("N1"));
        assert_eq!(vm.var_name(2), Some("N2"));
        assert_eq!(vm.var_name(3), Some("V1"));

        // Out-of-bounds returns None
        assert_eq!(vm.node_index("nonexistent"), None);
        assert_eq!(vm.var_name(99), None);

        // Counts
        assert_eq!(vm.node_count(), 3); // ground + N1 + N2
        assert_eq!(vm.len(), 4);        // 3 nodes + 1 branch
    }

    #[test]
    fn ground_is_index_zero_on_new() {
        let vm = VarMap::new();
        assert_eq!(vm.node_index("0"), Some(0));
        assert_eq!(vm.var_name(0), Some("0"));
        assert_eq!(vm.node_count(), 1);
    }

    #[test]
    fn add_node_idempotent() {
        let mut vm = VarMap::new();
        let a = vm.add_node("A");
        let b = vm.add_node("A");
        assert_eq!(a, b);
        assert_eq!(vm.node_count(), 2);
    }

    #[test]
    fn add_branch_idempotent() {
        let mut vm = VarMap::new();
        vm.add_node("N1");
        let a = vm.add_branch("L1");
        let b = vm.add_branch("L1");
        assert_eq!(a, b);
        assert_eq!(vm.len(), 3); // ground + N1 + L1
    }

    #[test]
    fn branches_stay_after_nodes_when_node_added_late() {
        let mut vm = VarMap::new();
        // Add a branch before a node — branch starts at index 1.
        let vx_initial = vm.add_branch("Vx");
        assert_eq!(vx_initial, 1);
        // Now add a node — it should insert at index 1 and shift Vx to 2.
        let na = vm.add_node("Na");
        assert_eq!(na, 1);
        assert_eq!(vm.node_index("Vx"), Some(2));
        assert_eq!(vm.var_name(1), Some("Na"));
        assert_eq!(vm.var_name(2), Some("Vx"));
        assert_eq!(vm.node_count(), 2); // ground + Na
    }
}
