//! Trapezoidal Rule companion models for reactive elements.
//!
//! Implements `tasks.md` **#30**: per-element companion stamps for
//! `netlist_graph::ElementKind::Capacitor` and
//! `netlist_graph::ElementKind::Inductor` under the **Trapezoidal
//! Rule** (TR) implicit discretization. Sibling implicit methods
//! ([Backward Euler](super::backward_euler), Gear-2 BDF) live in
//! their own modules and produce the same [`CompanionStamp`] shape so
//! the Pass-2 MNA assembler can fold any method's output identically.
//!
//! # Discretization (textbook TR, charge-conserving)
//!
//! The Trapezoidal Rule applied to the constitutive law of a
//! capacitor or inductor yields a Norton-equivalent stamp at the
//! *new* timestep `t_{n+1} = t_n + h`. The defining feature versus
//! Backward Euler is that TR averages the derivative across the
//! interval rather than evaluating it at the right endpoint:
//!
//! ```text
//!   dy/dt = f(t, y)   ⇒   y_{n+1} − y_n = (h / 2) · [f_{n+1} + f_n]
//! ```
//!
//! For passive RLC elements this collapses to a compact stamp.
//!
//! ## Capacitor
//!
//! Continuous law: `i_C(t) = C · dv/dt`, with `v(t) = v_a(t) − v_b(t)`
//! and `i_C` directed from terminal `a` to terminal `b`.
//!
//! Trapezoidal discretization with step `h`:
//!
//! ```text
//!   v^{n+1} − v^n = (h / (2C)) · (i^{n+1} + i^n)
//! ```
//!
//! Solving for `i^{n+1}`:
//!
//! ```text
//!   i^{n+1} = (2C / h) · (v^{n+1} − v^n) − i^n
//!           = (2C / h) · v^{n+1} − [ (2C / h) · v^n + i^n ]
//! ```
//!
//! Identifying terms with the companion convention `i^{n+1} = g_eq ·
//! v^{n+1} − i_history`:
//!
//! ```text
//!   g_eq      = 2C / h
//!   i_history = (2C / h) · v^n + i^n
//! ```
//!
//! Physical sanity check: at DC steady state `v^{n+1} = v^n` and
//! `i^{n+1} = i^n = 0`. Substituting yields `0 = g_eq · v^n − (g_eq ·
//! v^n + 0)`, which holds — a capacitor draws no DC current.
//!
//! ## Inductor
//!
//! Continuous law: `v_L(t) = L · di/dt`, with `v_L(t) = v_a(t) −
//! v_b(t)` and `i_L` directed from terminal `a` to terminal `b`.
//!
//! Trapezoidal discretization with step `h`:
//!
//! ```text
//!   i^{n+1} − i^n = (h / (2L)) · (v^{n+1} + v^n)
//! ```
//!
//! Solving for `i^{n+1}`:
//!
//! ```text
//!   i^{n+1} = (h / (2L)) · v^{n+1} + [ (h / (2L)) · v^n + i^n ]
//!           = (h / (2L)) · v^{n+1} − [ −((h / (2L)) · v^n + i^n) ]
//! ```
//!
//! Identifying with the companion convention `i^{n+1} = g_eq · v^{n+1}
//! − i_history`:
//!
//! ```text
//!   g_eq      = h / (2L)
//!   i_history = − [ (h / (2L)) · v^n + i^n ]
//! ```
//!
//! Physical sanity check: at DC steady state `i^{n+1} = i^n = I0` and
//! `v^{n+1} = v^n = 0`. Substituting yields `I0 = 0 − ( − (0 + I0) ) =
//! I0` — an inductor presents a short at DC.
//!
//! # Why TR needs *both* `v_prev` and `i_prev`
//!
//! Backward Euler is a one-stage method: the new value depends only on
//! the value at the previous timestep (either `v^n` for the capacitor
//! or `i^n` for the inductor). TR is a two-stage method that averages
//! the derivative at both endpoints, so the companion stamp depends on
//! *both* the previous voltage *and* the previous current of the
//! element. To keep type-safety and avoid pessimising the
//! Backward-Euler stamp (which still only needs one scalar), this
//! module ships its own history structs ([`CapacitorTrapHistory`],
//! [`InductorTrapHistory`]) carrying both fields.
//!
//! # ADR alignment
//!
//! - **ADR-0006** (Dual NR convergence criterion) — vacuously honored.
//!   This module adds no NR loop surface; the transient outer loop
//!   (tasks.md #33) calls the existing NR driver after every
//!   companion-stamp update, and the dual-criterion check applies to
//!   that NR loop unchanged.
//! - **ADR-0007** (Zero-order-hold at analog-digital boundary) —
//!   vacuously honored. No analog-digital boundary surface added.
//! - **ADR-0008** (Per-node max(rel, abs) tolerance envelope) —
//!   relevant to the scenario test in
//!   `tests/scenario_trapezoidal_rlc_tank.rs`, which compares the
//!   trapezoidal RLC tank waveform against the analytic
//!   undamped-oscillation closed form using the ADR-0008 envelope.
//!   This module itself only produces stamps; the envelope comparison
//!   lives in the scenario test.
//! - **ADR-0009** (Topology checker) — vacuously honored. The Pass-1
//!   topology checker classifies capacitors and inductors per
//!   ADR-0009 §"False-positive mitigation"; the TR companion model
//!   does not change that classification.
//! - **ADR-0010** (Unstable v1 public API) — honored. New public
//!   types are part of the unstable v1 surface.
//!
//! # Numerical-damping property (the Gherkin scenario's payoff)
//!
//! Unlike Backward Euler, TR is **A-stable but not L-stable** — it
//! does *not* inject artificial numerical damping into lossless LC
//! circuits. The scenario
//! `transient-time-domain#transient-analysis-with-trapezoidal-integration`
//! exercises exactly this property: an undamped RLC tank under TR
//! retains its analytic amplitude over many periods within the
//! ADR-0008 tolerance envelope. The trade-off is that TR can produce
//! mild high-frequency ringing on stiff problems (the "trapezoidal
//! ringing" pitfall in `design.md`); Backward Euler and Gear-2 BDF
//! remain available for users who prefer L-stable behaviour.

