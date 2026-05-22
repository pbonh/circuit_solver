//! `LinearizedModel` stamp + Jacobian surface (tasks.md #8).
//!
//! This module owns the *contract* between the
//! [`device-modeling`](crate) context and the `numeric-solver`
//! context's MNA Assembler (tasks.md #14): for every nonlinear device
//! that appears in [`crate::DeviceModel`], the assembler hands the
//! device the current Newton-Raphson iterate's terminal voltages and
//! receives back a [`LinearizedModel`] — a fixed-shape, terminal-local
//! contribution that combines the Jacobian (small-signal conductance
//! matrix) and the companion current vector.
//!
//! # ADR-0005 dispatch (closed enum, no `dyn`)
//!
//! Per [ADR-0005](../../../wiki/decisions/0005-closed-enum-device-model-dispatch.md)
//! the stamp surface dispatches through a `match` on
//! [`crate::DeviceModel`]: see [`DeviceModel::linearize`]. Each arm
//! delegates to a per-family helper (e.g. [`linearize_diode`]). The
//! arms are exhaustive — the Rust compiler enforces that adding a
//! family variant breaks every site that lacks the new arm, which is
//! the property ADR-0005 buys.
//!
//! # Tasks-md slicing (this task vs. #9–#13)
//!
//! This task (tasks.md #8) lands:
//!
//! 1. The [`LinearizedModel`] data type — a closed enum that mirrors
//!    [`DeviceModel`]'s family shape, so the assembler can `match` on
//!    the linearization itself to learn stamp dimensions.
//! 2. The [`OperatingPoint`] inputs — per-family terminal voltage
//!    arrays (Diode = 2 terminals, BJT = 3, MOSFET = 4).
//! 3. The [`DeviceModel::linearize`] dispatch with its exhaustive
//!    `match` and per-family helper signatures.
//! 4. A documented *placeholder* body for each helper — a zero
//!    Jacobian and zero companion current, so the dispatch is callable
//!    end-to-end today but no device equation has been baked in yet.
//!
//! The per-family equation bodies land in:
//!
//! - tasks.md **#9** — Diode (Shockley equation + companion current),
//! - tasks.md **#10** — BJT (Ebers-Moll / Gummel-Poon),
//! - tasks.md **#11** — MOSFET Level-1 (Shichman-Hodges square law),
//! - tasks.md **#12** — MOSFET `BSIM3v3`,
//! - tasks.md **#13** — MOSFET BSIM4.
//!
//! Filling each helper in does *not* require changes to this file's
//! type surface — only to the helper's body and the helper-local
//! tests. That separation is intentional: it keeps the type-level
//! review (#8) and the per-device numerics (#9–#13) independently
//! mergeable.
//!
//! # Terminal-local coordinates
//!
//! [`LinearizedModel`] is expressed in *terminal-local* coordinates,
//! not graph node identifiers. The Jacobian for a Diode is a 2×2
//! matrix indexed `[anode, cathode]`; the BJT Jacobian is 3×3 indexed
//! `[collector, base, emitter]`; the MOSFET Jacobian is 4×4 indexed
//! `[drain, gate, source, bulk]`. The `numeric-solver` MNA assembler
//! (tasks.md #14) maps terminal indices to `NodeId` / `BranchId`
//! through the `FlattenedStructure`'s incidence; the device-modeling
//! context deliberately does **not** know about graph topology.
//!
//! This boundary preserves the dataflow declared in design.md
//! (line 54): `numeric-solver → "LinearizedModel request" →
//! device-modeling → "LinearizedModel stamp + Jacobian" →
//! numeric-solver`. The request is the [`OperatingPoint`]; the
//! response is the [`LinearizedModel`].

use crate::model::DeviceModel;
use crate::params::{BJTParams, DiodeParams, MOSFETParams};

// ---------------------------------------------------------------------
// Terminal counts
// ---------------------------------------------------------------------

/// Number of terminals on a diode (anode, cathode).
pub const DIODE_TERMINALS: usize = 2;

/// Number of terminals on a BJT (collector, base, emitter).
pub const BJT_TERMINALS: usize = 3;

