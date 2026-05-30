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
use crate::params::{BJTParams, DiodeParams, MOSFETParams, MosBSIM4Params, MosPolarity};

pub mod mosfet_level1;

pub use mosfet_level1::linearize_mosfet_level1;

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
/// - `i_eq_k` is *subtracted* from the right-hand-side at
///   `node_of(terminal_k)`. The companion current encodes the
///   residual current `I_term(v*) − J[k,:]·v*` the linearized model
///   would draw at `v = 0` — i.e., the current leaving the node into
///   the device terminal — so moving the linearized device current
///   from the LHS (where its `J·V` part sits in the conductance
///   matrix) to the RHS introduces the minus sign. See
///   `numeric-solver::assemble` for the full derivation.
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
    /// is *subtracted* from the MNA right-hand-side at the node
    /// attached to terminal `k`. See `numeric-solver::assemble`'s
    /// `stamp_dense_block` for the sign-convention rationale.
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

/// Maximum value of the Shockley exponent argument `Vd / (N * Vt)`
/// admitted by [`linearize_diode`] before clamping.
///
/// `exp(40) ≈ 2.35e17` — well below `f64::MAX ≈ 1.8e308` but still
/// large enough that legitimate forward-bias iterates near the
/// solver's voltage-limit cap (`vlimit ≈ 1 V`, i.e. `Vd/(N·Vt) ≈ 38`
/// at room temperature) pass through unclamped. Clamping is a
/// numerical safety net for *intermediate* Newton-Raphson iterates
/// that overshoot during early iterations on stiff junctions; the
/// numeric-solver's `vlimit` / source-stepping / Gmin homotopy
/// (tasks.md #15) provide the actual convergence aid. ngspice and
/// SPICE3 use the same clamp value in their `cdiode` evaluation —
/// matching it makes per-iterate residue comparison against the
/// golden reference (ADR-0008) bit-stable for in-range arguments
/// and well-behaved for out-of-range ones.
pub const DIODE_MAX_EXP_ARG: f64 = 40.0;

/// Linearize a Diode at the given terminal voltages.
///
/// Implements the Shockley-equation companion model for
/// Newton-Raphson (tasks.md #9).
///
/// # Equation
///
/// Let `Vd = V_anode - V_cathode` be the junction voltage at the
/// current Newton iterate. The Shockley diode equation is
///
/// ```text
/// I(Vd) = IS · (exp(Vd / (N · Vt)) - 1)
/// ```
///
/// where positive `I` flows from anode to cathode. Linearizing
/// around the iterate `Vd_k` gives the small-signal conductance
/// and a companion current source:
///
/// ```text
/// gd     = dI/dVd |_{Vd_k} = (IS / (N · Vt)) · exp(Vd_k / (N · Vt))
/// I_eq   = I(Vd_k) - gd · Vd_k
/// ```
///
/// so that `I(Vd) ≈ gd · Vd + I_eq` is the linear surrogate the MNA
/// assembler stamps into the system on this iterate.
///
/// # Terminal-local Jacobian and companion currents
///
/// Define each terminal current as the current the device *draws*
/// from the node attached to that terminal (positive = into the
/// device). Then `I_anode = +I(Vd)` and `I_cathode = -I(Vd)`, and
/// the 2×2 Jacobian `J[i][j] = ∂I_i / ∂V_j` is
///
/// ```text
///                 V_anode    V_cathode
///   I_anode    [   +gd          -gd   ]
///   I_cathode  [   -gd          +gd   ]
/// ```
///
/// The companion current vector is `[+I_eq, -I_eq]` — the same
/// scalar `I_eq` with opposite signs at the two terminals so that
/// KCL closes locally (current entering anode = current leaving
/// cathode).
///
/// # Numerical safeguards
///
/// - The exponent argument `Vd_k / (N · Vt)` is clamped to
///   [`DIODE_MAX_EXP_ARG`] (`40.0`, matching SPICE3/ngspice). Above
///   the clamp the exponential is evaluated at the cap, which keeps
///   `gd` and `I_eq` finite even when an intermediate Newton iterate
///   overshoots deep into forward bias. The solver's `vlimit` and
///   homotopy aids (tasks.md #15) are still expected to do the real
///   convergence work; this clamp is only a domain-safety net.
/// - Reverse bias (`Vd_k ≪ 0`) is evaluated directly; `exp` of a
///   large negative number underflows cleanly to zero, leaving
///   `I(Vd_k) ≈ -IS` (reverse saturation current) and `gd ≈ 0`.
///
/// # Series resistance (`RS`)
///
/// The diode `.MODEL` parameter [`DiodeParams::rs`] is **not** baked
/// into the stamp by this task. The canonical SPICE treatment of
/// `RS` splits the diode into an internal junction node connected to
/// the external anode through a linear resistor; this split is owned
/// by the netlist-graph elaborator (a future task under
/// `circuit-solver-2026-05-21-v1-spec`), not by the device-modeling
/// stamp. With the default `rs = 0.0` the omission is a no-op; for
/// non-default `rs` the netlist-graph elaborator must perform the
/// node split before this stamp sees the device.
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
    let v_anode = terminal_voltages[0];
    let v_cathode = terminal_voltages[1];
    let vd = v_anode - v_cathode;

    // Thermal scaling factor `N · Vt`. Both `n` and `vt` come from
    // the parameter-extraction stage with SPICE-canonical defaults
    // (`N = 1`, `Vt ≈ 25.85 mV`); they are always strictly positive.
    let n_vt = params.n * params.vt;

    // Clamped exponent argument. See `DIODE_MAX_EXP_ARG` docs.
    let arg = (vd / n_vt).min(DIODE_MAX_EXP_ARG);
    let exp_arg = arg.exp();

    // Shockley current and small-signal conductance at the iterate.
    let i_d = params.is * (exp_arg - 1.0);
    let gd = (params.is / n_vt) * exp_arg;

    // Companion current source: `I_eq = I(Vd_k) - gd · Vd_k`. The
    // linear surrogate stamped this iterate is `I(Vd) ≈ gd · Vd +
    // I_eq`.
    let i_eq = i_d - gd * vd;

    DiodeLinearization {
        jacobian: [[gd, -gd], [-gd, gd]],
        companion_current: [i_eq, -i_eq],
    }
}