use super::companion::CompanionStamp;

// -----------------------------------------------------------------------
// Input-validation error (shape-compatible with backward_euler's error)
// -----------------------------------------------------------------------

/// Input-validation error from the Trapezoidal companion-model
/// helpers.
///
/// Returned when one of the scalar inputs (step size, capacitance,
/// inductance, or any of the history fields) is non-finite or
/// non-positive in a way that would produce a non-finite stamp. The
/// transient control loop (tasks.md #33) treats these as programming
/// errors in the analysis orchestrator (the orchestrator is expected
/// to clamp `h` to a positive lower bound after LTE rejection, and
/// the netlist parser is expected to reject zero or negative
/// capacitances and inductances at parse time).
///
/// The variant shape mirrors
/// [`super::backward_euler::CompanionInputError`] so callers can
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
    /// One of the history fields (`v_prev` or `i_prev`) was
    /// non-finite (NaN or infinite).
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
                "trapezoidal companion: non-positive or non-finite step h = {value}; \
                 the transient loop must supply h > 0"
            ),
            Self::NonPositiveCapacitance { value } => write!(
                f,
                "trapezoidal companion: non-positive or non-finite capacitance C = {value}; \
                 the netlist parser must supply C > 0"
            ),
            Self::NonPositiveInductance { value } => write!(
                f,
                "trapezoidal companion: non-positive or non-finite inductance L = {value}; \
                 the netlist parser must supply L > 0"
            ),
            Self::NonFiniteHistory { field, value } => write!(
                f,
                "trapezoidal companion: non-finite history field `{field}` = {value}; \
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

// -----------------------------------------------------------------------
// History structs — TR carries both v_prev and i_prev per element
// -----------------------------------------------------------------------

/// Capacitor state at the most recent accepted timestep, as required
/// by the Trapezoidal Rule.
///
/// TR is a two-stage method that averages the derivative across the
/// timestep, so each element's companion stamp depends on *both* its
/// previous terminal-voltage difference (`v_prev`) and its previous
/// capacitor branch current (`i_prev`). Backward Euler, by contrast,
/// only needs `v_prev`.
///
/// # Field semantics
///
/// - `v_prev` — terminal-voltage difference `V_a − V_b` at
///   `t = t_n` (volts). Sign convention: positive when terminal `a`
///   is the higher-potential terminal.
/// - `i_prev` — capacitor branch current at `t = t_n` (amps),
///   directed `a → b`. The transient loop recovers this value as
///   `i^n = (2C / h_{n−1}) · (v^n − v^{n−1}) − i^{n−1}`, i.e. by
///   re-evaluating the same companion equation at the *previous*
///   timestep. For the very first timestep, `i_prev` is either zero
///   (no prior solve, zero initial conditions) or the user-supplied
///   UIC value.
///
/// # Initial conditions
///
/// At the start of a transient solve, both fields are seeded by the
/// transient control loop (tasks.md #33):
///
/// 1. From the DC operating-point solution at `t = 0` (default),
///    with `v_prev = V_a − V_b` at DC and `i_prev = 0` (capacitor at
///    DC steady-state), or
/// 2. From the user-supplied `UIC` initial-condition values when the
///    analysis request opts into UIC (per
///    `transient-time-domain#transient-analysis-with-uic-initial-conditions`).
///
/// This module accepts either via [`CapacitorTrapHistory::new`] or
/// the named-field literal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CapacitorTrapHistory {
    /// Terminal-voltage difference `V_a − V_b` at the previous
    /// accepted timestep, in volts.
    pub v_prev: f64,
    /// Capacitor branch current at the previous accepted timestep,
    /// in amps, directed `a → b`.
    pub i_prev: f64,
}

