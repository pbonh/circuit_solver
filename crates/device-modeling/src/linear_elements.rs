//! Concrete [`DeviceModel`] implementations for the five fundamental
//! linear elements: [`Resistor`], [`Capacitor`], [`Inductor`],
//! [`VoltageSource`], and [`CurrentSource`].
//!
//! Each struct implements [`DeviceModel`](crate::traits::DeviceModel).
//! Because all five elements are linear, their `stamp_nonlinear`
//! implementation simply delegates to `stamp_linear` — the
//! operating-point-independent stamp is the complete stamp.
//!
//! # MNA stamp reference
//!
//! ## Resistor
//!
//! For conductance `G = 1 / R` between nodes `p` (positive) and `m`
//! (negative):
//!
//! ```text
//! A[p,p] += G   A[p,m] -= G
//! A[m,p] -= G   A[m,m] += G
//! ```
//!
//! ## Capacitor
//!
//! For capacitance `C` with a companion conductance `G_eq = C / h`
//! (trapezoidal / Backward-Euler equivalent, `h` = timestep in seconds),
//! the DC analysis convention stamps `G_eq` using `h = 1` so the
//! matrix is non-trivial.  For full transient accuracy use
//! [`CapacitorCompanion`](crate::CapacitorCompanion) instead.
//!
//! ```text
//! A[p,p] += G_eq   A[p,m] -= G_eq
//! A[m,p] -= G_eq   A[m,m] += G_eq
//! ```
//!
//! ## Inductor
//!
//! An inductor between nodes `p` and `m` with branch variable at row
//! `br = node_count + branch_offset` (from [`VarMap`](crate::VarMap)):
//!
//! ```text
//! A[br, p] += 1    A[br, m] -= 1      (KVL row: v_p - v_m = 0 at DC)
//! A[p, br] += 1    A[m, br] -= 1      (incidence: branch current contribution)
//! ```
//!
//! ## `VoltageSource`
//!
//! Same incidence pattern as the inductor, plus an RHS entry for the
//! enforced voltage:
//!
//! ```text
//! A[br, p] += 1    A[br, m] -= 1
//! A[p, br] += 1    A[m, br] -= 1
//! b[br]    += E
//! ```
//!
//! ## `CurrentSource`
//!
//! Current `I` flows from `from` into the device and out of `to`.
//! KCL convention (positive current leaves a node into the device's
//! `from` terminal, enters from `to`):
//!
//! ```text
//! b[from] += I
//! b[to]   -= I
//! ```

use circuit_solver_types::{BranchId, NodeId};

use crate::mna_matrix::MnaMatrix;
use crate::traits::DeviceModel;
use crate::var_map::VarMap;

// ---------------------------------------------------------------------------
// Resistor
// ---------------------------------------------------------------------------

/// A two-terminal resistor with resistance `resistance_ohms`.
///
/// Terminals: `[positive, negative]`.
#[derive(Debug, Clone)]
pub struct Resistor {
    /// Terminal nodes: `[positive, negative]`.
    pub terminals: [NodeId; 2],
    /// Resistance in ohms.  Must be positive and finite.
    pub resistance_ohms: f64,
}

