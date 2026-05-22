//! Intrinsic device noise source modeling (tasks.md #36).
//!
//! This module owns the *contract* between the
//! [`device-modeling`](crate) context and the future noise-analysis
//! control loop (tasks.md #37): for every device that has intrinsic
//! noise sources, [`DeviceModel::noise_stamp`] returns a
//! [`DeviceNoiseStamp`] — a fixed-shape, terminal-local list of
//! independent noise current sources, each described by its constant
//! (white) and `1/f` power-spectral-density components.
//!
//! # Noise physics in scope at task #36
//!
//! Three physical mechanisms produce intrinsic noise inside an
//! electronic device, each with a canonical SPICE formula. The
//! tasks.md item names them explicitly:
//!
//! - **Thermal noise** of a resistive element (`4kTG`, where
//!   `G = 1/R` is the conductance). Produces a *white* (constant in
//!   `f`) current PSD `S_I(f) = 4·k_B·T·G` `[A²/Hz]` between the
//!   element's two terminals.
//! - **Shot noise** of a junction carrying DC current `I`
//!   (`2·q·|I|`). Also white: `S_I(f) = 2·q·|I|` `[A²/Hz]`.
//! - **Flicker (1/f) noise** of a current-carrying junction or MOSFET
//!   channel (`KF · I^AF / f`). Has the canonical `1/f`
//!   shape: `S_I(f) = KF · |I|^AF / f` `[A²/Hz]`.
//!
//! These are the three terms the
//! [`noise-spectral-density`](../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/noise-spectral-density/spec.md)
//! capability requires the simulator to model per device. The noise
//! control loop (tasks.md #37) sums these sources, propagates each
//! through the small-signal transfer function from the source's
//! terminal pair to the user-specified output node, and squares the
//! results to obtain the output-referred PSD.
//!
//! # Independence and squared summation
//!
//! All three mechanisms — thermal, shot, flicker — are assumed
//! *uncorrelated* between devices and between sources within a
//! device. The acceptance criterion in
//! `specs/noise-spectral-density/spec.md` is explicit: "each
//! intrinsic device noise source … contributes independently to the
//! total output noise." Independence justifies summation of *power*
//! spectral densities (squared magnitudes) rather than amplitudes,
//! which is what task #37's transfer-function machinery does.
//!
//! # White + 1/f decomposition (the per-source data layout)
//!
//! Every [`NoiseSource`] carries a `white_psd` (A²/Hz, constant in
//! `f`) and a `flicker_coeff` (A², so dividing by `f` gives A²/Hz).
//! The total PSD evaluated at frequency `f` is
//!
//! ```text
//! S_I(f) = white_psd + flicker_coeff / f
//! ```
//!
//! This shape is enough for tasks.md #37 to compute the per-frequency
//! contribution without re-entering [`DeviceModel::noise_stamp`]: the
//! stamp is computed *once* at the DC [`OperatingPoint`], and the
//! frequency sweep then evaluates [`NoiseSource::psd_at`] (or its
//! inlined equivalent) per point.
//!
//! Mechanisms that are *only* white (thermal, shot) ship with
//! `flicker_coeff = 0`. Mechanisms that are *only* `1/f` (the SPICE
//! flicker term proper) ship with `white_psd = 0`. The MNA noise
//! assembler does not need to know which mechanism produced a source
//! — but each [`NoiseSource`] carries a [`NoiseSource::mechanism`]
//! tag anyway so the optional per-device noise breakdown
//! (tasks.md #38) can split the output PSD by noise type.
//!
//! # Terminal-local coordinates (same as [`crate::stamp`])
//!
//! Every [`NoiseSource`] is expressed in terminal-local coordinates
//! `(a, b)`, mirroring the convention established by
//! [`crate::stamp::LinearizedModel`] for the device's Jacobian: the
//! source injects a stochastic current from terminal `a` into
//! terminal `b`. The MNA noise assembler (tasks.md #37) is
//! responsible for mapping `(a, b)` to graph `NodeId`s via the
//! `FlattenedStructure`'s incidence.
//!
//! Terminal orderings match [`crate::stamp::OperatingPoint`]:
//!
//! - Diode: `0 = anode`, `1 = cathode`
//! - BJT: `0 = collector`, `1 = base`, `2 = emitter`
//! - MOSFET: `0 = drain`, `1 = gate`, `2 = source`, `3 = bulk`
//!
//! # ADR-0005 closed-enum dispatch
//!
//! [`DeviceNoiseStamp`] is a closed enum that mirrors
//! [`crate::DeviceModel`] and [`crate::stamp::LinearizedModel`].
//! Adding a new device family breaks every `match` site, which is
//! exactly the property [ADR-0005](../../../wiki/decisions/0005-closed-enum-device-model-dispatch.md)
//! buys.
//!
//! # Out of scope at #36
//!
//! - **Noise transfer matrices** at each frequency — owned by
//!   tasks.md #37.
//! - **Per-device breakdown attached to `Result`** — owned by
//!   tasks.md #38.
//! - **Integrated noise over bandwidth** — owned by tasks.md #39.
//! - **Correlated noise** (e.g. shot/flicker correlation in the BSIM
//!   channel) — a superseding ADR would be required.
//! - **Coloured noise other than 1/f** — flicker is the only
//!   non-white mechanism we support; `1/f^α` with `α ≠ 1` is out of
//!   scope for v1.

use crate::model::DeviceModel;
use crate::params::{BJTParams, BJTPolarity, DiodeParams, MOSFETParams};
use crate::stamp::{
    OperatingPoint, OperatingPointFamilyMismatch, BJT_TERMINALS, DIODE_TERMINALS, MOSFET_TERMINALS,
};

// ---------------------------------------------------------------------
// Physical constants
// ---------------------------------------------------------------------

/// Boltzmann's constant `k_B` in joules per kelvin.
///
/// 2019 SI redefinition value (exact). Used in the thermal noise
/// formula `S_I(f) = 4·k_B·T·G`.
pub const BOLTZMANN_J_PER_K: f64 = 1.380_649e-23;

/// Elementary charge `q` in coulombs.
///
/// 2019 SI redefinition value (exact). Used in the shot noise
/// formula `S_I(f) = 2·q·|I|`.
pub const ELEMENTARY_CHARGE_C: f64 = 1.602_176_634e-19;

/// Reference temperature for the SPICE `TNOM` convention, in
/// kelvin. Equivalent to `27 °C`; this is the temperature SPICE
/// uses if none is supplied by the netlist.
pub const ROOM_TEMPERATURE_K: f64 = 300.15;

// ---------------------------------------------------------------------
// NoiseMechanism — diagnostic tag, surfaced by #38 breakdown
// ---------------------------------------------------------------------

/// Which physical mechanism produced a given [`NoiseSource`].
///
/// This tag is purely informational from the MNA noise assembler's
/// perspective — the assembler only consumes the PSD numbers. It
/// exists so the optional per-device noise breakdown (tasks.md #38)
/// can group output noise contributions by mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoiseMechanism {
    /// Johnson-Nyquist thermal noise of a resistive element.
    /// PSD shape: white (constant in `f`), magnitude `4·k_B·T·G`.
    Thermal,
    /// Shot noise of a current-carrying junction.
    /// PSD shape: white, magnitude `2·q·|I|`.
    Shot,
    /// `1/f` (flicker, pink) noise.
    /// PSD shape: `KF · |I|^AF / f`.
    Flicker,
}