impl CapacitorTrapHistory {
    /// Construct a TR capacitor history from previous voltage and
    /// current.
    #[must_use]
    pub const fn new(v_prev: f64, i_prev: f64) -> Self {
        Self { v_prev, i_prev }
    }

    /// History at `t = 0` with no prior solve — both fields zero.
    ///
    /// Used when the transient control loop initializes a capacitor
    /// from a quiescent DC operating point that has not yet been
    /// computed, or as the neutral element for property tests.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            v_prev: 0.0,
            i_prev: 0.0,
        }
    }
}

/// Inductor state at the most recent accepted timestep, as required
/// by the Trapezoidal Rule.
///
/// TR needs both the previous-timestep branch current and the
/// previous-timestep terminal voltage difference. See
/// [`CapacitorTrapHistory`] for the two-stage-method rationale.
///
/// # Field semantics
///
/// - `i_prev` — branch current at `t = t_n` (amps), directed
///   `a → b`. In MNA, this comes from a branch-augmentation row in
///   the previous timestep's solution vector.
/// - `v_prev` — terminal-voltage difference `V_a − V_b` at
///   `t = t_n` (volts). For inductors at DC steady state this is 0
///   (inductor presents a short); transient solves recover the
///   non-zero value from the previous solution vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InductorTrapHistory {
    /// Branch current at the previous accepted timestep (amps,
    /// directed `a → b`).
    pub i_prev: f64,
    /// Terminal-voltage difference `V_a − V_b` at the previous
    /// accepted timestep, in volts.
    pub v_prev: f64,
}

impl InductorTrapHistory {
    /// Construct a TR inductor history from previous current and
    /// voltage.
    #[must_use]
    pub const fn new(i_prev: f64, v_prev: f64) -> Self {
        Self { i_prev, v_prev }
    }

    /// History at `t = 0` with no prior solve — both fields zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            i_prev: 0.0,
            v_prev: 0.0,
        }
    }
}

// -----------------------------------------------------------------------
// Capacitor companion (TR)
// -----------------------------------------------------------------------