/// Number of terminals on a MOSFET (drain, gate, source, bulk).
pub const MOSFET_TERMINALS: usize = 4;

// ---------------------------------------------------------------------
// OperatingPoint — per-family terminal-voltage input
// ---------------------------------------------------------------------

/// Per-iteration terminal voltages handed to
/// [`DeviceModel::linearize`].
///
/// One variant per [`DeviceModel`] family. Voltages are
/// terminal-local — they are absolute node voltages relative to the
/// circuit's ground reference, not differential voltages — and are
/// laid out in the canonical SPICE ordering for each family:
///
/// - [`OperatingPoint::Diode`] — `[V_anode, V_cathode]`
/// - [`OperatingPoint::BJT`] — `[V_collector, V_base, V_emitter]`
/// - [`OperatingPoint::MOSFET`] — `[V_drain, V_gate, V_source, V_bulk]`
///
/// The numeric-solver's MNA assembler builds these from the current
/// Newton iterate by indexing the solution vector with each device
/// terminal's `NodeId`. The mapping from `NodeId` to terminal slot
/// is owned by the assembler, not by `device-modeling`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatingPoint {
    /// Diode iterate: `[V_anode, V_cathode]`.
    Diode([f64; DIODE_TERMINALS]),
    /// BJT iterate: `[V_collector, V_base, V_emitter]`.
    BJT([f64; BJT_TERMINALS]),
    /// MOSFET iterate: `[V_drain, V_gate, V_source, V_bulk]`.
    MOSFET([f64; MOSFET_TERMINALS]),
}

impl OperatingPoint {
    /// Number of terminals carried by this operating point.
    ///
    /// Cheap discriminant accessor used by tests and by the MNA
    /// assembler when validating that an `OperatingPoint` matches the
    /// device family it is paired with.
    #[must_use]
    pub fn terminal_count(&self) -> usize {
        match self {
            Self::Diode(_) => DIODE_TERMINALS,
            Self::BJT(_) => BJT_TERMINALS,
            Self::MOSFET(_) => MOSFET_TERMINALS,
        }
    }
}

// ---------------------------------------------------------------------
// LinearizedModel — the response type
// ---------------------------------------------------------------------

/// Diode linearization: 2×2 conductance Jacobian + 2-vector of
/// companion currents (one per terminal).
///
/// In MNA the Diode stamp contributes:
///
/// - `g_ij` to the conductance matrix at `(terminal_i, terminal_j)`,
///   i.e. add `jacobian[i][j]` to `G[node_of(i), node_of(j)]`,
/// - `i_eq_k` to the right-hand-side at `node_of(terminal_k)` (current
///   leaving the node into the device companion model).
///
/// The exact equation that produces these numbers is the Diode
/// companion model from tasks.md #9 (Shockley equation linearized at
/// the iterate). This task (#8) ships a zero Jacobian and zero
/// companion current as placeholders.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiodeLinearization {
    /// Jacobian matrix, terminal-local. `jacobian[i][j]` is
    /// ∂`I_i` / ∂`V_j` evaluated at the operating point.
    pub jacobian: [[f64; DIODE_TERMINALS]; DIODE_TERMINALS],
    /// Companion current vector, terminal-local. `companion_current[k]`
    /// is added to the MNA right-hand-side at the node attached to
    /// terminal `k`.
    pub companion_current: [f64; DIODE_TERMINALS],
}

impl DiodeLinearization {
    /// All-zero linearization — the placeholder this task ships per
    /// the docstring above. tasks.md #9 replaces this with the
    /// Shockley-equation companion model.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            jacobian: [[0.0; DIODE_TERMINALS]; DIODE_TERMINALS],
            companion_current: [0.0; DIODE_TERMINALS],
        }
    }
}

/// BJT linearization: 3×3 Jacobian + 3-vector of companion currents.
///
/// Terminal ordering is `[collector, base, emitter]` per SPICE
/// convention. Equation bodies land in tasks.md #10
/// (Ebers-Moll / Gummel-Poon).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BJTLinearization {
    /// Jacobian matrix indexed `[collector, base, emitter]`.
    pub jacobian: [[f64; BJT_TERMINALS]; BJT_TERMINALS],
    /// Companion current vector indexed `[collector, base, emitter]`.
    pub companion_current: [f64; BJT_TERMINALS],
}