// ---------------------------------------------------------------------
// NoiseSource — one independent noise current source
// ---------------------------------------------------------------------

/// One independent intrinsic noise current source on a device.
///
/// A `NoiseSource` represents a *stochastic* current `i_n(t)`
/// flowing between two terminals of a device, with the power
/// spectral density of `i_n` decomposed into a white component and a
/// `1/f` component:
///
/// ```text
/// S_I(f) = white_psd + flicker_coeff / f      [A² / Hz]
/// ```
///
/// All amplitudes are non-negative. The MNA noise assembler
/// (tasks.md #37) treats sources from different devices, and
/// different sources on the same device, as mutually uncorrelated.
///
/// # Terminal indexing
///
/// `(a, b)` are *terminal-local* indices into the device's terminal
/// ordering (see [`crate::stamp::OperatingPoint`]). They are not
/// graph `NodeId`s. The noise current flows from terminal `a` into
/// terminal `b`; for an uncorrelated noise source the sign is
/// physically irrelevant (PSD is the same for `+i_n` and `-i_n`),
/// but pinning a direction lets us stamp the source consistently
/// into the MNA right-hand-side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseSource {
    /// Source terminal of the noise current (terminal-local index).
    pub a: usize,
    /// Sink terminal of the noise current (terminal-local index).
    pub b: usize,
    /// Physical mechanism that produced this source (diagnostic /
    /// breakdown tag; the assembler ignores it).
    pub mechanism: NoiseMechanism,
    /// White component of the PSD, in A²/Hz. Constant in `f`.
    pub white_psd: f64,
    /// `1/f` numerator: PSD contribution at frequency `f` equals
    /// `flicker_coeff / f`. Units: A² (so the quotient is A²/Hz).
    pub flicker_coeff: f64,
}

impl NoiseSource {
    /// Evaluate this source's noise PSD at frequency `f` (hertz).
    ///
    /// Returns `white_psd + flicker_coeff / f`. The caller must
    /// ensure `f > 0` — `f = 0` would diverge any source with a
    /// non-zero flicker term, which is the physical signature of
    /// `1/f` noise and not a software bug. Noise analyses always
    /// sweep `f > 0` per the spec.
    ///
    /// # Returns
    ///
    /// The PSD in A²/Hz at the requested frequency. Always
    /// non-negative when constructed via the helpers in this module
    /// (both `white_psd` and `flicker_coeff` are non-negative).
    #[must_use]
    pub fn psd_at(&self, f: f64) -> f64 {
        debug_assert!(f > 0.0, "noise PSD evaluated at non-positive frequency");
        self.white_psd + self.flicker_coeff / f
    }

    /// `true` if this source has no white *and* no `1/f` component.
    ///
    /// Useful for filtering empty contributions before stamping;
    /// devices whose `OperatingPoint` is at zero current produce
    /// only zero shot/flicker sources, and the MNA noise assembler
    /// can skip them.
    #[must_use]
    pub fn is_silent(&self) -> bool {
        // Bit-exact zero only: a source constructed by our helpers
        // is either initialized to a closed-form expression or to
        // exactly 0.0. Floating-point drift is impossible at the
        // initialization site.
        self.white_psd == 0.0 && self.flicker_coeff == 0.0
    }
}

// ---------------------------------------------------------------------
// Resistor thermal noise (Johnson-Nyquist) — not a DeviceModel
// ---------------------------------------------------------------------

/// Build the thermal-noise current source for a linear resistor.
///
/// The Johnson-Nyquist formula gives the white-noise current PSD of
/// a resistor `R` at temperature `T` as
///
/// ```text
/// S_I(f) = 4·k_B·T / R     [A² / Hz]
/// ```
///
/// This source is emitted between the resistor's two terminals
/// (terminal-local indices `0` and `1`).
///
/// # Arguments
///
/// - `resistance_ohms` — `R > 0`, in ohms.
/// - `temperature_k` — device temperature, in kelvin. Pass
///   [`ROOM_TEMPERATURE_K`] (`300.15 K`) for the SPICE default.
///
/// # Returns
///
/// A single [`NoiseSource`] tagged [`NoiseMechanism::Thermal`].
///
/// # Panics
///
/// Panics in debug builds if `resistance_ohms <= 0.0` or
/// `temperature_k <= 0.0`. A zero-ohm resistor has no thermal noise
/// model; a non-positive temperature is unphysical and a programming
/// error in the parameter extractor.
#[must_use]
pub fn resistor_thermal_noise(resistance_ohms: f64, temperature_k: f64) -> NoiseSource {
    debug_assert!(
        resistance_ohms > 0.0,
        "resistor_thermal_noise: R must be > 0"
    );
    debug_assert!(temperature_k > 0.0, "resistor_thermal_noise: T must be > 0");
    let conductance = 1.0 / resistance_ohms;
    let white = 4.0 * BOLTZMANN_J_PER_K * temperature_k * conductance;
    NoiseSource {
        a: 0,
        b: 1,
        mechanism: NoiseMechanism::Thermal,
        white_psd: white,
        flicker_coeff: 0.0,
    }
}

// ---------------------------------------------------------------------
// Per-family noise stamps
// ---------------------------------------------------------------------

/// Diode noise stamp (tasks.md #36, Diode case).
///
/// Models two intrinsic mechanisms per the SPICE `.MODEL D`
/// convention:
///
/// 1. Series-resistance thermal noise of `RS` between anode and
///    cathode (omitted when `RS == 0`).
/// 2. Junction shot + flicker noise of the diode current `I_D`,
///    also between anode and cathode. The shot PSD is `2·q·|I_D|`;
///    the flicker PSD numerator is `KF·|I_D|^AF` (giving
///    `KF·|I_D|^AF / f` after the assembler divides by `f`).
///
/// At task #36 the diode terminal current `I_D` consumed by these
/// formulas is supplied by the caller via [`DiodeOperatingState`].
/// The current itself derives from the Shockley-equation companion
/// model that tasks.md #9 fills in. Until then the caller must
/// supply `i_d = 0` (which produces no shot or flicker noise,
/// only thermal); the contract is the same once #9 lands.
#[derive(Debug, Clone, PartialEq)]
pub struct DiodeNoiseStamp {
    /// Independent noise sources for this diode. Length 0–2:
    /// the `RS` thermal source is omitted when `RS == 0`, and the
    /// junction shot+flicker source is omitted when `I_D == 0` AND
    /// `KF == 0`. (We keep two arrays separate to make the
    /// per-mechanism breakdown in #38 unambiguous.)
    pub sources: Vec<NoiseSource>,
}

/// Operating-point inputs the diode noise stamp needs that are not
/// already in [`DiodeParams`].
///
/// Currently a single field — the DC diode current at the operating
/// point. tasks.md #9 introduces the Shockley companion model that
/// produces this number; until then a caller can pass `0.0` and the
/// stamp will emit only `RS` thermal noise.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DiodeOperatingState {
    /// DC diode current `I_D` at the operating point, in amperes.
    /// Sign convention: positive when anode-to-cathode (forward
    /// bias).
    pub i_d: f64,
}