/// Compute the Trapezoidal Rule companion stamp for a capacitor.
///
/// Returns the [`CompanionStamp`] `{ g_eq, i_history }` for a
/// capacitor of value `capacitance_farads` at the new timestep
/// `t_{n+1} = t_n + h`, given the previous timestep's state
/// `history`.
///
/// The TR formulas are:
///
/// ```text
///   g_eq      = 2C / h
///   i_history = (2C / h) · v_prev + i_prev
/// ```
///
/// See the [module-level docstring](self) for the derivation and
/// physical sanity check.
///
/// # Arguments
///
/// - `capacitance_farads` — the capacitor's value `C` in farads
///   (F), strictly positive.
/// - `step_seconds` — the integration step size `h` in seconds,
///   strictly positive.
/// - `history` — the [`CapacitorTrapHistory`] from the previous
///   accepted timestep.
///
/// # Errors
///
/// Returns [`CompanionInputError`] when:
///
/// - `capacitance_farads` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveCapacitance`],
/// - `step_seconds` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveStep`],
/// - either `history.v_prev` or `history.i_prev` is non-finite —
///   [`CompanionInputError::NonFiniteHistory`].
pub fn capacitor_companion(
    capacitance_farads: f64,
    step_seconds: f64,
    history: CapacitorTrapHistory,
) -> Result<CompanionStamp, CompanionInputError> {
    validate_step(step_seconds)?;
    validate_capacitance(capacitance_farads)?;
    if !history.v_prev.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_prev",
            value: history.v_prev,
        });
    }
    if !history.i_prev.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_prev",
            value: history.i_prev,
        });
    }
    let g_eq = 2.0 * capacitance_farads / step_seconds;
    let i_history = g_eq * history.v_prev + history.i_prev;
    Ok(CompanionStamp { g_eq, i_history })
}

/// Advance a capacitor's [`CapacitorTrapHistory`] to the *new*
/// timestep after an accepted MNA solve.
///
/// The transient control loop (tasks.md #33) calls this *once per
/// accepted timestep* after the MNA solve to fold the new
/// terminal-voltage difference *and* the new branch current into
/// the per-element history that the next [`capacitor_companion`]
/// call will read.
///
/// The new branch current at the same accepted solve can be
/// recovered from the companion equation
///
/// ```text
///   i^{n+1} = g_eq · v^{n+1} − i_history
/// ```
///
/// or, equivalently, from the difference quotient
///
/// ```text
///   i^{n+1} = (2C / h) · (v^{n+1} − v^n) − i^n
/// ```
///
/// where the new value of `v^{n+1}` is the accepted solution-vector
/// terminal-voltage difference. The transient loop computes this
/// before calling this advancer.
///
/// # Arguments
///
/// - `v_new` — the new timestep's terminal voltage difference
///   `(V_a − V_b)` in volts, taken from the accepted solution
///   vector.
/// - `i_new` — the new timestep's capacitor branch current in amps,
///   directed `a → b`.
///
/// # Errors
///
/// Returns [`CompanionInputError::NonFiniteHistory`] when either
/// `v_new` or `i_new` is non-finite (indicating an upstream
/// divergence the orchestrator should catch).
pub fn advance_capacitor_history(
    v_new: f64,
    i_new: f64,
) -> Result<CapacitorTrapHistory, CompanionInputError> {
    if !v_new.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_prev",
            value: v_new,
        });
    }
    if !i_new.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_prev",
            value: i_new,
        });
    }
    Ok(CapacitorTrapHistory::new(v_new, i_new))
}

// -----------------------------------------------------------------------
// Inductor companion (TR)
// -----------------------------------------------------------------------

