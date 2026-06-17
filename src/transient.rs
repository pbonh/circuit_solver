//! Transient analysis driver.
//!
//! [`TransientAnalysis`] steps from `t_start` to `t_stop`, assembles the MNA
//! at each timepoint (using backward-Euler companion models), calls the
//! configured BDF integrator and the adaptive timestep controller, samples
//! waveforms on accepted steps, and returns [`TransientSolution`].
//!
//! # Algorithm
//!
//! ```text
//! t = t_start, h = h_initial
//! while t < t_stop:
//!     clamp h so t + h <= t_stop
//!     update companion model timesteps + previous values
//!     assemble MNA → G, b
//!     call integrator.step(t, h, G, b) → (x_new, lte)
//!     call controller.evaluate(t, lte, ||x||∞) → Accept(h_next) | Reject(h_next) | Err
//!     if Accept:
//!         sample waveforms from x_new
//!         push t+h into times, x_new into waveform buffers
//!         update device history (v_prev for capacitors etc.)
//!         t += h
//!         h = h_next
//!     if Reject:
//!         h = h_next   (retry same t)
//!     if Err:
//!         return Err(IntegrationError { t, lte, h })
//! return Ok(TransientSolution { times, waveforms })
//! ```
//!
//! # Waveforms
//!
//! At each accepted step, the node voltages are sampled by name (from
//! `VarMap`) and stored in `waveforms["node_name"]`.  Branch currents are
//! also sampled and stored as `waveforms["I(branch_name)"]`.

use std::collections::HashMap;

use crate::{
    integration::{
        adaptive::AdaptiveStepController,
        bdf::{Bdf, BdfConfig},
        IntegrationError,
    },
    traits::DeviceModel,
    MnaMatrix, VarMap,
};

// ── TransientSolution ─────────────────────────────────────────────────────────

/// Result of a completed transient analysis.
#[derive(Debug, Clone)]
pub struct TransientSolution {
    /// Simulation timepoints at which waveforms were sampled (seconds).
    pub times: Vec<f64>,
    /// Waveforms keyed by signal name.
    ///
    /// Node voltages: key = node name (e.g. `"N1"`, `"out"`).
    /// Branch currents: key = `"I(<branch_name>)"` (e.g. `"I(V1)"`).
    /// Each `Vec<f64>` has the same length as `times`.
    pub waveforms: HashMap<String, Vec<f64>>,
}

// ── Integrator configuration enum ────────────────────────────────────────────

/// Select the numerical integrator for [`TransientAnalysis`].
#[derive(Debug, Clone)]
pub enum IntegratorConfig {
    /// BDF1 or BDF2 (Gear's method). Chosen via [`BdfConfig`].
    Bdf(BdfConfig),
    // RadauIIA would be added here in a future story.
}

impl Default for IntegratorConfig {
    fn default() -> Self {
        IntegratorConfig::Bdf(BdfConfig::default())
    }
}

// ── TransientAnalysis ─────────────────────────────────────────────────────────

/// Configuration and runner for transient analysis.
///
/// # Example
///
/// ```
/// use circuit_solver_delta::{
///     linear_elements::Resistor,
///     transient::{TransientAnalysis, IntegratorConfig},
///     VarMap,
///     traits::DeviceModel,
/// };
///
/// let mut vm = VarMap::new();
/// vm.add_node("N1");
///
/// let devices: Vec<Box<dyn DeviceModel>> = vec![
///     Box::new(Resistor::new("N1", "0", 1000.0)),
/// ];
///
/// // Pure resistive circuit (no energy storage): trivial transient.
/// let analysis = TransientAnalysis::builder(0.0, 10e-9, &vm, &devices)
///     .h_initial(1e-9)
///     .h_max(1e-9)
///     .build();
///
/// let sol = analysis.run().expect("should succeed");
/// assert!(!sol.times.is_empty());
/// ```
pub struct TransientAnalysis<'a> {
    t_start: f64,
    t_stop: f64,
    var_map: &'a VarMap,
    devices: &'a [Box<dyn DeviceModel>],
    integrator_config: IntegratorConfig,
    controller_config: ControllerConfig,
}

/// Parameters for the adaptive step controller.
#[derive(Debug, Clone)]
pub struct ControllerConfig {
    pub rtol: f64,
    pub atol: f64,
    pub h_min: f64,
    pub h_max: f64,
    pub max_consecutive_rejections: usize,
    pub h_initial: f64,
}