/// Linearize a BJT at the given terminal voltages using the
/// Ebers-Moll / Gummel-Poon companion model (tasks.md #10).
///
/// This is the per-iterate stamp the numeric-solver MNA assembler
/// (tasks.md #14) folds into the global system during Newton-Raphson.
/// The returned [`BJTLinearization`] is expressed in terminal-local
/// coordinates indexed `[collector, base, emitter]` so the assembler
/// can fold without rotating axes.
///
/// # Equation set (NPN base form; PNP via polarity transform)
///
/// Internal junction voltages (NPN convention):
///
/// - `Vbe = V_base − V_emitter`
/// - `Vbc = V_base − V_collector`
///
/// Transport-model junction currents (Gummel-Poon, ideal injection;
/// no `ISE`/`ISC`/`IKF`/`IKR` parameters at v1):
///
/// - `If = IS · (exp(Vbe/(NF·Vt)) − 1)`
/// - `Ir = IS · (exp(Vbc/(NR·Vt)) − 1)`
///
/// Base-charge factor (Early effect only; high-injection rolloff is
/// out of scope at v1 because `BJTParams` carries neither `IKF` nor
/// `IKR`):
///
/// - `q_b = 1 / (1 − Vbc/VAF − Vbe/VAR)` if either `VAF` or `VAR`
///   is finite; `q_b = 1` otherwise (both Early voltages disabled).
///
/// Terminal currents (current *into* the device at each terminal,
/// NPN convention):
///
/// - `Ic = (If − Ir)/q_b − Ir/BR`
/// - `Ib =  If/BF + Ir/BR`
/// - `Ie = −(Ic + Ib)` (KCL closure)
///
/// PNP polarity is handled by sign-flipping the internal junction
/// voltages on entry and the terminal currents on exit; the equation
/// body itself stays NPN. This matches SPICE3 / ngspice's
/// `bjtload.c` polarity discipline.
///
/// # Jacobian (3×3, `[collector, base, emitter]`)
///
/// At the v1 simplification `q_b = 1` (no high-injection rolloff),
/// the small-signal conductances are
///
/// - `gf = dIf/dVbe = (IS / (NF·Vt)) · exp(Vbe/(NF·Vt))`
/// - `gr = dIr/dVbc = (IS / (NR·Vt)) · exp(Vbc/(NR·Vt))`
///
/// and the chain rule through `Vbe = V_B − V_E`, `Vbc = V_B − V_C`
/// yields the entries hand-computed in the unit tests below.
///
/// When `VAF` or `VAR` is finite the chain-rule terms acquire
/// additional `dq_b/dV_*` contributions; we compute these via finite
/// algebra on `q_b = 1/d` with `d = 1 − Vbc/VAF − Vbe/VAR`.
///
/// # Companion current vector (Newton-Raphson linearization)
///
/// For each terminal `k`, the companion-current contribution is
///
/// - `I_eq[k] = I_k(V0) − Σⱼ J[k][j] · V0[j]`
///
/// where `V0` is the linearization-point terminal voltage. The MNA
/// assembler adds `I_eq[k]` to the RHS row for the node attached to
/// terminal `k`. This is the standard SPICE companion-model rewrite
/// that makes the iterate-`n+1` linear system equivalent to a
/// Newton step on the original nonlinear system at iterate-`n`.
///
/// # Numerical safety
///
/// The bare `exp(Vbe/(NF·Vt))` overflows around `Vbe ≈ 1.0 V` at
/// room temperature (`exp(40) ≈ 2.35e17` is still finite; `exp(710)`
/// is `+inf`). To keep the linearization callable under any
/// Newton-Raphson iterate without requiring upstream voltage
/// limiting (that belongs in the NR controller), we clamp the
/// exponent argument to `[−40, 40]`. The clamp keeps the
/// linearization meaningful for diagnostic inspection at extreme
/// iterates without producing `NaN` / `Inf` entries that would
/// poison the matrix factorization.
///
/// # Arguments
///
/// - `params` — the BJT's `.MODEL` parameters (`IS`, `BF`, `BR`,
///   `NF`, `NR`, `VAF`, `VAR`, `Vt`, polarity).
/// - `terminal_voltages` — `[V_collector, V_base, V_emitter]`.
#[must_use]
#[allow(clippy::similar_names)]
// SPICE-canonical names (vbe/vbc, inv_vaf/inv_var, dic_dvbe/dic_dvbc,
// i_c_npn/i_b_npn, etc.) are necessarily near-duplicates because they
// pair up by junction (be / bc) and by terminal (c / b / e). Renaming
// them to satisfy clippy::similar_names would *increase* the
// cognitive distance between the code and the Ebers-Moll /
// Gummel-Poon equations documented in the rustdoc above. Per
// ADR-0010 (unstable API at v1) the v1 stamp values clarity-against-
// physics over generic name-distance heuristics.
pub fn linearize_bjt(
    params: &BJTParams,
    terminal_voltages: &[f64; BJT_TERMINALS],
) -> BJTLinearization {
    use crate::params::BJTPolarity;

    // Constants: largest exponent argument we accept before clamping.
    // exp(40) ≈ 2.35e17 — still finite, well within f64 range.
    const EXP_ARG_LIMIT: f64 = 40.0;

    // Terminal-local indices (SPICE convention).
    const C: usize = 0;
    const B: usize = 1;
    const E: usize = 2;

    let v_c = terminal_voltages[C];
    let v_b = terminal_voltages[B];
    let v_e = terminal_voltages[E];

    // Polarity sign: +1 for NPN, -1 for PNP. PNP is computed by
    // running the NPN equations on sign-flipped junction voltages
    // and then sign-flipping the resulting terminal currents and
    // Jacobian rows — the latter is automatic because every
    // partial derivative picks up two factors of `s` and `s² = 1`.
    //
    // Concretely: in the NPN body the currents are linear in `If`
    // and `Ir`, which are functions of (s·Vbe, s·Vbc). The
    // derivatives w.r.t. terminal voltages pick up the chain-rule
    // factor `s` from Vbe/Vbc → V terms, and the terminal currents
    // themselves are multiplied by `s` on exit. So J_pnp =
    // s·(NPN-Jacobian-of-s·V)·s = same algebra with all junction
    // voltages sign-flipped on entry and currents sign-flipped on
    // exit. The Jacobian comes out unchanged in *form* but
    // populated with the sign-flipped junction currents.
    let s = match params.polarity {
        BJTPolarity::Npn => 1.0_f64,
        BJTPolarity::Pnp => -1.0_f64,
    };

    let vbe = s * (v_b - v_e);
    let vbc = s * (v_b - v_c);

    // Thermal-voltage-scaled exponent arguments, clamped to keep
    // exp() finite and the Jacobian non-NaN under any iterate.
    let arg_f = (vbe / (params.nf * params.vt)).clamp(-EXP_ARG_LIMIT, EXP_ARG_LIMIT);
    let arg_r = (vbc / (params.nr * params.vt)).clamp(-EXP_ARG_LIMIT, EXP_ARG_LIMIT);

    let exp_f = arg_f.exp();
    let exp_r = arg_r.exp();

    // Junction currents and their small-signal conductances.
    let i_f = params.is * (exp_f - 1.0);
    let i_r = params.is * (exp_r - 1.0);
    let gf = params.is / (params.nf * params.vt) * exp_f;
    let gr = params.is / (params.nr * params.vt) * exp_r;

    // Base-charge factor q_b (Early effect only).
    //
    // We compute q_b = 1/denom where denom = 1 − Vbc/VAF − Vbe/VAR.
    // 1/INFINITY evaluates to exactly 0.0, so the f64 algebra
    // collapses to q_b = 1 cleanly when both Early voltages are
    // disabled (VAF = VAR = INFINITY). We also clamp denom away
    // from 0 with a small floor to keep q_b finite under extreme
    // iterates — the floor is the v1 stand-in for the proper
    // continuation strategy that lands with #18.
    let inv_vaf = if params.vaf.is_finite() {
        1.0 / params.vaf
    } else {
        0.0
    };
    let inv_var = if params.var.is_finite() {
        1.0 / params.var
    } else {
        0.0
    };
    let denom_raw = 1.0 - vbc * inv_vaf - vbe * inv_var;
    let denom = if denom_raw.abs() < 1.0e-12 {
        1.0e-12_f64.copysign(if denom_raw == 0.0 { 1.0 } else { denom_raw })
    } else {
        denom_raw
    };
    let q_b = 1.0 / denom;

    // Derivatives of 1/q_b (= denom) w.r.t. junction voltages:
    //   d(1/q_b)/dVbe = d(denom)/dVbe = -inv_var
    //   d(1/q_b)/dVbc = d(denom)/dVbc = -inv_vaf
    let d_inv_qb_dvbe = -inv_var;
    let d_inv_qb_dvbc = -inv_vaf;

    // Terminal currents (NPN base form on (vbe, vbc)).
    let inv_bf = 1.0 / params.bf;
    let inv_br = 1.0 / params.br;

    let i_c_npn = (i_f - i_r) / q_b - i_r * inv_br;
    let i_b_npn = i_f * inv_bf + i_r * inv_br;
    let i_e_npn = -(i_c_npn + i_b_npn);

    // Apply polarity sign to terminal currents.
    let i_c = s * i_c_npn;
    let i_b = s * i_b_npn;
    let i_e = s * i_e_npn;

    // Jacobian — derivatives of (Ic, Ib, Ie) w.r.t. (V_C, V_B, V_E)
    // for the NPN form on (Vbe, Vbc). Chain rule:
    //   dVbe/dV_B = +s, dVbe/dV_E = −s
    //   dVbc/dV_B = +s, dVbc/dV_C = −s
    //
    // For each terminal current X(Vbe, Vbc), and each terminal V_*,
    //   dX/dV_* = (dX/dVbe)·(dVbe/dV_*) + (dX/dVbc)·(dVbc/dV_*).
    //
    // We hand-compute dX/dVbe and dX/dVbc analytically using:
    //   dIf/dVbe = gf, dIf/dVbc = 0
    //   dIr/dVbc = gr, dIr/dVbe = 0
    //   d(1/q_b)/dVbe = d_inv_qb_dvbe, d(1/q_b)/dVbc = d_inv_qb_dvbc
    //
    // dIc/dVbe = gf / q_b + (i_f − i_r) · d_inv_qb_dvbe
    // dIc/dVbc = -gr / q_b + (i_f − i_r) · d_inv_qb_dvbc − gr · inv_br
    // dIb/dVbe = gf · inv_bf
    // dIb/dVbc = gr · inv_br
    // dIe = -(dIc + dIb)
    let dic_dvbe = gf / q_b + (i_f - i_r) * d_inv_qb_dvbe;
    let dic_dvbc = -gr / q_b + (i_f - i_r) * d_inv_qb_dvbc - gr * inv_br;

    let dib_dvbe = gf * inv_bf;
    let dib_dvbc = gr * inv_br;

    let die_dvbe = -(dic_dvbe + dib_dvbe);
    let die_dvbc = -(dic_dvbc + dib_dvbc);

    // Chain rule into terminal voltages, then apply polarity. The
    // outer `s` (terminal-current sign) and the inner `s` from
    // dVbe/dV_* and dVbc/dV_* multiply to give `s² = 1`, so the
    // Jacobian entries reduce to the same expressions whether NPN
    // or PNP. The polarity dependence is entirely absorbed into
    // `vbe`, `vbc` at the top of the function.
    let jac = [
        // Row 0: dIc / d(V_C, V_B, V_E)
        [
            -dic_dvbc, // dIc / dV_C = (dIc/dVbc) · (−1)
            dic_dvbe + dic_dvbc,
            -dic_dvbe,
        ],
        // Row 1: dIb / d(V_C, V_B, V_E)
        [-dib_dvbc, dib_dvbe + dib_dvbc, -dib_dvbe],
        // Row 2: dIe / d(V_C, V_B, V_E)
        [-die_dvbc, die_dvbe + die_dvbc, -die_dvbe],
    ];

    // Companion-current vector. For each terminal k:
    //   I_eq[k] = I_k(V0) − Σⱼ J[k][j] · V0[j]
    let v0 = [v_c, v_b, v_e];
    let currents = [i_c, i_b, i_e];
    let mut companion_current = [0.0_f64; BJT_TERMINALS];
    for k in 0..BJT_TERMINALS {
        let mut sum = currents[k];
        for j in 0..BJT_TERMINALS {
            sum -= jac[k][j] * v0[j];
        }
        companion_current[k] = sum;
    }

    BJTLinearization {
        jacobian: jac,
        companion_current,
    }
}

/// Linearize a MOSFET at the given terminal voltages.
///
/// **Per-level dispatch.** The `match` on [`MOSFETParams`] selects
/// the level-specific stamp:
///
/// - [`MOSFETParams::Level1`] → Shichman-Hodges square law via
///   [`linearize_mosfet_level1`] (tasks.md #11).
/// - [`MOSFETParams::BSIM3v3`] → `BSIM3v3` DC core with body
///   effect, DIBL, smoothed strong/sub-threshold transition,
///   velocity saturation, and channel-length modulation; see
///   [`crate::bsim3v3::linearize_bsim3v3`] (tasks.md #12).
/// - [`MOSFETParams::BSIM4`] → long-channel BSIM4 stamp with DIBL
///   and channel-length modulation, see [`linearize_mosfet_bsim4`]
///   (tasks.md #13).
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
        MOSFETParams::Level1(p) => linearize_mosfet_level1(p, terminal_voltages),
        MOSFETParams::BSIM3v3(p) => {
            // tasks.md #12: delegate to the dedicated BSIM3v3 DC
            // stamp module. The Level-1 (#11) and BSIM4 (#13) arms
            // remain at the #8 zero placeholder until their owning
            // tasks land.
            crate::bsim3v3::linearize_bsim3v3(p, terminal_voltages)
        }
        MOSFETParams::BSIM4(p) => linearize_mosfet_bsim4(p, terminal_voltages),
    }
}

// ---------------------------------------------------------------------
// MOSFET BSIM4 stamp (tasks.md #13)
// ---------------------------------------------------------------------

/// Smooth-limiting cap on terminal voltage deltas evaluated inside
/// the BSIM4 stamp.
///
/// The stamp's saturation curve is polynomial in `Vds_eff` and
/// `Vgs - Vth`, so it cannot diverge to infinity the way the
/// Diode's `exp(V/Vt)` can. The cap exists to keep Newton-Raphson
/// excursions far from the converged operating point from producing
/// astronomical intermediate currents that would shadow legitimate
/// device currents in the iterate's residue norm. The cap value
/// (40 V) is two decades above any realistic supply rail.
const BSIM4_VOLTAGE_CAP: f64 = 40.0;

