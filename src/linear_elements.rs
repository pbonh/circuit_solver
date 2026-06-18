//! Linear device models: Resistor, Capacitor, Inductor, VoltageSource, CurrentSource.
//!
//! All are smooth (analytic I-V), so `is_smooth()` returns `true`.
//! `stamp_nonlinear` simply delegates to `stamp_linear` because there is no
//! operating-point dependency.

use crate::{
    stamp_capacitor, stamp_current_source, stamp_inductor, stamp_resistor, stamp_voltage_source,
    traits::DeviceModel, MnaMatrix, VarMap,
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

    fn set_timestep(&mut self, h: f64) {
        self.timestep_s = h;
    }

    fn advance_state(&mut self, solution: &[f64], var_map: &VarMap) {
        let p_raw = var_map.node_index(&self.n_pos);
        let q_raw = var_map.node_index(&self.n_neg);
        let voltage = |idx: Option<usize>| match idx {
            Some(0) | None => 0.0,
            Some(i) => solution.get(i - 1).copied().unwrap_or(0.0),
        };
        self.v_prev = voltage(p_raw) - voltage(q_raw);
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

    fn set_timestep(&mut self, h: f64) {
        self.timestep_s = h;
    }

    fn advance_state(&mut self, solution: &[f64], var_map: &VarMap) {
        if let Some(br) = var_map.node_index(&self.branch_name)
            && br > 0
        {
            self.i_prev = solution.get(br - 1).copied().unwrap_or(0.0);
        }
    }
}

// ── VoltageSource ─────────────────────────────────────────────────────────────

/// An ideal DC voltage source between two nets.
///
/// Introduces one branch-current unknown named `branch_name` (e.g. `"V1"`).
#[derive(Debug, Clone)]
pub struct VoltageSource {
    /// Positive terminal net name.
    pub n_pos: String,
    /// Negative terminal net name.
    pub n_neg: String,
    /// Branch-current variable name (must be registered in `VarMap`).
    pub branch_name: String,
    /// DC voltage (volts).
    pub voltage: f64,
}

impl VoltageSource {
    pub fn new(
        n_pos: impl Into<String>,
        n_neg: impl Into<String>,
        branch_name: impl Into<String>,
        voltage: f64,
    ) -> Self {
        VoltageSource {
            n_pos: n_pos.into(),
            n_neg: n_neg.into(),
            branch_name: branch_name.into(),
            voltage,
        }
    }
}

impl DeviceModel for VoltageSource {
    fn terminals(&self) -> Vec<String> {
        vec![self.n_pos.clone(), self.n_neg.clone()]
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix, var_map: &VarMap) {
        let to_opt = |idx: Option<usize>| match idx {
            Some(0) | None => None,
            Some(i) => Some(i - 1),
        };
        let p = to_opt(var_map.node_index(&self.n_pos));
        let q = to_opt(var_map.node_index(&self.n_neg));
        if let Some(br_idx) = var_map.node_index(&self.branch_name) {
            let br = br_idx - 1; // convert to 0-based MNA row (ground excluded)
            stamp_voltage_source(matrix, p, q, br, self.voltage);
        }
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, _solution: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }
}

// ── CurrentSource ─────────────────────────────────────────────────────────────

/// An ideal DC current source between two nets.
///
/// Current flows from `n_neg` to `n_pos` (into `n_pos`).
#[derive(Debug, Clone)]
pub struct CurrentSource {
    /// Positive terminal net name (current flows in here).
    pub n_pos: String,
    /// Negative terminal net name.
    pub n_neg: String,
    /// DC current (amperes).
    pub current: f64,
}

impl CurrentSource {
    pub fn new(n_pos: impl Into<String>, n_neg: impl Into<String>, current: f64) -> Self {
        CurrentSource { n_pos: n_pos.into(), n_neg: n_neg.into(), current }
    }
}

impl DeviceModel for CurrentSource {
    fn terminals(&self) -> Vec<String> {
        vec![self.n_pos.clone(), self.n_neg.clone()]
    }

    fn stamp_linear(&self, matrix: &mut MnaMatrix, var_map: &VarMap) {
        let to_opt = |idx: Option<usize>| match idx {
            Some(0) | None => None,
            Some(i) => Some(i - 1),
        };
        let p = to_opt(var_map.node_index(&self.n_pos));
        let q = to_opt(var_map.node_index(&self.n_neg));
        stamp_current_source(matrix, p, q, self.current);
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