/// BJT noise stamp (tasks.md #36, BJT case).
///
/// Models three SPICE-convention noise mechanisms:
///
/// 1. Base shot noise `2·q·|I_B|` between base and emitter.
/// 2. Collector shot noise `2·q·|I_C|` between collector and
///    emitter.
/// 3. Base-current flicker noise `KF·|I_B|^AF / f` between base and
///    emitter.
///
/// SPICE attaches flicker noise to the base current, not the
/// collector current, by convention. Bulk-resistance thermal noise
/// (`RB`, `RC`, `RE`) is *not* modeled at task #36 because the
/// current `BJTParams` does not carry those parameters; if they are
/// added in a future change, the stamp here is extended in lockstep.
#[derive(Debug, Clone, PartialEq)]
pub struct BJTNoiseStamp {
    /// Independent noise sources for this BJT. Length 0–3
    /// depending on which of `I_B`, `I_C`, `KF·I_B^AF` are non-zero.
    pub sources: Vec<NoiseSource>,
}

/// Operating-point inputs the BJT noise stamp needs that are not
/// already in [`BJTParams`].
///
/// tasks.md #10 (Ebers-Moll / Gummel-Poon) supplies `I_B` and `I_C`
/// at the operating point. Until then callers pass `0.0` for both.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BJTOperatingState {
    /// DC base current `I_B` at the operating point, in amperes.
    /// Sign convention: positive flowing into the base terminal.
    pub i_b: f64,
    /// DC collector current `I_C` at the operating point, in
    /// amperes. Sign convention: positive flowing into the
    /// collector terminal.
    pub i_c: f64,
}

/// MOSFET noise stamp (tasks.md #36, MOSFET case).
///
/// Models the two intrinsic MOSFET noise mechanisms common to all
/// levels:
///
/// 1. Channel thermal noise. In saturation the long-channel formula
///    is `S_I(f) = (8/3)·k_B·T·g_m`. We use the more general form
///    `4·k_B·T·γ·g_m` with `γ = 2/3` (long-channel saturation); for
///    short-channel devices the level-specific stamp may override
///    `γ` in a future change.
/// 2. Drain-current flicker noise `KF·|I_D|^AF / f`.
///
/// At task #36 the caller supplies the per-iterate transconductance
/// `g_m` and DC drain current `I_D` via [`MosfetOperatingState`].
/// tasks.md #11–#13 introduce per-level stamps that produce these
/// numbers; until then callers pass `0.0` and the stamp emits only
/// the zero-current degenerate case.
#[derive(Debug, Clone, PartialEq)]
pub struct MosfetNoiseStamp {
    /// Independent noise sources for this MOSFET. Length 0–2.
    /// The channel-thermal source is omitted when `g_m == 0`; the
    /// drain-flicker source is omitted when `I_D == 0` AND
    /// `KF == 0`.
    pub sources: Vec<NoiseSource>,
}

/// Operating-point inputs the MOSFET noise stamp needs that are not
/// already in [`MOSFETParams`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MosfetOperatingState {
    /// DC drain current `I_D` at the operating point, in amperes.
    /// Sign convention: positive when drain-to-source.
    pub i_d: f64,
    /// Small-signal transconductance `g_m = ∂I_D/∂V_GS`, in
    /// siemens. Provided by the per-level [`crate::stamp`]
    /// linearization (tasks.md #11–#13).
    pub g_m: f64,
    /// Device temperature in kelvin (used by the channel-thermal
    /// formula). Pass [`ROOM_TEMPERATURE_K`] for the SPICE default.
    pub temperature_k: f64,
}

// ---------------------------------------------------------------------
// DeviceNoiseStamp — family-tagged top-level return value
// ---------------------------------------------------------------------

/// Family-tagged noise stamp returned by
/// [`DeviceModel::noise_stamp`].
///
/// One variant per [`DeviceModel`] family. The MNA noise assembler
/// (tasks.md #37) matches on the variant to learn the device's
/// terminal count, then walks the inner `sources` `Vec` to stamp
/// each independent noise source into the noise-injection matrix.
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceNoiseStamp {
    /// 2-terminal diode noise stamp.
    Diode(DiodeNoiseStamp),
    /// 3-terminal BJT noise stamp.
    BJT(BJTNoiseStamp),
    /// 4-terminal MOSFET noise stamp.
    MOSFET(MosfetNoiseStamp),
}

impl DeviceNoiseStamp {
    /// Number of terminals on the device this stamp belongs to.
    ///
    /// Mirrors [`crate::stamp::LinearizedModel::terminal_count`]:
    /// the MNA noise assembler uses this when mapping
    /// terminal-local source indices to graph `NodeId`s.
    #[must_use]
    pub fn terminal_count(&self) -> usize {
        match self {
            Self::Diode(_) => DIODE_TERMINALS,
            Self::BJT(_) => BJT_TERMINALS,
            Self::MOSFET(_) => MOSFET_TERMINALS,
        }
    }

    /// Borrow the underlying [`NoiseSource`] slice regardless of
    /// family. Useful for the assembler's main loop and for tests.
    #[must_use]
    pub fn sources(&self) -> &[NoiseSource] {
        match self {
            Self::Diode(s) => &s.sources,
            Self::BJT(s) => &s.sources,
            Self::MOSFET(s) => &s.sources,
        }
    }
}

// ---------------------------------------------------------------------
// Operating-state input — per-family enum
// ---------------------------------------------------------------------

/// Per-family auxiliary operating-state input to
/// [`DeviceModel::noise_stamp`].
///
/// The thermal/shot/flicker noise formulas need quantities that go
/// beyond the bare terminal voltages carried by
/// [`crate::stamp::OperatingPoint`]: the DC junction current(s) and,
/// for MOSFETs, the transconductance `g_m`. Those are products of
/// the per-family linearization (tasks.md #9–#13). Rather than
/// re-deriving them inside the noise stamp (which would double the
/// stamp cost in the noise sweep), we accept them as a structured
/// input here. The noise control loop (tasks.md #37) computes the
/// per-family state once after the DC solve and reuses it across
/// every frequency point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceOperatingState {
    /// Diode operating state (DC `I_D`).
    Diode(DiodeOperatingState),
    /// BJT operating state (DC `I_B`, `I_C`).
    BJT(BJTOperatingState),
    /// MOSFET operating state (DC `I_D`, `g_m`, temperature).
    MOSFET(MosfetOperatingState),
}

impl DeviceOperatingState {
    /// Family name as a static string, for diagnostic messages.
    #[must_use]
    pub const fn family(&self) -> &'static str {
        match self {
            Self::Diode(_) => "Diode",
            Self::BJT(_) => "BJT",
            Self::MOSFET(_) => "MOSFET",
        }
    }
}

// ---------------------------------------------------------------------
// Per-family helpers
// ---------------------------------------------------------------------