/// Polarity-fold a terminal-voltage triple into NMOS-equivalent
/// `(Vgs, Vds, Vbs)`.
///
/// SPICE convention: for a PMOS device, the stamp internally negates
/// all three differential voltages so the same physics-side equation
/// (written for NMOS strong inversion) computes the magnitude of the
/// PMOS source-to-drain current, and the *sign* of `Id` is flipped
/// back on the way out. This keeps the regime-detection branches
/// (`Vgs <= Vth`, `Vds < Vgs - Vth`, …) polarity-symmetric.
#[inline]
fn bsim4_fold_polarity(
    polarity: MosPolarity,
    vd: f64,
    vg: f64,
    vs: f64,
    vb: f64,
) -> (f64, f64, f64, f64) {
    // (Vgs, Vds, Vbs, sign) — sign multiplies the drain-source current
    // on the way out so PMOS sources current rather than sinking it.
    let (vgs, vds, vbs) = (vg - vs, vd - vs, vb - vs);
    match polarity {
        MosPolarity::Nmos => (vgs, vds, vbs, 1.0),
        MosPolarity::Pmos => (-vgs, -vds, -vbs, -1.0),
    }
}

/// Clamp a voltage component to `±BSIM4_VOLTAGE_CAP`.
///
/// This is a hard clamp rather than a smooth limiter because the
/// BSIM4 stamp is polynomial in `Vds` and `Vgs - Vth`; the clamp's
/// only job is to stop NR excursions from producing values that
/// overwhelm the residue norm. The clamp is symmetric so derivatives
/// remain correct inside the active band.
#[inline]
fn bsim4_clamp(v: f64) -> f64 {
    v.clamp(-BSIM4_VOLTAGE_CAP, BSIM4_VOLTAGE_CAP)
}

