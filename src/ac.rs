//! AC small-signal frequency sweep analysis.
//!
//! # Algorithm
//!
//! 1. Obtain the DC operating point via one Newton-Raphson solve (linearises
//!    nonlinear devices at their quiescent point).
//! 2. Assemble the small-signal conductance matrix **G** and capacitance matrix
//!    **C** from the device stamps at the DC solution:
//!    - G-only stamp: set h = 1e30 so capacitor G_eff = C/h ≈ 0.
//!    - G+C stamp:    set h = 1.0  so capacitor G_eff = C/1 = C.
//!    - C_mat = (G+C stamp) - (G-only stamp).
//! 3. For each log-spaced frequency `f` in `[f_start, f_stop]`:
//!    - ω = 2π·f
//!    - Solve `(G + jωC)·V = b_ac` via the real/imag 2N×2N interleaved system:
//!
//!      ```text
//!      [ G    -ωC ] [ Vr ]   [ br ]
//!      [ ωC    G  ] [ Vi ] = [ bi ]
//!      ```
//!
//!    - b_ac is the DC RHS from the G+C stamp (independent source contributions).
//!    - bi = 0 (real-valued sources).
//! 4. Return `AcSolution { freqs, voltages }`.

use std::collections::HashMap;

use crate::{sparse_lu::SparseLU, traits::DeviceModel, MnaMatrix, NewtonRaphson, VarMap};

// ── Public types ──────────────────────────────────────────────────────────────

/// Result of an AC frequency sweep.
#[derive(Debug, Clone)]
pub struct AcSolution {
    /// Log-spaced frequencies in Hz.
    pub freqs: Vec<f64>,
    /// Voltage phasors at each non-ground node.
    ///
    /// Key: node name (as registered in `VarMap`).
    /// Value: one `(real, imag)` pair per frequency point.
    pub voltages: HashMap<String, Vec<(f64, f64)>>,
}

/// AC small-signal frequency sweep analyser.
pub struct AcAnalysis {
    /// Start frequency in Hz (must be > 0).
    pub f_start: f64,
    /// Stop frequency in Hz (must be ≥ f_start).
    pub f_stop: f64,
    /// Number of log-spaced frequency points (must be ≥ 1).
    pub n_points: usize,
    /// Variable map for the circuit.
    pub var_map: VarMap,
    /// Device list (boxed trait objects).
    pub devices: Vec<Box<dyn DeviceModel>>,
}

impl AcAnalysis {
    /// Create a new `AcAnalysis`.
    pub fn new(
        f_start: f64,
        f_stop: f64,
        n_points: usize,
        var_map: VarMap,
        devices: Vec<Box<dyn DeviceModel>>,
    ) -> Self {
        assert!(f_start > 0.0, "f_start must be positive");
        assert!(f_stop >= f_start, "f_stop must be >= f_start");
        assert!(n_points >= 1, "n_points must be >= 1");
        AcAnalysis { f_start, f_stop, n_points, var_map, devices }
    }