impl BJTLinearization {
    /// All-zero linearization placeholder (see crate-level docstring).
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            jacobian: [[0.0; BJT_TERMINALS]; BJT_TERMINALS],
            companion_current: [0.0; BJT_TERMINALS],
        }
    }
}

/// MOSFET linearization: 4×4 Jacobian + 4-vector of companion
/// currents.
///
/// Terminal ordering is `[drain, gate, source, bulk]` per SPICE
/// convention. The same linearization shape is shared across all
/// three MOS levels (Level-1, `BSIM3v3`, BSIM4); only the equation
/// bodies that populate the Jacobian differ between levels. Those
/// equation bodies land in tasks.md #11 (Level-1), #12 (`BSIM3v3`),
/// and #13 (BSIM4).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MOSFETLinearization {
    /// Jacobian matrix indexed `[drain, gate, source, bulk]`.
    pub jacobian: [[f64; MOSFET_TERMINALS]; MOSFET_TERMINALS],
    /// Companion current vector indexed `[drain, gate, source, bulk]`.
    pub companion_current: [f64; MOSFET_TERMINALS],
}

impl MOSFETLinearization {
    /// All-zero linearization placeholder (see crate-level docstring).
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            jacobian: [[0.0; MOSFET_TERMINALS]; MOSFET_TERMINALS],
            companion_current: [0.0; MOSFET_TERMINALS],
        }
    }
}

/// Family-tagged linearization returned by
/// [`DeviceModel::linearize`].
///
/// This is the unit of communication between the device-modeling
/// context and the numeric-solver's MNA Assembler (tasks.md #14): the
/// assembler matches on the variant to learn the stamp shape, then
/// folds the per-variant `jacobian` and `companion_current` into the
/// global MNA system.
///
/// # Why a family-tagged enum rather than a flat `(matrix, vec)` pair
///
/// A flat representation would force the assembler to carry a
/// separate terminal-count alongside every linearization. The
/// family-tagged enum keeps the dimensionality on the type, so the
/// assembler's match is exhaustive in the same way [`DeviceModel`]'s
/// is — and ADR-0005's compile-time exhaustiveness extends from
/// device parameters all the way through to the MNA stamp loop.
///
/// # Layout commitment
///
/// Per ADR-0005 each variant carries its linearization inline; no
/// `Box`, no `dyn`. The enum is `Copy + Clone + PartialEq`, so the
/// assembler can pass it by value or collect a
/// `Vec<LinearizedModel>` over a flattened element list with no
/// indirection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearizedModel {
    /// Diode 2-terminal linearization.
    Diode(DiodeLinearization),
    /// BJT 3-terminal linearization.
    BJT(BJTLinearization),
    /// MOSFET 4-terminal linearization (level-agnostic shape).
    MOSFET(MOSFETLinearization),
}

impl LinearizedModel {
    /// Number of terminals contributed by this linearization's stamp.
    ///
    /// The MNA assembler (tasks.md #14) uses this to size the
    /// terminal-to-node index map when folding the stamp into the
    /// global system.
    #[must_use]
    pub fn terminal_count(&self) -> usize {
        match self {
            Self::Diode(_) => DIODE_TERMINALS,
            Self::BJT(_) => BJT_TERMINALS,
            Self::MOSFET(_) => MOSFET_TERMINALS,
        }
    }
}

// ---------------------------------------------------------------------
// Dispatch entry point on DeviceModel
// ---------------------------------------------------------------------

/// Mismatched-family error from [`DeviceModel::linearize`].
///
/// Returned when the supplied [`OperatingPoint`] variant does not
/// match the [`DeviceModel`] variant being linearized — e.g. a
/// caller hands `DeviceModel::Diode(..)` a `OperatingPoint::BJT(..)`.
///
/// The mismatch is a *bug* in the MNA assembler, not user input, so
/// the error carries enough context for the assembler's diagnostic
/// to identify the device by its [`ModelName`](circuit_solver_types::ModelName).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatingPointFamilyMismatch {
    /// The family carried by the [`DeviceModel`].
    pub expected: &'static str,
    /// The family carried by the [`OperatingPoint`] handed in.
    pub actual: &'static str,
}

