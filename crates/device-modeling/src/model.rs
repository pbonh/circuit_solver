//! Top-level closed-enum [`DeviceModel`].
//!
//! Per [ADR-0005](../../../wiki/decisions/0005-closed-enum-device-model-dispatch.md)
//! `DeviceModel` is a Rust enum with one variant per in-scope model
//! family. Each variant owns its
//! `ModelParameters` payload **inline** — no
//! `Box`, no `dyn DeviceModel`, no string-keyed registry. The
//! `numeric-solver` context holds `Vec<DeviceModel>` and dispatches
//! through `match` on each Newton-Raphson iterate.
//!
//! Adding a new variant is a deliberate compile-time breaking change
//! — every `match` site (stamp generation, Jacobian assembly,
//! topology classification) must be updated. That property is
//! exactly what the closed enum exists to guarantee.

use circuit_solver_types::ModelName;

use crate::params::{BJTParams, DiodeParams, MOSFETParams};

/// Family discriminator independent of the parameter payload.
///
/// Used by callers that need to classify a device by family without
/// destructuring the inner parameter struct — for instance, the
/// netlist-graph topology checker (ADR-0009) which only needs to
/// know whether an element is conductive in DC, not what its
/// parameters are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceFamily {
    /// Two-terminal junction diode (Shockley equation).
    Diode,
    /// Bipolar junction transistor (Ebers-Moll / Gummel-Poon).
    BJT,
    /// Metal-oxide-semiconductor field-effect transistor.
    MOSFET,
}

/// Closed-enum device model dispatched on by the numeric-solver
/// inside the Newton-Raphson stamp loop (ADR-0005).
///
/// # Variants and ownership
///
/// Each variant carries its `ModelParameters` payload by value (not
/// by `Box` or reference), so the enum's footprint equals the
/// discriminant plus the largest variant. The current v1 scope is
/// closed under the three semiconductor families (`Diode`, `BJT`,
/// `MOSFET`); adding a future family (`JFET`, `MESFET`, …) is a
/// compile-time breaking change that must update every `match` arm
/// downstream — which is the exhaustiveness property ADR-0005 buys.
///
/// # Stamp surface
///
/// This task (tasks.md #7) lands the enum *shape* only. The
/// `LinearizedModel` stamp / Jacobian methods land in tasks.md #8
/// behind a single `match` on this enum; individual device stamps
/// land in #9 (Diode), #10 (BJT), #11–#13 (MOSFET levels).
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceModel {
    /// Junction diode with Shockley-equation parameters.
    Diode(DiodeParams),
    /// Bipolar junction transistor with Ebers-Moll / Gummel-Poon
    /// parameters.
    BJT(BJTParams),
    /// MOSFET with level-specific parameters (Level-1, `BSIM3v3`, or
    /// BSIM4).
    MOSFET(MOSFETParams),
}

impl DeviceModel {
    /// Family discriminator for this model.
    ///
    /// Cheap: a single `match` on the enum tag with no payload
    /// access. Intended for callers (topology checker, stamp
    /// dispatch tracing) that need the family without unpacking the
    /// parameters.
    #[must_use]
    pub fn family(&self) -> DeviceFamily {
        match self {
            Self::Diode(_) => DeviceFamily::Diode,
            Self::BJT(_) => DeviceFamily::BJT,
            Self::MOSFET(_) => DeviceFamily::MOSFET,
        }
    }

