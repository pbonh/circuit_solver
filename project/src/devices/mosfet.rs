//! MOSFET device model — project-level integration for the MOSFET
//! family of the closed-enum [`DeviceModel`](crate::devices::DeviceModel).
//!
//! This module re-exports the MOSFET-specific linearization helpers,
//! parameter sub-types, and polarity discriminator from the
//! `device-modeling` crate, and provides conformance tests that
//! exercise the monomorphized dispatch path
//! `DeviceModel::MOSFET → linearize() → LinearizedModel::MOSFET`
//! for all three level variants (Level-1, BSIM3v3, BSIM4).
//!
//! # Closed-enum discipline (ADR-0005)
//!
//! MOSFET is a single `DeviceModel::MOSFET(MOSFETParams)` variant.
//! `MOSFETParams` is itself a closed sub-enum over the supported
//! MOS levels. Adding a new level (e.g. BSIM6) is a compile-time
//! breaking change that updates every `match` on `MOSFETParams` —
//! the same exhaustiveness property ADR-0005 buys for `DeviceModel`
//! itself.
//!
//! # Terminal ordering
//!
//! All MOSFET stamps use the SPICE-canonical 4-terminal ordering
//! `[drain, gate, source, bulk]`, carried by
//! [`MOSFET_TERMINALS`](crate::devices::model::MOSFET_TERMINALS).

// Re-export the top-level MOSFET linearization dispatcher and
// per-level helpers for direct use in conformance tests.
pub use device_modeling::stamp::{
    linearize_mosfet, linearize_mosfet_level1, linearize_mosfet_bsim4,
};

// Re-export the BSIM3v3 helper (declared as a sibling module in
// device-modeling, not under stamp/).
pub use device_modeling::linearize_bsim3v3;