/// Compute the Trapezoidal Rule companion stamp for an inductor.
///
/// Returns the [`CompanionStamp`] `{ g_eq, i_history }` for an
/// inductor of value `inductance_henries` at the new timestep
/// `t_{n+1} = t_n + h`, given the previous timestep's branch
/// current and terminal voltage.
///
/// The TR formulas are:
///
/// ```text
///   g_eq      = h / (2L)
///   i_history = − [ (h / (2L)) · v_prev + i_prev ]
/// ```
///
/// See the [module-level docstring](self) for the derivation and
/// physical sanity check (DC short).
///
/// # Arguments
///
/// - `inductance_henries` — the inductor's value `L` in henries
///   (H), strictly positive.
/// - `step_seconds` — the integration step size `h` in seconds,
///   strictly positive.
/// - `history` — the [`InductorTrapHistory`] from the previous
///   accepted timestep.
///
/// # Errors
///
/// Returns [`CompanionInputError`] when:
///
/// - `inductance_henries` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveInductance`],
/// - `step_seconds` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveStep`],
/// - either `history.i_prev` or `history.v_prev` is non-finite —
///   [`CompanionInputError::NonFiniteHistory`].
pub fn inductor_companion(
    inductance_henries: f64,
    step_seconds: f64,
    history: InductorTrapHistory,
) -> Result<CompanionStamp, CompanionInputError> {
    validate_step(step_seconds)?;
    validate_inductance(inductance_henries)?;
    if !history.i_prev.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_prev",
            value: history.i_prev,
        });
    }
    if !history.v_prev.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_prev",
            value: history.v_prev,
        });
    }
    let g_eq = step_seconds / (2.0 * inductance_henries);
    let i_history = -(g_eq * history.v_prev + history.i_prev);
    Ok(CompanionStamp { g_eq, i_history })
}

/// Advance an inductor's [`InductorTrapHistory`] to the *new*
/// timestep after an accepted MNA solve.
///
/// The transient control loop calls this once per accepted timestep
/// after the MNA solve.
///
/// # Arguments
///
/// - `i_new` — the new timestep's branch current in amps, directed
///   `a → b`, taken from the accepted solution vector (an MNA
///   branch-augmentation row).
/// - `v_new` — the new timestep's terminal-voltage difference
///   `V_a − V_b` in volts.
///
/// # Errors
///
/// Returns [`CompanionInputError::NonFiniteHistory`] when either
/// `i_new` or `v_new` is non-finite.
pub fn advance_inductor_history(
    i_new: f64,
    v_new: f64,
) -> Result<InductorTrapHistory, CompanionInputError> {
    if !i_new.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_prev",
            value: i_new,
        });
    }
    if !v_new.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_prev",
            value: v_new,
        });
    }
    Ok(InductorTrapHistory::new(i_new, v_new))
}