impl core::fmt::Display for OperatingPointFamilyMismatch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "device-model / operating-point family mismatch: device is {} but operating point is {}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for OperatingPointFamilyMismatch {}

impl DeviceModel {
    /// Compute the [`LinearizedModel`] for this device at the given
    /// [`OperatingPoint`].
    ///
    /// This is the dispatch entry point the numeric-solver MNA
    /// Assembler (tasks.md #14) calls on each Newton-Raphson iterate:
    /// `numeric-solver → "LinearizedModel request" → device-modeling →
    /// "LinearizedModel stamp + Jacobian" → numeric-solver`
    /// (design.md line 54).
    ///
    /// # Dispatch (ADR-0005)
    ///
    /// The implementation is a single `match` on `self`, with one arm
    /// per [`DeviceModel`] family. Each arm delegates to a per-family
    /// helper ([`linearize_diode`], [`linearize_bjt`],
    /// [`linearize_mosfet`]) so each task #9–#13 can grow its body
    /// without re-touching this dispatch.
    ///
    /// # Errors
    ///
    /// Returns [`OperatingPointFamilyMismatch`] if `op`'s family does
    /// not match `self`'s family. This is a programming error in the
    /// assembler, not a runtime convergence concern.
    ///
    /// # Placeholder behavior at task #8
    ///
    /// Every per-family helper currently returns the all-zero
    /// linearization defined by `*Linearization::zero()`. tasks.md
    /// #9–#13 replace those zero bodies with the actual device
    /// equations under the same dispatch site.
    pub fn linearize(
        &self,
        op: &OperatingPoint,
    ) -> Result<LinearizedModel, OperatingPointFamilyMismatch> {
        match (self, op) {
            (Self::Diode(p), OperatingPoint::Diode(v)) => {
                Ok(LinearizedModel::Diode(linearize_diode(p, v)))
            }
            (Self::BJT(p), OperatingPoint::BJT(v)) => Ok(LinearizedModel::BJT(linearize_bjt(p, v))),
            (Self::MOSFET(p), OperatingPoint::MOSFET(v)) => {
                Ok(LinearizedModel::MOSFET(linearize_mosfet(p, v)))
            }
            // Mismatched-family arms: spell each one out so the
            // exhaustiveness check still bites if a new DeviceModel
            // variant is added without a matching OperatingPoint arm.
            (Self::Diode(_), op) => Err(OperatingPointFamilyMismatch {
                expected: "Diode",
                actual: op_family_name(op),
            }),
            (Self::BJT(_), op) => Err(OperatingPointFamilyMismatch {
                expected: "BJT",
                actual: op_family_name(op),
            }),
            (Self::MOSFET(_), op) => Err(OperatingPointFamilyMismatch {
                expected: "MOSFET",
                actual: op_family_name(op),
            }),
        }
    }
}

fn op_family_name(op: &OperatingPoint) -> &'static str {
    match op {
        OperatingPoint::Diode(_) => "Diode",
        OperatingPoint::BJT(_) => "BJT",
        OperatingPoint::MOSFET(_) => "MOSFET",
    }
}

// ---------------------------------------------------------------------
// Per-family helpers — bodies filled in by tasks #9–#13
// ---------------------------------------------------------------------

/// Linearize a Diode at the given terminal voltages.
///
/// **Placeholder at task #8.** Returns
/// [`DiodeLinearization::zero`]. The Shockley-equation companion
/// model lands in tasks.md #9.
///
/// # Arguments
///
/// - `params` — the diode's `.MODEL` parameters (`IS`, `N`, `RS`, `Vt`).
/// - `terminal_voltages` — `[V_anode, V_cathode]`.
///
/// # Returns
///
/// A [`DiodeLinearization`] holding the 2×2 Jacobian and 2-vector
/// companion current in terminal-local coordinates.
#[must_use]
pub fn linearize_diode(
    params: &DiodeParams,
    terminal_voltages: &[f64; DIODE_TERMINALS],
) -> DiodeLinearization {
    // Suppress the unused-parameter lint while the body is a
    // placeholder. tasks.md #9 replaces this with the Shockley
    // companion model and removes these explicit `_` bindings.
    let _ = params;
    let _ = terminal_voltages;
    DiodeLinearization::zero()
}