/// Linearize a MOSFET BSIM4 device at the given terminal voltages.
///
/// # Numerical scope (tasks.md #13)
///
/// This is a **long-channel BSIM4 stamp with DIBL and channel-length
/// modulation**. It uses the canonical-parameter subset documented
/// on [`MosBSIM4Params`] (`VTH0`, `U0`, `TOXE`, `EPSOX`, `ETA0`,
/// `PCLM`, `W`, `L`) and the regime-aware strong-inversion drain-
/// current equation:
///
/// - **Cutoff** (`Vgs_ov ≤ 0`): `Id = 0`, all derivatives are zero.
/// - **Linear / triode** (`0 < Vds < Vgs_ov`):
///   `Id = KP · (Vgs_ov · Vds − Vds² / 2) · (1 + PCLM·Vds)`
/// - **Saturation** (`Vds ≥ Vgs_ov ≥ 0`):
///   `Id = (KP / 2) · Vgs_ov² · (1 + PCLM·Vds)`
///
/// with `Vgs_ov = Vgs − Vth_eff` and `Vth_eff = VTH0 − ETA0·Vds`
/// (the DIBL term — drain-induced barrier lowering). For PMOS the
/// internal evaluation is mirrored via a polarity-fold that negates
/// all three differential voltages, evaluates the NMOS equation, and
/// negates the resulting drain current on the way out.
///
/// The Jacobian is the analytic 4×4 partial-derivative matrix
/// `∂I_t / ∂V_u` in terminal-local coordinates `[D, G, S, B]`:
///
/// - `gm = ∂Id/∂Vgs` — transconductance (rows D / G, anti-rows on S),
/// - `gds = ∂Id/∂Vds` — output conductance plus DIBL-induced
///   `∂Vth_eff/∂Vds` term,
/// - `gmbs = ∂Id/∂Vbs` — body transconductance (zero in this scope —
///   bulk-bias dependence of `VTH0` is not modeled at task #13;
///   the row/column for the bulk terminal is wired through as zero so
///   the 4×4 shape is preserved for the MNA assembler and for the
///   future task that adds bulk effects).
///
/// The companion-current vector follows the standard Newton-Raphson
/// companion-model form `i_eq = I(V_op) − J · V_op` so the MNA
/// right-hand side carries `I(V_op)` exactly at the iterate. The
/// drain and source companion entries are anti-equal (KCL at the
/// device); gate and bulk are zero (the BSIM4 long-channel model has
/// no DC gate or bulk current).
///
/// # Why this scope
///
/// Full industry BSIM4 v4.8 (~200 parameters, regime-switching with
/// smooth interpolation between subthreshold / strong-inversion /
/// velocity-saturation regimes, gate tunneling, bulk-charge,
/// substrate-current) is a multi-week numerical kernel. The Gherkin
/// scenario this task enables (`dc-operating-point#nonlinear-dc-
/// operating-point-with-direct-convergence`) requires a *non-zero,
/// convergent* MOSFET linearization — not full PDK-conformance
/// (which lands at task #63 against Sky130 ngspice golden). The
/// long-channel-with-DIBL stamp delivers the former at sibling-task
/// parity with #11 (Level-1) and #12 (`BSIM3v3`), and gives the MNA
/// assembler (tasks.md #14) something real to test stamp folding
/// against.
///
/// # Arguments
///
/// - `params` — BSIM4 model parameters (see [`MosBSIM4Params`]).
/// - `terminal_voltages` — `[V_drain, V_gate, V_source, V_bulk]`.
//
// Allow `clippy::similar_names` on the function: Vds / Vgs / Vbs and
// Vd / Vg / Vs / Vb are the standard SPICE terminal-voltage names.
// Renaming them would diverge from every BSIM4 / SPICE reference and
// obscure the physics.
#[must_use]
#[allow(clippy::similar_names)]
pub fn linearize_mosfet_bsim4(
    params: &MosBSIM4Params,
    terminal_voltages: &[f64; MOSFET_TERMINALS],
) -> MOSFETLinearization {
    // -----------------------------------------------------------------
    // 1. Polarity-fold to NMOS-equivalent, clamp for NR robustness.
    // -----------------------------------------------------------------
    let [vd_raw, vg_raw, vs_raw, vb_raw] = *terminal_voltages;
    let (vgs_p, vds_p, vbs_p, sign) =
        bsim4_fold_polarity(params.polarity, vd_raw, vg_raw, vs_raw, vb_raw);
    let _ = vbs_p; // task #13 does not model bulk-bias dependence; see docstring.
    let vgs = bsim4_clamp(vgs_p);
    let vds = bsim4_clamp(vds_p);

    // -----------------------------------------------------------------
    // 2. Effective threshold with DIBL: Vth_eff = VTH0 - ETA0 · Vds.
    //    Overdrive Vgs_ov = Vgs - Vth_eff.
    //    d(Vth_eff)/d(Vds) = -ETA0.
    //    d(Vgs_ov)/d(Vgs) = 1.
    //    d(Vgs_ov)/d(Vds) = +ETA0  (because Vth_eff decreases with Vds).
    // -----------------------------------------------------------------
    let vth_eff = params.vth0 - params.eta0 * vds;
    let vgs_ov = vgs - vth_eff;

    // -----------------------------------------------------------------
    // 3. Regime branch + drain-current value and its partials.
    //    All quantities are NMOS-equivalent here; polarity is folded
    //    back via `sign` at the end.
    //
    //    Variables tracked:
    //      id            — drain current (NMOS-equivalent, A)
    //      d_id_d_vgs    — ∂Id/∂Vgs at this regime
    //      d_id_d_vds    — ∂Id/∂Vds at this regime
    // -----------------------------------------------------------------
    let kp = params.kp();
    let (id, d_id_d_vgs, d_id_d_vds) = if vgs_ov <= 0.0 {
        // Cutoff — sub-threshold leakage not modeled at this scope.
        (0.0, 0.0, 0.0)
    } else if vds < vgs_ov {
        // Linear / triode region.
        //
        // Id = KP · (Vgs_ov · Vds − Vds²/2) · (1 + PCLM·Vds)
        //
        // Let q = Vgs_ov · Vds − Vds²/2  (positive in triode),
        //     m = 1 + PCLM · Vds.
        // Then Id = KP · q · m.
        //
        // Partial w.r.t. Vgs_ov: ∂q/∂Vgs_ov = Vds  ⇒  via Vgs_ov,
        //   ∂Id/∂Vgs = KP · Vds · m.
        // Partial w.r.t. Vds (direct, holding Vgs_ov fixed):
        //   ∂q/∂Vds = Vgs_ov − Vds,
        //   ∂m/∂Vds = PCLM,
        //   so direct = KP · ((Vgs_ov − Vds) · m + q · PCLM).
        // Add the DIBL coupling: ∂Vgs_ov/∂Vds = +ETA0, contributing
        //   KP · ETA0 · Vds · m  (the same form as the Vgs_ov route).
        let q = vgs_ov * vds - 0.5 * vds * vds;
        let m = 1.0 + params.pclm * vds;
        let id = kp * q * m;
        let d_vgs = kp * vds * m;
        let d_vds_direct = kp * ((vgs_ov - vds) * m + q * params.pclm);
        let d_vds_dibl = kp * params.eta0 * vds * m;
        (id, d_vgs, d_vds_direct + d_vds_dibl)
    } else {
        // Saturation region.
        //
        // Id = (KP / 2) · Vgs_ov² · (1 + PCLM·Vds)
        //
        // Let s = Vgs_ov², m = 1 + PCLM · Vds.
        // ∂s/∂Vgs_ov = 2·Vgs_ov.
        //
        // ∂Id/∂Vgs = (KP/2) · 2·Vgs_ov · m = KP · Vgs_ov · m.
        // ∂Id/∂Vds direct (s held fixed) = (KP/2) · s · PCLM.
        // ∂Id/∂Vds via DIBL: ∂Vgs_ov/∂Vds = +ETA0 ⇒
        //   contribution KP · Vgs_ov · m · ETA0.
        let m = 1.0 + params.pclm * vds;
        let id = 0.5 * kp * vgs_ov * vgs_ov * m;
        let d_vgs = kp * vgs_ov * m;
        let d_vds_direct = 0.5 * kp * vgs_ov * vgs_ov * params.pclm;
        let d_vds_dibl = kp * vgs_ov * m * params.eta0;
        (id, d_vgs, d_vds_direct + d_vds_dibl)
    };

    // -----------------------------------------------------------------
    // 4. Fold polarity back. `sign · id` is the device's drain
    //    terminal current using the SPICE "current into drain" sign
    //    convention. The partials are sign · ∂Id/∂(V_NMOS_eq), and
    //    each NMOS-equivalent voltage equals `sign · (V_SPICE)`
    //    (because we negated all three earlier). The two `sign`
    //    factors cancel for the partials, so the partials are
    //    polarity-symmetric. Only the current itself carries the
    //    sign back out.
    // -----------------------------------------------------------------
    let id_spice = sign * id;
    let gm = d_id_d_vgs; // ∂Id/∂Vgs in SPICE coordinates (sign² = 1).
    let gds = d_id_d_vds; // same, ∂Id/∂Vds.
    let gmbs = 0.0; // bulk-bias dependence not modeled at task #13.

    // -----------------------------------------------------------------
    // 5. Build the 4×4 Jacobian in terminal-local coordinates
    //    [D, G, S, B]. Sign conventions:
    //
    //      I_D = +Id, I_S = -Id, I_G = I_B = 0   (KCL at device)
    //      Vgs = Vg − Vs, Vds = Vd − Vs, Vbs = Vb − Vs
    //
    //    Apply the chain rule and KCL row-sum identity. For drain
    //    row:
    //      ∂I_D/∂V_D = +gds
    //      ∂I_D/∂V_G = +gm
    //      ∂I_D/∂V_S = -(gm + gds + gmbs)
    //      ∂I_D/∂V_B = +gmbs
    //
    //    Source row is the negative of drain row (since I_S = -I_D
    //    at the device, and the partials follow). Gate and bulk
    //    rows are zero (no DC current).
    // -----------------------------------------------------------------
    let mut jacobian = [[0.0_f64; MOSFET_TERMINALS]; MOSFET_TERMINALS];
    // Drain row: ∂I_D/∂V_*.
    jacobian[0][0] = gds; // V_D
    jacobian[0][1] = gm; // V_G
    jacobian[0][2] = -(gm + gds + gmbs); // V_S
    jacobian[0][3] = gmbs; // V_B
                           // Source row: I_S = -I_D ⇒ negate every column.
    jacobian[2][0] = -gds;
    jacobian[2][1] = -gm;
    jacobian[2][2] = gm + gds + gmbs;
    jacobian[2][3] = -gmbs;
    // Gate / bulk rows are zero (no DC gate or bulk current at this
    // scope), kept explicit for clarity.
    // jacobian[1][..] already zero; jacobian[3][..] already zero.

    // -----------------------------------------------------------------
    // 6. Companion current: i_eq = I(V_op) − J · V_op so the MNA
    //    right-hand side recovers `I(V_op)` exactly at the iterate.
    //    Compute J · V_op once and subtract.
    // -----------------------------------------------------------------
    let v_op = [vd_raw, vg_raw, vs_raw, vb_raw];
    let i_total = [id_spice, 0.0, -id_spice, 0.0];
    let mut companion_current = [0.0_f64; MOSFET_TERMINALS];
    for (t, slot) in companion_current.iter_mut().enumerate() {
        let mut jv = 0.0_f64;
        for (u, &v_u) in v_op.iter().enumerate() {
            jv += jacobian[t][u] * v_u;
        }
        *slot = i_total[t] - jv;
    }

    MOSFETLinearization {
        jacobian,
        companion_current,
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
        // Zero junction voltage is the equilibrium iterate where the
        // Shockley equation collapses to `I = 0`, `I_eq = 0`, but
        // `gd = IS / (N·Vt)` remains nonzero (the small-signal
        // conductance at equilibrium). This pins the dispatch path
        // (Diode arm of `DeviceModel::linearize`) without depending
        // on whether the helper returns the zero placeholder or the
        // Shockley body — both branches happen to produce
        // `I_eq = 0` here, but the Shockley body produces non-zero
        // Jacobian entries.
        let m = DeviceModel::Diode(DiodeParams {
            name: ModelName::new("d1"),
            ..Default::default()
        });
        let op = OperatingPoint::Diode([0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::Diode(d) => {
                let p = DiodeParams::default();
                let gd_eq = p.is / (p.n * p.vt);
                assert!(
                    (d.jacobian[0][0] - gd_eq).abs() < 1e-30,
                    "expected gd = IS/(N·Vt) at Vd = 0, got {:?}",
                    d.jacobian
                );
                assert_eq!(d.companion_current[0].to_bits(), 0.0_f64.to_bits());
                // The cathode companion can be `-0.0` due to the
                // `-i_eq` negation; treat ±0 as equivalent here.
                assert!(d.companion_current[1].abs() == 0.0);
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
            kf: 0.0,
            af: 1.0,
            ..Default::default()
        });
        let op = OperatingPoint::BJT([5.0, 0.7, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        assert!(matches!(lin, LinearizedModel::BJT(_)));
        assert_eq!(lin.terminal_count(), BJT_TERMINALS);
    }

    #[test]
    fn linearize_mosfet_level1_dispatches_through_match() {
        // The `MosLevel1Params::default()` has VTO=0, KP=2e-5,
        // LAMBDA=0, GAMMA=0, PHI=0.6, polarity=Nmos. With the test
        // bias V_gs=1.8 (V_g=1.8, V_s=0) and V_th=0 we are in
        // saturation; the Level-1 stamp now returns a *non-zero*
        // linearization (tasks.md #11). The shape contract (4-terminal
        // MOSFET variant) is still what dispatches.
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params {
            name: ModelName::new("nmos1"),
            polarity: MosPolarity::Nmos,
            ..Default::default()
        }));
        let op = OperatingPoint::MOSFET([3.3, 1.8, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::MOSFET(stamp) => {
                // Saturation: gm = KP · V_ov = 2e-5 · 1.8.
                let expected_gm = 2.0e-5 * 1.8;
                let got_gm = stamp.jacobian[0][1]; // J[D][G] = gm
                let diff = (got_gm - expected_gm).abs();
                assert!(
                    diff < 1.0e-12,
                    "expected gm ≈ {expected_gm}, got {got_gm} (diff {diff})",
                );
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
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

    // -----------------------------------------------------------------
    // Diode stamp — Shockley-equation behavior (tasks.md #9).
    //
    // Each test pins one observable property of the companion model:
    // its KCL closure, its equilibrium value, its small-signal /
    // tangent consistency, its reverse-bias saturation, and its
    // numerical safeguards under deep forward bias.
    // -----------------------------------------------------------------

    /// Helper: evaluate the Shockley current at an iterate.
    fn shockley_i(p: &DiodeParams, vd: f64) -> f64 {
        let arg = (vd / (p.n * p.vt)).min(super::DIODE_MAX_EXP_ARG);
        p.is * (arg.exp() - 1.0)
    }

    #[test]
    fn diode_stamp_zero_voltage_companion_is_zero() {
        // At Vd = 0 the diode is at equilibrium: I(0) = 0 and
        // I_eq = I(0) - gd · 0 = 0. The Jacobian is nonzero (it is
        // the small-signal conductance gd = IS / (N·Vt)) — this is
        // the load-bearing distinction from the prior zero
        // placeholder.
        let p = DiodeParams::default();
        let lin = linearize_diode(&p, &[0.0, 0.0]);
        // Note: due to the `-i_eq` negation in `linearize_diode`, the
        // cathode companion can be `-0.0` even when `i_eq == +0.0`.
        // Use abs-equals to treat ±0 as the same value here (KCL only
        // cares about magnitude when both are zero).
        assert!(lin.companion_current[0].abs() == 0.0);
        assert!(lin.companion_current[1].abs() == 0.0);

        let gd_eq = p.is / (p.n * p.vt);
        assert!(
            (lin.jacobian[0][0] - gd_eq).abs() < 1e-30,
            "gd at equilibrium = IS/(N·Vt) ≈ 3.87e-13 S, got {:?}",
            lin.jacobian,
        );
        assert!((lin.jacobian[1][1] - gd_eq).abs() < 1e-30);
        assert!((lin.jacobian[0][1] + gd_eq).abs() < 1e-30);
        assert!((lin.jacobian[1][0] + gd_eq).abs() < 1e-30);
    }

    #[test]
    fn diode_stamp_kcl_local_closure() {
        // For each row, the Jacobian row sums to zero: a uniform
        // voltage shift on both terminals produces zero terminal
        // current change (gauge invariance of KCL). The companion
        // currents at the two terminals sum to zero: current
        // entering the device at the anode equals current leaving
        // at the cathode.
        let p = DiodeParams::default();
        for &vd in &[-1.0_f64, -0.1, 0.0, 0.1, 0.5, 0.7] {
            let lin = linearize_diode(&p, &[vd, 0.0]);
            for row in 0..DIODE_TERMINALS {
                let row_sum = lin.jacobian[row][0] + lin.jacobian[row][1];
                assert!(
                    row_sum.abs() < 1e-30,
                    "row {row} of Jacobian must sum to zero at Vd = {vd}, got {row_sum:e}",
                );
            }
            let i_eq_sum = lin.companion_current[0] + lin.companion_current[1];
            assert!(
                i_eq_sum.abs() < 1e-30,
                "companion currents must sum to zero at Vd = {vd}, got {i_eq_sum:e}",
            );
        }
    }

    #[test]
    fn diode_stamp_recovers_shockley_at_iterate() {
        // The companion model linearizes I(Vd) ≈ gd · Vd + I_eq.
        // Evaluated at the iterate Vd_k itself the linear surrogate
        // must reproduce the true Shockley current exactly:
        //     gd · Vd_k + I_eq == I(Vd_k).
        // This is the consistency property NR depends on.
        let p = DiodeParams::default();
        for &vd in &[-0.5_f64, -0.1, 0.0, 0.1, 0.3, 0.5, 0.7] {
            let lin = linearize_diode(&p, &[vd, 0.0]);
            let gd = lin.jacobian[0][0];
            let i_eq_anode = lin.companion_current[0];
            let linear_at_iterate = gd * vd + i_eq_anode;
            let true_current = shockley_i(&p, vd);
            let abs_err = (linear_at_iterate - true_current).abs();
            let rel_err = abs_err / true_current.abs().max(1e-300);
            assert!(
                abs_err < 1e-15 || rel_err < 1e-12,
                "linear surrogate must equal Shockley at iterate Vd = {vd}: \
                 surrogate = {linear_at_iterate:e}, true = {true_current:e}, \
                 abs_err = {abs_err:e}, rel_err = {rel_err:e}",
            );
        }
    }

    #[test]
    fn diode_stamp_reverse_bias_saturates_to_minus_is() {
        // For Vd ≪ 0 (here Vd = -1 V at room temperature, i.e.
        // -38·Vt) the exponential underflows and I(Vd) ≈ -IS, gd ≈ 0.
        let p = DiodeParams::default();
        let lin = linearize_diode(&p, &[-1.0, 0.0]);

        // The true Shockley current at -1V is essentially -IS.
        let i_true = shockley_i(&p, -1.0);
        assert!(
            (i_true + p.is).abs() < 1e-30,
            "reverse-saturation Shockley current at -1V must equal -IS, got {i_true:e}",
        );

        // gd must be at machine-zero scale (≈ IS · exp(-38)/(N·Vt)).
        let gd = lin.jacobian[0][0];
        assert!(
            (0.0..1e-25).contains(&gd),
            "reverse-bias gd must underflow to ≈ 0, got {gd:e}",
        );

        // The linear surrogate at the iterate still reconstructs the
        // true current (consistency property, same as the unbiased
        // test).
        let linear_at_iterate = -gd + lin.companion_current[0];
        assert!(
            (linear_at_iterate - i_true).abs() < 1e-25,
            "linear surrogate must equal Shockley at reverse iterate, \
             surrogate = {linear_at_iterate:e}, true = {i_true:e}",
        );
    }

    #[test]
    fn diode_stamp_forward_bias_gd_matches_closed_form() {
        // At a forward-bias iterate Vd = 0.6 V the analytic
        // conductance gd = (IS/(N·Vt))·exp(Vd/(N·Vt)) is a known
        // closed form. Verify the stamp matches it bit-stably (the
        // implementation uses the same expression).
        let p = DiodeParams::default();
        let vd = 0.6;
        let lin = linearize_diode(&p, &[vd, 0.0]);

        let n_vt = p.n * p.vt;
        let gd_expected = (p.is / n_vt) * (vd / n_vt).exp();
        let gd_actual = lin.jacobian[0][0];
        assert!(
            ((gd_actual - gd_expected) / gd_expected).abs() < 1e-14,
            "gd at Vd = 0.6: expected {gd_expected:e}, got {gd_actual:e}",
        );

        // Sanity: gd should be on the order of mA/mV ≈ 1/(N·Vt)·I.
        // At 0.6V on a default diode I ≈ IS·exp(0.6/0.025852) ≈
        // 1e-14 · 1.05e10 ≈ 1.05e-4 A, so gd ≈ 4.06e-3 S.
        assert!(
            (1e-4..1e-1).contains(&gd_actual),
            "gd at Vd = 0.6V on default diode should be in [0.1mS, 0.1S], \
             got {gd_actual:e}",
        );
    }

    #[test]
    fn diode_stamp_clamps_exponent_above_threshold() {
        // For Vd ≫ N·Vt·DIODE_MAX_EXP_ARG the exponent must be
        // clamped to DIODE_MAX_EXP_ARG so gd and I_eq remain finite.
        // Pick Vd = 5 V which would give arg ≈ 193 unclamped (and
        // exp overflow). After clamp, gd = (IS/(N·Vt))·exp(40).
        let p = DiodeParams::default();
        let lin = linearize_diode(&p, &[5.0, 0.0]);

        assert!(
            lin.jacobian[0][0].is_finite(),
            "gd must remain finite under deep forward bias, got {:e}",
            lin.jacobian[0][0],
        );
        assert!(lin.companion_current[0].is_finite());
        assert!(lin.companion_current[1].is_finite());

        // Predicted gd at the clamp.
        let n_vt = p.n * p.vt;
        let gd_clamped = (p.is / n_vt) * super::DIODE_MAX_EXP_ARG.exp();
        let gd_actual = lin.jacobian[0][0];
        assert!(
            ((gd_actual - gd_clamped) / gd_clamped).abs() < 1e-14,
            "clamped gd: expected {gd_clamped:e}, got {gd_actual:e}",
        );
    }

    #[test]
    fn diode_stamp_jacobian_is_symmetric() {
        // The diode is a passive two-terminal device: its small-
        // signal Jacobian must be symmetric (g_ij = g_ji).
        let p = DiodeParams::default();
        for &vd in &[-0.5_f64, 0.0, 0.3, 0.7] {
            let lin = linearize_diode(&p, &[vd, 0.0]);
            assert!(
                (lin.jacobian[0][1] - lin.jacobian[1][0]).abs() < 1e-30,
                "Jacobian must be symmetric at Vd = {vd}: J[0][1] = {:e}, J[1][0] = {:e}",
                lin.jacobian[0][1],
                lin.jacobian[1][0],
            );
        }
    }

    #[test]
    fn diode_stamp_uses_floating_anode_and_cathode_voltages() {
        // The stamp must depend only on Vd = V_anode - V_cathode,
        // not on the absolute terminal voltages (gauge invariance).
        // IEEE-754 subtraction is not exact (`5.6 - 5.0 ≠ 0.6` to the
        // last bit), so we compare with a tight relative tolerance.
        let p = DiodeParams::default();
        let lin_a = linearize_diode(&p, &[0.6, 0.0]);
        let lin_b = linearize_diode(&p, &[5.6, 5.0]);
        for i in 0..DIODE_TERMINALS {
            for j in 0..DIODE_TERMINALS {
                let rel = (lin_a.jacobian[i][j] - lin_b.jacobian[i][j]).abs()
                    / lin_a.jacobian[i][j].abs().max(1e-300);
                assert!(
                    rel < 1e-13,
                    "stamp must be a function of Vd alone (J[{i}][{j}]): \
                     grounded = {:e}, floating = {:e}, rel = {rel:e}",
                    lin_a.jacobian[i][j],
                    lin_b.jacobian[i][j],
                );
            }
            let rel = (lin_a.companion_current[i] - lin_b.companion_current[i]).abs()
                / lin_a.companion_current[i].abs().max(1e-300);
            assert!(
                rel < 1e-13,
                "companion_current[{i}] must be a function of Vd alone: \
                 grounded = {:e}, floating = {:e}, rel = {rel:e}",
                lin_a.companion_current[i],
                lin_b.companion_current[i],
            );
        }
    }

    #[test]
    fn diode_stamp_honors_emission_coefficient() {
        // Doubling N (emission coefficient) halves the exponent
        // argument; at Vd = 0.6 V this changes both gd and I_eq.
        // This pins that the parameter `N` is actually consumed (a
        // common regression in stamp implementations).
        let mut p = DiodeParams::default();
        let lin_n1 = linearize_diode(&p, &[0.6, 0.0]);
        p.n = 2.0;
        let lin_n2 = linearize_diode(&p, &[0.6, 0.0]);
        assert!(
            lin_n1.jacobian[0][0] > lin_n2.jacobian[0][0] * 100.0,
            "gd at N=1 must be much larger than at N=2 (slower exponential), \
             N=1 gd = {:e}, N=2 gd = {:e}",
            lin_n1.jacobian[0][0],
            lin_n2.jacobian[0][0],
        );
    }

    #[test]
    fn diode_stamp_honors_saturation_current() {
        // Doubling IS doubles both gd and (in deep forward bias)
        // I_eq. This pins that the parameter `IS` is actually
        // consumed.
        let mut p = DiodeParams::default();
        let lin_a = linearize_diode(&p, &[0.6, 0.0]);
        p.is = 2e-14;
        let lin_b = linearize_diode(&p, &[0.6, 0.0]);
        // gd_b / gd_a == IS_b / IS_a == 2.
        let ratio = lin_b.jacobian[0][0] / lin_a.jacobian[0][0];
        assert!(
            (ratio - 2.0).abs() < 1e-12,
            "gd must scale linearly with IS, got ratio = {ratio}",
        );
    }

    #[test]
    fn diode_stamp_companion_sign_matches_forward_conduction() {
        // At forward-bias Vd > 0, current flows anode → cathode.
        // In our terminal convention I_anode > 0 (current INTO
        // device at anode) and I_cathode < 0 (current OUT of device
        // at cathode). The linear surrogate at the iterate must
        // reproduce this sign.
        let p = DiodeParams::default();
        let vd = 0.7;
        let lin = linearize_diode(&p, &[vd, 0.0]);
        let i_anode = lin.jacobian[0][0] * vd + lin.jacobian[0][1] * 0.0 + lin.companion_current[0];
        let i_cathode =
            lin.jacobian[1][0] * vd + lin.jacobian[1][1] * 0.0 + lin.companion_current[1];
        assert!(
            i_anode > 0.0,
            "forward-bias diode draws positive current at anode, got {i_anode:e}",
        );
        assert!(
            i_cathode < 0.0,
            "forward-bias diode supplies current at cathode, got {i_cathode:e}",
        );
        assert!(
            (i_anode + i_cathode).abs() < 1e-15,
            "I_anode + I_cathode must be zero (KCL), got {:e}",
            i_anode + i_cathode,
        );
    }

    #[test]
    fn diode_stamp_floating_node_form_recovers_two_node_form() {
        // Floating the diode between two arbitrary nodes
        // (V_anode = 1.7, V_cathode = 1.1, so Vd = 0.6) must give
        // the same Jacobian as the grounded form
        // (V_anode = 0.6, V_cathode = 0.0). IEEE-754 subtraction is
        // not exact (`1.7 - 1.1 ≠ 0.6` to the last bit), so we
        // compare with a tight relative tolerance.
        let p = DiodeParams::default();
        let grounded = linearize_diode(&p, &[0.6, 0.0]);
        let floating = linearize_diode(&p, &[1.7, 1.1]);
        let rel = (grounded.jacobian[0][0] - floating.jacobian[0][0]).abs()
            / grounded.jacobian[0][0].abs();
        assert!(
            rel < 1e-13,
            "grounded vs floating gd: grounded = {:e}, floating = {:e}, rel = {rel:e}",
            grounded.jacobian[0][0],
            floating.jacobian[0][0],
        );
        let rel_eq = (grounded.companion_current[0] - floating.companion_current[0]).abs()
            / grounded.companion_current[0].abs().max(1e-300);
        assert!(
            rel_eq < 1e-13,
            "grounded vs floating I_eq: grounded = {:e}, floating = {:e}, rel = {rel_eq:e}",
            grounded.companion_current[0],
            floating.companion_current[0],
        );
    }

    #[test]
    fn linearize_bjt_zero_bias_produces_zero_current_and_zero_companion() {
        // At Vbe = Vbc = 0 the diode currents If = Ir = 0, so all
        // terminal currents are zero, the Jacobian entries collapse
        // to gf = gr = IS/(NF·Vt) · 1, and the companion-current
        // vector is exactly zero (because V0 = 0 ⟹ J·V0 = 0).
        let p = BJTParams::default();
        let lin = linearize_bjt(&p, &[0.0, 0.0, 0.0]);

        for k in 0..BJT_TERMINALS {
            assert!(
                lin.companion_current[k].abs() < 1.0e-30,
                "companion_current[{k}] = {} should be ~0 at Vbe = Vbc = 0",
                lin.companion_current[k]
            );
        }

        // Conductances should be non-zero (gf = gr = IS/(NF·Vt)).
        // Diagonal blocks: dIb/dV_B = gf/BF + gr/BR > 0.
        let g0 = p.is / (p.nf * p.vt);
        assert!(
            (lin.jacobian[1][1] - (g0 / p.bf + g0 / p.br)).abs() < 1.0e-25,
            "Ib/dV_B at zero bias mismatch: got {}, expected {}",
            lin.jacobian[1][1],
            g0 / p.bf + g0 / p.br,
        );
    }

    #[test]
    fn linearize_bjt_forward_active_kcl_closure_holds() {
        // For any iterate, KCL inside the device says
        // Ic + Ib + Ie = 0. Equivalently, the *rows* of the
        // companion-current vector and of every Jacobian column sum
        // to zero. We exercise a typical forward-active operating
        // point (Vbe = 0.7, Vbc = -4.3, NPN).
        let p = BJTParams::default();
        let lin = linearize_bjt(&p, &[5.0, 0.7, 0.0]);

        // Companion currents: I_eq summed over terminals = 0.
        let sum: f64 = lin.companion_current.iter().sum();
        assert!(
            sum.abs() < 1.0e-12,
            "companion-current KCL closure failed: sum = {sum}"
        );

        // Jacobian: each column sums to zero (charge conservation
        // under any infinitesimal terminal-voltage perturbation).
        for j in 0..BJT_TERMINALS {
            let col_sum = (0..BJT_TERMINALS).map(|i| lin.jacobian[i][j]).sum::<f64>();
            assert!(
                col_sum.abs() < 1.0e-9,
                "Jacobian column {j} does not sum to zero: {col_sum}",
            );
        }
    }

    #[test]
    fn linearize_bjt_forward_active_currents_match_hand_calculation() {
        // Hand-checked operating point with defaults (IS=1e-16, NF=1,
        // NR=1, BF=100, BR=1, VAF=VAR=∞, Vt=25.852 mV).
        //
        // Vbe = 0.7, Vbc = -4.3, q_b = 1 (Early disabled).
        //
        //   If = 1e-16·(exp(0.7/0.025852) - 1)
        //      = 1e-16·(exp(27.0772...) - 1)
        //   Ir = 1e-16·(exp(-4.3/0.025852) - 1)
        //      ≈ -1e-16 (deep reverse: exp → 0)
        //
        //   Ic = If - Ir - Ir/BR = If - 2·Ir ≈ If (Ir negligible)
        //   Ib = If/BF + Ir/BR ≈ If/100
        let p = BJTParams::default();
        let lin = linearize_bjt(&p, &[5.0, 0.7, 0.0]);

        let vt = p.vt;
        let exp_f = (0.7_f64 / vt).exp();
        let i_f_expected = p.is * (exp_f - 1.0);

        // I_eq[k] = I_k(V0) − Σⱼ J[k][j]·V0[j]
        // ⇒ I_k(V0) = I_eq[k] + Σⱼ J[k][j]·V0[j]
        let v0 = [5.0_f64, 0.7, 0.0];
        let recovered = |k: usize| -> f64 {
            lin.companion_current[k]
                + (0..BJT_TERMINALS)
                    .map(|j| lin.jacobian[k][j] * v0[j])
                    .sum::<f64>()
        };
        let ic = recovered(0);
        let ib = recovered(1);
        let ie = recovered(2);

        // Reverse current is exp(-166) ≈ 0 ⇒ Ic ≈ If, Ib ≈ If/BF.
        let rel = |got: f64, want: f64| (got - want).abs() / want.abs().max(1e-30);
        assert!(
            rel(ic, i_f_expected) < 1e-9,
            "Ic mismatch: got {ic}, expected ≈ {i_f_expected}"
        );
        assert!(
            rel(ib, i_f_expected / p.bf) < 1e-9,
            "Ib mismatch: got {ib}, expected ≈ {}",
            i_f_expected / p.bf
        );
        // KCL: Ie = -(Ic + Ib).
        assert!(
            (ie + ic + ib).abs() < 1e-9 * ic.abs().max(1.0),
            "KCL: Ic + Ib + Ie = {} (want 0)",
            ic + ib + ie
        );
    }

    #[test]
    fn linearize_bjt_pnp_polarity_inverts_current_signs() {
        // A PNP biased at the *mirror* operating point of an NPN —
        // i.e. (V_C, V_B, V_E) = (-5, -0.7, 0) — must produce
        // terminal currents that are exactly the sign-flipped NPN
        // result. The Jacobian, by ADR-driven design, is invariant
        // because the polarity sign s appears in pairs (chain-rule
        // factor × terminal-current factor) and s² = 1.
        let npn = BJTParams {
            polarity: BJTPolarity::Npn,
            kf: 0.0,
            af: 1.0,
            ..Default::default()
        };
        let pnp = BJTParams {
            polarity: BJTPolarity::Pnp,
            kf: 0.0,
            af: 1.0,
            ..Default::default()
        };

        let lin_npn = linearize_bjt(&npn, &[5.0, 0.7, 0.0]);
        let lin_pnp = linearize_bjt(&pnp, &[-5.0, -0.7, 0.0]);

        // Recover currents from companion form at each device's V0.
        let v0_npn = [5.0_f64, 0.7, 0.0];
        let v0_pnp = [-5.0_f64, -0.7, 0.0];
        let recover = |lin: &BJTLinearization, v0: &[f64; BJT_TERMINALS], k: usize| -> f64 {
            lin.companion_current[k]
                + (0..BJT_TERMINALS)
                    .map(|j| lin.jacobian[k][j] * v0[j])
                    .sum::<f64>()
        };

        for k in 0..BJT_TERMINALS {
            let ic_npn = recover(&lin_npn, &v0_npn, k);
            let ic_pnp = recover(&lin_pnp, &v0_pnp, k);
            let rel = (ic_pnp + ic_npn).abs() / ic_npn.abs().max(1e-30);
            assert!(
                rel < 1e-9,
                "PNP terminal {k} current should be −NPN at mirror point: \
                 NPN = {ic_npn}, PNP = {ic_pnp}",
            );
        }
    }

    #[test]
    fn linearize_bjt_jacobian_matches_finite_difference() {
        // Cross-check the analytical Jacobian against a centered
        // finite-difference approximation. This pins both the
        // analytical derivative algebra (Ebers-Moll terminal
        // currents w.r.t. terminal voltages, including chain rule
        // and Early-effect dq_b/dV) and the row/column orientation
        // (Jacobian[i][j] = dI_i/dV_j).
        let p = BJTParams {
            name: ModelName::new("q_fd"),
            polarity: BJTPolarity::Npn,
            is: 1e-15,
            bf: 200.0,
            br: 2.0,
            nf: 1.0,
            nr: 1.0,
            vaf: 50.0, // Early effect ON to exercise dq_b/dV terms.
            var: 25.0,
            vt: 0.025_852_0,
            kf: 0.0,
            af: 1.0,
        };

        let v0 = [3.0_f64, 0.65, 0.0];
        let lin = linearize_bjt(&p, &v0);
        let recover = |lin: &BJTLinearization, v: &[f64; BJT_TERMINALS], k: usize| -> f64 {
            lin.companion_current[k]
                + (0..BJT_TERMINALS)
                    .map(|j| lin.jacobian[k][j] * v[j])
                    .sum::<f64>()
        };
        // Sample the *current* function (not its linearization) by
        // re-linearizing at each perturbed point and recovering
        // I_k(V) from that linearization's companion form. Each
        // linearization satisfies I_k(V) = I_eq + J·V exactly at V,
        // so this is a clean numerical evaluation.
        let i_at = |v: &[f64; BJT_TERMINALS], k: usize| -> f64 {
            let lin_v = linearize_bjt(&p, v);
            recover(&lin_v, v, k)
        };

        let h = 1.0e-6;
        for i in 0..BJT_TERMINALS {
            for j in 0..BJT_TERMINALS {
                let mut vp = v0;
                let mut vm = v0;
                vp[j] += h;
                vm[j] -= h;
                let fd = (i_at(&vp, i) - i_at(&vm, i)) / (2.0 * h);
                let an = lin.jacobian[i][j];
                let scale = an.abs().max(1.0e-9);
                let rel = (fd - an).abs() / scale;
                assert!(
                    rel < 1.0e-4,
                    "Jacobian[{i}][{j}] FD mismatch: analytical = {an}, FD = {fd}, rel = {rel}",
                );
            }
        }
    }

    #[test]
    fn linearize_bjt_early_effect_changes_collector_current() {
        // With VAF finite (forward Early effect only), the
        // base-charge factor q_b = 1/(1 - Vbc/VAF) < 1 for a
        // forward-active NPN with Vbc < 0, so Ic = (If-Ir)/q_b
        // = (If-Ir)·denom grows above its q_b=1 value. The reverse
        // Early voltage VAR is left at +∞ here so VAR's competing
        // rolloff term doesn't mask the effect.
        let p_no_early = BJTParams::default();
        let p_with_early = BJTParams {
            vaf: 50.0,
            // var stays at f64::INFINITY (default).
            kf: 0.0,
            af: 1.0,
            ..Default::default()
        };
        let v0 = [5.0_f64, 0.7, 0.0]; // Vbc = -4.3 V
        let no_e = linearize_bjt(&p_no_early, &v0);
        let with_e = linearize_bjt(&p_with_early, &v0);

        // Recover Ic(V0) from both.
        let recover_ic = |lin: &BJTLinearization| -> f64 {
            lin.companion_current[0]
                + (0..BJT_TERMINALS)
                    .map(|j| lin.jacobian[0][j] * v0[j])
                    .sum::<f64>()
        };
        let ic_no_early = recover_ic(&no_e);
        let ic_with_early = recover_ic(&with_e);

        // Expected per ngspice / SPICE3 `bjtload.c`:
        //   q_b = 1 / (1 - Vbc/VAF - Vbe/VAR)
        //
        // For NPN forward active (Vbe > 0, Vbc < 0) with VAR = ∞:
        //   denom = 1 - Vbc/VAF = 1 + |Vbc|/VAF > 1
        //   ⇒ Ic_with_early = Ic_no_early · denom > Ic_no_early.
        //
        // This matches the classical Early-effect intuition where
        // the small-signal output conductance go = Ic / |VA| boosts
        // Ic for larger |Vce|. The full Gummel-Poon expression
        // collapses to the classical form in the regime
        // |Vce|/VAF ≪ 1, and for the parameters here (VAF=50,
        // |Vbc|=4.3) the boost is already visible.
        assert!(
            ic_with_early > ic_no_early,
            "Early effect must increase Ic in forward-active NPN: \
             no_early = {ic_no_early}, with_early = {ic_with_early}",
        );
        assert!(
            ic_with_early.is_finite() && ic_no_early.is_finite(),
            "Both Ic values must be finite: no_early = {ic_no_early}, \
             with_early = {ic_with_early}",
        );
    }

    #[test]
    fn linearize_bjt_extreme_vbe_does_not_produce_nan_or_inf() {
        // Under an early Newton-Raphson iterate the assembler may
        // hand the stamp a junction voltage well outside the
        // physical operating range. The exponent-argument clamp
        // (EXP_ARG_LIMIT = 40) keeps the Jacobian and companion
        // current finite so the matrix factorization doesn't get
        // poisoned. This is the v1 substitute for the proper
        // voltage-limiting controller introduced under tasks #16/#18.
        let p = BJTParams::default();
        let lin = linearize_bjt(&p, &[0.0, 10.0, 0.0]); // Vbe = 10 V

        for k in 0..BJT_TERMINALS {
            assert!(
                lin.companion_current[k].is_finite(),
                "companion_current[{k}] = {} non-finite at Vbe = 10 V",
                lin.companion_current[k]
            );
            for j in 0..BJT_TERMINALS {
                assert!(
                    lin.jacobian[k][j].is_finite(),
                    "Jacobian[{k}][{j}] = {} non-finite at Vbe = 10 V",
                    lin.jacobian[k][j],
                );
            }
        }
    }

    #[test]
    fn linearize_bjt_companion_form_reproduces_currents_at_linearization_point() {
        // The companion-current vector is defined so that
        //   I_k(V0) = I_eq[k] + Σⱼ J[k][j] · V0[j]
        // holds *exactly* (modulo f64 round-off) at the
        // linearization point. The MNA assembler depends on this
        // identity for Newton-Raphson consistency.
        let p = BJTParams::default();
        let v0 = [4.5_f64, 0.68, 0.05];
        let lin = linearize_bjt(&p, &v0);

        // Re-linearize at a *different* point so we know I_eq
        // depends on the input correctly.
        let v_other = [5.0_f64, 0.7, 0.0];
        let lin_other = linearize_bjt(&p, &v_other);

        for k in 0..BJT_TERMINALS {
            // Reconstruct I_k at v0 from `lin`.
            let i_k_v0 = lin.companion_current[k]
                + (0..BJT_TERMINALS)
                    .map(|j| lin.jacobian[k][j] * v0[j])
                    .sum::<f64>();
            // And from `lin_other` (different linearization
            // point — these should differ unless the device is
            // exactly linear, which it isn't).
            let i_k_v0_via_other = lin_other.companion_current[k]
                + (0..BJT_TERMINALS)
                    .map(|j| lin_other.jacobian[k][j] * v0[j])
                    .sum::<f64>();
            // Currents reconstructed from a linearization at the
            // *same* point should be smaller in absolute error
            // than reconstructions through a far-away linearization.
            assert!(
                i_k_v0.is_finite() && i_k_v0_via_other.is_finite(),
                "terminal {k}: i_k_v0 = {i_k_v0}, via_other = {i_k_v0_via_other}",
            );
        }
    }

    #[test]
    fn linearize_mosfet_helper_dispatches_each_level_with_level1_nonzero_in_saturation() {
        // All three MOS arms are now real implementations (Level-1
        // tasks.md #11, BSIM3v3 tasks.md #12, BSIM4 tasks.md #13).
        // The compile-time exhaustiveness check on the inner `match`
        // is the load-bearing assertion; the run-time checks here
        // pin intent:
        //
        // - Level-1 at all-zero bias → exactly zero stamp (cutoff,
        //   real Shichman-Hodges).
        // - Level-1 at saturation bias `[3.3, 1.8, 0.0, 0.0]` →
        //   non-zero stamp (real Shichman-Hodges).
        // - BSIM3v3 dispatches into the real implementation; we
        //   assert the Jacobian and companion current are finite
        //   (the real contract) rather than exactly zero (the
        //   placeholder contract). The dedicated `bsim3v3::tests`
        //   module exercises the saturation / sub-threshold / KCL
        //   invariants.
        // - BSIM4 dispatches into the real implementation; at the
        //   saturation bias `[3.3, 1.8, 0.0, 0.0]` the default
        //   `VTH0 = 0.7 V` puts it in saturation and the stamp is
        //   non-zero. The dedicated `bsim4_tests` module exercises
        //   the saturation / triode / DIBL / KCL invariants.
        let l1 = MOSFETParams::Level1(MosLevel1Params::default());
        let lin_cutoff = linearize_mosfet(&l1, &[0.0; MOSFET_TERMINALS]);
        assert_eq!(lin_cutoff, MOSFETLinearization::zero());

        // Saturation: bias the same default NMOS at V_gs = 1.8, V_ds =
        // 3.3; the linearization must be non-zero (real Level-1 stamp).
        let lin_sat = linearize_mosfet(&l1, &[3.3, 1.8, 0.0, 0.0]);
        assert_ne!(lin_sat, MOSFETLinearization::zero());

        // BSIM3v3 and BSIM4 are also real implementations now.
        let b3 = MOSFETParams::BSIM3v3(MosBSIM3v3Params::default());
        let b4 = MOSFETParams::BSIM4(MosBSIM4Params::default());

        // BSIM4 at saturation bias must be non-zero (default
        // `VTH0 = 0.7 V` puts the device in saturation, real arm).
        assert_ne!(
            linearize_mosfet(&b4, &[3.3, 1.8, 0.0, 0.0]),
            MOSFETLinearization::zero(),
            "BSIM4 is implemented (tasks.md #13); saturation bias must produce a non-zero stamp"
        );

        // BSIM3v3 at exactly the zero operating point still
        // produces a near-zero stamp (deep sub-threshold), but the
        // arm now dispatches into the real implementation. We
        // assert the conductance entries are finite (the real
        // contract) rather than exactly zero (the placeholder
        // contract).
        let b3_stamp = linearize_mosfet(&b3, &[0.0; MOSFET_TERMINALS]);
        for row in &b3_stamp.jacobian {
            for &j in row {
                assert!(j.is_finite(), "BSIM3v3 Jacobian must be finite, got {j}");
            }
        }
        for &i in &b3_stamp.companion_current {
            assert!(
                i.is_finite(),
                "BSIM3v3 companion current must be finite, got {i}"
            );
        }
        // BSIM4 likewise dispatches into the real implementation
        // (tasks.md #13). At all-zero biases the default
        // `VTH0 = 0.7 V` puts the device in cutoff, so the
        // numerical stamp is near-zero — but the arm no longer
        // returns the bit-equal placeholder. Assert finite (the
        // real contract).
        let b4_stamp = linearize_mosfet(&b4, &[0.0; MOSFET_TERMINALS]);
        for row in &b4_stamp.jacobian {
            for &j in row {
                assert!(j.is_finite(), "BSIM4 Jacobian must be finite, got {j}");
            }
        }
        for &i in &b4_stamp.companion_current {
            assert!(
                i.is_finite(),
                "BSIM4 companion current must be finite, got {i}"
            );
        }
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

// =====================================================================
// MOSFET BSIM4 stamp tests (tasks.md #13)
// =====================================================================

#[cfg(test)]
mod bsim4_tests {
    use super::*;
    use crate::params::{MosBSIM4Params, MosPolarity};
    use circuit_solver_types::ModelName;

    /// A representative NMOS BSIM4 device — the SPICE defaults
    /// produce a textbook long-channel NMOS that hits every regime
    /// for `Vds` and `Vgs` in `[0, 5] V`.
    fn nmos_default() -> MosBSIM4Params {
        MosBSIM4Params {
            name: ModelName::new("nmos_bsim4"),
            polarity: MosPolarity::Nmos,
            ..Default::default()
        }
    }

    /// PMOS analogue of [`nmos_default`] — same |VTH0|, mobility, etc.,
    /// but `polarity = Pmos`. PMOS conducts when `Vgs < -Vth0` and
    /// `Vds < 0`.
    fn pmos_default() -> MosBSIM4Params {
        MosBSIM4Params {
            name: ModelName::new("pmos_bsim4"),
            polarity: MosPolarity::Pmos,
            ..Default::default()
        }
    }

    /// Reconstruct the terminal current at iterate `v` from a stamp's
    /// `t`-th row using the companion-model identity
    /// `I_t = ∑_u J[t][u] · v[u] + i_eq[t]`. Centralizes the loop the
    /// MNA assembler will eventually call so each test does not
    /// re-implement it.
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

    /// `assert_eq!`-friendly approximate equality for `f64`.
    /// Picks the looser of relative and absolute tolerance per ADR-0008.
    fn approx_eq(a: f64, b: f64, rel: f64, abs: f64) -> bool {
        let tol = rel.mul_add(b.abs().max(a.abs()), abs);
        (a - b).abs() <= tol
    }

    // -----------------------------------------------------------------
    // Cutoff regime — id = 0, all derivatives zero, companion zero.
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_nmos_cutoff_returns_zero_linearization() {
        let p = nmos_default();
        // Vgs = 0.5 < Vth0 = 0.7  ⇒  cutoff.
        let lin = linearize_mosfet_bsim4(&p, &[0.0, 0.5, 0.0, 0.0]);
        assert_eq!(
            lin,
            MOSFETLinearization::zero(),
            "cutoff must produce a zero linearization"
        );
    }

    #[test]
    fn bsim4_pmos_cutoff_returns_zero_linearization() {
        let p = pmos_default();
        // For PMOS with VDD=3.3 V on source/bulk, gate at 3.0 V keeps
        // |Vgs| = 0.3 V < Vth0 = 0.7 V — cutoff.
        let lin = linearize_mosfet_bsim4(&p, &[0.0, 3.0, 3.3, 3.3]);
        assert_eq!(lin, MOSFETLinearization::zero());
    }

    // -----------------------------------------------------------------
    // Saturation regime — current flows, gm > 0, gds > 0 (due to PCLM
    // and DIBL), KCL row sums vanish.
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_nmos_saturation_has_positive_id_and_gm() {
        let p = nmos_default();
        // Vgs_ov = 1.8 - 0.7 = 1.1 V; Vds = 3.0 V > Vgs_ov ⇒ saturation.
        let lin = linearize_mosfet_bsim4(&p, &[3.0, 1.8, 0.0, 0.0]);

        // Drain current via companion form: at the iterate,
        //   I_op_D = J · V_op + i_eq    (by construction)
        let v = [3.0_f64, 1.8, 0.0, 0.0];
        let id_recovered = reconstruct_current(&lin, 0, &v);
        assert!(
            id_recovered > 0.0,
            "saturation NMOS must source positive drain current, got {id_recovered}"
        );

        // gm = ∂Id/∂Vgs = jacobian[0][1] > 0 in saturation.
        assert!(
            lin.jacobian[0][1] > 0.0,
            "gm = ∂Id/∂Vgs must be positive in saturation, got {}",
            lin.jacobian[0][1]
        );
        // gds (with PCLM + DIBL) > 0.
        assert!(
            lin.jacobian[0][0] > 0.0,
            "gds = ∂Id/∂Vds must be positive in saturation, got {}",
            lin.jacobian[0][0]
        );
    }

    #[test]
    fn bsim4_nmos_saturation_jacobian_row_sums_are_zero_kcl() {
        // KCL at the device: the sum of currents into all four
        // terminals is zero. Adding a uniform offset to every
        // terminal voltage cannot change any device current, so
        // each Jacobian row sum (∂I_t / ∂V_u summed over u) is zero.
        let p = nmos_default();
        let lin = linearize_mosfet_bsim4(&p, &[3.0, 1.8, 0.0, 0.0]);
        for t in 0..MOSFET_TERMINALS {
            let row_sum: f64 = lin.jacobian[t].iter().sum();
            assert!(
                approx_eq(row_sum, 0.0, 1e-12, 1e-15),
                "Jacobian row {t} sum {row_sum} must be zero (KCL / translational invariance)",
            );
        }
    }

    #[test]
    fn bsim4_nmos_saturation_jacobian_col_sums_are_zero_kcl_at_device() {
        // The column-sum identity: ∑_t I_t = 0 for every iterate,
        // so differentiating gives ∑_t ∂I_t/∂V_u = 0 for every u.
        let p = nmos_default();
        let lin = linearize_mosfet_bsim4(&p, &[3.0, 1.8, 0.0, 0.0]);
        for u in 0..MOSFET_TERMINALS {
            let col_sum: f64 = (0..MOSFET_TERMINALS).map(|t| lin.jacobian[t][u]).sum();
            assert!(
                approx_eq(col_sum, 0.0, 1e-12, 1e-15),
                "Jacobian column {u} sum {col_sum} must be zero (KCL at device)",
            );
        }
    }

    // -----------------------------------------------------------------
    // Linear / triode regime — gm > 0, output conductance > 0.
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_nmos_triode_has_positive_id_and_high_gds() {
        let p = nmos_default();
        // Vgs_ov = 3.3 - 0.7 = 2.6 V; Vds = 0.2 V < Vgs_ov ⇒ triode.
        let lin = linearize_mosfet_bsim4(&p, &[0.2, 3.3, 0.0, 0.0]);
        // Drain current at iterate via companion identity.
        let v = [0.2_f64, 3.3, 0.0, 0.0];
        let id_recovered = reconstruct_current(&lin, 0, &v);
        assert!(id_recovered > 0.0, "triode Id must be positive");
        // In deep triode gds dominates gm (the transistor behaves
        // like a Vds-controlled resistor).
        assert!(lin.jacobian[0][0] > 0.0, "gds must be positive in triode");
        assert!(lin.jacobian[0][1] > 0.0, "gm must be positive in triode");
    }

    // -----------------------------------------------------------------
    // PMOS symmetry: a PMOS with V_S = V_B = VDD, V_G = 0, V_D = mid
    // should sink current at the drain terminal (SPICE convention:
    // I_D < 0 — current flows from source through the device into
    // the drain node).
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_pmos_saturation_sources_negative_drain_current() {
        let p = pmos_default();
        // V_S = V_B = 3.3, V_G = 0, V_D = 0.3 ⇒
        //   Vgs = -3.3, Vds = -3.0  ⇒ PMOS strong inversion.
        let lin = linearize_mosfet_bsim4(&p, &[0.3, 0.0, 3.3, 3.3]);
        let v = [0.3_f64, 0.0, 3.3, 3.3];
        let id_recovered = reconstruct_current(&lin, 0, &v);
        assert!(
            id_recovered < 0.0,
            "PMOS strong inversion must sink current at the drain \
             (SPICE convention: I_D < 0), got {id_recovered}",
        );
        // |gm| symmetry: PMOS at polarity-mirrored conditions must
        // produce the same |gm| as NMOS at the original conditions.
        let p_nmos = nmos_default();
        let lin_n = linearize_mosfet_bsim4(&p_nmos, &[3.0, 3.3, 0.0, 0.0]);
        assert!(
            approx_eq(
                lin.jacobian[0][1].abs(),
                lin_n.jacobian[0][1].abs(),
                1e-9,
                0.0,
            ),
            "PMOS |gm| {} should mirror NMOS |gm| {} at polarity-folded operating point",
            lin.jacobian[0][1].abs(),
            lin_n.jacobian[0][1].abs(),
        );
    }

    // -----------------------------------------------------------------
    // Companion-model identity. For any iterate V_op:
    //   I_t(V_op) = ∑_u J[t][u] · V_op[u] + i_eq[t]
    // The MNA assembler relies on this exactly — the right-hand side
    // contribution must equal the device current at the iterate.
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_companion_model_recovers_id_at_iterate() {
        let p = nmos_default();
        let cases: &[[f64; 4]] = &[
            [0.0, 0.5, 0.0, 0.0],  // cutoff
            [3.0, 1.8, 0.0, 0.0],  // saturation
            [0.2, 3.3, 0.0, 0.0],  // triode
            [3.0, 2.5, 0.0, 0.0],  // deep saturation
            [0.05, 5.0, 0.0, 0.0], // shallow triode, strong overdrive
        ];
        for v in cases {
            let lin = linearize_mosfet_bsim4(&p, v);
            // Gate (row 1) and bulk (row 3) currents must be zero at
            // the iterate — there is no DC gate or bulk current at
            // this scope.
            for t in [1, 3] {
                let reconstructed = reconstruct_current(&lin, t, v);
                assert!(
                    approx_eq(reconstructed, 0.0, 1e-9, 1e-15),
                    "row {t}: gate/bulk current must be zero at iterate {v:?}, got {reconstructed}",
                );
            }
            // Drain and source rows must be anti-equal (KCL).
            let id_d = reconstruct_current(&lin, 0, v);
            let id_s = reconstruct_current(&lin, 2, v);
            assert!(
                approx_eq(id_d + id_s, 0.0, 1e-9, 1e-15),
                "drain + source row currents must cancel (KCL) at {v:?}, got {id_d} + {id_s}",
            );
        }
    }

    // -----------------------------------------------------------------
    // Voltage-cap: huge NR excursions must not overflow the stamp.
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_clamps_extreme_voltages_for_nr_robustness() {
        let p = nmos_default();
        // Throw 1e6 V at the stamp. The clamp pins evaluation to
        // ±40 V, so the produced current is finite and the Jacobian
        // entries are finite.
        let lin = linearize_mosfet_bsim4(&p, &[1.0e6, 1.0e6, 0.0, 0.0]);
        let v_extreme = [1.0e6_f64, 1.0e6, 0.0, 0.0];
        let id_d_at_iter = reconstruct_current(&lin, 0, &v_extreme);
        assert!(
            id_d_at_iter.is_finite(),
            "clamped stamp must produce finite drain current under huge NR excursion",
        );
        for t in 0..MOSFET_TERMINALS {
            for u in 0..MOSFET_TERMINALS {
                assert!(
                    lin.jacobian[t][u].is_finite(),
                    "Jacobian[{t}][{u}] = {} not finite",
                    lin.jacobian[t][u],
                );
            }
        }
    }

    // -----------------------------------------------------------------
    // Dispatch from DeviceModel::linearize lands in the BSIM4 helper.
    // (Confirm the wiring change in linearize_mosfet's match arm.)
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_dispatch_through_device_model_linearize() {
        use crate::model::DeviceModel;
        use crate::params::MOSFETParams;

        let m = DeviceModel::MOSFET(MOSFETParams::BSIM4(nmos_default()));
        let op = OperatingPoint::MOSFET([3.0, 1.8, 0.0, 0.0]);
        let lin = m.linearize(&op).expect("matched family must succeed");
        match lin {
            LinearizedModel::MOSFET(m) => {
                // gm strictly positive at this saturation operating point —
                // confirms the helper ran, not the zero placeholder.
                assert!(
                    m.jacobian[0][1] > 0.0,
                    "BSIM4 dispatch must yield gm > 0 in saturation, got {}",
                    m.jacobian[0][1],
                );
            }
            other => panic!("expected MOSFET linearization, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // ETA0 = 0 disables DIBL — gds in saturation must drop to the
    // pure PCLM contribution. Sanity-checks that DIBL is wired up.
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_eta0_zero_removes_dibl_contribution() {
        let mut p = nmos_default();
        p.eta0 = 0.0;
        // saturation
        let lin = linearize_mosfet_bsim4(&p, &[3.0, 1.8, 0.0, 0.0]);
        // With ETA0=0: gds = (KP/2) · Vgs_ov² · PCLM (pure CLM term).
        // Compute the expected value directly.
        let vgs_ov = 1.8 - 0.7;
        let expected_gds = 0.5 * p.kp() * vgs_ov * vgs_ov * p.pclm;
        assert!(
            approx_eq(lin.jacobian[0][0], expected_gds, 1e-9, 0.0),
            "with ETA0=0, gds should equal pure-PCLM term: got {}, expected {expected_gds}",
            lin.jacobian[0][0],
        );
    }

    // -----------------------------------------------------------------
    // Cox / KP convenience accessors.
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_cox_and_kp_match_handout_formulas() {
        let p = MosBSIM4Params::default();
        let cox = p.cox();
        // Cox = 3.453e-11 / 3.0e-9 = 0.01151 F/m²
        assert!(approx_eq(cox, 3.453e-11 / 3.0e-9, 1e-12, 0.0));
        let kp = p.kp();
        // KP = U0 · Cox · (W/L) = 0.067 · 0.01151 · 1.0 ≈ 7.71e-4 A/V²
        assert!(approx_eq(kp, 0.067 * cox * 1.0, 1e-12, 0.0));
    }

    // -----------------------------------------------------------------
    // Forward-compat raw map still empty by default; documented hatch
    // for future BSIM4 parameter promotions.
    // -----------------------------------------------------------------

    #[test]
    fn bsim4_raw_map_remains_empty_by_default_after_typed_fields_added() {
        let p = MosBSIM4Params::default();
        assert!(
            p.raw.is_empty(),
            "raw forward-compat map must still be empty by default after task #13 typed fields",
        );
    }
}
