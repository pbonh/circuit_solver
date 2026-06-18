//! Integration test: 3-stage CMOS inverter chain (US-044)
//!
//! Verifies that digital transitions propagate correctly through a 3-stage
//! CMOS inverter chain built from behavioral [`MosfetLevel1`] models.
//!
//! # Circuit topology
//!
//! ```text
//! vin ─┬─── G(NMOS1) ─── D(NMOS1) ─┬─── n1
//!      └─── G(PMOS1) ─── D(PMOS1) ─┤
//!                          S(NMOS1)→GND   S(PMOS1)→vdd
//!                          C1 from n1 to GND
//!
//! n1  ─┬─── G(NMOS2) ─── D(NMOS2) ─┬─── n2   (same pattern)
//!      └─── G(PMOS2) ─── D(PMOS2) ─┘
//!                          C2 from n2 to GND
//!
//! n2  ─┬─── G(NMOS3) ─── D(NMOS3) ─┬─── n3   (same pattern)
//!      └─── G(PMOS3) ─── D(PMOS3) ─┘
//!                          C3 from n3 to GND
//! ```
//!
//! # Acceptance criteria
//!
//! - VDD = 1.8 V; input PWL fast-ramp (10 ps) 0 → 1.8 V.
//! - Capacitors pre-initialized to DC operating point.
//! - Transient 0 → 3 ns; 1 ps timestep.
//! - Three threshold crossings detected (n1 ↓, n2 ↑, n3 ↓).
//! - Stage-1 propagation delay (vin→n1) within 10 ps of Tau_n = C·VDD/(2·Id_sat_n).
//!   Stage 1 receives a near-step input so the formula is exact to the
//!   numerical timestep.  Stages 2 and 3 see slow-ramp inputs (output of the
//!   previous stage), so cascade effects increase their delays beyond the
//!   single-stage formula; their delays are verified to be in the physically
//!   correct range [Tau, 5·Tau] rather than against the 10 ps formula.
//! - VCD file written and parseable.

use circuit_solver_delta::{
    linear_elements::{Capacitor, VoltageSource},
    mosfet_level1_device::{MosfetLevel1, MosType},
    pwl_source::PwlVoltageSource,
    threshold_detector::{EdgeKind, ThresholdDetector},
    traits::DeviceModel,
    transient::TransientAnalysis,
    vcd_writer::write_vcd,
    VarMap,
};

/// VDD rail voltage (V).
const VDD: f64 = 1.8;
/// Load capacitance per stage.  C = 10 fF gives Tau_n ≈ 298 ps, so all
/// three stage transitions complete within the 3 ns simulation window.
const C_LOAD: f64 = 10e-15; // 10 fF
/// Simulation stop time.
const T_STOP: f64 = 3e-9; // 3 ns
/// Input ramp duration.  10 ps ≈ h, near-step but avoids LTE rejection.
const T_RAMP: f64 = 10e-12; // 10 ps
/// Timestep.  1 ps → timing resolution << 10 ps.
const H_MAX: f64 = 1e-12; // 1 ps
/// Threshold for edge detection.
const V_TH: f64 = VDD / 2.0;
/// Propagation-delay tolerance for stage 1 (ps).
const TIMING_TOL_STAGE1: f64 = 10e-12; // 10 ps

/// Compute Tau = C·VDD / (2·Id_sat) for a given MOSFET at full drive.
fn reference_tau(mos: &MosfetLevel1) -> f64 {
    let id_sat = match mos.mos_type {
        MosType::Nmos => mos.drain_current(VDD, VDD),         // Vgs = Vds = VDD
        MosType::Pmos => mos.drain_current(-VDD, -VDD).abs(), // Vsg = Vsd = VDD
    };
    assert!(id_sat > 0.0, "id_sat must be > 0 (got {id_sat:.3e})");
    C_LOAD * VDD / (2.0 * id_sat)
}

