//! Gear-2 BDF (BDF-2) companion models for reactive elements.
//!
//! Implements `tasks.md` **#31**: per-element companion stamps for
//! `netlist_graph::ElementKind::Capacitor` and
//! `netlist_graph::ElementKind::Inductor` under the **Gear-2
//! backward-differentiation-formula** (BDF-2, also called "Gear-2")
//! implicit discretization. Sibling implicit methods
//! ([Backward Euler](super::backward_euler),
//! [Trapezoidal](super::trapezoidal)) live in their own modules and
//! produce the same flat-struct [`CompanionStamp`] shape so the
//! Pass-2 MNA assembler can fold any method's output identically.
//!
//! # Discretization (textbook BDF-2, charge-conserving, 2nd-order)
//!
//! BDF-2 replaces the first derivative at the new timestep by the
//! 3-point backward-difference quotient:
//!
//! ```text
//!   dy/dt |_{t_{n+1}}  ≈  (3·y_{n+1}  −  4·y_n  +  y_{n−1}) / (2h)
//! ```
//!
//! where `h = t_{n+1} − t_n` and `y_{n−1}` is the value at the
//! timestep *before* `y_n`. This formula is 2nd-order accurate
//! (`p = 2`, leading truncation coefficient `c_3 = −2/3 · h³`) and
//! **stiffly stable** (a strictly weaker property than A-stability
//! that is nonetheless adequate for the SPICE-class stiff systems
//! this crate targets).
//!
//! Per ADR-0010 the public API is unstable at v1.0.0; callers must
//! pin to exact versions.
//!
//! ## Capacitor
//!
//! Continuous law: `i_C(t) = C · dv/dt`, with `v(t) = v_a(t) − v_b(t)`
//! and `i_C` directed `a → b`.
//!
//! BDF-2 discretization with step `h`:
//!
//! ```text
//!   i^{n+1}  =  C · (3·v^{n+1} − 4·v^n + v^{n−1}) / (2h)
//!            =  (3C / (2h)) · v^{n+1}   −   (C / (2h)) · (4·v^n − v^{n−1})
//! ```
//!
//! Identifying with the companion convention `i^{n+1} = g_eq · v^{n+1} −
//! i_history`:
//!
//! ```text
//!   g_eq      = 3C / (2h)
//!   i_history = (C / (2h)) · (4·v^n − v^{n−1})
//! ```
//!
//! Physical sanity check: at DC steady state `v^{n+1} = v^n = v^{n−1} =
//! V`. Then `i_history = (C/(2h)) · (4V − V) = (3C/(2h)) · V = g_eq · V`,
//! so `i^{n+1} = g_eq · V − g_eq · V = 0` — a capacitor draws no DC
//! current. ✓
//!
//! Ratio against Backward Euler (BE has `g_eq = C/h`): the BDF-2
//! capacitor conductance is exactly **3/2 ×** the BE conductance,
//! reflecting the wider effective derivative stencil.
//!
//! ## Inductor
//!
//! Continuous law: `v_L(t) = L · di/dt`, with `v_L(t) = v_a(t) − v_b(t)`
//! and `i_L` directed `a → b`.
//!
//! BDF-2 discretization with step `h`:
//!
//! ```text
//!   v^{n+1}  =  L · (3·i^{n+1} − 4·i^n + i^{n−1}) / (2h)
//! ```
//!
//! Solving for `i^{n+1}` (the Norton form the MNA assembler expects):
//!
//! ```text
//!   3·i^{n+1}  =  (2h / L) · v^{n+1}  +  4·i^n  −  i^{n−1}
//!   i^{n+1}    =  (2h / (3L)) · v^{n+1}  +  (4·i^n − i^{n−1}) / 3
//! ```
//!
//! Identifying with `i^{n+1} = g_eq · v^{n+1} − i_history`:
//!
//! ```text
//!   g_eq      =  2h / (3L)
//!   i_history = −(4·i^n − i^{n−1}) / 3
//! ```
//!
//! Physical sanity check: at DC steady state `i^{n+1} = i^n = i^{n−1} =
//! I0`, `v^{n+1} = 0`. Then `i_history = −(4I0 − I0)/3 = −I0`, so
//! `i^{n+1} = g_eq · 0 − (−I0) = I0` — an inductor presents a short at
//! DC and "remembers" its prior branch current. ✓
//!
//! Ratio against Backward Euler (BE has `g_eq = h/L`): the BDF-2
//! inductor conductance is exactly **2/3 ×** the BE conductance. Note
//! that branch-row form `v = r_eq · i − v_eq` with `r_eq = 3L/(2h)` is
//! mathematically equivalent but is *not* what the Pass-2 MNA
//! assembler (tasks.md #14) consumes — it requires Norton form for
//! every reactive element.
//!
//! # Why two-step history needs a per-method struct
//!
//! Backward Euler is a 1-step method: new value depends only on `v^n`
//! (capacitor) or `i^n` (inductor). Trapezoidal is also effectively
//! 1-step on either of `{v, i}` but couples both at the same step
//! (`v^n` *and* `i^n`). Gear-2 BDF is a true 2-step method: it needs
//! `v^n` and `v^{n−1}` for capacitors, and `i^n` and `i^{n−1}` for
//! inductors. To avoid pessimising the BE one-scalar history struct,
//! this module ships its own [`CapacitorGear2History`] and
//! [`InductorGear2History`] carrying two prior-state scalars apiece
//! — the same compositional pattern [`super::trapezoidal`] uses.
//!
//! # Startup step (`t_0` → `t_1`)
//!
//! Gear-2 is a 2-step method: it needs both `x^n` and `x^{n−1}` to
//! compute `x^{n+1}`. At the very first transient step (`t_1`),
//! `x^{−1}` does not exist. The canonical SPICE convention — adopted
//! here — is to **fall back to Backward Euler for the first step**.
//! That gives a 1st-order opening step (truncation `O(h²)` rather than
//! `O(h³)`) but recovers full Gear-2 order from `t_2` onward.
//!
//! The fallback is exposed via [`capacitor_startup`] /
//! [`inductor_startup`] — dedicated functions returning a
//! [`CompanionStamp`] with the BE coefficients (`g_eq = C/h`,
//! `i_history = (C/h) · v^n` for capacitors; `g_eq = h/L`,
//! `i_history = −i^n` for inductors). The orchestrator selects
//! `*_startup` for `t_1` and `*_companion` from `t_2` onward.
//!
//! # ADR alignment
//!
//! - **ADR-0005** (Closed-enum dispatch on `DeviceModel`) — vacuously
//!   honored. This module does not add `ElementKind` arms; it adds
//!   per-method *helpers* alongside [`super::backward_euler`] and
//!   [`super::trapezoidal`], and the MNA assembler decides per
//!   element which method's helpers to call.
//! - **ADR-0006** (Dual NR convergence criterion) — vacuously
//!   honored. No NR loop surface added; the transient outer loop
//!   (tasks.md #33) calls the existing NR driver after every
//!   companion-stamp update.
//! - **ADR-0007** (Zero-order-hold at analog-digital boundary) —
//!   vacuously honored. No analog-digital boundary surface added.
//! - **ADR-0008** (Per-node `max(rel, abs)` tolerance envelope) —
//!   vacuously honored. No tolerance comparison surface added in this
//!   module; scenario tests for stiff problems will use the envelope.
//! - **ADR-0009** (Topology checker) — vacuously honored. The Pass-1
//!   topology checker classifies capacitors and inductors per ADR-0009
//!   §"False-positive mitigation"; the Gear-2 companion model does
//!   not change that classification.
//! - **ADR-0010** (Unstable v1 public API) — honored. New public
//!   types are part of the unstable v1 surface.
//!
//! # Numerical-damping caveat
//!
//! Gear-2 BDF is L-stable like Backward Euler but has a much smaller
//! local truncation error (`O(h³)` vs `O(h²)`). Like BE, it injects
//! real-valued numerical damping that dissipates energy in lossless
//! LC circuits over many timesteps — though far less aggressively
//! than BE. Callers who care about *energy-preserving* behaviour on
//! lossless LC tanks should pick Trapezoidal
//! ([`super::trapezoidal`]); callers who care about stiff stability
//! and improved accuracy over BE should pick Gear-2.

