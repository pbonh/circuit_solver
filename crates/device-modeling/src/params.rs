//! Per-family `ModelParameters` payload structs for
//! [`DeviceModel`](crate::model::DeviceModel).
//!
//! Each top-level [`DeviceModel`](crate::DeviceModel) variant owns its
//! parameters inline (ADR-0005); this module defines the shape of
//! those payloads.
//!
//! # Closed-enum policy
//!
//! The MOSFET family is itself a closed sub-enum
//! ([`MOSFETParams`]) over the supported levels (Level-1, `BSIM3v3`,
//! BSIM4) per the design slice at
//! `openspec/changes/circuit-solver-2026-05-21-v1-spec/design.md`
//! line 118. Adding a new level is a compile-time breaking change to
//! the stamp surface (introduced in tasks.md #8).
//!
//! # Default values
//!
//! Where SPICE has a long-standing convention for a parameter default
//! (e.g., Shockley `IS = 1e-14 A`, `N = 1`, thermal voltage `Vt(300K)
//! ≈ 25.85 mV`) the struct's `Default` impl uses that convention.
//! Otherwise the field is required and has no default.
//!
//! # Equality and hashing
//!
//! These types carry `f64` parameter values, so they intentionally
//! implement neither [`Eq`] nor [`Hash`]. Comparisons in tests use
//! `PartialEq` with exact bit-for-bit equality on the default values
//! we ship; downstream callers needing tolerance-based comparison
//! should compose their own.
//!
//! # Stability
//!
//! Per [ADR-0010](../../../wiki/decisions/0010-unstable-public-rust-api-surface-for-v1.md)
//! these structs are part of the unstable v1 surface.

use circuit_solver_types::ModelName;

// ---------------------------------------------------------------------
// Diode
// ---------------------------------------------------------------------

/// `ModelParameters` for the [`Diode`](crate::DeviceModel::Diode)
/// variant (Shockley equation `I = IS·(exp(V/(N·Vt)) - 1)`).
///
/// The fields are the canonical SPICE diode .MODEL parameters that
/// the Diode stamp (tasks.md #9) reads. This struct intentionally
/// stops at the parameters the v1 stamp needs; junction capacitance,
/// breakdown, and high-injection corrections live in future variants
/// added under a superseding ADR.
#[derive(Debug, Clone, PartialEq)]
pub struct DiodeParams {
    /// Model identifier as resolved by the netlist-graph elaborator.
    /// Carried so `match`-arms can emit a diagnostic that names the
    /// `.MODEL` card without re-threading the model library.
    pub name: ModelName,

    /// Saturation current `IS`, in amperes.
    pub is: f64,

    /// Emission coefficient (ideality factor) `N`, dimensionless.
    pub n: f64,

    /// Series ohmic resistance `RS`, in ohms.
    pub rs: f64,

    /// Thermal voltage at the model's operating temperature
    /// (`Vt = k·T/q`), in volts. Pre-computed at parameter-extraction
    /// time so the stamp loop does not re-derive it per iterate.
    pub vt: f64,
}

impl Default for DiodeParams {
    /// SPICE-canonical defaults: `IS = 1e-14 A`, `N = 1`, `RS = 0 Ω`,
    /// `Vt = 25.85 mV` (room temperature `T = 300.15 K`).
    fn default() -> Self {
        Self {
            name: ModelName::new(""),
            is: 1e-14,
            n: 1.0,
            rs: 0.0,
            vt: 0.025_852_0,
        }
    }
}

// ---------------------------------------------------------------------
// BJT
// ---------------------------------------------------------------------

/// BJT polarity discriminator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BJTPolarity {
    /// NPN (electrons as majority carriers in base).
    Npn,
    /// PNP (holes as majority carriers in base).
    Pnp,
}

/// `ModelParameters` for the [`BJT`](crate::DeviceModel::BJT)
/// variant. The v1 stamp (tasks.md #10) is Ebers-Moll / Gummel-Poon
/// — the parameter set here is the intersection that both stamp
/// formulations require.
#[derive(Debug, Clone, PartialEq)]
pub struct BJTParams {
    /// Model identifier as resolved by the netlist-graph elaborator.
    pub name: ModelName,

    /// NPN / PNP polarity.
    pub polarity: BJTPolarity,

    /// Transport saturation current `IS`, in amperes.
    pub is: f64,

    /// Forward common-emitter current gain `BF`, dimensionless.
    pub bf: f64,

    /// Reverse common-emitter current gain `BR`, dimensionless.
    pub br: f64,

    /// Forward emission coefficient `NF`, dimensionless.
    pub nf: f64,

    /// Reverse emission coefficient `NR`, dimensionless.
    pub nr: f64,

    /// Forward Early voltage `VAF`, in volts. `f64::INFINITY` disables
    /// the Early effect.
    pub vaf: f64,

    /// Reverse Early voltage `VAR`, in volts. `f64::INFINITY` disables
    /// the reverse Early effect.
    pub var: f64,