impl DeviceModel for Resistor {
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap) {
        let p = var_map
            .node_index(self.terminals[0])
            .expect("Resistor: positive terminal not in VarMap");
        let m = var_map
            .node_index(self.terminals[1])
            .expect("Resistor: negative terminal not in VarMap");
        let g = 1.0 / self.resistance_ohms;
        matrix.add_element(p, p, g);
        matrix.add_element(m, m, g);
        matrix.add_element(p, m, -g);
        matrix.add_element(m, p, -g);
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, _x: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// Capacitor
// ---------------------------------------------------------------------------

/// A two-terminal capacitor with capacitance `capacitance_farads`.
///
/// Terminals: `[positive, negative]`.
///
/// The `stamp_linear` implementation uses a companion conductance
/// `G_eq = capacitance_farads / timestep_s` (default `timestep_s = 1.0`
/// for a non-trivial DC stamp).  For accurate transient simulation use
/// [`CapacitorCompanion`](crate::CapacitorCompanion) which accepts the
/// actual timestep and previous state.
#[derive(Debug, Clone)]
pub struct Capacitor {
    /// Terminal nodes: `[positive, negative]`.
    pub terminals: [NodeId; 2],
    /// Capacitance in farads.  Must be positive and finite.
    pub capacitance_farads: f64,
    /// Timestep `h` used to compute `G_eq = C / h`.  Defaults to `1.0`.
    pub timestep_s: f64,
}

impl Capacitor {
    /// Construct a `Capacitor` using the default unit timestep.
    #[must_use]
    pub fn new(terminals: [NodeId; 2], capacitance_farads: f64) -> Self {
        Self {
            terminals,
            capacitance_farads,
            timestep_s: 1.0,
        }
    }
}

impl DeviceModel for Capacitor {
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap) {
        let p = var_map
            .node_index(self.terminals[0])
            .expect("Capacitor: positive terminal not in VarMap");
        let m = var_map
            .node_index(self.terminals[1])
            .expect("Capacitor: negative terminal not in VarMap");
        let g_eq = self.capacitance_farads / self.timestep_s;
        matrix.add_element(p, p, g_eq);
        matrix.add_element(m, m, g_eq);
        matrix.add_element(p, m, -g_eq);
        matrix.add_element(m, p, -g_eq);
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, _x: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// Inductor
// ---------------------------------------------------------------------------

/// A two-terminal inductor with inductance `inductance_henries`.
///
/// Terminals: `[positive, negative]`.
///
/// At DC the inductor is a short circuit.  `stamp_linear` places the
/// standard KVL / incidence entries in the MNA matrix using the branch
/// variable identified by `branch_id`.  No RHS contribution is added
/// (the enforced voltage `v_p - v_m = 0`).
#[derive(Debug, Clone)]
pub struct Inductor {
    /// Terminal nodes: `[positive, negative]`.
    pub terminals: [NodeId; 2],
    /// Inductance in henries.  Must be positive and finite.
    pub inductance_henries: f64,
    /// The MNA branch variable allocated for this inductor's current.
    pub branch_id: BranchId,
}

impl DeviceModel for Inductor {
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap) {
        let p = var_map
            .node_index(self.terminals[0])
            .expect("Inductor: positive terminal not in VarMap");
        let m = var_map
            .node_index(self.terminals[1])
            .expect("Inductor: negative terminal not in VarMap");
        let br = var_map
            .branch_index(self.branch_id)
            .expect("Inductor: branch_id not in VarMap");
        // KVL row: v_p - v_m = 0 (DC short).
        matrix.add_element(br, p, 1.0);
        matrix.add_element(br, m, -1.0);
        // Incidence: branch current contributes to node KCL rows.
        matrix.add_element(p, br, 1.0);
        matrix.add_element(m, br, -1.0);
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, _x: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// VoltageSource
// ---------------------------------------------------------------------------

/// An ideal independent voltage source enforcing `v_plus - v_minus = voltage_volts`.
///
/// Terminals: `[plus, minus]`.
///
/// `stamp_linear` adds the standard KVL / incidence stamp and places the
/// enforced voltage on the branch RHS entry.
#[derive(Debug, Clone)]
pub struct VoltageSource {
    /// Terminal nodes: `[plus, minus]`.
    pub terminals: [NodeId; 2],
    /// Source voltage in volts (`v_plus - v_minus`).
    pub voltage_volts: f64,
    /// The MNA branch variable allocated for this source's current.
    pub branch_id: BranchId,
}

impl DeviceModel for VoltageSource {
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap) {
        let p = var_map
            .node_index(self.terminals[0])
            .expect("VoltageSource: plus terminal not in VarMap");
        let m = var_map
            .node_index(self.terminals[1])
            .expect("VoltageSource: minus terminal not in VarMap");
        let br = var_map
            .branch_index(self.branch_id)
            .expect("VoltageSource: branch_id not in VarMap");
        // KVL row: v_plus - v_minus = E.
        matrix.add_element(br, p, 1.0);
        matrix.add_element(br, m, -1.0);
        // Incidence: branch current in node KCL rows.
        matrix.add_element(p, br, 1.0);
        matrix.add_element(m, br, -1.0);
        // RHS: enforced voltage.
        matrix.add_rhs(br, self.voltage_volts);
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, _x: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// CurrentSource
// ---------------------------------------------------------------------------

/// An ideal independent current source pushing `current_amperes` from
/// `from` through the device and out of `to`.
///
/// Terminals: `[from, to]`.
///
/// SPICE convention: positive current flows *into* the device at the
/// `from` terminal and *out of* the device at the `to` terminal.  KCL at
/// `from` gains `+I` on the RHS (current exits the node into the device);
/// KCL at `to` gains `-I` (current returns from the device into the node).
#[derive(Debug, Clone)]
pub struct CurrentSource {
    /// Terminal nodes: `[from, to]`.
    pub terminals: [NodeId; 2],
    /// Source current in amperes.
    pub current_amperes: f64,
}

impl DeviceModel for CurrentSource {
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap) {
        let from = var_map
            .node_index(self.terminals[0])
            .expect("CurrentSource: from terminal not in VarMap");
        let to = var_map
            .node_index(self.terminals[1])
            .expect("CurrentSource: to terminal not in VarMap");
        matrix.add_rhs(from, self.current_amperes);
        matrix.add_rhs(to, -self.current_amperes);
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, _x: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_solver_types::{BranchId, NodeId};

    use crate::mna_matrix::MnaMatrix;
    use crate::var_map::VarMap;

    // --- helpers ---

    fn two_node_system() -> (VarMap, Vec<f64>, Vec<f64>) {
        let nodes = [NodeId::GROUND, NodeId::new(1)];
        let var_map = VarMap::from_nodes(&nodes);
        let a = vec![0.0_f64; 4]; // 2×2
        let b = vec![0.0_f64; 2];
        (var_map, a, b)
    }

    /// Build a 3-row system: 2 nodes + 1 branch variable.
    fn two_node_one_branch_system(bid: BranchId) -> (VarMap, Vec<f64>, Vec<f64>) {
        let nodes = [NodeId::GROUND, NodeId::new(1)];
        let var_map = VarMap::from_nodes(&nodes).with_branches(&[bid]);
        let dim = var_map.dim();
        let a = vec![0.0_f64; dim * dim];
        let b = vec![0.0_f64; dim];
        (var_map, a, b)
    }

    // --- Resistor ---

    #[test]
    fn resistor_stamps_conductance_correctly() {
        let (var_map, mut a, mut b) = two_node_system();
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);

        let r = Resistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            resistance_ohms: 1000.0,
        };
        r.stamp_linear(&mut matrix, &var_map);

        let g = 1.0 / 1000.0;
        assert!((matrix.element(0, 0) - g).abs() < f64::EPSILON);
        assert!((matrix.element(1, 1) - g).abs() < f64::EPSILON);
        assert!((matrix.element(0, 1) + g).abs() < f64::EPSILON);
        assert!((matrix.element(1, 0) + g).abs() < f64::EPSILON);
        // RHS untouched.
        assert_eq!(matrix.rhs(0), 0.0);
        assert_eq!(matrix.rhs(1), 0.0);
    }

    #[test]
    fn resistor_is_smooth() {
        let r = Resistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            resistance_ohms: 1000.0,
        };
        assert!(r.is_smooth());
    }

    #[test]
    fn resistor_stamp_nonlinear_matches_stamp_linear() {
        let (var_map, mut a1, mut b1) = two_node_system();
        let (_, mut a2, mut b2) = two_node_system();
        let r = Resistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            resistance_ohms: 500.0,
        };
        {
            let mut m1 = MnaMatrix::new(&mut a1, &mut b1, 2);
            r.stamp_linear(&mut m1, &var_map);
        }
        {
            let mut m2 = MnaMatrix::new(&mut a2, &mut b2, 2);
            r.stamp_nonlinear(&mut m2, &var_map, &[0.0, 5.0]);
        }
        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn resistor_terminals_are_correct() {
        let r = Resistor {
            terminals: [NodeId::GROUND, NodeId::new(3)],
            resistance_ohms: 1.0,
        };
        let t = r.terminals();
        assert_eq!(t.len(), 2);
        assert_eq!(t[0], NodeId::GROUND);
        assert_eq!(t[1], NodeId::new(3));
    }

    // --- Capacitor ---

    #[test]
    fn capacitor_stamps_companion_conductance_correctly() {
        let (var_map, mut a, mut b) = two_node_system();
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);

        let c = Capacitor::new([NodeId::GROUND, NodeId::new(1)], 1e-6);
        c.stamp_linear(&mut matrix, &var_map);

        let g_eq = 1e-6 / 1.0; // timestep_s = 1.0
        assert!((matrix.element(0, 0) - g_eq).abs() < 1e-15);
        assert!((matrix.element(1, 1) - g_eq).abs() < 1e-15);
        assert!((matrix.element(0, 1) + g_eq).abs() < 1e-15);
        assert!((matrix.element(1, 0) + g_eq).abs() < 1e-15);
    }

    #[test]
    fn capacitor_is_smooth() {
        let c = Capacitor::new([NodeId::GROUND, NodeId::new(1)], 1e-6);
        assert!(c.is_smooth());
    }

    #[test]
    fn capacitor_stamp_nonlinear_matches_stamp_linear() {
        let (var_map, mut a1, mut b1) = two_node_system();
        let (_, mut a2, mut b2) = two_node_system();
        let c = Capacitor::new([NodeId::GROUND, NodeId::new(1)], 1e-9);
        {
            let mut m1 = MnaMatrix::new(&mut a1, &mut b1, 2);
            c.stamp_linear(&mut m1, &var_map);
        }
        {
            let mut m2 = MnaMatrix::new(&mut a2, &mut b2, 2);
            c.stamp_nonlinear(&mut m2, &var_map, &[0.0, 0.0]);
        }
        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
    }

    // --- Inductor ---

    #[test]
    fn inductor_stamps_kvl_incidence_correctly() {
        let bid = BranchId::new(0);
        let (var_map, mut a, mut b) = two_node_one_branch_system(bid);
        let dim = var_map.dim(); // 3
        let mut matrix = MnaMatrix::new(&mut a, &mut b, dim);

        let l = Inductor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            inductance_henries: 1e-3,
            branch_id: bid,
        };
        l.stamp_linear(&mut matrix, &var_map);

        // branch row (row 2) has +1 at col 0 (GROUND) and -1 at col 1 (node 1).
        assert_eq!(matrix.element(2, 0), 1.0);
        assert_eq!(matrix.element(2, 1), -1.0);
        // node rows have ±1 in branch column (col 2).
        assert_eq!(matrix.element(0, 2), 1.0);
        assert_eq!(matrix.element(1, 2), -1.0);
        // RHS all zero (DC short: no forced voltage).
        assert_eq!(matrix.rhs(0), 0.0);
        assert_eq!(matrix.rhs(1), 0.0);
        assert_eq!(matrix.rhs(2), 0.0);
    }

    #[test]
    fn inductor_is_smooth() {
        let l = Inductor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            inductance_henries: 1e-3,
            branch_id: BranchId::new(0),
        };
        assert!(l.is_smooth());
    }

    #[test]
    fn inductor_stamp_nonlinear_matches_stamp_linear() {
        let bid = BranchId::new(0);
        let (var_map, mut a1, mut b1) = two_node_one_branch_system(bid);
        let (_, mut a2, mut b2) = two_node_one_branch_system(bid);
        let dim = var_map.dim();
        let l = Inductor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            inductance_henries: 1e-3,
            branch_id: bid,
        };
        {
            let mut m1 = MnaMatrix::new(&mut a1, &mut b1, dim);
            l.stamp_linear(&mut m1, &var_map);
        }
        {
            let mut m2 = MnaMatrix::new(&mut a2, &mut b2, dim);
            l.stamp_nonlinear(&mut m2, &var_map, &[0.0, 0.0, 0.0]);
        }
        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
    }

    // --- VoltageSource ---

    #[test]
    fn voltage_source_stamps_kvl_and_rhs_correctly() {
        let bid = BranchId::new(0);
        let (var_map, mut a, mut b) = two_node_one_branch_system(bid);
        let dim = var_map.dim(); // 3
        let mut matrix = MnaMatrix::new(&mut a, &mut b, dim);

        let v = VoltageSource {
            terminals: [NodeId::new(1), NodeId::GROUND], // plus=node1, minus=GND
            voltage_volts: 5.0,
            branch_id: bid,
        };
        v.stamp_linear(&mut matrix, &var_map);

        // plus = index 1, minus = index 0, branch = index 2.
        // KVL row (2): +1 at col 1, -1 at col 0.
        assert_eq!(matrix.element(2, 1), 1.0);
        assert_eq!(matrix.element(2, 0), -1.0);
        // Incidence: node1(1) += 1 at br col, node0(0) -= 1 at br col.
        assert_eq!(matrix.element(1, 2), 1.0);
        assert_eq!(matrix.element(0, 2), -1.0);
        // RHS[branch] = 5.0.
        assert_eq!(matrix.rhs(2), 5.0);
    }

    #[test]
    fn voltage_source_is_smooth() {
        let v = VoltageSource {
            terminals: [NodeId::new(1), NodeId::GROUND],
            voltage_volts: 5.0,
            branch_id: BranchId::new(0),
        };
        assert!(v.is_smooth());
    }

    #[test]
    fn voltage_source_stamp_nonlinear_matches_stamp_linear() {
        let bid = BranchId::new(0);
        let (var_map, mut a1, mut b1) = two_node_one_branch_system(bid);
        let (_, mut a2, mut b2) = two_node_one_branch_system(bid);
        let dim = var_map.dim();
        let v = VoltageSource {
            terminals: [NodeId::new(1), NodeId::GROUND],
            voltage_volts: 3.3,
            branch_id: bid,
        };
        {
            let mut m1 = MnaMatrix::new(&mut a1, &mut b1, dim);
            v.stamp_linear(&mut m1, &var_map);
        }
        {
            let mut m2 = MnaMatrix::new(&mut a2, &mut b2, dim);
            v.stamp_nonlinear(&mut m2, &var_map, &[0.0, 0.0, 0.0]);
        }
        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
    }

    // --- CurrentSource ---

    #[test]
    fn current_source_stamps_rhs_correctly() {
        let (var_map, mut a, mut b) = two_node_system();
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);

        let i = CurrentSource {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            current_amperes: 2e-3, // 2 mA
        };
        i.stamp_linear(&mut matrix, &var_map);

        // from=0 gains +I, to=1 gains -I.
        assert!((matrix.rhs(0) - 2e-3).abs() < f64::EPSILON);
        assert!((matrix.rhs(1) + 2e-3).abs() < f64::EPSILON);
        // A matrix untouched.
        for row in 0..2 {
            for col in 0..2 {
                assert_eq!(matrix.element(row, col), 0.0);
            }
        }
    }

    #[test]
    fn current_source_is_smooth() {
        let i = CurrentSource {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            current_amperes: 1e-3,
        };
        assert!(i.is_smooth());
    }

    #[test]
    fn current_source_stamp_nonlinear_matches_stamp_linear() {
        let (var_map, mut a1, mut b1) = two_node_system();
        let (_, mut a2, mut b2) = two_node_system();
        let i = CurrentSource {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            current_amperes: 5e-3,
        };
        {
            let mut m1 = MnaMatrix::new(&mut a1, &mut b1, 2);
            i.stamp_linear(&mut m1, &var_map);
        }
        {
            let mut m2 = MnaMatrix::new(&mut a2, &mut b2, 2);
            i.stamp_nonlinear(&mut m2, &var_map, &[0.0, 0.0]);
        }
        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn current_source_terminals_are_correct() {
        let i = CurrentSource {
            terminals: [NodeId::new(2), NodeId::new(3)],
            current_amperes: 1.0,
        };
        let t = i.terminals();
        assert_eq!(t[0], NodeId::new(2));
        assert_eq!(t[1], NodeId::new(3));
    }

    // --- dyn DeviceModel usability ---

    #[test]
    fn all_elements_usable_as_dyn_device_model() {
        fn check(d: &dyn DeviceModel) {
            assert!(d.is_smooth());
            assert_eq!(d.terminals().len(), 2);
        }
        check(&Resistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            resistance_ohms: 1.0,
        });
        check(&Capacitor::new([NodeId::GROUND, NodeId::new(1)], 1e-6));
        check(&Inductor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            inductance_henries: 1e-3,
            branch_id: BranchId::new(0),
        });
        check(&VoltageSource {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            voltage_volts: 1.0,
            branch_id: BranchId::new(0),
        });
        check(&CurrentSource {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            current_amperes: 1e-3,
        });
    }
}