use super::companion::CompanionStamp;

// =======================================================================
// Input-validation error (shape-compatible with backward_euler / trapezoidal)
// =======================================================================

/// Input-validation error from the Gear-2 BDF companion-model helpers.
///
/// Returned when one of the scalar inputs (step size, capacitance,
/// inductance, or any history field) is non-finite or non-positive
/// in a way that would produce a non-finite stamp. The transient
/// control loop (tasks.md #33) treats these as programming errors in
/// the analysis orchestrator (the orchestrator is expected to clamp
/// `h` to a positive lower bound after LTE rejection, and the
/// netlist parser is expected to reject zero or negative
/// capacitances and inductances at parse time).
///
/// The variant shape mirrors
/// [`super::backward_euler::CompanionInputError`] and
/// [`super::trapezoidal::CompanionInputError`] so callers can
/// pattern-match identically across integration methods.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CompanionInputError {
    /// The integration step size `h` was non-positive or non-finite.
    NonPositiveStep {
        /// The offending value as supplied (seconds).
        value: f64,
    },
    /// The capacitor value `C` was non-positive or non-finite.
    NonPositiveCapacitance {
        /// The offending value as supplied (farads).
        value: f64,
    },
    /// The inductor value `L` was non-positive or non-finite.
    NonPositiveInductance {
        /// The offending value as supplied (henries).
        value: f64,
    },
    /// One of the history fields was non-finite (NaN or infinite).
    NonFiniteHistory {
        /// Which history field violated the contract.
        field: &'static str,
        /// The offending value as supplied.
        value: f64,
    },
}

impl core::fmt::Display for CompanionInputError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonPositiveStep { value } => write!(
                f,
                "gear-2 BDF companion: non-positive or non-finite step h = {value}; \
                 the transient loop must supply h > 0"
            ),
            Self::NonPositiveCapacitance { value } => write!(
                f,
                "gear-2 BDF companion: non-positive or non-finite capacitance C = {value}; \
                 the netlist parser must supply C > 0"
            ),
            Self::NonPositiveInductance { value } => write!(
                f,
                "gear-2 BDF companion: non-positive or non-finite inductance L = {value}; \
                 the netlist parser must supply L > 0"
            ),
            Self::NonFiniteHistory { field, value } => write!(
                f,
                "gear-2 BDF companion: non-finite history field `{field}` = {value}; \
                 indicates upstream divergence"
            ),
        }
    }
}

impl std::error::Error for CompanionInputError {}

fn validate_step(h: f64) -> Result<(), CompanionInputError> {
    if !h.is_finite() || h <= 0.0 {
        Err(CompanionInputError::NonPositiveStep { value: h })
    } else {
        Ok(())
    }
}

fn validate_capacitance(c: f64) -> Result<(), CompanionInputError> {
    if !c.is_finite() || c <= 0.0 {
        Err(CompanionInputError::NonPositiveCapacitance { value: c })
    } else {
        Ok(())
    }
}