    /// Thermal voltage `Vt = k·T/q`, in volts.
    pub vt: f64,
}

impl Default for BJTParams {
    /// SPICE-canonical NPN defaults: `IS = 1e-16 A`, `BF = 100`,
    /// `BR = 1`, `NF = NR = 1`, Early effect disabled
    /// (`VAF = VAR = f64::INFINITY`), `Vt = 25.85 mV`.
    fn default() -> Self {
        Self {
            name: ModelName::new(""),
            polarity: BJTPolarity::Npn,
            is: 1e-16,
            bf: 100.0,
            br: 1.0,
            nf: 1.0,
            nr: 1.0,
            vaf: f64::INFINITY,
            var: f64::INFINITY,
            vt: 0.025_852_0,
        }
    }
}

// ---------------------------------------------------------------------
// MOSFET
// ---------------------------------------------------------------------

/// MOSFET channel polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MosPolarity {
    /// N-channel (electrons as majority carriers in channel).
    Nmos,
    /// P-channel (holes as majority carriers in channel).
    Pmos,
}

/// `ModelParameters` for the [`MOSFET`](crate::DeviceModel::MOSFET)
/// variant.
///
/// Per the design slice at
/// `openspec/changes/circuit-solver-2026-05-21-v1-spec/design.md`
/// line 118, `MOSFETParams` is itself a closed enum over the
/// supported MOS levels. The level variants correspond directly to
/// tasks.md items #11 (Level-1), #12 (`BSIM3v3`), and #13 (BSIM4) which
/// implement each level's stamp.
///
/// The polarity (NMOS / PMOS) is carried *inside each level's
/// parameter struct* because polarity is a per-instance attribute,
/// not a per-level attribute (a Level-1 model can be either an NMOS
/// or PMOS device).
#[derive(Debug, Clone, PartialEq)]
pub enum MOSFETParams {
    /// MOSFET Level-1 (Shichman-Hodges square-law model).
    Level1(MosLevel1Params),
    /// MOSFET `BSIM3v3` (industry-standard short-channel model).
    BSIM3v3(MosBSIM3v3Params),
    /// MOSFET BSIM4 (extended short-channel / RF model).
    BSIM4(MosBSIM4Params),
}

impl MOSFETParams {
    /// Borrow the model name regardless of which level is active.
    #[must_use]
    pub fn name(&self) -> &ModelName {
        match self {
            Self::Level1(p) => &p.name,
            Self::BSIM3v3(p) => &p.name,
            Self::BSIM4(p) => &p.name,
        }
    }

    /// Channel polarity regardless of which level is active.
    #[must_use]
    pub fn polarity(&self) -> MosPolarity {
        match self {
            Self::Level1(p) => p.polarity,
            Self::BSIM3v3(p) => p.polarity,
            Self::BSIM4(p) => p.polarity,
        }
    }
}

/// MOSFET Level-1 (Shichman-Hodges square-law) parameters
/// (tasks.md #11 stamp).
#[derive(Debug, Clone, PartialEq)]
pub struct MosLevel1Params {
    /// Model identifier.
    pub name: ModelName,
    /// Channel polarity.
    pub polarity: MosPolarity,
    /// Threshold voltage `VTO`, in volts.
    pub vto: f64,
    /// Transconductance parameter `KP = μ·Cox`, in A/V².
    pub kp: f64,
    /// Channel-length modulation `LAMBDA`, in 1/V.
    pub lambda: f64,
    /// Body-effect coefficient `GAMMA`, in √V.
    pub gamma: f64,
    /// Surface potential `PHI`, in volts.
    pub phi: f64,
}

impl Default for MosLevel1Params {
    /// SPICE-canonical Level-1 defaults: `VTO = 0 V`, `KP = 2e-5
    /// A/V²`, `LAMBDA = 0 /V` (no CLM), `GAMMA = 0 √V` (no body
    /// effect), `PHI = 0.6 V`, NMOS polarity.
    fn default() -> Self {
        Self {
            name: ModelName::new(""),
            polarity: MosPolarity::Nmos,
            vto: 0.0,
            kp: 2.0e-5,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
        }
    }
}

/// MOSFET `BSIM3v3` parameters (tasks.md #12 stamp).
///
/// `BSIM3v3` has ~100 fitted parameters; this stub carries only the
/// identification fields plus a sparse parameter map. Concrete
/// fields are added incrementally as the #12 stamp lands, under the
/// same ADR-0005.
#[derive(Debug, Clone, PartialEq)]
pub struct MosBSIM3v3Params {
    /// Model identifier.
    pub name: ModelName,
    /// Channel polarity.
    pub polarity: MosPolarity,
    /// Sparse parameter map (`BSIM3v3` has ~100 named parameters; we
    /// carry them by name so the #12 stamp can grow its consumed-set
    /// without breaking other variants). Keys are SPICE-card names
    /// (e.g., `"vth0"`, `"u0"`, `"tox"`).
    pub raw: std::collections::BTreeMap<String, f64>,
}