    /// Run the AC sweep.
    ///
    /// Requires `&mut self` to call [`DeviceModel::set_timestep`] on reactive
    /// devices during matrix assembly.
    ///
    /// # Errors
    /// Returns a `String` if the DC operating-point NR solve fails or if the
    /// system matrix is singular at any frequency point.
    pub fn run(&mut self) -> Result<AcSolution, String> {
        let n = self.var_map.len() - 1; // exclude ground row (index 0)

        // ── 1. DC operating point ─────────────────────────────────────────────
        let dc_sol = NewtonRaphson::default()
            .solve(n, &self.devices, &self.var_map)
            .map_err(|e| format!("DC operating-point failed: {e}"))?;

        // ── 2. G-only stamp (h = 1e30 → cap G_eff ≈ 0) ───────────────────────
        for dev in &mut self.devices {
            dev.set_timestep(1.0e30);
        }
        let mut mna_g = MnaMatrix::new(n);
        for dev in &self.devices {
            dev.stamp_nonlinear(&mut mna_g, &self.var_map, &dc_sol);
        }
        let csr_g = mna_g.to_csr();

        // ── 3. (G+C) stamp (h = 1.0 → cap G_eff = C) ────────────────────────
        for dev in &mut self.devices {
            dev.set_timestep(1.0);
        }
        let mut mna_gc = MnaMatrix::new(n);
        for dev in &self.devices {
            dev.stamp_nonlinear(&mut mna_gc, &self.var_map, &dc_sol);
        }
        let csr_gc = mna_gc.to_csr();
        let b_ac: Vec<f64> = csr_gc.rhs.clone();

        // ── 4. Extract dense G and C matrices ────────────────────────────────
        let mut g_dense = vec![0.0f64; n * n];
        let mut c_dense = vec![0.0f64; n * n];
        for r in 0..n {
            for c in 0..n {
                let g_val  = csr_g.get(r, c);
                let gc_val = csr_gc.get(r, c);
                g_dense[r * n + c] = g_val;
                c_dense[r * n + c] = gc_val - g_val; // C_eff = (G+C) - G
            }
        }

        // ── 5. Log-spaced frequency vector ────────────────────────────────────
        let freqs: Vec<f64> = if self.n_points == 1 {
            vec![self.f_start]
        } else {
            let log_start = self.f_start.log10();
            let log_stop  = self.f_stop.log10();
            (0..self.n_points)
                .map(|i| {
                    let t = i as f64 / (self.n_points - 1) as f64;
                    10.0_f64.powf(log_start + t * (log_stop - log_start))
                })
                .collect()
        };

        // ── 6. Node names (non-ground) ────────────────────────────────────────
        // VarMap indices: 0 = ground, 1..=n cover nodes and branches in order.
        // MNA rows correspond to indices 1..=n (offset by -1).
        let node_names: Vec<String> = (1..=n)
            .filter_map(|idx| self.var_map.var_name(idx).map(String::from))
            .collect();

        // ── 7. Frequency sweep ─────────────────────────────────────────────────
        let mut voltage_map: HashMap<String, Vec<(f64, f64)>> = node_names
            .iter()
            .map(|name| (name.clone(), Vec::with_capacity(freqs.len())))
            .collect();

        for &f in &freqs {
            let omega = 2.0 * std::f64::consts::PI * f;
            let n2 = 2 * n;

            // Build 2N×2N system:
            //   [ G    -ωC ] [ Vr ]   [ br ]
            //   [ ωC    G  ] [ Vi ] = [ 0  ]
            let mut mna2 = MnaMatrix::new(n2);

            for r in 0..n {
                for c in 0..n {
                    let gv = g_dense[r * n + c];
                    let cv = c_dense[r * n + c];
                    // Top-left: G
                    if gv != 0.0 {
                        mna2.stamp(r, c, gv);
                    }
                    // Top-right: -ωC
                    let neg_wc = -omega * cv;
                    if neg_wc != 0.0 {
                        mna2.stamp(r, n + c, neg_wc);
                    }
                    // Bottom-left: ωC
                    let wc = omega * cv;
                    if wc != 0.0 {
                        mna2.stamp(n + r, c, wc);
                    }
                    // Bottom-right: G
                    if gv != 0.0 {
                        mna2.stamp(n + r, n + c, gv);
                    }
                }
            }

            // RHS: [br; 0]
            for (r, &b) in b_ac.iter().enumerate() {
                if b != 0.0 {
                    mna2.stamp_rhs(r, b);
                }
            }

            let csr2 = mna2.to_csr();
            let lu = SparseLU::factorize(&csr2)
                .map_err(|e| format!("SparseLU singular at f={f:.3e} Hz: {e}"))?;
            let x2 = lu.solve(&csr2.rhs);

            // x2[0..n] = Vr,  x2[n..2n] = Vi
            for (i, name) in node_names.iter().enumerate() {
                let vr = x2[i];
                let vi = x2[n + i];
                voltage_map.get_mut(name).unwrap().push((vr, vi));
            }
        }

        Ok(AcSolution { freqs, voltages: voltage_map })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        linear_elements::{Capacitor, Resistor},
        stamper::stamp_voltage_source,
        traits::DeviceModel,
        MnaMatrix, VarMap,
    };