fn validate_inductance(l: f64) -> Result<(), CompanionInputError> {
    if !l.is_finite() || l <= 0.0 {
        Err(CompanionInputError::NonPositiveInductance { value: l })
    } else {
        Ok(())
    }
}

// =======================================================================
// History structs — Gear-2 carries two prior-step scalars per element
// =======================================================================

/// Capacitor state at the two most recent accepted timesteps, as
/// required by Gear-2 BDF.
///
/// BDF-2 is a 2-step method whose new-step stamp depends on *both*
/// the previous (`v^n`) and previous-previous (`v^{n−1}`)
/// terminal-voltage differences. Backward Euler, by contrast, needs
/// only `v^n`. To avoid pessimising the BE one-scalar history (and
/// to keep the type system honest about how many prior steps each
/// method consumes), this module ships its own history struct.
///
/// # Field semantics
///
/// - `v_n` — terminal-voltage difference `V_a − V_b` at `t = t_n`
///   (volts). Sign convention: positive when terminal `a` is the
///   higher-potential terminal.
/// - `v_n_minus_1` — terminal-voltage difference at `t = t_{n−1}`
///   (volts).
///
/// # Initial conditions and step rotation
///
/// On the first Gear-2 step (the `t_2` step, after the BE startup at
/// `t_1`), the transient control loop seeds `v_n_minus_1` with the
/// DC operating-point / UIC value at `t_0` and `v_n` with the
/// solution at `t_1`. From each accepted step onward,
/// [`advance_capacitor_history`] rotates: `(v_n_minus_1, v_n) ←
/// (v_n, v_new)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CapacitorGear2History {
    /// Terminal-voltage difference `V_a − V_b` at `t_n`, in volts.
    pub v_n: f64,
    /// Terminal-voltage difference `V_a − V_b` at `t_{n−1}`, in volts.
    pub v_n_minus_1: f64,
}

impl CapacitorGear2History {
    /// Construct a Gear-2 capacitor history from the two most recent
    /// terminal-voltage differences.
    #[must_use]
    pub const fn new(v_n: f64, v_n_minus_1: f64) -> Self {
        Self { v_n, v_n_minus_1 }
    }

    /// History with both fields zero — useful as the neutral element
    /// for property tests and as the seed for a UIC-quiescent solve.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            v_n: 0.0,
            v_n_minus_1: 0.0,
        }
    }
}

/// Inductor state at the two most recent accepted timesteps, as
/// required by Gear-2 BDF.
///
/// BDF-2 needs both `i^n` and `i^{n−1}` to evaluate the inductor
/// companion at the new timestep. See [`CapacitorGear2History`] for
/// the same rationale on the capacitor side.
///
/// # Field semantics
///
/// - `i_n` — inductor branch current at `t = t_n` (amps), directed
///   from terminal `a` to terminal `b` per the conventional current
///   direction.
/// - `i_n_minus_1` — branch current at `t = t_{n−1}` (amps).
///
/// # MNA branch augmentation note
///
/// In MNA the inductor's branch current is represented as an extra
/// state variable (an MNA branch-augmentation row), not derived from
/// node voltages. The transient control loop reads it from the
/// previous timesteps' solution vectors and feeds it back into
/// [`InductorGear2History::new`] for the next step's companion stamp.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InductorGear2History {
    /// Branch current at `t_n`, in amps, directed `a → b`.
    pub i_n: f64,
    /// Branch current at `t_{n−1}`, in amps, directed `a → b`.
    pub i_n_minus_1: f64,
}

impl InductorGear2History {
    /// Construct a Gear-2 inductor history from the two most recent
    /// branch currents.
    #[must_use]
    pub const fn new(i_n: f64, i_n_minus_1: f64) -> Self {
        Self { i_n, i_n_minus_1 }
    }

    /// History with both fields zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            i_n: 0.0,
            i_n_minus_1: 0.0,
        }
    }
}

// =======================================================================
// Capacitor companion (Gear-2 BDF, full 2-step)
// =======================================================================

/// Compute the Gear-2 BDF companion stamp for a capacitor.
///
/// Returns the [`CompanionStamp`] `{ g_eq, i_history }` for a
/// capacitor of value `capacitance_farads` at the new timestep
/// `t_{n+1} = t_n + h`, given the two most recent terminal-voltage
/// differences in `history`.
///
/// The Gear-2 BDF formulas are:
///
/// ```text
///   g_eq      = 3C / (2h)
///   i_history = (C / (2h)) · (4·v_n − v_{n−1})
/// ```
///
/// See the [module-level docstring](self) for the derivation and
/// physical sanity check.
///
/// **Note:** this is the full Gear-2 step. At `t_1` no `v^{n−1}`
/// exists; call [`capacitor_startup`] there to get a Backward-Euler
/// fallback stamp.
///
/// # Arguments
///
/// - `capacitance_farads` — the capacitor's value `C` in farads (F),
///   strictly positive.
/// - `step_seconds` — the integration step size `h` in seconds,
///   strictly positive.
/// - `history` — the [`CapacitorGear2History`] from the two most
///   recent accepted timesteps.
///
/// # Errors
///
/// Returns [`CompanionInputError`] when:
///
/// - `capacitance_farads` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveCapacitance`],
/// - `step_seconds` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveStep`],
/// - either `history.v_n` or `history.v_n_minus_1` is non-finite —
///   [`CompanionInputError::NonFiniteHistory`].
pub fn capacitor_companion(
    capacitance_farads: f64,
    step_seconds: f64,
    history: CapacitorGear2History,
) -> Result<CompanionStamp, CompanionInputError> {
    validate_step(step_seconds)?;
    validate_capacitance(capacitance_farads)?;
    if !history.v_n.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_n",
            value: history.v_n,
        });
    }
    if !history.v_n_minus_1.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_n_minus_1",
            value: history.v_n_minus_1,
        });
    }
    let alpha = capacitance_farads / (2.0 * step_seconds);
    let g_eq = 3.0 * alpha;
    let i_history = alpha * 4.0_f64.mul_add(history.v_n, -history.v_n_minus_1);
    Ok(CompanionStamp { g_eq, i_history })
}