/// Build the noise stamp for a single diode.
///
/// Emits up to two [`NoiseSource`]s between anode (terminal `0`) and
/// cathode (terminal `1`):
///
/// 1. **`RS` thermal noise** when `params.rs > 0`:
///    `S_I = 4·k_B·T / R_S` (white).
/// 2. **Junction shot + flicker** when either `I_D ≠ 0` (shot) or
///    `KF > 0` (flicker):
///    `S_I(f) = 2·q·|I_D| + KF·|I_D|^AF / f`.
///
/// Both sources are independent and use the same `(anode, cathode)`
/// terminal pair; the MNA noise assembler sums their PSDs.
///
/// The temperature `temperature_k` is used only by the `RS` thermal
/// term; the junction-noise formulas have no explicit temperature
/// dependence in the SPICE convention (`Vt` already carries it via
/// the params).
///
/// # Arguments
///
/// - `params` — diode `.MODEL` parameters.
/// - `state` — DC operating state (currently just `I_D`).
/// - `temperature_k` — device temperature, kelvin.
///
/// # Panics (debug builds only)
///
/// Panics if `temperature_k <= 0.0` and `params.rs > 0.0`.
#[must_use]
pub fn noise_stamp_diode(
    params: &DiodeParams,
    state: &DiodeOperatingState,
    temperature_k: f64,
) -> DiodeNoiseStamp {
    let mut sources = Vec::with_capacity(2);

    // (1) RS thermal noise.
    if params.rs > 0.0 {
        sources.push(resistor_thermal_noise(params.rs, temperature_k));
    }

    // (2) Junction shot + flicker. Combine them into a single
    // NoiseSource on the (anode, cathode) pair because they share
    // the same physical injection site; per-mechanism breakdown
    // (#38) splits them by mechanism tag — so we actually keep them
    // as TWO sources tagged Shot and Flicker, so the breakdown is
    // unambiguous. That matches the spec's per-noise-type breakdown
    // requirement.
    let i_abs = state.i_d.abs();
    let shot_psd = 2.0 * ELEMENTARY_CHARGE_C * i_abs;
    if shot_psd > 0.0 {
        sources.push(NoiseSource {
            a: 0,
            b: 1,
            mechanism: NoiseMechanism::Shot,
            white_psd: shot_psd,
            flicker_coeff: 0.0,
        });
    }

    let flicker_coeff = if params.kf > 0.0 && i_abs > 0.0 {
        params.kf * i_abs.powf(params.af)
    } else {
        0.0
    };
    if flicker_coeff > 0.0 {
        sources.push(NoiseSource {
            a: 0,
            b: 1,
            mechanism: NoiseMechanism::Flicker,
            white_psd: 0.0,
            flicker_coeff,
        });
    }

    DiodeNoiseStamp { sources }
}

/// Build the noise stamp for a single BJT.
///
/// Emits up to three [`NoiseSource`]s (terminal ordering
/// `0 = C, 1 = B, 2 = E`):
///
/// 1. **Base shot** between `B` and `E`: `2·q·|I_B|` (white).
/// 2. **Collector shot** between `C` and `E`: `2·q·|I_C|` (white).
/// 3. **Base flicker** between `B` and `E`:
///    `KF·|I_B|^AF / f`. SPICE attaches `1/f` to the base current.
///
/// Polarity (NPN vs PNP) only affects the sign of `I_B`/`I_C`; PSD
/// uses `|·|` so the stamp is polarity-agnostic.
#[must_use]
pub fn noise_stamp_bjt(params: &BJTParams, state: &BJTOperatingState) -> BJTNoiseStamp {
    let mut sources = Vec::with_capacity(3);

    let abs_i_b = state.i_b.abs();
    let abs_i_c = state.i_c.abs();

    // (1) Base shot.
    let base_shot = 2.0 * ELEMENTARY_CHARGE_C * abs_i_b;
    if base_shot > 0.0 {
        sources.push(NoiseSource {
            a: 1, // base
            b: 2, // emitter
            mechanism: NoiseMechanism::Shot,
            white_psd: base_shot,
            flicker_coeff: 0.0,
        });
    }

    // (2) Collector shot.
    let coll_shot = 2.0 * ELEMENTARY_CHARGE_C * abs_i_c;
    if coll_shot > 0.0 {
        sources.push(NoiseSource {
            a: 0, // collector
            b: 2, // emitter
            mechanism: NoiseMechanism::Shot,
            white_psd: coll_shot,
            flicker_coeff: 0.0,
        });
    }

    // (3) Base flicker.
    let flicker_coeff = if params.kf > 0.0 && abs_i_b > 0.0 {
        params.kf * abs_i_b.powf(params.af)
    } else {
        0.0
    };
    if flicker_coeff > 0.0 {
        sources.push(NoiseSource {
            a: 1, // base
            b: 2, // emitter
            mechanism: NoiseMechanism::Flicker,
            white_psd: 0.0,
            flicker_coeff,
        });
    }

    // Polarity is part of the contract documentation, not the stamp
    // math — but discard-binding here keeps clippy quiet about the
    // unused field and makes the intent explicit at the call site.
    debug_assert!(matches!(
        params.polarity,
        BJTPolarity::Npn | BJTPolarity::Pnp
    ));

    BJTNoiseStamp { sources }
}

/// Build the noise stamp for a single MOSFET.
///
/// Emits up to two [`NoiseSource`]s (terminal ordering
/// `0 = D, 1 = G, 2 = S, 3 = B`):
///
/// 1. **Channel thermal** between drain and source:
///    `S_I(f) = 4·k_B·T·γ·g_m`, with `γ = 2/3` (long-channel
///    saturation) — equivalent to the SPICE `(8/3)·k_B·T·g_m`
///    formula.
/// 2. **Drain flicker** between drain and source:
///    `KF·|I_D|^AF / f`.
///
/// Level-specific overrides (BSIM `γ`, per-area scaling) land with
/// their respective stamps (tasks.md #11–#13); at #36 we only model
/// the level-agnostic intersection. The Level-1 stamp `KF`/`AF`
/// values are read directly from [`crate::params::MosLevel1Params`]; for BSIM
/// levels the parameters are pulled from their `raw` parameter
/// map (`"kf"`, `"af"`) and default to 0 / 1 when absent.
#[must_use]
pub fn noise_stamp_mosfet(params: &MOSFETParams, state: &MosfetOperatingState) -> MosfetNoiseStamp {
    // Long-channel saturation γ. Future short-channel models may
    // raise this; we hard-code the textbook value here.
    const GAMMA_CHANNEL: f64 = 2.0 / 3.0;

    let mut sources = Vec::with_capacity(2);
    let g_m_abs = state.g_m.abs();
    let i_d_abs = state.i_d.abs();

    // (1) Channel thermal noise. Requires temperature > 0 to make
    // physical sense; an unset temperature_k (== 0) silently
    // produces zero noise, but we debug-assert to catch upstream
    // bugs.
    if g_m_abs > 0.0 {
        debug_assert!(
            state.temperature_k > 0.0,
            "MOSFET channel thermal noise: T must be > 0"
        );
        let psd = 4.0 * BOLTZMANN_J_PER_K * state.temperature_k * GAMMA_CHANNEL * g_m_abs;
        if psd > 0.0 {
            sources.push(NoiseSource {
                a: 0, // drain
                b: 2, // source
                mechanism: NoiseMechanism::Thermal,
                white_psd: psd,
                flicker_coeff: 0.0,
            });
        }
    }

    // (2) Drain flicker noise.
    let (kf, af) = extract_mosfet_kf_af(params);
    let flicker_coeff = if kf > 0.0 && i_d_abs > 0.0 {
        kf * i_d_abs.powf(af)
    } else {
        0.0
    };
    if flicker_coeff > 0.0 {
        sources.push(NoiseSource {
            a: 0, // drain
            b: 2, // source
            mechanism: NoiseMechanism::Flicker,
            white_psd: 0.0,
            flicker_coeff,
        });
    }

    MosfetNoiseStamp { sources }
}