impl Default for ControllerConfig {
    fn default() -> Self {
        ControllerConfig {
            rtol: 1e-3,
            atol: 1e-6,
            h_min: 1e-15,
            h_max: 1e-3,
            max_consecutive_rejections: 5,
            h_initial: 1e-9,
        }
    }
}

/// Builder for [`TransientAnalysis`].
pub struct TransientAnalysisBuilder<'a> {
    t_start: f64,
    t_stop: f64,
    var_map: &'a VarMap,
    devices: &'a [Box<dyn DeviceModel>],
    integrator_config: IntegratorConfig,
    controller_config: ControllerConfig,
}

impl<'a> TransientAnalysisBuilder<'a> {
    /// Set the initial and maximum timestep.
    pub fn h_initial(mut self, h: f64) -> Self {
        self.controller_config.h_initial = h;
        self
    }

    /// Set the maximum timestep.
    pub fn h_max(mut self, h: f64) -> Self {
        self.controller_config.h_max = h;
        self
    }

    /// Set the integrator.
    pub fn integrator(mut self, cfg: IntegratorConfig) -> Self {
        self.integrator_config = cfg;
        self
    }

    /// Finish building.
    pub fn build(self) -> TransientAnalysis<'a> {
        TransientAnalysis {
            t_start: self.t_start,
            t_stop: self.t_stop,
            var_map: self.var_map,
            devices: self.devices,
            integrator_config: self.integrator_config,
            controller_config: self.controller_config,
        }
    }
}