/// Compute the Backward-Euler-fallback capacitor companion stamp for
/// the first transient step (`t_1`).
///
/// At `t_1` no `v^{−1}` exists, so the Gear-2 3-point
/// backward-difference quotient is not yet defined. This function
/// applies the standard 1-step Backward Euler companion instead:
///
/// ```text
///   g_eq      = C / h
///   i_history = (C / h) · v_n        (with v_n = v_0)
/// ```
///
/// From `t_2` onward, switch to [`capacitor_companion`] with both
/// `v_n` and `v_{n−1}` populated.
///
/// # Arguments
///
/// - `capacitance_farads` — the capacitor's value `C` in farads (F),
///   strictly positive.
/// - `step_seconds` — the integration step size `h` in seconds,
///   strictly positive.
/// - `v_n` — terminal-voltage difference at `t = t_0` (volts), the
///   only history available before the first solve.
///
/// # Errors
///
/// Returns [`CompanionInputError`] for the usual non-positive /
/// non-finite reasons; the error variants match [`capacitor_companion`].
pub fn capacitor_startup(
    capacitance_farads: f64,
    step_seconds: f64,
    v_n: f64,
) -> Result<CompanionStamp, CompanionInputError> {
    validate_step(step_seconds)?;
    validate_capacitance(capacitance_farads)?;
    if !v_n.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_n",
            value: v_n,
        });
    }
    let g_eq = capacitance_farads / step_seconds;
    let i_history = g_eq * v_n;
    Ok(CompanionStamp { g_eq, i_history })
}

/// Advance a capacitor's [`CapacitorGear2History`] to the *new*
/// accepted timestep.
///
/// The transient control loop (tasks.md #33) calls this once per
/// accepted timestep after the MNA solve to rotate the per-element
/// history: the previous `v_n` becomes the new `v_{n−1}`, and the
/// solved `v_new` becomes the new `v_n`.
///
/// # Arguments
///
/// - `v_new` — the new accepted timestep's terminal voltage
///   difference `(V_a − V_b)` in volts.
/// - `history` — the history from the previous accepted timestep.
///
/// # Errors
///
/// Returns [`CompanionInputError::NonFiniteHistory`] when `v_new` is
/// non-finite.
pub fn advance_capacitor_history(
    v_new: f64,
    history: CapacitorGear2History,
) -> Result<CapacitorGear2History, CompanionInputError> {
    if !v_new.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_n",
            value: v_new,
        });
    }
    Ok(CapacitorGear2History::new(v_new, history.v_n))
}

// =======================================================================
// Inductor companion (Gear-2 BDF, full 2-step, Norton form)
// =======================================================================

/// Compute the Gear-2 BDF companion stamp for an inductor.
///
/// Returns the [`CompanionStamp`] `{ g_eq, i_history }` for an
/// inductor of value `inductance_henries` at the new timestep
/// `t_{n+1} = t_n + h`, given the two most recent branch currents in
/// `history`.
///
/// The Gear-2 BDF formulas, solved into **Norton form** as required
/// by trunk's flat-struct [`CompanionStamp`] contract:
///
/// ```text
///   g_eq      =  2h / (3L)
///   i_history = −(4·i_n − i_{n−1}) / 3
/// ```
///
/// (The mathematically-equivalent *branch-row* form `v = r_eq · i −
/// v_eq` with `r_eq = 3L/(2h)` and `v_eq = (L/(2h))·(4·i_n −
/// i_{n−1})` is **not** what the Pass-2 MNA assembler consumes — it
/// requires every reactive element to expose a Norton equivalent.)
///
/// See the [module-level docstring](self) for the derivation and
/// physical sanity check (DC short).
///
/// **Note:** this is the full Gear-2 step. At `t_1` no `i^{n−1}`
/// exists; call [`inductor_startup`] there for the Backward-Euler
/// fallback.
///
/// # Arguments
///
/// - `inductance_henries` — the inductor's value `L` in henries (H),
///   strictly positive.
/// - `step_seconds` — the integration step size `h` in seconds,
///   strictly positive.
/// - `history` — the [`InductorGear2History`] from the two most
///   recent accepted timesteps.
///
/// # Errors
///
/// Returns [`CompanionInputError`] when:
///
/// - `inductance_henries` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveInductance`],
/// - `step_seconds` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveStep`],
/// - either `history.i_n` or `history.i_n_minus_1` is non-finite —
///   [`CompanionInputError::NonFiniteHistory`].
pub fn inductor_companion(
    inductance_henries: f64,
    step_seconds: f64,
    history: InductorGear2History,
) -> Result<CompanionStamp, CompanionInputError> {
    validate_step(step_seconds)?;
    validate_inductance(inductance_henries)?;
    if !history.i_n.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_n",
            value: history.i_n,
        });
    }
    if !history.i_n_minus_1.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_n_minus_1",
            value: history.i_n_minus_1,
        });
    }
    let g_eq = (2.0 * step_seconds) / (3.0 * inductance_henries);
    let i_history = -(4.0_f64.mul_add(history.i_n, -history.i_n_minus_1)) / 3.0;
    Ok(CompanionStamp { g_eq, i_history })
}

