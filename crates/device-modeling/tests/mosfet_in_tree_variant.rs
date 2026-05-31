//! Integration test: MOSFET in-tree variant dispatch (ADR-0005 + ADR-0007).
//!
//! Validates that the closed-enum `DeviceModel::MOSFET` dispatches correctly
//! to Level-1 (Shichman-Hodges) and BSIM3v3 stamps and that the codegen seam
//! (ADR-0007) produces variants that exercise the full
//! params → linearize → stamp pipeline.
//!
//! This test lives under `project/` so it can depend on multiple workspace
//! crates without violating the per-crate dependency boundaries enforced by
//! ADR-0008.

use circuit_solver_types::ModelName;
use device_modeling::{
    DeviceFamily, DeviceModel,
    MosLevel1Params, MosPolarity, MOSFETParams,
};
use device_modeling::stamp::{
    linearize_mosfet, LinearizedModel, MOSFET_TERMINALS,
};

// ---------------------------------------------------------------------------
// 1. MOSFETParams::Level1 constructs and is reachable via DeviceModel
// ---------------------------------------------------------------------------

#[test]
fn mosfet_level1_in_closed_enum() {
    let params = MOSFETParams::Level1(MosLevel1Params {
        name: ModelName::new("nmos_l1"),
        polarity: MosPolarity::Nmos,
        vto: 0.7,
        kp: 120.0e-6,
        ..MosLevel1Params::default()
    });
    let model = DeviceModel::MOSFET(params);
    match &model {
        DeviceModel::MOSFET(MOSFETParams::Level1(p)) => {
            assert_eq!(p.vto, 0.7);
            assert_eq!(p.kp, 120.0e-6);
        }
        _ => panic!("expected MOSFET Level1 variant"),
    }
}

// ---------------------------------------------------------------------------
// 2. MOSFETParams::BSIM3v3 constructs and is reachable via DeviceModel
// ---------------------------------------------------------------------------

#[test]
fn mosfet_bsim3v3_in_closed_enum() {
    use device_modeling::MosBSIM3v3Params;
    let mut raw = std::collections::BTreeMap::new();
    raw.insert("vth0".to_string(), 0.4);
    raw.insert("u0".to_string(), 0.06);
    let params = MOSFETParams::BSIM3v3(MosBSIM3v3Params {
        name: ModelName::new("nmos_bsim3"),
        polarity: MosPolarity::Nmos,
        raw,
    });
    let model = DeviceModel::MOSFET(params);
    match &model {
        DeviceModel::MOSFET(MOSFETParams::BSIM3v3(p)) => {
            assert_eq!(p.raw.get("vth0"), Some(&0.4));
        }
        _ => panic!("expected MOSFET BSIM3v3 variant"),
    }
}

// ---------------------------------------------------------------------------
// 3. MOSFETParams dispatches linearize_mosfet to Level-1 branch
// ---------------------------------------------------------------------------

#[test]
fn linearize_dispatches_level1() {
    let params = MOSFETParams::Level1(MosLevel1Params {
        name: ModelName::new("nmos_lin"),
        polarity: MosPolarity::Nmos,
        vto: 1.0,
        kp: 50.0e-6,
        lambda: 0.02,
        gamma: 0.5,
        phi: 0.6,
        ..MosLevel1Params::default()
    });
    let v: [f64; 4] = [5.0, 3.0, 0.0, 0.0];
    let lin = linearize_mosfet(&params, &v);

    // The MOSFETLinearization should have 4×4 Jacobian (MOSFET_TERMINALS)
    assert_eq!(MOSFET_TERMINALS, 4);
    assert_eq!(lin.jacobian.len(), 4);
    assert_eq!(lin.jacobian[0].len(), 4);
    assert_eq!(lin.companion_current.len(), 4);

    // Drain current should be positive for NMOS in saturation
    let ids = lin.companion_current[0]
        + lin.jacobian[0][0] * v[0]
        + lin.jacobian[0][1] * v[1]
        + lin.jacobian[0][2] * v[2]
        + lin.jacobian[0][3] * v[3];
    assert!(ids > 0.0, "NMOS saturation: ids should be positive, got {ids}");
}

