//! Linear device models: Resistor, Capacitor, Inductor.
//!
//! All three are smooth (analytic I-V), so `is_smooth()` returns `true`.
//! `stamp_nonlinear` simply delegates to `stamp_linear` because there is no
//! operating-point dependency.

use crate::{
    stamp_capacitor, stamp_inductor, stamp_resistor, traits::DeviceModel, MnaMatrix, VarMap,
};

// ── Resistor ─────────────────────────────────────────────────────────────────

/// A linear resistor between two nets.
#[derive(Debug, Clone)]
pub struct Resistor {
    /// Positive terminal net name.
    pub n_pos: String,
    /// Negative terminal net name.
    pub n_neg: String,
    /// Resistance in ohms (must be > 0).
    pub resistance: f64,
}

impl Resistor {
    pub fn new(n_pos: impl Into<String>, n_neg: impl Into<String>, resistance: f64) -> Self {
        assert!(resistance > 0.0, "resistance must be positive");
        Resistor {
            n_pos: n_pos.into(),
            n_neg: n_neg.into(),
            resistance,
        }
    }
}

impl DeviceModel for Resistor {
    fn terminals(&self) -> Vec<String> {
        vec![self.n_pos.clone(), self.n_neg.clone()]
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix, var_map: &VarMap) {
        let p = var_map.node_index(&self.n_pos);
        let q = var_map.node_index(&self.n_neg);
        // Ground is index 0; map it to None for the stamper helper.
        let to_opt = |idx: Option<usize>| match idx {
            Some(0) | None => None,
            Some(i) => Some(i - 1),
        };
        stamp_resistor(matrix, to_opt(p), to_opt(q), self.resistance);
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, _solution: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ── Capacitor ────────────────────────────────────────────────────────────────

/// A linear capacitor between two nets (backward-Euler companion model).
#[derive(Debug, Clone)]
pub struct Capacitor {
    /// Positive terminal net name.
    pub n_pos: String,
    /// Negative terminal net name.
    pub n_neg: String,
    /// Capacitance in farads (must be > 0).
    pub capacitance: f64,
    /// Time step in seconds (must be > 0; default 1.0 for DC analysis).
    pub timestep_s: f64,
    /// Voltage across capacitor at previous time step.
    pub v_prev: f64,
}

impl Capacitor {
    pub fn new(n_pos: impl Into<String>, n_neg: impl Into<String>, capacitance: f64) -> Self {
        assert!(capacitance > 0.0, "capacitance must be positive");
        Capacitor {
            n_pos: n_pos.into(),
            n_neg: n_neg.into(),
            capacitance,
            timestep_s: 1.0,
            v_prev: 0.0,
        }
    }
}

impl DeviceModel for Capacitor {
    fn terminals(&self) -> Vec<String> {
        vec![self.n_pos.clone(), self.n_neg.clone()]
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix, var_map: &VarMap) {
        let p = var_map.node_index(&self.n_pos);
        let q = var_map.node_index(&self.n_neg);
        let to_opt = |idx: Option<usize>| match idx {
            Some(0) | None => None,
            Some(i) => Some(i - 1),
        };
        stamp_capacitor(
            matrix,
            to_opt(p),
            to_opt(q),
            self.capacitance,
            self.timestep_s,
            self.v_prev,
        );
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, _solution: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ── Inductor ─────────────────────────────────────────────────────────────────

/// A linear inductor between two nets (backward-Euler companion model).
///
/// Requires a pre-allocated branch-current row in the MNA matrix, identified
/// by the branch net name (e.g. `"L1"`).
#[derive(Debug, Clone)]
pub struct Inductor {
    /// Positive terminal net name.
    pub n_pos: String,
    /// Negative terminal net name.
    pub n_neg: String,
    /// Branch-current variable name (must be registered in VarMap).
    pub branch_name: String,
    /// Inductance in henries (must be > 0).
    pub inductance: f64,
    /// Time step in seconds (must be > 0; default 1.0 for DC analysis).
    pub timestep_s: f64,
    /// Inductor current at previous time step.
    pub i_prev: f64,
}

impl Inductor {
    pub fn new(
        n_pos: impl Into<String>,
        n_neg: impl Into<String>,
        branch_name: impl Into<String>,
        inductance: f64,
    ) -> Self {
        assert!(inductance > 0.0, "inductance must be positive");
        Inductor {
            n_pos: n_pos.into(),
            n_neg: n_neg.into(),
            branch_name: branch_name.into(),
            inductance,
            timestep_s: 1.0,
            i_prev: 0.0,
        }
    }
}

impl DeviceModel for Inductor {
    fn terminals(&self) -> Vec<String> {
        vec![self.n_pos.clone(), self.n_neg.clone()]
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix, var_map: &VarMap) {
        let p = var_map.node_index(&self.n_pos);
        let q = var_map.node_index(&self.n_neg);
        let br = var_map
            .node_index(&self.branch_name)
            .expect("branch variable must be in VarMap");
        let to_opt = |idx: Option<usize>| match idx {
            Some(0) | None => None,
            Some(i) => Some(i - 1),
        };
        stamp_inductor(
            matrix,
            to_opt(p),
            to_opt(q),
            br - 1, // branch row is 0-indexed in stamper
            self.inductance,
            self.timestep_s,
            self.i_prev,
        );
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, _solution: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resistor_is_smooth() {
        let r = Resistor::new("N1", "0", 1000.0);
        assert!(r.is_smooth());
    }

    #[test]
    fn capacitor_is_smooth() {
        let c = Capacitor::new("N1", "0", 1e-6);
        assert!(c.is_smooth());
    }

    #[test]
    fn inductor_is_smooth() {
        let l = Inductor::new("N1", "N2", "L1", 1e-3);
        assert!(l.is_smooth());
    }

    #[test]
    fn resistor_terminals() {
        let r = Resistor::new("A", "B", 100.0);
        assert_eq!(r.terminals(), vec!["A", "B"]);
    }

    #[test]
    fn capacitor_terminals() {
        let c = Capacitor::new("A", "B", 1e-9);
        assert_eq!(c.terminals(), vec!["A", "B"]);
    }

    #[test]
    fn inductor_terminals() {
        let l = Inductor::new("A", "B", "L1", 1e-6);
        assert_eq!(l.terminals(), vec!["A", "B"]);
    }
}