impl Default for MosBSIM3v3Params {
    fn default() -> Self {
        Self {
            name: ModelName::new(""),
            polarity: MosPolarity::Nmos,
            raw: std::collections::BTreeMap::new(),
        }
    }
}

/// MOSFET BSIM4 parameters (tasks.md #13 stamp).
///
/// Same shape rationale as [`MosBSIM3v3Params`]: a sparse raw map
/// for now, concrete fields landing with the #13 stamp.
#[derive(Debug, Clone, PartialEq)]
pub struct MosBSIM4Params {
    /// Model identifier.
    pub name: ModelName,
    /// Channel polarity.
    pub polarity: MosPolarity,
    /// Sparse parameter map (BSIM4 has ~200 named parameters).
    pub raw: std::collections::BTreeMap<String, f64>,
}

impl Default for MosBSIM4Params {
    fn default() -> Self {
        Self {
            name: ModelName::new(""),
            polarity: MosPolarity::Nmos,
            raw: std::collections::BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diode_defaults_match_spice_convention() {
        let p = DiodeParams::default();
        // Use to_bits() for exact float comparison to satisfy
        // clippy::float_cmp; the Default impl produces exact bit
        // patterns for these SPICE constants.
        assert_eq!(p.is.to_bits(), 1e-14_f64.to_bits());
        assert_eq!(p.n.to_bits(), 1.0_f64.to_bits());
        assert_eq!(p.rs.to_bits(), 0.0_f64.to_bits());
        assert_eq!(p.vt.to_bits(), 0.025_852_0_f64.to_bits());
        assert!(p.name.is_empty());
    }

    #[test]
    fn bjt_defaults_match_spice_npn_convention() {
        let p = BJTParams::default();
        assert_eq!(p.polarity, BJTPolarity::Npn);
        assert_eq!(p.is.to_bits(), 1e-16_f64.to_bits());
        assert_eq!(p.bf.to_bits(), 100.0_f64.to_bits());
        assert_eq!(p.br.to_bits(), 1.0_f64.to_bits());
        assert_eq!(p.nf.to_bits(), 1.0_f64.to_bits());
        assert_eq!(p.nr.to_bits(), 1.0_f64.to_bits());
        assert!(p.vaf.is_infinite());
        assert!(p.var.is_infinite());
    }

    #[test]
    fn mos_level1_defaults_match_spice_convention() {
        let p = MosLevel1Params::default();
        assert_eq!(p.polarity, MosPolarity::Nmos);
        assert_eq!(p.vto.to_bits(), 0.0_f64.to_bits());
        assert_eq!(p.kp.to_bits(), 2.0e-5_f64.to_bits());
        assert_eq!(p.lambda.to_bits(), 0.0_f64.to_bits());
        assert_eq!(p.gamma.to_bits(), 0.0_f64.to_bits());
        assert_eq!(p.phi.to_bits(), 0.6_f64.to_bits());
    }

    #[test]
    fn mosfet_params_name_dispatches_through_match() {
        let l1 = MOSFETParams::Level1(MosLevel1Params {
            name: ModelName::new("nmos_lvt"),
            ..Default::default()
        });
        let b3 = MOSFETParams::BSIM3v3(MosBSIM3v3Params {
            name: ModelName::new("pmos_b3"),
            ..Default::default()
        });
        let b4 = MOSFETParams::BSIM4(MosBSIM4Params {
            name: ModelName::new("nmos_b4"),
            polarity: MosPolarity::Nmos,
            ..Default::default()
        });
        assert_eq!(l1.name().as_str(), "nmos_lvt");
        assert_eq!(b3.name().as_str(), "pmos_b3");
        assert_eq!(b4.name().as_str(), "nmos_b4");
    }

    #[test]
    fn mosfet_params_polarity_dispatches_through_match() {
        let pmos = MOSFETParams::Level1(MosLevel1Params {
            polarity: MosPolarity::Pmos,
            ..Default::default()
        });
        assert_eq!(pmos.polarity(), MosPolarity::Pmos);

        let nmos = MOSFETParams::BSIM3v3(MosBSIM3v3Params {
            polarity: MosPolarity::Nmos,
            ..Default::default()
        });
        assert_eq!(nmos.polarity(), MosPolarity::Nmos);
    }

    #[test]
    fn bsim_raw_map_is_empty_by_default() {
        let p = MosBSIM3v3Params::default();
        assert!(p.raw.is_empty());

        let p4 = MosBSIM4Params::default();
        assert!(p4.raw.is_empty());
    }

    #[test]
    fn polarity_pairs_are_comparable_by_value() {
        // BJTPolarity and MosPolarity both derive Eq for use as map
        // keys / set members in the netlist-graph elaborator.
        assert_eq!(BJTPolarity::Npn, BJTPolarity::Npn);
        assert_ne!(BJTPolarity::Npn, BJTPolarity::Pnp);
        assert_eq!(MosPolarity::Nmos, MosPolarity::Nmos);
        assert_ne!(MosPolarity::Nmos, MosPolarity::Pmos);
    }
}