/// Compute the Backward-Euler-fallback inductor companion stamp for
/// the first transient step (`t_1`).
///
/// At `t_1` no `i^{−1}` exists. This function applies the standard
/// 1-step Backward Euler companion instead, in Norton form
/// matching trunk's [`CompanionStamp`]:
///
/// ```text
///   g_eq      = h / L
///   i_history = −i_n        (with i_n = i_0)
/// ```
///
/// From `t_2` onward, switch to [`inductor_companion`] with both
/// `i_n` and `i_{n−1}` populated.
///
/// # Arguments
///
/// - `inductance_henries` — the inductor's value `L` in henries (H),
///   strictly positive.
/// - `step_seconds` — the integration step size `h` in seconds,
///   strictly positive.
/// - `i_n` — branch current at `t = t_0` (amps), directed `a → b`.
///
/// # Errors
///
/// Returns [`CompanionInputError`] for the usual non-positive /
/// non-finite reasons.
pub fn inductor_startup(
    inductance_henries: f64,
    step_seconds: f64,
    i_n: f64,
) -> Result<CompanionStamp, CompanionInputError> {
    validate_step(step_seconds)?;
    validate_inductance(inductance_henries)?;
    if !i_n.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_n",
            value: i_n,
        });
    }
    let g_eq = step_seconds / inductance_henries;
    let i_history = -i_n;
    Ok(CompanionStamp { g_eq, i_history })
}

/// Advance an inductor's [`InductorGear2History`] to the *new*
/// accepted timestep.
///
/// The transient control loop calls this once per accepted timestep
/// after the MNA solve. The previous `i_n` becomes the new
/// `i_{n−1}`, and the solved `i_new` becomes the new `i_n`.
///
/// # Arguments
///
/// - `i_new` — the new accepted timestep's branch current in amps,
///   directed `a → b`.
/// - `history` — the history from the previous accepted timestep.
///
/// # Errors
///
/// Returns [`CompanionInputError::NonFiniteHistory`] when `i_new` is
/// non-finite.
pub fn advance_inductor_history(
    i_new: f64,
    history: InductorGear2History,
) -> Result<InductorGear2History, CompanionInputError> {
    if !i_new.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_n",
            value: i_new,
        });
    }
    Ok(InductorGear2History::new(i_new, history.i_n))
}