/// Linearize a BJT at the given terminal voltages.
///
/// **Placeholder at task #8.** Returns [`BJTLinearization::zero`].
/// Ebers-Moll / Gummel-Poon equations land in tasks.md #10.
///
/// # Arguments
///
/// - `params` — the BJT's `.MODEL` parameters (`IS`, `BF`, `BR`,
///   `NF`, `NR`, `VAF`, `VAR`, `Vt`, polarity).
/// - `terminal_voltages` — `[V_collector, V_base, V_emitter]`.
#[must_use]
pub fn linearize_bjt(
    params: &BJTParams,
    terminal_voltages: &[f64; BJT_TERMINALS],
) -> BJTLinearization {
    let _ = params;
    let _ = terminal_voltages;
    BJTLinearization::zero()
}

/// Linearize a MOSFET at the given terminal voltages.
///
/// **Placeholder at task #8.** The level-discriminating `match` on
/// [`MOSFETParams`] is in place (each level's arm currently returns
/// the zero linearization). tasks.md #11–#13 fill each arm in:
///
/// - [`MOSFETParams::Level1`] → tasks.md #11 (Shichman-Hodges),
/// - [`MOSFETParams::BSIM3v3`] → tasks.md #12,
/// - [`MOSFETParams::BSIM4`] → tasks.md #13.
///
/// The match is exhaustive (ADR-0005): adding a new MOS level to
/// [`MOSFETParams`] breaks this site, which is the intended
/// compile-time guard against silent omission.
///
/// # Arguments
///
/// - `params` — the MOSFET's level-specific parameter payload.
/// - `terminal_voltages` — `[V_drain, V_gate, V_source, V_bulk]`.
#[must_use]
pub fn linearize_mosfet(
    params: &MOSFETParams,
    terminal_voltages: &[f64; MOSFET_TERMINALS],
) -> MOSFETLinearization {
    match params {
        MOSFETParams::Level1(p) => {
            let _ = p;
            let _ = terminal_voltages;
            MOSFETLinearization::zero()
        }
        MOSFETParams::BSIM3v3(p) => {
            let _ = p;
            let _ = terminal_voltages;
            MOSFETLinearization::zero()
        }
        MOSFETParams::BSIM4(p) => {
            let _ = p;
            let _ = terminal_voltages;
            MOSFETLinearization::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::{
        BJTPolarity, MosBSIM3v3Params, MosBSIM4Params, MosLevel1Params, MosPolarity,
    };
    use circuit_solver_types::ModelName;

    // -----------------------------------------------------------------
    // Terminal-count constants reflect the SPICE convention.
    // -----------------------------------------------------------------

    #[test]
    fn terminal_counts_are_spice_canonical() {
        assert_eq!(DIODE_TERMINALS, 2);
        assert_eq!(BJT_TERMINALS, 3);
        assert_eq!(MOSFET_TERMINALS, 4);
    }

    // -----------------------------------------------------------------
    // Dispatch happy paths — one per DeviceModel variant.
    // -----------------------------------------------------------------

    #[test]
    fn linearize_diode_dispatches_through_match() {
        let m = DeviceModel::Diode(DiodeParams {
            name: ModelName::new("d1"),
            ..Default::default()
        });
        let op = OperatingPoint::Diode([0.6, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::Diode(d) => {
                assert_eq!(d, DiodeLinearization::zero(), "placeholder is zero");
            }
            other => panic!("expected Diode linearization, got {other:?}"),
        }
        assert_eq!(lin.terminal_count(), DIODE_TERMINALS);
    }

    #[test]
    fn linearize_bjt_dispatches_through_match() {
        let m = DeviceModel::BJT(BJTParams {
            name: ModelName::new("q1"),
            polarity: BJTPolarity::Npn,
            ..Default::default()
        });
        let op = OperatingPoint::BJT([5.0, 0.7, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        assert!(matches!(lin, LinearizedModel::BJT(_)));
        assert_eq!(lin.terminal_count(), BJT_TERMINALS);
    }

    #[test]
    fn linearize_mosfet_level1_dispatches_through_match() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params {
            name: ModelName::new("nmos1"),
            polarity: MosPolarity::Nmos,
            ..Default::default()
        }));
        let op = OperatingPoint::MOSFET([3.3, 1.8, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        assert!(matches!(lin, LinearizedModel::MOSFET(_)));
        assert_eq!(lin.terminal_count(), MOSFET_TERMINALS);
    }

    #[test]
    fn linearize_mosfet_bsim3v3_dispatches_through_match() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM3v3(MosBSIM3v3Params {
            name: ModelName::new("nmos_b3"),
            ..Default::default()
        }));
        let op = OperatingPoint::MOSFET([3.3, 1.8, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        assert!(matches!(lin, LinearizedModel::MOSFET(_)));
    }

    #[test]
    fn linearize_mosfet_bsim4_dispatches_through_match() {
        let m = DeviceModel::MOSFET(MOSFETParams::BSIM4(MosBSIM4Params {
            name: ModelName::new("pmos_b4"),
            polarity: MosPolarity::Pmos,
            ..Default::default()
        }));
        let op = OperatingPoint::MOSFET([0.0, 0.0, 3.3, 3.3]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        assert!(matches!(lin, LinearizedModel::MOSFET(_)));
    }

    // -----------------------------------------------------------------
    // Mismatched-family dispatch — one per device family.
    // -----------------------------------------------------------------

    #[test]
    fn diode_with_bjt_op_returns_family_mismatch() {
        let m = DeviceModel::Diode(DiodeParams::default());
        let op = OperatingPoint::BJT([0.0; BJT_TERMINALS]);
        let err = m.linearize(&op).unwrap_err();
        assert_eq!(err.expected, "Diode");
        assert_eq!(err.actual, "BJT");
    }

    #[test]
    fn bjt_with_mosfet_op_returns_family_mismatch() {
        let m = DeviceModel::BJT(BJTParams::default());
        let op = OperatingPoint::MOSFET([0.0; MOSFET_TERMINALS]);
        let err = m.linearize(&op).unwrap_err();
        assert_eq!(err.expected, "BJT");
        assert_eq!(err.actual, "MOSFET");
    }

    #[test]
    fn mosfet_with_diode_op_returns_family_mismatch() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params::default()));
        let op = OperatingPoint::Diode([0.0; DIODE_TERMINALS]);
        let err = m.linearize(&op).unwrap_err();
        assert_eq!(err.expected, "MOSFET");
        assert_eq!(err.actual, "Diode");
    }

    #[test]
    fn family_mismatch_error_displays_helpfully() {
        let m = DeviceModel::Diode(DiodeParams::default());
        let op = OperatingPoint::BJT([0.0; BJT_TERMINALS]);
        let err = m.linearize(&op).unwrap_err();
        let rendered = format!("{err}");
        assert!(
            rendered.contains("Diode"),
            "expected to mention device family, got: {rendered}"
        );
        assert!(
            rendered.contains("BJT"),
            "expected to mention operating-point family, got: {rendered}"
        );
    }

    // -----------------------------------------------------------------
    // OperatingPoint terminal-count reflects each family's SPICE shape.
    // -----------------------------------------------------------------

    #[test]
    fn operating_point_terminal_counts_match_constants() {
        assert_eq!(
            OperatingPoint::Diode([0.0; DIODE_TERMINALS]).terminal_count(),
            DIODE_TERMINALS,
        );
        assert_eq!(
            OperatingPoint::BJT([0.0; BJT_TERMINALS]).terminal_count(),
            BJT_TERMINALS,
        );
        assert_eq!(
            OperatingPoint::MOSFET([0.0; MOSFET_TERMINALS]).terminal_count(),
            MOSFET_TERMINALS,
        );
    }

    // -----------------------------------------------------------------
    // Placeholder helpers return zero linearizations. tasks.md #9–#13
    // are expected to *replace* these assertions in the same files
    // when the device equations land.
    // -----------------------------------------------------------------

    #[test]
    fn linearize_diode_helper_returns_zero_placeholder() {
        let p = DiodeParams::default();
        let lin = linearize_diode(&p, &[0.6, 0.0]);
        assert_eq!(lin, DiodeLinearization::zero());
    }

    #[test]
    fn linearize_bjt_helper_returns_zero_placeholder() {
        let p = BJTParams::default();
        let lin = linearize_bjt(&p, &[5.0, 0.7, 0.0]);
        assert_eq!(lin, BJTLinearization::zero());
    }

    #[test]
    fn linearize_mosfet_helper_dispatches_each_level_to_zero_placeholder() {
        // Each MOS level dispatches independently — covered by the
        // inner `match` on MOSFETParams. The compile-time check is
        // the load-bearing assertion; the run-time check pins
        // intent.
        let l1 = MOSFETParams::Level1(MosLevel1Params::default());
        let b3 = MOSFETParams::BSIM3v3(MosBSIM3v3Params::default());
        let b4 = MOSFETParams::BSIM4(MosBSIM4Params::default());
        assert_eq!(
            linearize_mosfet(&l1, &[0.0; MOSFET_TERMINALS]),
            MOSFETLinearization::zero()
        );
        assert_eq!(
            linearize_mosfet(&b3, &[0.0; MOSFET_TERMINALS]),
            MOSFETLinearization::zero()
        );
        assert_eq!(
            linearize_mosfet(&b4, &[0.0; MOSFET_TERMINALS]),
            MOSFETLinearization::zero()
        );
    }

    // -----------------------------------------------------------------
    // Layout witnesses: LinearizedModel is the same size as its
    // largest variant — proving no heap indirection per ADR-0005.
    // -----------------------------------------------------------------

    #[test]
    fn linearized_model_inlines_largest_variant_no_heap() {
        // ADR-0005: each variant owns its payload inline; no Box.
        fn assert_sized<T: Sized>() {}
        assert_sized::<LinearizedModel>();
        assert!(
            std::mem::size_of::<LinearizedModel>() >= std::mem::size_of::<MOSFETLinearization>(),
            "LinearizedModel must inline its largest variant per ADR-0005"
        );
    }

    #[test]
    fn linearized_model_is_copy() {
        // Copy + Clone is part of the contract so the MNA assembler
        // (tasks.md #14) can fold the result into the stamp loop
        // without ownership friction.
        fn assert_copy<T: Copy>() {}
        assert_copy::<LinearizedModel>();
        assert_copy::<OperatingPoint>();
        assert_copy::<DiodeLinearization>();
        assert_copy::<BJTLinearization>();
        assert_copy::<MOSFETLinearization>();
    }

    // -----------------------------------------------------------------
    // Exhaustiveness witness — pin ADR-0005 intent under test, so a
    // new DeviceModel variant added without updating this dispatch
    // breaks the build *here* in addition to in the production code.
    // -----------------------------------------------------------------

    #[test]
    fn dispatch_match_is_exhaustive_witness() {
        // The Rust compiler enforces this via the `match` in
        // `DeviceModel::linearize`. The witness exists so that a
        // future PR that adds a `DeviceModel` variant without a
        // matching `OperatingPoint` and dispatch arm breaks the test
        // module — making the omission visible in code review even
        // when the production `match` has been `_`-shadowed.
        fn family_tag(m: &DeviceModel) -> &'static str {
            match m {
                DeviceModel::Diode(_) => "Diode",
                DeviceModel::BJT(_) => "BJT",
                DeviceModel::MOSFET(_) => "MOSFET",
            }
        }
        assert_eq!(
            family_tag(&DeviceModel::Diode(DiodeParams::default())),
            "Diode"
        );
        assert_eq!(family_tag(&DeviceModel::BJT(BJTParams::default())), "BJT");
        assert_eq!(
            family_tag(&DeviceModel::MOSFET(MOSFETParams::Level1(
                MosLevel1Params::default()
            ))),
            "MOSFET"
        );
    }
}