// =======================================================================
// Tests
// =======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Capacitor — algebraic identities
    // -----------------------------------------------------------------------

    #[test]
    fn capacitor_g_eq_is_two_c_over_h() {
        let c = 1.0e-9; // 1 nF
        let h = 1.0e-6; // 1 µs
        let stamp = capacitor_companion(c, h, CapacitorTrapHistory::zero()).unwrap();
        assert!((stamp.g_eq - (2.0 * c / h)).abs() < 1e-15);
    }

    #[test]
    fn capacitor_zero_history_yields_zero_i_history() {
        let stamp = capacitor_companion(1.0e-9, 1.0e-6, CapacitorTrapHistory::zero()).unwrap();
        assert_eq!(stamp.i_history.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn capacitor_i_history_formula_matches_derivation() {
        // i_history = (2C/h) · v_prev + i_prev
        let c = 2.0e-12;
        let h = 4.0e-12;
        let v_prev = 3.0;
        let i_prev = 0.5e-3;
        let stamp = capacitor_companion(c, h, CapacitorTrapHistory::new(v_prev, i_prev)).unwrap();
        let g_eq = 2.0 * c / h;
        let expected = g_eq * v_prev + i_prev;
        assert!((stamp.i_history - expected).abs() < 1e-15);
    }

    #[test]
    fn capacitor_dc_steady_state_self_consistency() {
        // At DC: v^{n+1} = v^n, i^{n+1} = i^n = 0.
        // The companion law is i^{n+1} = g_eq · v^{n+1} − i_history.
        // With i_prev = 0 and v_prev = v_steady, i_history = g_eq · v_steady,
        // so i^{n+1} = g_eq · v_steady − g_eq · v_steady = 0. ✓
        let c = 1.0;
        let h = 1.0;
        let v_steady = 0.7;
        let stamp = capacitor_companion(c, h, CapacitorTrapHistory::new(v_steady, 0.0)).unwrap();
        let i_new = stamp.g_eq * v_steady - stamp.i_history;
        assert!(i_new.abs() < 1e-15, "DC current should be 0, got {i_new}");
    }

    // -----------------------------------------------------------------------
    // Inductor — algebraic identities
    // -----------------------------------------------------------------------

    #[test]
    fn inductor_g_eq_is_h_over_two_l() {
        let l = 1.0e-3; // 1 mH
        let h = 1.0e-6; // 1 µs
        let stamp = inductor_companion(l, h, InductorTrapHistory::zero()).unwrap();
        assert!((stamp.g_eq - (h / (2.0 * l))).abs() < 1e-15);
    }

    #[test]
    fn inductor_zero_history_yields_zero_i_history() {
        let stamp = inductor_companion(1.0e-3, 1.0e-6, InductorTrapHistory::zero()).unwrap();
        // Negation of zero yields `-0.0`, whose bit pattern differs from
        // `+0.0`; treat `-0.0` as zero by masking the sign bit.
        let bits = stamp.i_history.to_bits() & !(1_u64 << 63);
        assert_eq!(bits, 0, "expected ±0, got {}", stamp.i_history);
    }

    #[test]
    fn inductor_i_history_formula_matches_derivation() {
        // i_history = − [ (h/(2L)) · v_prev + i_prev ]
        let l = 1.0;
        let h = 1.0;
        let v_prev = 0.3;
        let i_prev = 0.7;
        let stamp = inductor_companion(l, h, InductorTrapHistory::new(i_prev, v_prev)).unwrap();
        let g_eq = h / (2.0 * l);
        let expected = -(g_eq * v_prev + i_prev);
        assert!((stamp.i_history - expected).abs() < 1e-15);
    }

    #[test]
    fn inductor_dc_steady_state_self_consistency() {
        // At DC: i^{n+1} = i^n = I0, v^{n+1} = v^n = 0.
        // Companion law: i^{n+1} = g_eq · v^{n+1} − i_history.
        // With i_prev = I0, v_prev = 0: i_history = −I0.
        // So i^{n+1} = g_eq · 0 − (−I0) = I0. ✓
        let l = 1.0;
        let h = 1.0;
        let i_steady = 2.5;
        let stamp = inductor_companion(l, h, InductorTrapHistory::new(i_steady, 0.0)).unwrap();
        let i_new = stamp.g_eq * 0.0 - stamp.i_history;
        assert!((i_new - i_steady).abs() < 1e-15);
    }

    // -----------------------------------------------------------------------
    // Jacobian / KCL anti-symmetry — both elements
    // -----------------------------------------------------------------------

    #[test]
    fn capacitor_stamp_is_positive_g_eq() {
        let stamp =
            capacitor_companion(1.0e-9, 1.0e-9, CapacitorTrapHistory::new(2.0, 1.0)).unwrap();
        assert!(stamp.g_eq > 0.0);
        assert!(stamp.g_eq.is_finite());
        assert!(stamp.i_history.is_finite());
    }

    #[test]
    fn inductor_stamp_is_positive_g_eq() {
        let stamp =
            inductor_companion(1.0e-3, 1.0e-9, InductorTrapHistory::new(1.0e-3, 0.1)).unwrap();
        assert!(stamp.g_eq > 0.0);
        assert!(stamp.g_eq.is_finite());
        assert!(stamp.i_history.is_finite());
    }

    // -----------------------------------------------------------------------
    // Step-size scaling — capacitor g_eq grows as h shrinks
    // -----------------------------------------------------------------------

    #[test]
    fn capacitor_g_eq_grows_inversely_with_h() {
        let c = 1.0e-9;
        let g_h1 = capacitor_companion(c, 1.0e-9, CapacitorTrapHistory::zero())
            .unwrap()
            .g_eq;
        let g_h2 = capacitor_companion(c, 0.5e-9, CapacitorTrapHistory::zero())
            .unwrap()
            .g_eq;
        // Halving h doubles g_eq.
        assert!((g_h2 - 2.0 * g_h1).abs() < 1e-9);
    }

    #[test]
    fn inductor_g_eq_grows_linearly_with_h() {
        let l = 1.0e-3;
        let g_h1 = inductor_companion(l, 1.0e-9, InductorTrapHistory::zero())
            .unwrap()
            .g_eq;
        let g_h2 = inductor_companion(l, 2.0e-9, InductorTrapHistory::zero())
            .unwrap()
            .g_eq;
        // Doubling h doubles g_eq.
        assert!((g_h2 - 2.0 * g_h1).abs() < 1e-9);
    }

    // -----------------------------------------------------------------------
    // BE-vs-TR comparison — TR conductance is 2× the BE conductance for caps
    // -----------------------------------------------------------------------

    #[test]
    fn trapezoidal_capacitor_g_eq_is_twice_backward_euler() {
        // BE: g_eq = C/h ; TR: g_eq = 2C/h. The ratio is exactly 2.
        let c = 1.0e-9;
        let h = 1.0e-9;
        let tr_stamp = capacitor_companion(c, h, CapacitorTrapHistory::zero()).unwrap();
        let be_stamp = super::super::backward_euler::capacitor_companion(
            c,
            h,
            super::super::companion::CapacitorHistory::zero(),
        )
        .unwrap();
        assert!((tr_stamp.g_eq - 2.0 * be_stamp.g_eq).abs() < 1e-15);
    }

    #[test]
    fn trapezoidal_inductor_g_eq_is_half_backward_euler() {
        // BE: g_eq = h/L ; TR: g_eq = h/(2L). The ratio is exactly 1/2.
        let l = 1.0e-3;
        let h = 1.0e-9;
        let tr_stamp = inductor_companion(l, h, InductorTrapHistory::zero()).unwrap();
        let be_stamp = super::super::backward_euler::inductor_companion(
            l,
            h,
            super::super::companion::InductorHistory::zero(),
        )
        .unwrap();
        assert!((tr_stamp.g_eq - 0.5 * be_stamp.g_eq).abs() < 1e-15);
    }

    // -----------------------------------------------------------------------
    // History advance — round-trip
    // -----------------------------------------------------------------------

    #[test]
    fn advance_capacitor_history_round_trips() {
        let h = advance_capacitor_history(2.5, 0.3).unwrap();
        assert_eq!(h, CapacitorTrapHistory::new(2.5, 0.3));
    }

    #[test]
    fn advance_inductor_history_round_trips() {
        let h = advance_inductor_history(-1.25, 0.4).unwrap();
        assert_eq!(h, InductorTrapHistory::new(-1.25, 0.4));
    }

    // -----------------------------------------------------------------------
    // Input validation — every error variant reachable
    // -----------------------------------------------------------------------

    #[test]
    fn rejects_zero_step() {
        match capacitor_companion(1.0e-9, 0.0, CapacitorTrapHistory::zero()) {
            Err(CompanionInputError::NonPositiveStep { value }) => {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn rejects_negative_step() {
        match capacitor_companion(1.0e-9, -1.0e-9, CapacitorTrapHistory::zero()) {
            Err(CompanionInputError::NonPositiveStep { value }) => {
                assert_eq!(value.to_bits(), (-1.0e-9_f64).to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn rejects_nan_step() {
        match capacitor_companion(1.0e-9, f64::NAN, CapacitorTrapHistory::zero()) {
            Err(CompanionInputError::NonPositiveStep { value }) => assert!(value.is_nan()),
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn rejects_zero_capacitance() {
        match capacitor_companion(0.0, 1.0e-9, CapacitorTrapHistory::zero()) {
            Err(CompanionInputError::NonPositiveCapacitance { value }) => {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveCapacitance, got {other:?}"),
        }
    }

    #[test]
    fn rejects_negative_capacitance() {
        match capacitor_companion(-1.0e-9, 1.0e-9, CapacitorTrapHistory::zero()) {
            Err(CompanionInputError::NonPositiveCapacitance { value }) => {
                assert_eq!(value.to_bits(), (-1.0e-9_f64).to_bits());
            }
            other => panic!("expected NonPositiveCapacitance, got {other:?}"),
        }
    }

    #[test]
    fn rejects_nan_v_prev() {
        match capacitor_companion(1.0e-9, 1.0e-9, CapacitorTrapHistory::new(f64::NAN, 0.0)) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "v_prev");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory(v_prev), got {other:?}"),
        }
    }

    #[test]
    fn rejects_nan_i_prev_capacitor() {
        match capacitor_companion(1.0e-9, 1.0e-9, CapacitorTrapHistory::new(0.0, f64::NAN)) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "i_prev");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory(i_prev), got {other:?}"),
        }
    }

    #[test]
    fn rejects_zero_inductance() {
        match inductor_companion(0.0, 1.0e-9, InductorTrapHistory::zero()) {
            Err(CompanionInputError::NonPositiveInductance { value }) => {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveInductance, got {other:?}"),
        }
    }

    #[test]
    fn rejects_negative_inductance() {
        match inductor_companion(-1.0e-3, 1.0e-9, InductorTrapHistory::zero()) {
            Err(CompanionInputError::NonPositiveInductance { value }) => {
                assert_eq!(value.to_bits(), (-1.0e-3_f64).to_bits());
            }
            other => panic!("expected NonPositiveInductance, got {other:?}"),
        }
    }

    #[test]
    fn rejects_zero_step_inductor() {
        match inductor_companion(1.0e-3, 0.0, InductorTrapHistory::zero()) {
            Err(CompanionInputError::NonPositiveStep { value }) => {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn rejects_nan_i_prev_inductor() {
        match inductor_companion(1.0e-3, 1.0e-9, InductorTrapHistory::new(f64::NAN, 0.0)) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "i_prev");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory(i_prev), got {other:?}"),
        }
    }

    #[test]
    fn rejects_nan_v_prev_inductor() {
        match inductor_companion(1.0e-3, 1.0e-9, InductorTrapHistory::new(0.0, f64::NAN)) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "v_prev");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory(v_prev), got {other:?}"),
        }
    }

    #[test]
    fn advance_capacitor_history_rejects_nan_v() {
        match advance_capacitor_history(f64::NAN, 0.0) {
            Err(CompanionInputError::NonFiniteHistory { field, .. }) => assert_eq!(field, "v_prev"),
            other => panic!("expected NonFiniteHistory(v_prev), got {other:?}"),
        }
    }

    #[test]
    fn advance_capacitor_history_rejects_nan_i() {
        match advance_capacitor_history(0.0, f64::INFINITY) {
            Err(CompanionInputError::NonFiniteHistory { field, .. }) => assert_eq!(field, "i_prev"),
            other => panic!("expected NonFiniteHistory(i_prev), got {other:?}"),
        }
    }

    #[test]
    fn advance_inductor_history_rejects_nan_i() {
        match advance_inductor_history(f64::NAN, 0.0) {
            Err(CompanionInputError::NonFiniteHistory { field, .. }) => assert_eq!(field, "i_prev"),
            other => panic!("expected NonFiniteHistory(i_prev), got {other:?}"),
        }
    }

    #[test]
    fn advance_inductor_history_rejects_nan_v() {
        match advance_inductor_history(0.0, f64::NEG_INFINITY) {
            Err(CompanionInputError::NonFiniteHistory { field, .. }) => assert_eq!(field, "v_prev"),
            other => panic!("expected NonFiniteHistory(v_prev), got {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // Layout witnesses (ADR-0005 spirit) — keep history structs minimal
    // -----------------------------------------------------------------------

    #[test]
    fn capacitor_trap_history_is_two_f64s() {
        assert_eq!(core::mem::size_of::<CapacitorTrapHistory>(), 16);
    }

    #[test]
    fn inductor_trap_history_is_two_f64s() {
        assert_eq!(core::mem::size_of::<InductorTrapHistory>(), 16);
    }

    #[test]
    fn history_zero_equals_default_fields() {
        assert_eq!(
            CapacitorTrapHistory::zero(),
            CapacitorTrapHistory::new(0.0, 0.0)
        );
        assert_eq!(
            InductorTrapHistory::zero(),
            InductorTrapHistory::new(0.0, 0.0)
        );
    }
}