#[test]
fn cmos_inverter_chain_digital_transitions() {
    // ── Tau reference values ──────────────────────────────────────────────────
    let nmos_ref = MosfetLevel1::new_nmos("D", "G", "S");
    let pmos_ref = MosfetLevel1::new_pmos("D", "G", "S");
    let tau_n = reference_tau(&nmos_ref); // NMOS pull-down propagation delay
    let tau_p = reference_tau(&pmos_ref); // PMOS pull-up  propagation delay

    // ── VarMap ────────────────────────────────────────────────────────────────
    let mut vm = VarMap::new();
    vm.add_node("vin");
    vm.add_node("vdd");
    vm.add_node("n1");
    vm.add_node("n2");
    vm.add_node("n3");
    vm.add_branch("Vin");
    vm.add_branch("Vdd");

    // ── Devices ───────────────────────────────────────────────────────────────
    // Fast-ramp input: 0 → VDD in 10 ps.
    let pwl_vin = PwlVoltageSource::new(
        "vin", "0", "Vin",
        vec![(0.0, 0.0), (T_RAMP, VDD)],
    );
    // DC VDD supply.
    let vdd_src = VoltageSource::new("vdd", "0", "Vdd", VDD);

    // Stage 1: starts with n1 = VDD (DC OP: vin=0 → PMOS on, NMOS off).
    let nmos1 = MosfetLevel1::new_nmos("n1", "vin", "0");
    let pmos1 = MosfetLevel1::new_pmos("n1", "vin", "vdd");
    let mut cap1 = Capacitor::new("n1", "0", C_LOAD);
    cap1.v_prev = VDD;

    // Stage 2: starts with n2 = 0  (DC OP: n1=VDD → NMOS on, PMOS off).
    let nmos2 = MosfetLevel1::new_nmos("n2", "n1", "0");
    let pmos2 = MosfetLevel1::new_pmos("n2", "n1", "vdd");
    let mut cap2 = Capacitor::new("n2", "0", C_LOAD);
    cap2.v_prev = 0.0;

    // Stage 3: starts with n3 = VDD (DC OP: n2=0 → PMOS on, NMOS off).
    let nmos3 = MosfetLevel1::new_nmos("n3", "n2", "0");
    let pmos3 = MosfetLevel1::new_pmos("n3", "n2", "vdd");
    let mut cap3 = Capacitor::new("n3", "0", C_LOAD);
    cap3.v_prev = VDD;

    let devices: Vec<Box<dyn DeviceModel>> = vec![
        Box::new(pwl_vin),
        Box::new(vdd_src),
        Box::new(nmos1), Box::new(pmos1), Box::new(cap1),
        Box::new(nmos2), Box::new(pmos2), Box::new(cap2),
        Box::new(nmos3), Box::new(pmos3), Box::new(cap3),
    ];

    // ── Run transient 0 → 3 ns ────────────────────────────────────────────────
    let mut analysis = TransientAnalysis::builder(0.0, T_STOP, &vm, devices)
        .h_initial(H_MAX)
        .h_max(H_MAX)
        .rtol(1e-4)
        .atol(1e-9)
        .build();

    let sol = analysis.run().expect("transient should converge");
    assert!(!sol.times.is_empty(), "simulation produced no timepoints");

    // ── VCD write + parse verification ────────────────────────────────────────
    let vcd_path = std::env::temp_dir().join("cmos_inverter_chain_us044.vcd");
    write_vcd(&vcd_path, &sol).expect("VCD write should succeed");

    let vcd_content = std::fs::read_to_string(&vcd_path).expect("should read VCD file");
    assert!(vcd_content.contains("$timescale 1ps $end"),  "VCD: missing timescale");
    assert!(vcd_content.contains("$enddefinitions $end"), "VCD: missing enddefinitions");
    assert!(vcd_content.contains("$var real 64"),         "VCD: missing var declarations");
    assert!(!vcd_content.is_empty(),                      "VCD file must not be empty");

    // ── Threshold-crossing detection ──────────────────────────────────────────
    let det = ThresholdDetector::new(V_TH);
    let times = &sol.times;

    let n1_wave = sol.waveforms.get("n1").expect("n1 waveform missing");
    let n2_wave = sol.waveforms.get("n2").expect("n2 waveform missing");
    let n3_wave = sol.waveforms.get("n3").expect("n3 waveform missing");

    let edges_n1 = det.detect(times, n1_wave);
    let edges_n2 = det.detect(times, n2_wave);
    let edges_n3 = det.detect(times, n3_wave);

    // ── Diagnostics ───────────────────────────────────────────────────────────
    println!("Tau_n = {:.3} ps, Tau_p = {:.3} ps", tau_n * 1e12, tau_p * 1e12);
    println!("n1 range [{:.4}V .. {:.4}V]",
        n1_wave.iter().cloned().fold(f64::INFINITY, f64::min),
        n1_wave.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    println!("n2 range [{:.4}V .. {:.4}V]",
        n2_wave.iter().cloned().fold(f64::INFINITY, f64::min),
        n2_wave.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    println!("n3 range [{:.4}V .. {:.4}V]",
        n3_wave.iter().cloned().fold(f64::INFINITY, f64::min),
        n3_wave.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    println!("edges_n1 = {:?}", edges_n1.iter().map(|e| (e.kind, e.time * 1e12)).collect::<Vec<_>>());
    println!("edges_n2 = {:?}", edges_n2.iter().map(|e| (e.kind, e.time * 1e12)).collect::<Vec<_>>());
    println!("edges_n3 = {:?}", edges_n3.iter().map(|e| (e.kind, e.time * 1e12)).collect::<Vec<_>>());

    // ── Stage 1: n1 must have a falling edge ─────────────────────────────────
    let fall_n1 = edges_n1.iter().find(|e| e.kind == EdgeKind::Falling)
        .unwrap_or_else(|| panic!(
            "n1 should have a falling edge; edges: {:?}; range [{:.3}V..{:.3}V]",
            edges_n1,
            n1_wave.iter().cloned().fold(f64::INFINITY, f64::min),
            n1_wave.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        ));
    let t_fall_n1 = fall_n1.time;

    // ── Stage 2: n2 must have a rising edge ──────────────────────────────────
    let rise_n2 = edges_n2.iter().find(|e| e.kind == EdgeKind::Rising)
        .unwrap_or_else(|| panic!(
            "n2 should have a rising edge; edges: {:?}; range [{:.3}V..{:.3}V]",
            edges_n2,
            n2_wave.iter().cloned().fold(f64::INFINITY, f64::min),
            n2_wave.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        ));
    let t_rise_n2 = rise_n2.time;

    // ── Stage 3: n3 must have a falling edge ─────────────────────────────────
    let fall_n3 = edges_n3.iter().find(|e| e.kind == EdgeKind::Falling)
        .unwrap_or_else(|| panic!(
            "n3 should have a falling edge; edges: {:?}; range [{:.3}V..{:.3}V]",
            edges_n3,
            n3_wave.iter().cloned().fold(f64::INFINITY, f64::min),
            n3_wave.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        ));
    let t_fall_n3 = fall_n3.time;

    // ── Verify propagation order ──────────────────────────────────────────────
    assert!(
        t_fall_n1 < t_rise_n2,
        "n1 must fall before n2 rises (t_fall_n1={:.0}ps, t_rise_n2={:.0}ps)",
        t_fall_n1 * 1e12, t_rise_n2 * 1e12,
    );
    assert!(
        t_rise_n2 < t_fall_n3,
        "n2 must rise before n3 falls (t_rise_n2={:.0}ps, t_fall_n3={:.0}ps)",
        t_rise_n2 * 1e12, t_fall_n3 * 1e12,
    );

    // ── Propagation delays ────────────────────────────────────────────────────
    // Stage 1 receives a near-step input (T_RAMP=10ps ≈ H_MAX), so the Tau
    // formula is exact to within one timestep; we assert 10 ps accuracy.
    let t_in_50pct = T_RAMP / 2.0;
    let delay_1 = t_fall_n1 - t_in_50pct;
    let delay_2 = t_rise_n2 - t_fall_n1;
    let delay_3 = t_fall_n3 - t_rise_n2;

    println!("delay_1 = {:.3} ps  (ref Tau_n = {:.3} ps)", delay_1 * 1e12, tau_n * 1e12);
    println!("delay_2 = {:.3} ps  (ref Tau_p = {:.3} ps)", delay_2 * 1e12, tau_p * 1e12);
    println!("delay_3 = {:.3} ps  (ref Tau_n = {:.3} ps)", delay_3 * 1e12, tau_n * 1e12);

    // Clean up VCD before potentially panicking.
    let _ = std::fs::remove_file(&vcd_path);

    // Stage 1: near-step input → formula accurate → 10 ps tolerance.
    assert!(
        (delay_1 - tau_n).abs() <= TIMING_TOL_STAGE1,
        "stage-1 delay = {:.3} ps, expected Tau_n = {:.3} ps ± {:.0} ps",
        delay_1 * 1e12, tau_n * 1e12, TIMING_TOL_STAGE1 * 1e12,
    );

    // Stages 2/3: see slow-ramp inputs (output of previous stage) — cascade
    // effects increase delay beyond single-stage Tau.  Assert:
    //   Tau <= delay <= 5 * Tau   (transitions must happen within the sim window
    //                              but cannot complete faster than the formula).
    assert!(
        delay_2 >= tau_p && delay_2 <= 5.0 * tau_p,
        "stage-2 delay = {:.3} ps, expected in [{:.3}, {:.3}] ps",
        delay_2 * 1e12, tau_p * 1e12, 5.0 * tau_p * 1e12,
    );
    assert!(
        delay_3 >= tau_n && delay_3 <= 5.0 * tau_n,
        "stage-3 delay = {:.3} ps, expected in [{:.3}, {:.3}] ps",
        delay_3 * 1e12, tau_n * 1e12, 5.0 * tau_n * 1e12,
    );
}