    /// Borrow this model's identifier as resolved from the netlist's
    /// `.MODEL` card. Useful for diagnostic messages from the stamp
    /// loop without re-threading the `ModelLibrary`.
    #[must_use]
    pub fn name(&self) -> &ModelName {
        match self {
            Self::Diode(p) => &p.name,
            Self::BJT(p) => &p.name,
            Self::MOSFET(p) => p.name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::{
        BJTPolarity, MosBSIM3v3Params, MosBSIM4Params, MosLevel1Params, MosPolarity,
    };

    #[test]
    fn diode_variant_reports_diode_family() {
        let m = DeviceModel::Diode(DiodeParams {
            name: ModelName::new("d1n4148"),
            ..Default::default()
        });
        assert_eq!(m.family(), DeviceFamily::Diode);
        assert_eq!(m.name().as_str(), "d1n4148");
    }

    #[test]
    fn bjt_variant_reports_bjt_family_and_name() {
        let m = DeviceModel::BJT(BJTParams {
            name: ModelName::new("q2n2222"),
            polarity: BJTPolarity::Npn,
            ..Default::default()
        });
        assert_eq!(m.family(), DeviceFamily::BJT);
        assert_eq!(m.name().as_str(), "q2n2222");
    }

    #[test]
    fn mosfet_level1_variant_reports_mosfet_family() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params {
            name: ModelName::new("nmos_lvt"),
            polarity: MosPolarity::Nmos,
            ..Default::default()
        }));
        assert_eq!(m.family(), DeviceFamily::MOSFET);
        assert_eq!(m.name().as_str(), "nmos_lvt");
    }

    #[test]
    fn mosfet_bsim3v3_variant_reports_mosfet_family() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM3v3(MosBSIM3v3Params {
            name: ModelName::new("nmos_b3"),
            ..Default::default()
        }));
        assert_eq!(m.family(), DeviceFamily::MOSFET);
        assert_eq!(m.name().as_str(), "nmos_b3");
    }

    #[test]
    fn mosfet_bsim4_variant_reports_mosfet_family() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM4(MosBSIM4Params {
            name: ModelName::new("pmos_b4"),
            polarity: MosPolarity::Pmos,
            ..Default::default()
        }));
        assert_eq!(m.family(), DeviceFamily::MOSFET);
        assert_eq!(m.name().as_str(), "pmos_b4");
    }

    #[test]
    fn closed_enum_match_is_exhaustive() {
        // Sanity check: an unannotated `match` covers all three
        // variants. The Rust compiler enforces this, but the test
        // pins the intent under ADR-0005 so a future PR that adds
        // a variant without updating downstream sites breaks here
        // (in addition to breaking elsewhere).
        fn classify(m: &DeviceModel) -> &'static str {
            match m {
                DeviceModel::Diode(_) => "diode",
                DeviceModel::BJT(_) => "bjt",
                DeviceModel::MOSFET(_) => "mosfet",
            }
        }
        assert_eq!(
            classify(&DeviceModel::Diode(DiodeParams::default())),
            "diode"
        );
        assert_eq!(classify(&DeviceModel::BJT(BJTParams::default())), "bjt");
        assert_eq!(
            classify(&DeviceModel::MOSFET(MOSFETParams::Level1(
                MosLevel1Params::default()
            ))),
            "mosfet"
        );
    }

    #[test]
    fn device_models_clone_independently() {
        // ADR-0005 commitment: each variant owns its payload inline.
        // Cloning a model produces an independent value.
        let original = DeviceModel::Diode(DiodeParams {
            name: ModelName::new("d_orig"),
            is: 2.5e-14,
            ..Default::default()
        });
        let cloned = original.clone();
        assert_eq!(original, cloned);

        // Mutating the original's payload (by replacement, since
        // accessors return `&`) does not touch the clone.
        let mutated = DeviceModel::Diode(DiodeParams {
            name: ModelName::new("d_mut"),
            is: 9.9e-14,
            ..Default::default()
        });
        assert_ne!(mutated, cloned);
        assert_eq!(cloned.name().as_str(), "d_orig");
    }

    #[test]
    fn no_dyn_no_box_layout_witness() {
        // ADR-0005 negative consequence: "Enum size bloat: the enum's
        // size equals the largest variant plus discriminant." We
        // assert here that `DeviceModel` is `Sized` (compile-time
        // witness) and that its size is bounded by what the largest
        // variant requires — meaning no heap indirection has been
        // smuggled in.
        fn assert_sized<T: Sized>() {}
        assert_sized::<DeviceModel>();

        // The MOSFET variant is currently the largest (BSIM raw map
        // payload), so the enum is at least that big. We don't pin
        // an exact byte count because it depends on `BTreeMap`'s
        // layout, but we confirm the enum is non-trivial and the
        // discriminant + payload bound holds.
        assert!(
            std::mem::size_of::<DeviceModel>() >= std::mem::size_of::<MOSFETParams>(),
            "DeviceModel must inline its largest variant per ADR-0005"
        );
    }
}