/// Pull `(KF, AF)` out of a MOSFET level-specific parameter
/// payload.
///
/// - [`MOSFETParams::Level1`]: read the typed `kf` / `af` fields.
/// - [`MOSFETParams::BSIM3v3`] and `BSIM4`: read string keys `"kf"`
///   and `"af"` from the sparse `raw` map. Both default to the
///   SPICE convention `(KF = 0, AF = 1)` when absent.
fn extract_mosfet_kf_af(params: &MOSFETParams) -> (f64, f64) {
    match params {
        MOSFETParams::Level1(p) => (p.kf, p.af),
        MOSFETParams::BSIM3v3(p) => {
            let kf = p.raw.get("kf").copied().unwrap_or(0.0);
            let af = p.raw.get("af").copied().unwrap_or(1.0);
            (kf, af)
        }
        MOSFETParams::BSIM4(p) => {
            let kf = p.raw.get("kf").copied().unwrap_or(0.0);
            let af = p.raw.get("af").copied().unwrap_or(1.0);
            (kf, af)
        }
    }
}

// ---------------------------------------------------------------------
// Dispatch on DeviceModel
// ---------------------------------------------------------------------

impl DeviceModel {
    /// Compute the [`DeviceNoiseStamp`] for this device at the given
    /// operating point.
    ///
    /// Dispatched through a `match` on `self` per ADR-0005, in the
    /// same shape as [`DeviceModel::linearize`](crate::DeviceModel).
    /// Each arm delegates to a per-family helper
    /// ([`noise_stamp_diode`], [`noise_stamp_bjt`],
    /// [`noise_stamp_mosfet`]).
    ///
    /// # Arguments
    ///
    /// - `op` — terminal voltages at the operating point. Only used
    ///   by the helpers for family-consistency checking at this
    ///   task; the noise math itself reads currents / `g_m` from
    ///   `state`, not voltages.
    /// - `state` — per-family DC operating state (`I_D`, `I_B`,
    ///   `I_C`, `g_m`, temperature). Produced by the noise control
    ///   loop (tasks.md #37) once per analysis, after the DC solve.
    ///
    /// # Errors
    ///
    /// Returns [`OperatingPointFamilyMismatch`] if any of `self`,
    /// `op`, and `state` carry inconsistent device families. This is
    /// a programming error in the noise control loop, not a runtime
    /// convergence concern.
    pub fn noise_stamp(
        &self,
        op: &OperatingPoint,
        state: &DeviceOperatingState,
    ) -> Result<DeviceNoiseStamp, OperatingPointFamilyMismatch> {
        match (self, op, state) {
            (Self::Diode(p), OperatingPoint::Diode(_), DeviceOperatingState::Diode(s)) => Ok(
                DeviceNoiseStamp::Diode(noise_stamp_diode(p, s, ROOM_TEMPERATURE_K)),
            ),
            (Self::BJT(p), OperatingPoint::BJT(_), DeviceOperatingState::BJT(s)) => {
                Ok(DeviceNoiseStamp::BJT(noise_stamp_bjt(p, s)))
            }
            (Self::MOSFET(p), OperatingPoint::MOSFET(_), DeviceOperatingState::MOSFET(s)) => {
                Ok(DeviceNoiseStamp::MOSFET(noise_stamp_mosfet(p, s)))
            }
            // Mismatches. Spell each model arm so exhaustiveness
            // bites when a new DeviceModel variant lands.
            (Self::Diode(_), op, state) => Err(family_mismatch("Diode", op, state)),
            (Self::BJT(_), op, state) => Err(family_mismatch("BJT", op, state)),
            (Self::MOSFET(_), op, state) => Err(family_mismatch("MOSFET", op, state)),
        }
    }
}