// ---------------------------------------------------------------------------
// 4. PMOS Level-1: polarity flips drain-current sign
// ---------------------------------------------------------------------------

#[test]
fn pmos_level1_drain_current_is_negative() {
    let params = MOSFETParams::Level1(MosLevel1Params {
        name: ModelName::new("pmos_lin"),
        polarity: MosPolarity::Pmos,
        vto: -1.0,
        kp: 50.0e-6,
        lambda: 0.02,
        gamma: 0.5,
        phi: 0.6,
        ..MosLevel1Params::default()
    });
    // PMOS: source at highest potential, drain lower
    // V_D = 0.0, V_G = 0.0, V_S = 5.0, V_B = 5.0
    // → V_GS = -5.0, V_DS = -5.0 → saturation
    let v: [f64; 4] = [0.0, 0.0, 5.0, 5.0];
    let lin = linearize_mosfet(&params, &v);

    let ids = lin.companion_current[0]
        + lin.jacobian[0][0] * v[0]
        + lin.jacobian[0][1] * v[1]
        + lin.jacobian[0][2] * v[2]
        + lin.jacobian[0][3] * v[3];
    assert!(ids < 0.0, "PMOS saturation: ids should be negative, got {ids}");
}

// ---------------------------------------------------------------------------
// 5. Level-1 cutoff: V_GS < V_th → zero drain current
// ---------------------------------------------------------------------------

#[test]
fn nmos_level1_cutoff_zero_current() {
    let params = MOSFETParams::Level1(MosLevel1Params {
        name: ModelName::new("nmos_cutoff"),
        polarity: MosPolarity::Nmos,
        vto: 1.0,
        kp: 50.0e-6,
        ..MosLevel1Params::default()
    });
    // V_GS = 0.5 < VTO = 1.0 → cutoff
    let v: [f64; 4] = [0.0, 0.5, 0.0, 0.0];
    let lin = linearize_mosfet(&params, &v);

    // In cutoff, the Jacobian drain row and companion should be essentially zero
    let ids = lin.companion_current[0]
        + lin.jacobian[0][0] * v[0]
        + lin.jacobian[0][1] * v[1]
        + lin.jacobian[0][2] * v[2]
        + lin.jacobian[0][3] * v[3];
    assert!(
        ids.abs() < 1e-20,
        "NMOS cutoff: ids should be ~0, got {ids}"
    );
}

// ---------------------------------------------------------------------------
// 6. LinearizedModel::MOSFET round-trips through the enum
// ---------------------------------------------------------------------------

#[test]
fn linearized_model_mosfet_roundtrip() {
    let params = MOSFETParams::Level1(MosLevel1Params {
        name: ModelName::new("nmos_rt"),
        polarity: MosPolarity::Nmos,
        vto: 1.0,
        kp: 50.0e-6,
        ..MosLevel1Params::default()
    });
    let lin = linearize_mosfet(&params, &[5.0, 3.0, 0.0, 0.0]);
    let model = LinearizedModel::MOSFET(lin.clone());

    // Must destructure as MOSFET variant
    match &model {
        LinearizedModel::MOSFET(m) => {
            assert_eq!(m.jacobian, lin.jacobian);
            assert_eq!(m.companion_current, lin.companion_current);
        }
        _ => panic!("expected LinearizedModel::MOSFET"),
    }
}

// ---------------------------------------------------------------------------
// 7. DeviceFamily::MOSFET is produced by closed-enum match
// ---------------------------------------------------------------------------

#[test]
fn mosfet_produces_mosfet_family() {
    let model = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params::default()));
    let family = match &model {
        DeviceModel::Diode(_) => DeviceFamily::Diode,
        DeviceModel::BJT(_) => DeviceFamily::BJT,
        DeviceModel::MOSFET(_) => DeviceFamily::MOSFET,
    };
    assert_eq!(family, DeviceFamily::MOSFET);
}