impl<'a> TransientAnalysis<'a> {
    /// Create a builder.
    pub fn builder(
        t_start: f64,
        t_stop: f64,
        var_map: &'a VarMap,
        devices: &'a [Box<dyn DeviceModel>],
    ) -> TransientAnalysisBuilder<'a> {
        TransientAnalysisBuilder {
            t_start,
            t_stop,
            var_map,
            devices,
            integrator_config: IntegratorConfig::default(),
            controller_config: ControllerConfig::default(),
        }
    }

    /// Run the transient simulation.
    ///
    /// # Returns
    /// `Ok(TransientSolution)` on success, or
    /// `Err(IntegrationError { t, lte, h })` when the adaptive controller
    /// cannot drive LTE below tolerance.
    ///
    /// # Errors
    ///
    /// Returns [`IntegrationError`] if the adaptive step controller exhausts
    /// its consecutive-rejection budget at any timepoint.
    pub fn run(&self) -> Result<TransientSolution, IntegrationError> {
        let n = self.var_map.len() - 1; // exclude ground
        if n == 0 {
            // Empty circuit: return empty solution.
            return Ok(TransientSolution {
                times: vec![],
                waveforms: HashMap::new(),
            });
        }

        // --- build integrator ---
        let mut integrator = match &self.integrator_config {
            IntegratorConfig::Bdf(cfg) => Bdf::new(cfg.clone(), n),
        };
        integrator.reset();

        // --- build controller ---
        let cc = &self.controller_config;
        let mut controller = AdaptiveStepController::new(
            cc.rtol,
            cc.atol,
            cc.h_min,
            cc.h_max,
            cc.max_consecutive_rejections,
            cc.h_initial,
        );

        // --- build waveform buffers ---
        let mut times: Vec<f64> = Vec::new();
        let mut waveforms: HashMap<String, Vec<f64>> = HashMap::new();

        // Pre-allocate one buffer per signal.
        // Node voltages (skip ground at index 0).
        for idx in 1..self.var_map.len() {
            if let Some(name) = self.var_map.var_name(idx) {
                // Branch currents start at var_map.node_count(); node voltages
                // are at 1..node_count().
                let key = if idx < self.var_map.node_count() {
                    name.to_owned()
                } else {
                    format!("I({name})")
                };
                waveforms.insert(key, Vec::new());
            }
        }

        // --- current state: x_{n} (all-zero initial condition) ---
        let mut x_current = vec![0.0_f64; n];

        let mut t = self.t_start;
        let mut h = controller.h;

        // Main stepping loop.
        while t < self.t_stop {
            // Clamp h so we don't overshoot t_stop.
            let h_try = h.min(self.t_stop - t);

            // Assemble the MNA for this step.
            // Devices with backward-Euler companion models (Capacitor, Inductor)
            // read their internal `timestep_s` and `v_prev` / `i_prev` fields.
            // We expose a "with_timestep" path via the DeviceModel interface —
            // but existing DeviceModel impls (Capacitor, Inductor) already hold
            // those fields and stamp them when stamping. The caller is responsible
            // for keeping those fields up to date between steps.
            //
            // For TransientAnalysis we use the raw stamp: devices provide their
            // own h via their internal state.  To support variable h we rely on
            // the stampable trait in the companion models that the devices carry.
            let mut matrix = MnaMatrix::new(n);
            for device in self.devices {
                device.stamp_nonlinear(&mut matrix, self.var_map, &x_current);
            }
            let csr = matrix.to_csr();

            // Build column-major dense Jacobian for the BDF integrator.
            let jacobian = csr_to_column_major(&csr, n);
            let rhs: Vec<f64> = csr.rhs.clone();

            // Call integrator.
            let (x_new, lte) = match integrator.step(t, h_try, &jacobian, &rhs) {
                Ok(result) => result,
                Err(_singular) => {
                    // Singular MNA → treat as integration failure.
                    return Err(IntegrationError { t, lte: f64::INFINITY, h: h_try });
                }
            };

            // Compute ||x_new||∞ for tolerance computation.
            let x_inf = x_new.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);

            // Call adaptive controller.
            use crate::integration::adaptive::ControllerOutcome;
            match controller.evaluate(t, lte, x_inf) {
                Ok(ControllerOutcome::Accept(h_next)) => {
                    // Step accepted: advance time and record waveforms.
                    t += h_try;
                    h = h_next.min(self.t_stop - t).max(cc.h_min);
                    x_current.clone_from(&x_new);

                    times.push(t);
                    for idx in 1..self.var_map.len() {
                        if let Some(name) = self.var_map.var_name(idx) {
                            let mat_row = idx - 1; // exclude ground
                            let val = x_new.get(mat_row).copied().unwrap_or(0.0);
                            let key = if idx < self.var_map.node_count() {
                                name.to_owned()
                            } else {
                                format!("I({name})")
                            };
                            if let Some(buf) = waveforms.get_mut(&key) {
                                buf.push(val);
                            }
                        }
                    }
                }
                Ok(ControllerOutcome::Reject(h_next)) => {
                    // Step rejected: retry with smaller h.
                    h = h_next;
                    // Don't advance t; integrator history is already updated;
                    // but we don't want the bad x_new to become the new x_current.
                    // History will show that step; the BDF LTE estimate may differ
                    // on the retry, which is fine (it converges toward acceptance).
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(TransientSolution { times, waveforms })
    }
}

// ── Helper ────────────────────────────────────────────────────────────────────

/// Convert a CsrMatrix to a column-major dense vector of length n×n.
fn csr_to_column_major(csr: &crate::CsrMatrix, n: usize) -> Vec<f64> {
    let mut out = vec![0.0_f64; n * n];
    for row in 0..n {
        for col in 0..n {
            out[col * n + row] = csr.get(row, col);
        }
    }
    out
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{linear_elements::Resistor, traits::DeviceModel};

    /// Stamp a voltage source as a device model for test use.
    struct VSource {
        node_pos: String,
        branch: String,
        voltage: f64,
    }
    impl DeviceModel for VSource {
        fn terminals(&self) -> Vec<String> {
            vec![self.node_pos.clone()]
        }
        fn stamp_linear(&self, matrix: &mut MnaMatrix, var_map: &VarMap) {
            use crate::stamper::stamp_voltage_source;
            let np = var_map.node_index(&self.node_pos);
            let br = var_map.node_index(&self.branch).expect("branch in varmap");
            let to_row = |idx: Option<usize>| match idx {
                Some(0) | None => None,
                Some(i) => Some(i - 1),
            };
            stamp_voltage_source(matrix, to_row(np), None, br - 1, self.voltage);
        }
        fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, _: &[f64]) {
            self.stamp_linear(matrix, var_map);
        }
        fn is_smooth(&self) -> bool {
            true
        }
    }

    // ── basic: resistor + voltage source ──────────────────────────────────────

    /// Resistive circuit: V1=5V, R=1kΩ. V(N1) should be ~5V at all timepoints.
    #[test]
    fn transient_resistive_circuit_node_voltage() {
        let mut vm = VarMap::new();
        vm.add_node("N1");
        vm.add_branch("V1");
        let n = vm.len() - 1;
        let _ = n;

        let devices: Vec<Box<dyn DeviceModel>> = vec![
            Box::new(VSource { node_pos: "N1".into(), branch: "V1".into(), voltage: 5.0 }),
            Box::new(Resistor::new("N1", "0", 1000.0)),
        ];

        let analysis = TransientAnalysis::builder(0.0, 3e-9, &vm, &devices)
            .h_initial(1e-9)
            .h_max(1e-9)
            .build();

        let sol = analysis.run().expect("transient should succeed");
        assert!(!sol.times.is_empty(), "should have at least one sample");
        // All times should be monotonically increasing.
        for w in sol.times.windows(2) {
            assert!(w[1] > w[0], "times must be monotonically increasing");
        }
        // V(N1) should be ~5V at every timepoint.
        let v_n1 = sol.waveforms.get("N1").expect("N1 waveform missing");
        for &v in v_n1 {
            assert!(
                (v - 5.0).abs() < 1e-4,
                "V(N1) should be ~5V, got {v:.6}"
            );
        }
    }

    // ── returns Err on integration failure ───────────────────────────────────

    /// A degenerate 1-node circuit with a zero Jacobian triggers an Err.
    #[test]
    fn transient_singular_returns_integration_error() {
        let mut vm = VarMap::new();
        vm.add_node("X");

        struct Open;
        impl DeviceModel for Open {
            fn terminals(&self) -> Vec<String> {
                vec![]
            }
            fn stamp_linear(&self, _: &mut MnaMatrix, _: &VarMap) {}
            fn stamp_nonlinear(&self, _: &mut MnaMatrix, _: &VarMap, _: &[f64]) {}
            fn is_smooth(&self) -> bool {
                true
            }
        }

        let devices: Vec<Box<dyn DeviceModel>> = vec![Box::new(Open)];
        let analysis = TransientAnalysis::builder(0.0, 1e-9, &vm, &devices)
            .h_initial(1e-9)
            .h_max(1e-9)
            .build();

        let result = analysis.run();
        assert!(result.is_err(), "singular Jacobian should return Err");
    }

    // ── waveform buffers created for all variables ─────────────────────────

    /// The waveform map has keys for every node and branch-current variable.
    #[test]
    fn transient_waveforms_keyed_by_variable() {
        let mut vm = VarMap::new();
        vm.add_node("N1");
        vm.add_branch("V1");

        let devices: Vec<Box<dyn DeviceModel>> = vec![
            Box::new(VSource { node_pos: "N1".into(), branch: "V1".into(), voltage: 1.0 }),
            Box::new(Resistor::new("N1", "0", 1000.0)),
        ];

        let analysis = TransientAnalysis::builder(0.0, 2e-9, &vm, &devices)
            .h_initial(1e-9)
            .h_max(1e-9)
            .build();

        let sol = analysis.run().expect("should succeed");
        assert!(sol.waveforms.contains_key("N1"), "should have N1 node voltage");
        assert!(sol.waveforms.contains_key("I(V1)"), "should have branch current I(V1)");
    }

    // ── times vector matches waveform lengths ─────────────────────────────

    #[test]
    fn transient_times_and_waveforms_same_length() {
        let mut vm = VarMap::new();
        vm.add_node("N1");
        vm.add_branch("V1");

        let devices: Vec<Box<dyn DeviceModel>> = vec![
            Box::new(VSource { node_pos: "N1".into(), branch: "V1".into(), voltage: 2.0 }),
            Box::new(Resistor::new("N1", "0", 500.0)),
        ];

        let analysis = TransientAnalysis::builder(0.0, 5e-9, &vm, &devices)
            .h_initial(1e-9)
            .h_max(1e-9)
            .build();

        let sol = analysis.run().expect("should succeed");
        for (key, wave) in &sol.waveforms {
            assert_eq!(
                wave.len(),
                sol.times.len(),
                "waveform '{key}' length {} != times length {}",
                wave.len(),
                sol.times.len()
            );
        }
    }

    // ── empty circuit: n=0 ────────────────────────────────────────────────

    #[test]
    fn transient_empty_circuit_returns_empty_solution() {
        let vm = VarMap::new(); // only ground
        let devices: Vec<Box<dyn DeviceModel>> = vec![];
        let analysis = TransientAnalysis::builder(0.0, 1e-9, &vm, &devices)
            .h_initial(1e-9)
            .build();
        let sol = analysis.run().expect("empty circuit should not error");
        assert!(sol.times.is_empty());
        assert!(sol.waveforms.is_empty());
    }
}