    /// RC low-pass filter: R=1 kΩ, C=1 µF, Vin=1 V AC.
    ///
    /// Corner frequency fc = 1/(2π·R·C) ≈ 159.15 Hz.
    /// At fc, |H| = 1/√2 ≈ -3.01 dB.
    /// Acceptance criterion: magnitude at the closest sweep point is within
    /// 1 dB of -3 dB (i.e. in [-4 dB, -2 dB]).
    #[test]
    fn rc_lowpass_magnitude_at_corner_frequency_within_1db_of_3db() {
        let r_val = 1_000.0_f64; // 1 kΩ
        let c_val = 1.0e-6_f64;  // 1 µF
        let fc = 1.0 / (2.0 * std::f64::consts::PI * r_val * c_val); // ≈ 159.15 Hz

        // VarMap: ground=0, N1=1, N2=2, V1=3 (branch current for V-source).
        let mut var_map = VarMap::new();
        var_map.add_node("N1");
        var_map.add_node("N2");
        var_map.add_branch("V1");

        // Independent 1 V AC voltage source (N1 to ground).
        struct VSource {
            node_pos: String,
            branch:   String,
            voltage:  f64,
        }
        impl DeviceModel for VSource {
            fn terminals(&self) -> Vec<String> {
                vec![self.node_pos.clone()]
            }
            fn stamp_linear(&self, m: &mut MnaMatrix, vm: &VarMap) {
                let np = vm.node_index(&self.node_pos);
                let br = vm.node_index(&self.branch).expect("branch in varmap");
                let to_row = |idx: Option<usize>| match idx {
                    Some(0) | None => None,
                    Some(i) => Some(i - 1),
                };
                stamp_voltage_source(m, to_row(np), None, br - 1, self.voltage);
            }
            fn stamp_nonlinear(
                &self,
                m: &mut MnaMatrix,
                vm: &VarMap,
                _: &[f64],
            ) {
                self.stamp_linear(m, vm);
            }
            fn is_smooth(&self) -> bool {
                true
            }
        }

        let devices: Vec<Box<dyn DeviceModel>> = vec![
            Box::new(VSource {
                node_pos: "N1".into(),
                branch:   "V1".into(),
                voltage:  1.0,
            }),
            Box::new(Resistor::new("N1", "N2", r_val)),
            Box::new(Capacitor::new("N2", "0", c_val)),
        ];

        // Sweep 3 decades around fc: 10 Hz – 100 kHz, 50 log-spaced points.
        let mut ac = AcAnalysis::new(10.0, 100_000.0, 50, var_map, devices);
        let sol = ac.run().expect("AC sweep must succeed");

        // Find the frequency point closest to fc.
        let fc_idx = sol
            .freqs
            .iter()
            .enumerate()
            .min_by(|&(_, a), &(_, b)| {
                (a - fc).abs().partial_cmp(&(b - fc).abs()).unwrap()
            })
            .map(|(i, _)| i)
            .expect("at least one frequency point");

        let (vr, vi) = sol.voltages["N2"][fc_idx];
        let magnitude = (vr * vr + vi * vi).sqrt();
        let magnitude_db = 20.0 * magnitude.log10();

        assert!(
            magnitude_db >= -4.0 && magnitude_db <= -2.0,
            "Expected |V(N2)| at fc in [-4, -2] dB, got {magnitude_db:.2} dB \
             (|V| = {magnitude:.4}, f_nearest = {:.2} Hz, fc = {fc:.2} Hz)",
            sol.freqs[fc_idx]
        );
    }
}