// =======================================================================
// Tests
// =======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Tight equality for derived float quantities — these helpers
    /// only do a handful of multiplications/divisions on small,
    /// well-conditioned inputs, so we expect bit-identical results
    /// up to a few ulps.
    fn approx_eq(a: f64, b: f64) {
        let tol = (a.abs() + b.abs()).mul_add(1e-12, 1e-15);
        assert!(
            (a - b).abs() <= tol,
            "approx_eq failed: |{a} - {b}| = {} > {tol}",
            (a - b).abs(),
        );
    }

    // -----------------------------------------------------------------
    // Capacitor — algebraic identities (full Gear-2 step)
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_g_eq_is_3c_over_2h() {
        let c = 1.0e-9; // 1 nF
        let h = 1.0e-6; // 1 µs
        let stamp = capacitor_companion(c, h, CapacitorGear2History::zero()).unwrap();
        approx_eq(stamp.g_eq, 3.0 * c / (2.0 * h));
    }

    #[test]
    fn capacitor_zero_history_yields_zero_i_history() {
        let stamp = capacitor_companion(1.0e-9, 1.0e-6, CapacitorGear2History::zero()).unwrap();
        approx_eq(stamp.i_history, 0.0);
    }

    #[test]
    fn capacitor_i_history_formula_matches_derivation() {
        // i_history = (C/(2h)) · (4·v_n − v_{n−1})
        let c = 1.0e-12;
        let h = 1.0e-9;
        let v_n = 1.0;
        let v_n_minus_1 = 0.5;
        let stamp =
            capacitor_companion(c, h, CapacitorGear2History::new(v_n, v_n_minus_1)).unwrap();
        let alpha = c / (2.0 * h);
        let expected = alpha * (4.0 * v_n - v_n_minus_1);
        approx_eq(stamp.i_history, expected);
    }

    #[test]
    fn capacitor_dc_steady_state_yields_zero_current() {
        // In steady state v_{n+1} = v_n = v_{n-1} = V_dc; companion
        // equation i = g_eq · v − i_history must give i = 0.
        let c = 2.5e-12;
        let h = 5.0e-10;
        let v_dc = 1.8;
        let stamp = capacitor_companion(c, h, CapacitorGear2History::new(v_dc, v_dc)).unwrap();
        let i_predicted = stamp.g_eq * v_dc - stamp.i_history;
        approx_eq(i_predicted, 0.0);
    }

    #[test]
    fn capacitor_companion_reproduces_dv_dt_for_linear_ramp() {
        // For v(t) = v0 + s·t with constant slope s:
        //   v_{n-1} = v0,   v_n = v0 + s·h,   v_{n+1} = v0 + 2·s·h
        // The exact i_C is C·s. Gear-2 BDF is exact on linear
        // functions (in fact on quadratics), so the companion must
        // reproduce exactly C·s.
        let c = 4.7e-9;
        let h = 1.0e-7;
        let v0 = 0.3;
        let slope = 1.5e6; // V/s
        let v_n_minus_1 = v0;
        let v_n = v0 + slope * h;
        let v_n_plus_1 = v0 + 2.0 * slope * h;

        let stamp =
            capacitor_companion(c, h, CapacitorGear2History::new(v_n, v_n_minus_1)).unwrap();
        let i_predicted = stamp.g_eq * v_n_plus_1 - stamp.i_history;
        approx_eq(i_predicted, c * slope);
    }

    #[test]
    fn capacitor_companion_reproduces_quadratic_signal_exactly() {
        // BDF-2 is order 2: exact on v(t) = a_0 + a_1·t + a_2·t².
        // Pick coefficients and evaluate at three contiguous samples.
        let cap = 1.0; // dimensionless; verifying the math.
        let h = 0.1;
        let (a_0, a_1, a_2): (f64, f64, f64) = (0.4, 1.7, -2.3);
        let v = |t: f64| a_2.mul_add(t * t, a_1.mul_add(t, a_0));
        // dv/dt = a_1 + 2·a_2·t.
        let t_n_minus_1 = 0.0;
        let t_n = h;
        let t_n_plus_1 = 2.0 * h;
        let v_n_minus_1 = v(t_n_minus_1);
        let v_n = v(t_n);
        let v_n_plus_1 = v(t_n_plus_1);
        let exact_i = cap * 2.0_f64.mul_add(a_2 * t_n_plus_1, a_1);

        let stamp =
            capacitor_companion(cap, h, CapacitorGear2History::new(v_n, v_n_minus_1)).unwrap();
        let i_predicted = stamp.g_eq.mul_add(v_n_plus_1, -stamp.i_history);
        approx_eq(i_predicted, exact_i);
    }

    // -----------------------------------------------------------------
    // Capacitor — BE-fallback startup
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_startup_matches_backward_euler() {
        let c = 1.0e-12;
        let h = 1.0e-9;
        let v0 = 0.7;

        let stamp = capacitor_startup(c, h, v0).unwrap();
        approx_eq(stamp.g_eq, c / h);
        approx_eq(stamp.i_history, (c / h) * v0);
    }

    #[test]
    fn capacitor_startup_dc_steady_state_yields_zero_current() {
        // With v_1 = v_0 = V_dc the BE companion must predict i = 0.
        let c = 2.5e-12;
        let h = 5.0e-10;
        let v_dc = 1.8;
        let stamp = capacitor_startup(c, h, v_dc).unwrap();
        let i_predicted = stamp.g_eq * v_dc - stamp.i_history;
        approx_eq(i_predicted, 0.0);
    }

    #[test]
    fn capacitor_startup_matches_trunk_backward_euler_helper() {
        // Cross-check: gear-2 BE-fallback startup must produce a
        // bit-identical stamp to trunk's backward_euler::capacitor_companion.
        let c = 3.3e-12;
        let h = 2.5e-10;
        let v0 = 1.1;
        let our = capacitor_startup(c, h, v0).unwrap();
        let theirs = super::super::backward_euler::capacitor_companion(
            c,
            h,
            super::super::companion::CapacitorHistory::new(v0),
        )
        .unwrap();
        approx_eq(our.g_eq, theirs.g_eq);
        approx_eq(our.i_history, theirs.i_history);
    }

    // -----------------------------------------------------------------
    // Capacitor — input validation
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_companion_rejects_negative_capacitance() {
        match capacitor_companion(-1.0e-12, 1.0e-9, CapacitorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveCapacitance { value }) => {
                assert_eq!(value.to_bits(), (-1.0e-12_f64).to_bits());
            }
            other => panic!("expected NonPositiveCapacitance, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_zero_capacitance() {
        match capacitor_companion(0.0, 1.0e-9, CapacitorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveCapacitance { value }) => {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveCapacitance, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_nan_capacitance() {
        match capacitor_companion(f64::NAN, 1.0e-9, CapacitorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveCapacitance { value }) => assert!(value.is_nan()),
            other => panic!("expected NonPositiveCapacitance, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_zero_timestep() {
        match capacitor_companion(1.0e-12, 0.0, CapacitorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveStep { value }) => {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_negative_timestep() {
        match capacitor_companion(1.0e-12, -1.0e-9, CapacitorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveStep { value }) => {
                assert_eq!(value.to_bits(), (-1.0e-9_f64).to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_infinite_v_n() {
        match capacitor_companion(
            1.0e-12,
            1.0e-9,
            CapacitorGear2History::new(f64::INFINITY, 0.0),
        ) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "v_n");
                assert!(value.is_infinite());
            }
            other => panic!("expected NonFiniteHistory(v_n), got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_nan_v_n_minus_1() {
        match capacitor_companion(1.0e-12, 1.0e-9, CapacitorGear2History::new(0.0, f64::NAN)) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "v_n_minus_1");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory(v_n_minus_1), got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // Inductor — algebraic identities (full Gear-2 step, Norton form)
    // -----------------------------------------------------------------

    #[test]
    fn inductor_g_eq_is_2h_over_3l() {
        let l = 1.0e-6; // 1 µH
        let h = 1.0e-9; // 1 ns
        let stamp = inductor_companion(l, h, InductorGear2History::zero()).unwrap();
        approx_eq(stamp.g_eq, (2.0 * h) / (3.0 * l));
    }

    #[test]
    fn inductor_zero_history_yields_zero_i_history() {
        let stamp = inductor_companion(1.0e-3, 1.0e-6, InductorGear2History::zero()).unwrap();
        // Negation of zero can yield −0.0; treat ±0 as zero.
        let bits = stamp.i_history.to_bits() & !(1_u64 << 63);
        assert_eq!(bits, 0, "expected ±0, got {}", stamp.i_history);
    }

    #[test]
    fn inductor_i_history_formula_matches_derivation() {
        // i_history = −(4·i_n − i_{n-1}) / 3
        let l = 1.0e-6;
        let h = 1.0e-9;
        let i_n = 1.0e-3;
        let i_n_minus_1 = 0.5e-3;
        let stamp = inductor_companion(l, h, InductorGear2History::new(i_n, i_n_minus_1)).unwrap();
        let expected = -(4.0 * i_n - i_n_minus_1) / 3.0;
        approx_eq(stamp.i_history, expected);
    }

    #[test]
    fn inductor_dc_steady_state_yields_steady_current() {
        // In steady state i_{n+1} = i_n = i_{n−1} = I_dc, v_{n+1} = 0.
        // Companion law: i^{n+1} = g_eq · v^{n+1} − i_history.
        // With history = (I_dc, I_dc): i_history = −(4·I − I)/3 = −I_dc.
        // So i^{n+1} = 0 − (−I_dc) = I_dc. ✓
        let l = 1.0e-6;
        let h = 1.0e-8;
        let i_dc = 2.5e-3;
        let stamp = inductor_companion(l, h, InductorGear2History::new(i_dc, i_dc)).unwrap();
        let i_predicted = stamp.g_eq * 0.0 - stamp.i_history;
        approx_eq(i_predicted, i_dc);
    }

    #[test]
    fn inductor_companion_reproduces_di_dt_for_linear_ramp() {
        // i(t) = i0 + s·t  =>  v_L = L·s exactly. With the linear-ramp
        // history (i_{n-1}, i_n) the predicted v_{n+1} from the Norton
        // identity v = (i^{n+1} − history_term) / g_eq should equal L·s,
        // but the cleanest direct check is to plug the exact v_{n+1} and
        // verify the companion equation predicts the exact i_{n+1}.
        let l = 4.7e-6;
        let h = 1.0e-7;
        let i0 = 1.0e-3;
        let slope = 100.0; // A/s
        let v_constant = l * slope; // exact v_L on a linear-current ramp
        let i_n_minus_1 = i0;
        let i_n = i0 + slope * h;
        let i_n_plus_1 = i0 + 2.0 * slope * h;

        let stamp = inductor_companion(l, h, InductorGear2History::new(i_n, i_n_minus_1)).unwrap();
        let i_predicted = stamp.g_eq * v_constant - stamp.i_history;
        approx_eq(i_predicted, i_n_plus_1);
    }

    #[test]
    fn inductor_companion_reproduces_quadratic_signal_exactly() {
        // BDF-2 is order 2: exact on i(t) = a_0 + a_1·t + a_2·t².
        let ind = 1.0;
        let h = 0.05;
        let (a_0, a_1, a_2): (f64, f64, f64) = (0.1, 0.7, 0.9);
        let i = |t: f64| a_2.mul_add(t * t, a_1.mul_add(t, a_0));
        let t_n_minus_1 = 0.0;
        let t_n = h;
        let t_n_plus_1 = 2.0 * h;
        let i_n_minus_1 = i(t_n_minus_1);
        let i_n = i(t_n);
        let i_n_plus_1 = i(t_n_plus_1);
        let exact_v = ind * 2.0_f64.mul_add(a_2 * t_n_plus_1, a_1);

        let stamp =
            inductor_companion(ind, h, InductorGear2History::new(i_n, i_n_minus_1)).unwrap();
        let i_predicted = stamp.g_eq.mul_add(exact_v, -stamp.i_history);
        approx_eq(i_predicted, i_n_plus_1);
    }

    // -----------------------------------------------------------------
    // Inductor — BE-fallback startup
    // -----------------------------------------------------------------

    #[test]
    fn inductor_startup_matches_backward_euler() {
        let l = 1.0e-6;
        let h = 1.0e-9;
        let i0 = 1.0e-3;

        let stamp = inductor_startup(l, h, i0).unwrap();
        approx_eq(stamp.g_eq, h / l);
        approx_eq(stamp.i_history, -i0);
    }

    #[test]
    fn inductor_startup_dc_steady_state_holds_current() {
        // i^{n+1} = g_eq · v − i_history; v=0, i_history=−I0 ⇒ i = I0. ✓
        let l = 1.0e-6;
        let h = 1.0e-9;
        let i_dc = 2.5e-3;
        let stamp = inductor_startup(l, h, i_dc).unwrap();
        let i_predicted = stamp.g_eq * 0.0 - stamp.i_history;
        approx_eq(i_predicted, i_dc);
    }

    #[test]
    fn inductor_startup_matches_trunk_backward_euler_helper() {
        let l = 2.2e-6;
        let h = 4.5e-10;
        let i0 = 0.3e-3;
        let our = inductor_startup(l, h, i0).unwrap();
        let theirs = super::super::backward_euler::inductor_companion(
            l,
            h,
            super::super::companion::InductorHistory::new(i0),
        )
        .unwrap();
        approx_eq(our.g_eq, theirs.g_eq);
        approx_eq(our.i_history, theirs.i_history);
    }

    // -----------------------------------------------------------------
    // Inductor — input validation
    // -----------------------------------------------------------------

    #[test]
    fn inductor_companion_rejects_zero_inductance() {
        match inductor_companion(0.0, 1.0e-9, InductorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveInductance { value }) => {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveInductance, got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_negative_inductance() {
        match inductor_companion(-1.0e-6, 1.0e-9, InductorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveInductance { value }) => {
                assert_eq!(value.to_bits(), (-1.0e-6_f64).to_bits());
            }
            other => panic!("expected NonPositiveInductance, got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_nan_inductance() {
        match inductor_companion(f64::NAN, 1.0e-9, InductorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveInductance { value }) => assert!(value.is_nan()),
            other => panic!("expected NonPositiveInductance, got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_zero_step() {
        match inductor_companion(1.0e-6, 0.0, InductorGear2History::zero()) {
            Err(CompanionInputError::NonPositiveStep { value }) => {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_infinite_i_n() {
        match inductor_companion(
            1.0e-6,
            1.0e-9,
            InductorGear2History::new(f64::INFINITY, 0.0),
        ) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "i_n");
                assert!(value.is_infinite());
            }
            other => panic!("expected NonFiniteHistory(i_n), got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_nan_i_n_minus_1() {
        match inductor_companion(1.0e-6, 1.0e-9, InductorGear2History::new(0.0, f64::NAN)) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "i_n_minus_1");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory(i_n_minus_1), got {other:?}"),
        }
    }

    #[test]
    fn inductor_startup_rejects_nan_inductance() {
        match inductor_startup(f64::NAN, 1.0e-9, 0.0) {
            Err(CompanionInputError::NonPositiveInductance { value }) => assert!(value.is_nan()),
            other => panic!("expected NonPositiveInductance, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_startup_rejects_nan_v() {
        match capacitor_startup(1.0e-12, 1.0e-9, f64::NAN) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "v_n");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory(v_n), got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // History advance — rotation semantics
    // -----------------------------------------------------------------

    #[test]
    fn advance_capacitor_history_rotates() {
        // Before:  (v_n_minus_1, v_n) = (1.0, 2.0)
        // After advancing with v_new = 3.0:
        //          (v_n_minus_1, v_n) = (2.0, 3.0)
        let prior = CapacitorGear2History::new(2.0, 1.0);
        let next = advance_capacitor_history(3.0, prior).unwrap();
        assert_eq!(next, CapacitorGear2History::new(3.0, 2.0));
    }

    #[test]
    fn advance_inductor_history_rotates() {
        let prior = InductorGear2History::new(0.5e-3, 0.25e-3);
        let next = advance_inductor_history(0.75e-3, prior).unwrap();
        assert_eq!(next, InductorGear2History::new(0.75e-3, 0.5e-3));
    }

    #[test]
    fn advance_capacitor_history_rejects_nan() {
        match advance_capacitor_history(f64::NAN, CapacitorGear2History::zero()) {
            Err(CompanionInputError::NonFiniteHistory { field, .. }) => assert_eq!(field, "v_n"),
            other => panic!("expected NonFiniteHistory(v_n), got {other:?}"),
        }
    }

    #[test]
    fn advance_inductor_history_rejects_infinite() {
        match advance_inductor_history(f64::INFINITY, InductorGear2History::zero()) {
            Err(CompanionInputError::NonFiniteHistory { field, .. }) => assert_eq!(field, "i_n"),
            other => panic!("expected NonFiniteHistory(i_n), got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // BE-vs-Gear-2 conductance ratios
    // -----------------------------------------------------------------

    #[test]
    fn gear2_capacitor_g_eq_is_three_halves_backward_euler() {
        // BE: g_eq = C/h; Gear-2: g_eq = 3C/(2h). Ratio = 3/2.
        let c = 1.0e-9;
        let h = 1.0e-9;
        let g2 = capacitor_companion(c, h, CapacitorGear2History::zero())
            .unwrap()
            .g_eq;
        let be = super::super::backward_euler::capacitor_companion(
            c,
            h,
            super::super::companion::CapacitorHistory::zero(),
        )
        .unwrap()
        .g_eq;
        approx_eq(g2, 1.5 * be);
    }

    #[test]
    fn gear2_inductor_g_eq_is_two_thirds_backward_euler() {
        // BE: g_eq = h/L; Gear-2: g_eq = 2h/(3L). Ratio = 2/3.
        let l = 1.0e-3;
        let h = 1.0e-9;
        let g2 = inductor_companion(l, h, InductorGear2History::zero())
            .unwrap()
            .g_eq;
        let be = super::super::backward_euler::inductor_companion(
            l,
            h,
            super::super::companion::InductorHistory::zero(),
        )
        .unwrap()
        .g_eq;
        approx_eq(g2, (2.0 / 3.0) * be);
    }

    // -----------------------------------------------------------------
    // Layout witnesses — history structs stay minimal (two f64s)
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_gear2_history_is_two_f64s() {
        assert_eq!(core::mem::size_of::<CapacitorGear2History>(), 16);
    }

    #[test]
    fn inductor_gear2_history_is_two_f64s() {
        assert_eq!(core::mem::size_of::<InductorGear2History>(), 16);
    }

    #[test]
    fn history_zero_equals_default_fields() {
        assert_eq!(
            CapacitorGear2History::zero(),
            CapacitorGear2History::new(0.0, 0.0)
        );
        assert_eq!(
            InductorGear2History::zero(),
            InductorGear2History::new(0.0, 0.0)
        );
    }

    // -----------------------------------------------------------------
    // Display + Error coverage
    // -----------------------------------------------------------------

    #[test]
    fn companion_input_error_display_covers_every_variant() {
        let variants = [
            CompanionInputError::NonPositiveStep { value: 0.0 },
            CompanionInputError::NonPositiveCapacitance { value: -1.0 },
            CompanionInputError::NonPositiveInductance { value: 0.0 },
            CompanionInputError::NonFiniteHistory {
                field: "v_n",
                value: f64::NAN,
            },
        ];
        for v in &variants {
            let s = format!("{v}");
            assert!(!s.is_empty());
            assert!(
                s.contains("gear-2"),
                "Display impl should mention gear-2: got {s:?}"
            );
        }
    }

    #[test]
    fn companion_input_error_is_std_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}
        assert_error(&CompanionInputError::NonPositiveStep { value: 0.0 });
    }
}