// Re-export per-level parameter structs and the polarity
// discriminator so callers can construct `MOSFETParams` variants
// without depending on `device-modeling` directly.
pub use device_modeling::params::{MosLevel1Params, MosBSIM3v3Params, MosBSIM4Params, MosPolarity};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::model::{
        DeviceModel, DeviceFamily, LinearizedModel, OperatingPoint,
        MOSFETLinearization, MOSFETParams, MOSFET_TERMINALS,
    };
    use circuit_solver_types::ModelName;

    // -----------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------

    /// Long-channel NMOS Level-1 with textbook parameters.
    fn nmos_level1() -> MosLevel1Params {
        MosLevel1Params {
            name: ModelName::new("nmos_l1"),
            polarity: MosPolarity::Nmos,
            vto: 1.0,
            kp: 50.0e-6,
            lambda: 0.02,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        }
    }

    /// PMOS Level-1 (enhancement, VTO < 0 per SPICE convention).
    fn pmos_level1() -> MosLevel1Params {
        MosLevel1Params {
            name: ModelName::new("pmos_l1"),
            polarity: MosPolarity::Pmos,
            vto: -1.0,
            kp: 25.0e-6,
            lambda: 0.02,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        }
    }

    /// NMOS BSIM3v3 with a minimal parameter set.
    fn nmos_bsim3() -> MosBSIM3v3Params {
        MosBSIM3v3Params {
            name: ModelName::new("nmos_b3"),
            polarity: MosPolarity::Nmos,
            raw: std::collections::BTreeMap::new(),
        }
    }

    /// NMOS BSIM4 with SPICE defaults (long-channel textbook device).
    fn nmos_bsim4() -> MosBSIM4Params {
        MosBSIM4Params {
            name: ModelName::new("nmos_b4"),
            polarity: MosPolarity::Nmos,
            ..Default::default()
        }
    }

    /// PMOS BSIM4 with SPICE defaults.
    fn pmos_bsim4() -> MosBSIM4Params {
        MosBSIM4Params {
            name: ModelName::new("pmos_b4"),
            polarity: MosPolarity::Pmos,
            ..Default::default()
        }
    }

    /// Recover terminal current at iterate `v` from a stamp's row `t`
    /// using the companion-model identity
    /// `I_t = Σ_u J[t][u] · v[u] + i_eq[t]`.
    fn reconstruct_current(
        lin: &MOSFETLinearization,
        t: usize,
        v: &[f64; MOSFET_TERMINALS],
    ) -> f64 {
        lin.companion_current[t]
            + lin.jacobian[t]
                .iter()
                .zip(v.iter())
                .map(|(j, vu)| j * vu)
                .sum::<f64>()
    }

    /// f64 approximate equality blending relative and absolute tolerance.
    fn approx_eq(a: f64, b: f64, rel: f64, abs: f64) -> bool {
        let tol = rel.mul_add(b.abs().max(a.abs()), abs);
        (a - b).abs() <= tol
    }

    // -----------------------------------------------------------------
    // DeviceModel::MOSFET variant exists and reports MOSFET family
    // -----------------------------------------------------------------

    #[test]
    fn mosfet_variant_reports_mosfet_family() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(nmos_level1()));
        assert_eq!(m.family(), DeviceFamily::MOSFET);
        assert_eq!(m.name().as_str(), "nmos_l1");
    }

    #[test]
    fn mosfet_bsim3_variant_reports_mosfet_family() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM3v3(nmos_bsim3()));
        assert_eq!(m.family(), DeviceFamily::MOSFET);
        assert_eq!(m.name().as_str(), "nmos_b3");
    }

    #[test]
    fn mosfet_bsim4_variant_reports_mosfet_family() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM4(nmos_bsim4()));
        assert_eq!(m.family(), DeviceFamily::MOSFET);
        assert_eq!(m.name().as_str(), "nmos_b4");
    }

    // -----------------------------------------------------------------
    // Level-1 dispatch through DeviceModel::linearize
    // -----------------------------------------------------------------

    #[test]
    fn level1_cutoff_via_device_model_linearize() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(nmos_level1()));
        let op = OperatingPoint::MOSFET([0.0; MOSFET_TERMINALS]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::MOSFET(mos) => {
                assert_eq!(mos, MOSFETLinearization::zero(),
                    "cutoff must produce zero linearization");
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    #[test]
    fn level1_saturation_via_device_model_linearize() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(nmos_level1()));
        // Vgs = 3 V > Vto = 1 V, Vds = 5 V > Vov = 2 V → saturation.
        let op = OperatingPoint::MOSFET([5.0, 3.0, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::MOSFET(mos) => {
                // gm = KP · Vov > 0 in saturation.
                assert!(mos.jacobian[0][1] > 0.0,
                    "gm must be positive in saturation, got {}", mos.jacobian[0][1]);
                // gds > 0 with LAMBDA = 0.02.
                assert!(mos.jacobian[0][0] > 0.0,
                    "gds must be positive with LAMBDA > 0, got {}", mos.jacobian[0][0]);
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    #[test]
    fn level1_pmos_saturation_via_device_model_linearize() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(pmos_level1()));
        // PMOS: V_s = 3.3, V_g = 1.3, V_d = 0.3 → saturation.
        let op = OperatingPoint::MOSFET([0.3, 1.3, 3.3, 3.3]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::MOSFET(mos) => {
                let v = [0.3_f64, 1.3, 3.3, 3.3];
                let id = reconstruct_current(&mos, 0, &v);
                assert!(id < 0.0,
                    "PMOS drain current must be negative, got {id}");
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // BSIM3v3 dispatch through DeviceModel::linearize
    // -----------------------------------------------------------------

    #[test]
    fn bsim3v3_dispatch_via_device_model_linearize() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM3v3(nmos_bsim3()));
        let op = OperatingPoint::MOSFET([3.0, 1.8, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::MOSFET(mos) => {
                // Jacobian entries must be finite (real implementation contract).
                for row in &mos.jacobian {
                    for &j in row {
                        assert!(j.is_finite(), "BSIM3v3 Jacobian must be finite, got {j}");
                    }
                }
                for &i in &mos.companion_current {
                    assert!(i.is_finite(), "BSIM3v3 companion current must be finite, got {i}");
                }
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // BSIM4 dispatch through DeviceModel::linearize
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_nmos_saturation_via_device_model_linearize() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM4(nmos_bsim4()));
        // Vgs = 1.8 > Vth0 = 0.7 → saturation.
        let op = OperatingPoint::MOSFET([3.0, 1.8, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::MOSFET(mos) => {
                assert!(mos.jacobian[0][1] > 0.0,
                    "BSIM4 gm must be positive in saturation, got {}", mos.jacobian[0][1]);
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    #[test]
    fn bsim4_pmos_saturation_via_device_model_linearize() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM4(pmos_bsim4()));
        // PMOS strong inversion: V_S = V_B = 3.3, V_G = 0, V_D = 0.3.
        let op = OperatingPoint::MOSFET([0.3, 0.0, 3.3, 3.3]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::MOSFET(mos) => {
                let v = [0.3_f64, 0.0, 3.3, 3.3];
                let id = reconstruct_current(&mos, 0, &v);
                assert!(id < 0.0,
                    "PMOS drain current must be negative, got {id}");
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // KCL invariants across all levels
    // -----------------------------------------------------------------

    #[test]
    fn level1_saturation_jacobian_row_sums_are_zero_kcl() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(nmos_level1()));
        let op = OperatingPoint::MOSFET([5.0, 3.0, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family");
        match lin {
            LinearizedModel::MOSFET(mos) => {
                for t in 0..MOSFET_TERMINALS {
                    let row_sum: f64 = mos.jacobian[t].iter().sum();
                    assert!(approx_eq(row_sum, 0.0, 1e-12, 1e-15),
                        "Level-1 Jacobian row {t} sum {row_sum} must be zero (KCL)");
                }
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    #[test]
    fn bsim4_saturation_jacobian_row_sums_are_zero_kcl() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM4(nmos_bsim4()));
        let op = OperatingPoint::MOSFET([3.0, 1.8, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family");
        match lin {
            LinearizedModel::MOSFET(mos) => {
                for t in 0..MOSFET_TERMINALS {
                    let row_sum: f64 = mos.jacobian[t].iter().sum();
                    assert!(approx_eq(row_sum, 0.0, 1e-12, 1e-15),
                        "BSIM4 Jacobian row {t} sum {row_sum} must be zero (KCL)");
                }
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // OperatingPoint mismatch is correctly rejected
    // -----------------------------------------------------------------

    #[test]
    fn mosfet_linearize_rejects_mismatched_diode_op() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(nmos_level1()));
        let op = OperatingPoint::Diode([0.7, 0.0]);
        let err = m.linearize(&op).expect_err("should reject mismatched OP");
        assert_eq!(err.expected, "MOSFET");
        assert_eq!(err.actual, "Diode");
    }

    #[test]
    fn mosfet_linearize_rejects_mismatched_bjt_op() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM4(nmos_bsim4()));
        let op = OperatingPoint::BJT([1.0, 0.65, 0.0]);
        let err = m.linearize(&op).expect_err("should reject mismatched OP");
        assert_eq!(err.expected, "MOSFET");
        assert_eq!(err.actual, "BJT");
    }

    // -----------------------------------------------------------------
    // Exhaustiveness witness — pins ADR-0005 intent
    // -----------------------------------------------------------------

    #[test]
    fn closed_enum_match_on_mosfet_params_is_exhaustive() {
        // The Rust compiler enforces this; the test pins intent so
        // a future PR adding a MOSFETParams variant without updating
        // the linearize_mosfet dispatch breaks here.
        fn level_name(p: &MOSFETParams) -> &'static str {
            match p {
                MOSFETParams::Level1(_) => "Level1",
                MOSFETParams::BSIM3v3(_) => "BSIM3v3",
                MOSFETParams::BSIM4(_) => "BSIM4",
            }
        }
        assert_eq!(level_name(&MOSFETParams::Level1(nmos_level1())), "Level1");
        assert_eq!(level_name(&MOSFETParams::BSIM3v3(nmos_bsim3())), "BSIM3v3");
        assert_eq!(level_name(&MOSFETParams::BSIM4(nmos_bsim4())), "BSIM4");
    }
}