fn family_mismatch(
    expected: &'static str,
    op: &OperatingPoint,
    state: &DeviceOperatingState,
) -> OperatingPointFamilyMismatch {
    // If the OperatingPoint disagrees with `expected`, report that
    // first; otherwise blame the state. Both are programming errors
    // in the noise control loop.
    let op_family = match op {
        OperatingPoint::Diode(_) => "Diode",
        OperatingPoint::BJT(_) => "BJT",
        OperatingPoint::MOSFET(_) => "MOSFET",
    };
    if op_family == expected {
        OperatingPointFamilyMismatch {
            expected,
            actual: state.family(),
        }
    } else {
        OperatingPointFamilyMismatch {
            expected,
            actual: op_family,
        }
    }
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::{BJTPolarity, MosBSIM3v3Params, MosLevel1Params, MosPolarity};
    use circuit_solver_types::ModelName;

    // -----------------------------------------------------------------
    // Physical constants are exact 2019 SI redefinition values.
    // -----------------------------------------------------------------

    #[test]
    fn boltzmann_and_charge_match_si_2019_exact_values() {
        // Bit-exact comparison: the constants in the module are
        // literal-typed, so any drift from the 2019 SI values
        // triggers this test.
        assert_eq!(BOLTZMANN_J_PER_K.to_bits(), 1.380_649e-23_f64.to_bits());
        assert_eq!(
            ELEMENTARY_CHARGE_C.to_bits(),
            1.602_176_634e-19_f64.to_bits()
        );
    }

    #[test]
    fn room_temperature_matches_spice_tnom() {
        // SPICE TNOM = 27 °C = 300.15 K.
        assert_eq!(ROOM_TEMPERATURE_K.to_bits(), 300.15_f64.to_bits());
    }

    // -----------------------------------------------------------------
    // NoiseSource::psd_at — white + 1/f decomposition.
    // -----------------------------------------------------------------

    #[test]
    fn psd_at_returns_white_plus_one_over_f() {
        let s = NoiseSource {
            a: 0,
            b: 1,
            mechanism: NoiseMechanism::Thermal,
            white_psd: 2.0,
            flicker_coeff: 1.0,
        };
        // f = 1 Hz → 2 + 1/1 = 3.
        assert!((s.psd_at(1.0) - 3.0).abs() < 1e-15);
        // f = 100 Hz → 2 + 1/100 = 2.01.
        assert!((s.psd_at(100.0) - 2.01).abs() < 1e-15);
    }

    #[test]
    fn psd_at_pure_white_is_constant_in_frequency() {
        let s = NoiseSource {
            a: 0,
            b: 1,
            mechanism: NoiseMechanism::Thermal,
            white_psd: 4.0e-21,
            flicker_coeff: 0.0,
        };
        let v1 = s.psd_at(1.0);
        let v_high = s.psd_at(1.0e9);
        assert!((v1 - v_high).abs() < 1e-30);
    }

    #[test]
    fn psd_at_pure_flicker_decays_as_one_over_f() {
        let s = NoiseSource {
            a: 0,
            b: 1,
            mechanism: NoiseMechanism::Flicker,
            white_psd: 0.0,
            flicker_coeff: 1.0e-12,
        };
        assert!((s.psd_at(1.0) - 1.0e-12).abs() < 1e-25);
        assert!((s.psd_at(10.0) - 1.0e-13).abs() < 1e-26);
        assert!((s.psd_at(1.0e6) - 1.0e-18).abs() < 1e-31);
    }

    #[test]
    fn is_silent_detects_all_zero_source() {
        let zero = NoiseSource {
            a: 0,
            b: 1,
            mechanism: NoiseMechanism::Thermal,
            white_psd: 0.0,
            flicker_coeff: 0.0,
        };
        assert!(zero.is_silent());

        let white = NoiseSource {
            white_psd: 1.0,
            ..zero
        };
        assert!(!white.is_silent());

        let pink = NoiseSource {
            flicker_coeff: 1.0,
            ..zero
        };
        assert!(!pink.is_silent());
    }

    // -----------------------------------------------------------------
    // resistor_thermal_noise — 4 k_B T / R.
    // -----------------------------------------------------------------

    #[test]
    fn resistor_thermal_noise_matches_4kt_over_r() {
        // 1 kΩ at 300.15 K. Expected: 4 · 1.380649e-23 · 300.15 / 1e3
        // = 1.65775e-23 A²/Hz approximately. We compute the closed
        // form here and require bit-equality with the helper to pin
        // the formula.
        let expected = 4.0 * BOLTZMANN_J_PER_K * ROOM_TEMPERATURE_K * (1.0 / 1.0e3);
        let s = resistor_thermal_noise(1.0e3, ROOM_TEMPERATURE_K);
        assert_eq!(s.a, 0);
        assert_eq!(s.b, 1);
        assert_eq!(s.mechanism, NoiseMechanism::Thermal);
        assert_eq!(s.white_psd.to_bits(), expected.to_bits());
        assert_eq!(s.flicker_coeff.to_bits(), 0.0_f64.to_bits());
        // Pin the order of magnitude so a typo (e.g. 4kT·R instead of
        // 4kT/R) is caught.
        assert!(s.white_psd > 1.0e-24 && s.white_psd < 1.0e-22);
    }

    #[test]
    fn resistor_thermal_noise_scales_inversely_with_resistance() {
        // R₁ = 1 kΩ, R₂ = 10 kΩ at the same T. The 10 kΩ noise
        // current PSD must be 1/10 of the 1 kΩ value.
        let s1 = resistor_thermal_noise(1.0e3, ROOM_TEMPERATURE_K);
        let s10 = resistor_thermal_noise(10.0e3, ROOM_TEMPERATURE_K);
        let ratio = s1.white_psd / s10.white_psd;
        assert!((ratio - 10.0).abs() < 1.0e-12);
    }

    #[test]
    fn resistor_thermal_noise_doubles_with_temperature() {
        // S_I ∝ T → halving R or doubling T must double S_I.
        let s_cold = resistor_thermal_noise(1.0e3, 150.0);
        let s_hot = resistor_thermal_noise(1.0e3, 300.0);
        let ratio = s_hot.white_psd / s_cold.white_psd;
        assert!((ratio - 2.0).abs() < 1.0e-12);
    }

    // -----------------------------------------------------------------
    // Diode noise stamp.
    // -----------------------------------------------------------------

    #[test]
    fn diode_with_zero_rs_zero_current_zero_kf_emits_no_sources() {
        // SPICE defaults (RS = 0, KF = 0) plus I_D = 0 → no noise.
        let stamp = noise_stamp_diode(
            &DiodeParams::default(),
            &DiodeOperatingState::default(),
            ROOM_TEMPERATURE_K,
        );
        assert!(stamp.sources.is_empty());
    }

    #[test]
    fn diode_with_nonzero_rs_emits_thermal_noise() {
        let params = DiodeParams {
            rs: 100.0,
            ..Default::default()
        };
        let stamp = noise_stamp_diode(&params, &DiodeOperatingState::default(), ROOM_TEMPERATURE_K);
        assert_eq!(stamp.sources.len(), 1);
        assert_eq!(stamp.sources[0].mechanism, NoiseMechanism::Thermal);
        let expected = 4.0 * BOLTZMANN_J_PER_K * ROOM_TEMPERATURE_K / 100.0;
        assert_eq!(stamp.sources[0].white_psd.to_bits(), expected.to_bits());
        assert_eq!(stamp.sources[0].a, 0);
        assert_eq!(stamp.sources[0].b, 1);
    }

    #[test]
    fn diode_with_forward_current_emits_shot_noise() {
        let params = DiodeParams::default(); // RS = 0, KF = 0
        let state = DiodeOperatingState { i_d: 1.0e-3 };
        let stamp = noise_stamp_diode(&params, &state, ROOM_TEMPERATURE_K);
        assert_eq!(stamp.sources.len(), 1);
        let s = &stamp.sources[0];
        assert_eq!(s.mechanism, NoiseMechanism::Shot);
        let expected = 2.0 * ELEMENTARY_CHARGE_C * 1.0e-3;
        assert_eq!(s.white_psd.to_bits(), expected.to_bits());
        assert_eq!(s.flicker_coeff.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn diode_shot_noise_uses_absolute_current() {
        // Reverse-bias I_D = -100 µA → shot PSD must use |I_D|.
        let state = DiodeOperatingState { i_d: -1.0e-4 };
        let stamp = noise_stamp_diode(&DiodeParams::default(), &state, ROOM_TEMPERATURE_K);
        assert_eq!(stamp.sources.len(), 1);
        let expected = 2.0 * ELEMENTARY_CHARGE_C * 1.0e-4;
        assert_eq!(stamp.sources[0].white_psd.to_bits(), expected.to_bits());
    }

    #[test]
    fn diode_with_flicker_params_and_current_emits_flicker_source() {
        // KF = 1e-25, AF = 1, I_D = 1 mA → flicker_coeff = 1e-28.
        let params = DiodeParams {
            kf: 1.0e-25,
            af: 1.0,
            ..Default::default()
        };
        let state = DiodeOperatingState { i_d: 1.0e-3 };
        let stamp = noise_stamp_diode(&params, &state, ROOM_TEMPERATURE_K);

        // Two sources: shot + flicker (RS = 0).
        assert_eq!(stamp.sources.len(), 2);
        let flicker = stamp
            .sources
            .iter()
            .find(|s| s.mechanism == NoiseMechanism::Flicker)
            .expect("flicker source must be present");
        assert_eq!(flicker.white_psd.to_bits(), 0.0_f64.to_bits());
        let expected = 1.0e-25 * 1.0e-3_f64.powf(1.0);
        assert_eq!(flicker.flicker_coeff.to_bits(), expected.to_bits());
    }

    #[test]
    fn diode_with_full_setup_emits_three_sources() {
        // RS > 0 + I_D > 0 + KF > 0 → three sources (thermal, shot,
        // flicker), each tagged distinctly.
        let params = DiodeParams {
            rs: 50.0,
            kf: 1.0e-24,
            af: 1.0,
            ..Default::default()
        };
        let state = DiodeOperatingState { i_d: 2.0e-3 };
        let stamp = noise_stamp_diode(&params, &state, ROOM_TEMPERATURE_K);

        assert_eq!(stamp.sources.len(), 3);
        let mechs: Vec<_> = stamp.sources.iter().map(|s| s.mechanism).collect();
        assert!(mechs.contains(&NoiseMechanism::Thermal));
        assert!(mechs.contains(&NoiseMechanism::Shot));
        assert!(mechs.contains(&NoiseMechanism::Flicker));
    }

    // -----------------------------------------------------------------
    // BJT noise stamp.
    // -----------------------------------------------------------------

    #[test]
    fn bjt_with_zero_currents_zero_kf_emits_no_sources() {
        let stamp = noise_stamp_bjt(&BJTParams::default(), &BJTOperatingState::default());
        assert!(stamp.sources.is_empty());
    }

    #[test]
    fn bjt_at_quiescent_emits_two_shot_sources() {
        let params = BJTParams::default(); // KF = 0
        let state = BJTOperatingState {
            i_b: 10.0e-6,
            i_c: 1.0e-3,
        };
        let stamp = noise_stamp_bjt(&params, &state);
        assert_eq!(stamp.sources.len(), 2);
        let base = stamp
            .sources
            .iter()
            .find(|s| s.a == 1 && s.b == 2)
            .expect("base shot source");
        let coll = stamp
            .sources
            .iter()
            .find(|s| s.a == 0 && s.b == 2)
            .expect("collector shot source");
        assert_eq!(
            base.white_psd.to_bits(),
            (2.0 * ELEMENTARY_CHARGE_C * 10.0e-6).to_bits()
        );
        assert_eq!(
            coll.white_psd.to_bits(),
            (2.0 * ELEMENTARY_CHARGE_C * 1.0e-3).to_bits()
        );
        // BJT shot scales with current — collector must be ≫ base
        // when β = 100.
        assert!(coll.white_psd > 50.0 * base.white_psd);
    }

    #[test]
    fn bjt_flicker_attaches_to_base_current_per_spice_convention() {
        // SPICE attaches KF/AF to I_B, not I_C. Set up I_B = I_C
        // and verify the flicker source uses I_B's value
        // (specifically: |I_B|^AF, not |I_C|^AF).
        let params = BJTParams {
            kf: 1.0e-15,
            af: 1.0,
            ..Default::default()
        };
        let state = BJTOperatingState {
            i_b: 1.0e-6,
            i_c: 1.0e-3, // 1000x bigger than I_B
        };
        let stamp = noise_stamp_bjt(&params, &state);
        let flicker = stamp
            .sources
            .iter()
            .find(|s| s.mechanism == NoiseMechanism::Flicker)
            .expect("flicker source must be present");
        // |I_B|^AF * KF = 1e-6 * 1e-15 = 1e-21 — pinned to I_B.
        assert_eq!(
            flicker.flicker_coeff.to_bits(),
            (1.0e-15 * 1.0e-6_f64.powf(1.0)).to_bits()
        );
        // And the flicker source sits between base and emitter.
        assert_eq!(flicker.a, 1);
        assert_eq!(flicker.b, 2);
    }

    #[test]
    fn bjt_pnp_polarity_uses_absolute_currents() {
        let params = BJTParams {
            polarity: BJTPolarity::Pnp,
            ..Default::default()
        };
        let state = BJTOperatingState {
            i_b: -10.0e-6,
            i_c: -1.0e-3,
        };
        let stamp = noise_stamp_bjt(&params, &state);
        assert_eq!(stamp.sources.len(), 2);
        // Both PSDs must be positive (i.e. magnitude was used).
        for s in &stamp.sources {
            assert!(s.white_psd > 0.0);
        }
    }

    // -----------------------------------------------------------------
    // MOSFET noise stamp.
    // -----------------------------------------------------------------

    #[test]
    fn mosfet_at_quiescent_with_zero_gm_emits_no_sources() {
        let params = MOSFETParams::Level1(MosLevel1Params::default());
        let state = MosfetOperatingState {
            i_d: 0.0,
            g_m: 0.0,
            temperature_k: ROOM_TEMPERATURE_K,
        };
        let stamp = noise_stamp_mosfet(&params, &state);
        assert!(stamp.sources.is_empty());
    }

    #[test]
    fn mosfet_channel_thermal_uses_8_over_3_kt_gm_in_saturation() {
        // 4·k_B·T·γ·g_m with γ = 2/3 → (8/3)·k_B·T·g_m.
        let params = MOSFETParams::Level1(MosLevel1Params::default());
        let state = MosfetOperatingState {
            i_d: 1.0e-3,
            g_m: 1.0e-3, // 1 mS
            temperature_k: ROOM_TEMPERATURE_K,
        };
        let stamp = noise_stamp_mosfet(&params, &state);
        // Only thermal (KF = 0 default), no flicker.
        assert_eq!(stamp.sources.len(), 1);
        let s = &stamp.sources[0];
        assert_eq!(s.mechanism, NoiseMechanism::Thermal);
        let expected = (8.0 / 3.0) * BOLTZMANN_J_PER_K * ROOM_TEMPERATURE_K * 1.0e-3;
        // Tolerance accounts for the floating-point evaluation
        // order: the helper computes `4·k·T·γ·g_m` while the
        // expectation here writes `(8/3)·k·T·g_m`.
        assert!((s.white_psd - expected).abs() / expected < 1.0e-12);
        assert_eq!(s.a, 0); // drain
        assert_eq!(s.b, 2); // source
    }

    #[test]
    fn mosfet_level1_flicker_uses_typed_kf_af_fields() {
        let params = MOSFETParams::Level1(MosLevel1Params {
            kf: 1.0e-26,
            af: 1.0,
            ..Default::default()
        });
        let state = MosfetOperatingState {
            i_d: 100.0e-6,
            g_m: 0.0, // no thermal, only flicker
            temperature_k: ROOM_TEMPERATURE_K,
        };
        let stamp = noise_stamp_mosfet(&params, &state);
        assert_eq!(stamp.sources.len(), 1);
        let s = &stamp.sources[0];
        assert_eq!(s.mechanism, NoiseMechanism::Flicker);
        let expected = 1.0e-26 * 100.0e-6_f64.powf(1.0);
        assert_eq!(s.flicker_coeff.to_bits(), expected.to_bits());
    }

    #[test]
    fn mosfet_bsim3v3_flicker_reads_from_raw_map() {
        let mut raw = std::collections::BTreeMap::new();
        raw.insert("kf".to_string(), 5.0e-26);
        raw.insert("af".to_string(), 2.0);
        let params = MOSFETParams::BSIM3v3(MosBSIM3v3Params {
            name: ModelName::new("nch_b3"),
            polarity: MosPolarity::Nmos,
            raw,
        });
        let state = MosfetOperatingState {
            i_d: 10.0e-6,
            g_m: 0.0,
            temperature_k: ROOM_TEMPERATURE_K,
        };
        let stamp = noise_stamp_mosfet(&params, &state);
        assert_eq!(stamp.sources.len(), 1);
        let expected = 5.0e-26 * 10.0e-6_f64.powf(2.0);
        assert!((stamp.sources[0].flicker_coeff - expected).abs() / expected < 1.0e-12);
    }

    #[test]
    fn mosfet_bsim3v3_missing_kf_in_raw_map_defaults_to_zero_flicker() {
        // SPICE convention: an empty .MODEL with no KF / AF means
        // KF = 0 (no flicker noise).
        let params = MOSFETParams::BSIM3v3(MosBSIM3v3Params::default());
        let state = MosfetOperatingState {
            i_d: 1.0e-3,
            g_m: 0.0,
            temperature_k: ROOM_TEMPERATURE_K,
        };
        let stamp = noise_stamp_mosfet(&params, &state);
        assert!(stamp.sources.is_empty());
    }

    // -----------------------------------------------------------------
    // DeviceModel::noise_stamp dispatch.
    // -----------------------------------------------------------------

    #[test]
    fn noise_stamp_dispatches_diode_through_match() {
        let m = DeviceModel::Diode(DiodeParams {
            name: ModelName::new("d1"),
            rs: 100.0,
            ..Default::default()
        });
        let op = OperatingPoint::Diode([0.6, 0.0]);
        let state = DeviceOperatingState::Diode(DiodeOperatingState::default());
        let stamp = m.noise_stamp(&op, &state).unwrap();
        match stamp {
            DeviceNoiseStamp::Diode(s) => {
                assert_eq!(s.sources.len(), 1); // RS thermal
                assert_eq!(s.sources[0].mechanism, NoiseMechanism::Thermal);
            }
            _ => panic!("expected Diode variant"),
        }
    }

    #[test]
    fn noise_stamp_dispatches_bjt_through_match() {
        let m = DeviceModel::BJT(BJTParams::default());
        let op = OperatingPoint::BJT([5.0, 0.7, 0.0]);
        let state = DeviceOperatingState::BJT(BJTOperatingState {
            i_b: 10.0e-6,
            i_c: 1.0e-3,
        });
        let stamp = m.noise_stamp(&op, &state).unwrap();
        match stamp {
            DeviceNoiseStamp::BJT(s) => {
                assert_eq!(s.sources.len(), 2); // two shot sources
            }
            _ => panic!("expected BJT variant"),
        }
    }

    #[test]
    fn noise_stamp_dispatches_mosfet_through_match() {
        let m = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params::default()));
        let op = OperatingPoint::MOSFET([1.0, 0.7, 0.0, 0.0]);
        let state = DeviceOperatingState::MOSFET(MosfetOperatingState {
            i_d: 1.0e-3,
            g_m: 1.0e-3,
            temperature_k: ROOM_TEMPERATURE_K,
        });
        let stamp = m.noise_stamp(&op, &state).unwrap();
        match stamp {
            DeviceNoiseStamp::MOSFET(s) => {
                assert_eq!(s.sources.len(), 1); // channel thermal
            }
            _ => panic!("expected MOSFET variant"),
        }
    }

    #[test]
    fn noise_stamp_terminal_counts_match_stamp_module() {
        let diode = DeviceModel::Diode(DiodeParams::default())
            .noise_stamp(
                &OperatingPoint::Diode([0.0; 2]),
                &DeviceOperatingState::Diode(DiodeOperatingState::default()),
            )
            .unwrap();
        let bjt = DeviceModel::BJT(BJTParams::default())
            .noise_stamp(
                &OperatingPoint::BJT([0.0; 3]),
                &DeviceOperatingState::BJT(BJTOperatingState::default()),
            )
            .unwrap();
        let mosfet = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params::default()))
            .noise_stamp(
                &OperatingPoint::MOSFET([0.0; 4]),
                &DeviceOperatingState::MOSFET(MosfetOperatingState::default()),
            )
            .unwrap();
        assert_eq!(diode.terminal_count(), DIODE_TERMINALS);
        assert_eq!(bjt.terminal_count(), BJT_TERMINALS);
        assert_eq!(mosfet.terminal_count(), MOSFET_TERMINALS);
    }

    #[test]
    fn noise_stamp_rejects_op_family_mismatch() {
        let m = DeviceModel::Diode(DiodeParams::default());
        let op = OperatingPoint::BJT([0.0; 3]); // wrong family
        let state = DeviceOperatingState::Diode(DiodeOperatingState::default());
        let err = m.noise_stamp(&op, &state).unwrap_err();
        assert_eq!(err.expected, "Diode");
        assert_eq!(err.actual, "BJT");
    }

    #[test]
    fn noise_stamp_rejects_state_family_mismatch() {
        let m = DeviceModel::BJT(BJTParams::default());
        let op = OperatingPoint::BJT([0.0; 3]);
        let state = DeviceOperatingState::Diode(DiodeOperatingState::default());
        let err = m.noise_stamp(&op, &state).unwrap_err();
        assert_eq!(err.expected, "BJT");
        assert_eq!(err.actual, "Diode");
    }

    // -----------------------------------------------------------------
    // Exhaustiveness witness: per ADR-0005, adding a DeviceModel
    // variant must break a `match`. This test pins that the
    // DeviceNoiseStamp enum mirrors DeviceModel's family shape.
    // -----------------------------------------------------------------

    #[test]
    fn device_noise_stamp_family_match_is_exhaustive() {
        fn family(d: &DeviceNoiseStamp) -> &'static str {
            match d {
                DeviceNoiseStamp::Diode(_) => "Diode",
                DeviceNoiseStamp::BJT(_) => "BJT",
                DeviceNoiseStamp::MOSFET(_) => "MOSFET",
            }
        }
        assert_eq!(
            family(&DeviceNoiseStamp::Diode(DiodeNoiseStamp {
                sources: vec![],
            })),
            "Diode"
        );
        assert_eq!(
            family(&DeviceNoiseStamp::BJT(BJTNoiseStamp { sources: vec![] })),
            "BJT"
        );
        assert_eq!(
            family(&DeviceNoiseStamp::MOSFET(MosfetNoiseStamp {
                sources: vec![],
            })),
            "MOSFET"
        );
    }

    // -----------------------------------------------------------------
    // Spec scenario witness: a resistive-only circuit's thermal noise
    // matches the analytical 4kT/R prediction at every frequency.
    // -----------------------------------------------------------------

    #[test]
    fn spec_scenario_resistor_only_thermal_noise_matches_analytical_4ktr() {
        // Spec: `Then the total output noise density at each
        // frequency matches the theoretical 4kTR value within the
        // tolerance envelope`.
        //
        // For a single resistor R driving an open-circuit node,
        // the output noise voltage PSD is
        //
        //     S_V(f) = |Z_out|² · S_I(f) = R² · (4kT/R) = 4kT·R.
        //
        // Here we exercise the *current* PSD piece this task owns
        // (S_I = 4kT/R), and verify it is constant across a sweep
        // of frequencies and reproduces 4kT/R within 1 ULP.
        let r = 2.2e3;
        let t = ROOM_TEMPERATURE_K;
        let s = resistor_thermal_noise(r, t);

        let expected_si = 4.0 * BOLTZMANN_J_PER_K * t / r;
        for f in [1.0, 1.0e3, 1.0e6, 1.0e9_f64] {
            // White noise: psd_at(f) must equal expected_si exactly
            // for any f > 0.
            assert_eq!(s.psd_at(f).to_bits(), expected_si.to_bits());
        }
    }
}
