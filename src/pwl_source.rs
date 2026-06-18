//! Piecewise-linear (PWL) voltage source device model.
//!
//! A PWL voltage source introduces a branch-current unknown (same as
//! [`VoltageSource`]) and its voltage changes linearly between a sequence of
//! `(time, voltage)` breakpoints.  Between breakpoints the voltage is
//! interpolated; outside the breakpoints the first / last value is held.
//!
//! The transient driver calls [`DeviceModel::set_timestep`] and
//! [`DeviceModel::stamp_nonlinear`] at each step; the voltage is evaluated
//! at the current time via [`PwlVoltageSource::voltage_at`].
//!
//! # MNA stamp
//!
//! Identical to [`VoltageSource`]: KCL ±1 coupling at `(node, branch)` and
//! `(branch, node)`; the branch-row RHS is set to the current voltage value.

use crate::{stamp_voltage_source, traits::DeviceModel, MnaMatrix, VarMap};

/// A piecewise-linear voltage source.
///
/// # Example
///
/// ```
/// use circuit_solver_delta::pwl_source::PwlVoltageSource;
///
/// let src = PwlVoltageSource::new(
///     "N1", "0", "Vpwl",
///     vec![(0.0, 0.0), (10e-9, 1.8)],
/// );
/// assert!((src.voltage_at(5e-9) - 0.9).abs() < 1e-12);
/// assert!((src.voltage_at(0.0)  - 0.0).abs() < 1e-12);
/// assert!((src.voltage_at(10e-9) - 1.8).abs() < 1e-12);
/// ```
#[derive(Debug, Clone)]
pub struct PwlVoltageSource {
    /// Positive terminal net name.
    pub n_pos: String,
    /// Negative terminal net name.
    pub n_neg: String,
    /// Branch-current variable name (must be registered in `VarMap`).
    pub branch_name: String,
    /// Breakpoints: sorted list of (time_s, voltage_v).
    pub breakpoints: Vec<(f64, f64)>,
    /// Current simulation time (updated via [`DeviceModel::set_timestep`]).
    current_time: f64,
}

impl PwlVoltageSource {
    /// Create a new PWL voltage source.
    ///
    /// `breakpoints` must contain at least one point; they need not be
    /// pre-sorted — they are sorted ascending by time on construction.
    pub fn new(
        n_pos: impl Into<String>,
        n_neg: impl Into<String>,
        branch_name: impl Into<String>,
        mut breakpoints: Vec<(f64, f64)>,
    ) -> Self {
        assert!(!breakpoints.is_empty(), "PWL source needs at least one breakpoint");
        breakpoints.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        PwlVoltageSource {
            n_pos: n_pos.into(),
            n_neg: n_neg.into(),
            branch_name: branch_name.into(),
            breakpoints,
            current_time: 0.0,
        }
    }

    /// Evaluate the PWL waveform at `t` (seconds).
    ///
    /// - Before the first breakpoint: returns the first voltage.
    /// - After the last breakpoint: returns the last voltage.
    /// - Between breakpoints: linear interpolation.
    pub fn voltage_at(&self, t: f64) -> f64 {
        let bp = &self.breakpoints;
        if bp.is_empty() {
            return 0.0;
        }
        if t <= bp[0].0 {
            return bp[0].1;
        }
        if t >= bp[bp.len() - 1].0 {
            return bp[bp.len() - 1].1;
        }
        // Find the interval [bp[i], bp[i+1]] containing t.
        for i in 0..bp.len() - 1 {
            if t >= bp[i].0 && t <= bp[i + 1].0 {
                let dt = bp[i + 1].0 - bp[i].0;
                if dt == 0.0 {
                    return bp[i + 1].1;
                }
                let frac = (t - bp[i].0) / dt;
                return bp[i].1 + frac * (bp[i + 1].1 - bp[i].1);
            }
        }
        bp[bp.len() - 1].1
    }

    /// Set the current simulation time (called by the transient driver).
    pub fn set_time(&mut self, t: f64) {
        self.current_time = t;
    }
}

impl DeviceModel for PwlVoltageSource {
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
            let br = br_idx - 1;
            let v = self.voltage_at(self.current_time);
            stamp_voltage_source(matrix, p, q, br, v);
        }
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, _solution: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }

    fn is_smooth(&self) -> bool {
        true
    }

    fn set_time(&mut self, t: f64) {
        self.current_time = t;
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pwl_interpolates_midpoint() {
        let src = PwlVoltageSource::new("N1", "0", "Vpwl", vec![(0.0, 0.0), (10e-9, 1.8)]);
        let v = src.voltage_at(5e-9);
        assert!((v - 0.9).abs() < 1e-12, "midpoint should be 0.9V, got {v}");
    }

    #[test]
    fn pwl_clamps_before_first_breakpoint() {
        let src = PwlVoltageSource::new("N1", "0", "Vpwl", vec![(1e-9, 0.5), (5e-9, 1.0)]);
        assert!((src.voltage_at(0.0) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn pwl_clamps_after_last_breakpoint() {
        let src = PwlVoltageSource::new("N1", "0", "Vpwl", vec![(0.0, 0.0), (5e-9, 1.8)]);
        assert!((src.voltage_at(10e-9) - 1.8).abs() < 1e-12);
    }

    #[test]
    fn pwl_at_exact_breakpoints() {
        let src =
            PwlVoltageSource::new("N1", "0", "Vpwl", vec![(0.0, 0.0), (10e-9, 1.8)]);
        assert!((src.voltage_at(0.0) - 0.0).abs() < 1e-12);
        assert!((src.voltage_at(10e-9) - 1.8).abs() < 1e-12);
    }

    #[test]
    fn pwl_unsorted_input_sorted_on_new() {
        let src = PwlVoltageSource::new(
            "N1",
            "0",
            "Vpwl",
            vec![(10e-9, 1.8), (0.0, 0.0)], // reverse order
        );
        assert!((src.voltage_at(5e-9) - 0.9).abs() < 1e-12);
    }
}
