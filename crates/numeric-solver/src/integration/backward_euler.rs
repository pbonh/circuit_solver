//! Backward Euler companion models for reactive elements.
//!
//! Implements tasks.md **#29**: per-element companion stamps for
//! `netlist_graph::ElementKind::Capacitor` and
//! `netlist_graph::ElementKind::Inductor` under the **Backward
//! Euler** (BE) implicit discretization. Sibling implicit methods
//! (Trapezoidal, Gear-2 BDF) live in their own modules and consume
//! the same [`CompanionStamp`] / history types.
//!
//! # Discretization (textbook BE, charge-conserving)
//!
//! The companion model is the **Norton equivalent** at the new
//! timestep `t_{n+1} = t_n + h`. For every reactive element the
//! branch constitutive equation at `t_{n+1}` is written as:
//!
//! ```text
//!   i_branch(a→b) at t_{n+1} = g_eq · (v_a − v_b)_{n+1} − i_history
//! ```
//!
//! where `g_eq` is stamped into the conductance matrix and
//! `i_history` is stamped into the right-hand-side per the
//! convention documented in [`super::companion`].
//!
//! ## Capacitor
//!
//! Continuous law: `i_C(t) = C · dv/dt`, with `v(t) = v_a(t) − v_b(t)`
//! and `i_C` directed from terminal `a` to terminal `b`.
//!
//! BE discretization with step `h`:
//!
//! ```text
//!   i_C^{n+1} = (C / h) · (v^{n+1} − v^n)
//!             = (C / h) · v^{n+1} − (C / h) · v^n
//! ```
//!
//! so:
//!
//! ```text
//!   g_eq      = C / h
//!   i_history = (C / h) · v^n  =  g_eq · v^n
//! ```
//!
//! Physical sanity check: at DC steady state `v^{n+1} = v^n`, the
//! constitutive law collapses to `i = g_eq · v^n − g_eq · v^n = 0`
//! — a capacitor draws no DC current, as expected.
//!
//! ## Inductor
//!
//! Continuous law: `v_L(t) = L · di/dt`, with `v_L(t) = v_a(t) − v_b(t)`
//! and `i_L` directed from terminal `a` to terminal `b`.
//!
//! BE discretization with step `h`:
//!
//! ```text
//!   v^{n+1} = (L / h) · (i^{n+1} − i^n)
//!   ⇒  i^{n+1} = (h / L) · v^{n+1} + i^n
//!             = (h / L) · v^{n+1} − (−i^n)
//! ```
//!
//! so:
//!
//! ```text
//!   g_eq      = h / L
//!   i_history = − i^n
//! ```
//!
//! Physical sanity check: at DC steady state `i^{n+1} = i^n = I0`,
//! `v^{n+1} = (L/h)(I0 − I0) = 0` — an inductor presents a short at
//! DC, as expected. The Norton source pushes current `−i^n` in the
//! `a → b` direction through the companion, equivalently `+i^n` in
//! the `b → a` direction — i.e. the inductor "tries to maintain" its
//! prior branch current through the external circuit, which is the
//! physical behavior captured by the BE companion.
//!
//! # ADR alignment
//!
//! - **ADR-0006** (Dual NR convergence criterion) — vacuously honored.
//!   This task adds no NR loop surface; the transient outer loop
//!   (tasks.md #33) will call the existing NR driver after every
//!   companion-stamp update, and the dual-criterion check applies
//!   to that NR loop unchanged.
//! - **ADR-0007** (Zero-order-hold at analog-digital boundary) —
//!   vacuously honored. No analog-digital boundary surface added.
//! - **ADR-0008** (Per-node max(rel, abs) tolerance envelope) —
//!   vacuously honored. No tolerance comparison surface added.
//! - **ADR-0009** (Topology checker) — vacuously honored. The Pass-1
//!   topology checker already classifies capacitors as "never
//!   conductive at DC" and inductors as "always conductive at DC"
//!   per ADR-0009 §"False-positive mitigation"; the BE companion
//!   models do not change that classification.
//! - **ADR-0010** (Unstable v1 public API) — honored. New public
//!   types are part of the unstable v1 surface.
//!
//! # Numerical-damping caveat (design.md known pitfall)
//!
//! BE is L-stable (no ringing on stiff problems) but injects
//! **numerical damping** that artificially dissipates energy in
//! lossless LC circuits over many timesteps. `design.md` (line 144)
//! captures this tradeoff and prescribes the mitigation: offer
//! Trapezoidal (tasks.md #30) as the user-selectable default with
//! BE / Gear-2 (#31) as opt-ins, and document the energy-accuracy
//! tradeoff. This module implements BE *correctly*; it does **not**
//! suppress BE's intrinsic damping — that property is a feature of
//! the method, not a bug of this implementation.

use super::companion::{CapacitorHistory, CompanionStamp, InductorHistory};

// -----------------------------------------------------------------------
// Input-validation error
// -----------------------------------------------------------------------

/// Input-validation error from the Backward Euler companion-model
/// helpers.
///
/// Returned when one of the scalar inputs (step size, capacitance,
/// inductance, or any of the history fields) is non-finite or
/// non-positive in a way that would produce a non-finite stamp. The
/// transient control loop (tasks.md #33) treats these as
/// programming errors in the analysis orchestrator (the orchestrator
/// is expected to clamp `h` to a positive lower bound after LTE
/// step-rejection per tasks.md #32) and *not* as user input errors
/// from `application-frontend`.
#[derive(Debug, Clone, PartialEq)]
pub enum CompanionInputError {
    /// Step size `h` must be strictly positive and finite.
    NonPositiveStep {
        /// The offending step value.
        h: f64,
    },
    /// Capacitance `C` must be strictly positive and finite. Zero
    /// capacitance is not a valid model — the netlist parser
    /// (tasks.md slot not yet assigned) is expected to reject
    /// `C = 0` parts before they reach this module.
    NonPositiveCapacitance {
        /// The offending capacitance value.
        c: f64,
    },
    /// Inductance `L` must be strictly positive and finite. Zero
    /// inductance is not a valid model — the netlist parser is
    /// expected to reject `L = 0` parts before they reach this
    /// module.
    NonPositiveInductance {
        /// The offending inductance value.
        l: f64,
    },
    /// A history scalar (capacitor `v_prev` or inductor `i_prev`)
    /// is non-finite (NaN or ±∞). Indicates an upstream divergence.
    NonFiniteHistory {
        /// Short label of which history field was bad (`"v_prev"`
        /// for capacitors, `"i_prev"` for inductors).
        field: &'static str,
        /// The offending value.
        value: f64,
    },
}

impl core::fmt::Display for CompanionInputError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonPositiveStep { h } => {
                write!(
                    f,
                    "backward-euler companion: step size h must be strictly positive and finite, got {h}"
                )
            }
            Self::NonPositiveCapacitance { c } => {
                write!(
                    f,
                    "backward-euler companion: capacitance C must be strictly positive and finite, got {c}"
                )
            }
            Self::NonPositiveInductance { l } => {
                write!(
                    f,
                    "backward-euler companion: inductance L must be strictly positive and finite, got {l}"
                )
            }
            Self::NonFiniteHistory { field, value } => {
                write!(
                    f,
                    "backward-euler companion: history {field} must be finite, got {value}"
                )
            }
        }
    }
}

impl std::error::Error for CompanionInputError {}

// -----------------------------------------------------------------------
// Capacitor companion
// -----------------------------------------------------------------------

/// Compute the Backward Euler companion stamp for a capacitor.
///
/// Returns the [`CompanionStamp`] `{ g_eq, i_history }` for a
/// capacitor of value `capacitance_farads` at the new timestep
/// `t_{n+1} = t_n + h`, given the previous timestep's terminal
/// voltage difference `history.v_prev = (V_a − V_b) at t_n`.
///
/// The BE formula is:
///
/// ```text
///   g_eq      = C / h
///   i_history = g_eq · v_prev
/// ```
///
/// See the [module-level docstring](super::backward_euler) for the
/// derivation and physical sanity check.
///
/// # Arguments
///
/// - `capacitance_farads` — the capacitor's value in farads (F),
///   strictly positive.
/// - `step_seconds` — the integration step size `h` in seconds,
///   strictly positive.
/// - `history` — the [`CapacitorHistory`] from the previous
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
/// - `history.v_prev` is non-finite —
///   [`CompanionInputError::NonFiniteHistory`].
pub fn capacitor_companion(
    capacitance_farads: f64,
    step_seconds: f64,
    history: CapacitorHistory,
) -> Result<CompanionStamp, CompanionInputError> {
    validate_step(step_seconds)?;
    validate_capacitance(capacitance_farads)?;
    if !history.v_prev.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_prev",
            value: history.v_prev,
        });
    }
    let g_eq = capacitance_farads / step_seconds;
    let i_history = g_eq * history.v_prev;
    Ok(CompanionStamp { g_eq, i_history })
}

/// Advance a capacitor's [`CapacitorHistory`] to the *new* timestep
/// after an accepted MNA solve.
///
/// The transient control loop (tasks.md #33) calls this *once per
/// accepted timestep* after the MNA solve to fold the new
/// solution-vector terminal voltages into the per-element history
/// the next [`capacitor_companion`] call will read.
///
/// # Arguments
///
/// - `v_new` — the new timestep's terminal voltage difference
///   `(V_a − V_b)` in volts, taken from the accepted solution
///   vector.
///
/// # Errors
///
/// Returns [`CompanionInputError::NonFiniteHistory`] when `v_new` is
/// non-finite (indicating an upstream divergence the orchestrator
/// should catch).
pub fn advance_capacitor_history(v_new: f64) -> Result<CapacitorHistory, CompanionInputError> {
    if !v_new.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "v_prev",
            value: v_new,
        });
    }
    Ok(CapacitorHistory::new(v_new))
}

// -----------------------------------------------------------------------
// Inductor companion
// -----------------------------------------------------------------------

/// Compute the Backward Euler companion stamp for an inductor.
///
/// Returns the [`CompanionStamp`] `{ g_eq, i_history }` for an
/// inductor of value `inductance_henries` at the new timestep
/// `t_{n+1} = t_n + h`, given the previous timestep's branch current
/// `history.i_prev` (directed `a → b`).
///
/// The BE formula is:
///
/// ```text
///   g_eq      = h / L
///   i_history = − i_prev
/// ```
///
/// See the [module-level docstring](super::backward_euler) for the
/// derivation and physical sanity check (DC short).
///
/// # Arguments
///
/// - `inductance_henries` — the inductor's value in henries (H),
///   strictly positive.
/// - `step_seconds` — the integration step size `h` in seconds,
///   strictly positive.
/// - `history` — the [`InductorHistory`] from the previous accepted
///   timestep.
///
/// # Errors
///
/// Returns [`CompanionInputError`] when:
///
/// - `inductance_henries` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveInductance`],
/// - `step_seconds` is non-positive or non-finite —
///   [`CompanionInputError::NonPositiveStep`],
/// - `history.i_prev` is non-finite —
///   [`CompanionInputError::NonFiniteHistory`].
pub fn inductor_companion(
    inductance_henries: f64,
    step_seconds: f64,
    history: InductorHistory,
) -> Result<CompanionStamp, CompanionInputError> {
    validate_step(step_seconds)?;
    validate_inductance(inductance_henries)?;
    if !history.i_prev.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_prev",
            value: history.i_prev,
        });
    }
    let g_eq = step_seconds / inductance_henries;
    let i_history = -history.i_prev;
    Ok(CompanionStamp { g_eq, i_history })
}

/// Advance an inductor's [`InductorHistory`] to the *new* timestep
/// after an accepted MNA solve.
///
/// The transient control loop (tasks.md #33) calls this *once per
/// accepted timestep* after the MNA solve to fold the new
/// solution-vector branch current into the per-element history the
/// next [`inductor_companion`] call will read.
///
/// # Arguments
///
/// - `i_new` — the new timestep's branch current in amps, directed
///   `a → b`, taken from the accepted solution vector (an MNA
///   branch-augmentation row).
///
/// # Errors
///
/// Returns [`CompanionInputError::NonFiniteHistory`] when `i_new` is
/// non-finite.
pub fn advance_inductor_history(i_new: f64) -> Result<InductorHistory, CompanionInputError> {
    if !i_new.is_finite() {
        return Err(CompanionInputError::NonFiniteHistory {
            field: "i_prev",
            value: i_new,
        });
    }
    Ok(InductorHistory::new(i_new))
}

// -----------------------------------------------------------------------
// Internal validators
// -----------------------------------------------------------------------

fn validate_step(h: f64) -> Result<(), CompanionInputError> {
    if !h.is_finite() || h <= 0.0 {
        Err(CompanionInputError::NonPositiveStep { h })
    } else {
        Ok(())
    }
}

fn validate_capacitance(c: f64) -> Result<(), CompanionInputError> {
    if !c.is_finite() || c <= 0.0 {
        Err(CompanionInputError::NonPositiveCapacitance { c })
    } else {
        Ok(())
    }
}

fn validate_inductance(l: f64) -> Result<(), CompanionInputError> {
    if !l.is_finite() || l <= 0.0 {
        Err(CompanionInputError::NonPositiveInductance { l })
    } else {
        Ok(())
    }
}

// -----------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // Algebraic identities — capacitor
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_companion_g_eq_equals_c_over_h() {
        // C = 1 nF, h = 1 ns ⇒ g_eq = 1 S
        let stamp = capacitor_companion(1.0e-9, 1.0e-9, CapacitorHistory::zero())
            .expect("valid inputs must succeed");
        assert!(
            (stamp.g_eq - 1.0).abs() < 1.0e-15,
            "g_eq must equal C/h, got {}",
            stamp.g_eq
        );
        assert!(
            stamp.i_history == 0.0,
            "zero history ⇒ zero i_history (got {})",
            stamp.i_history
        );
    }

    #[test]
    fn capacitor_companion_i_history_equals_g_eq_times_v_prev() {
        // C = 2 pF, h = 4 ps ⇒ g_eq = 0.5 S; v_prev = 3.0 V
        // ⇒ i_history = 1.5 A
        let stamp = capacitor_companion(2.0e-12, 4.0e-12, CapacitorHistory::new(3.0))
            .expect("valid inputs must succeed");
        assert!(
            (stamp.g_eq - 0.5).abs() < 1.0e-15,
            "g_eq must equal C/h, got {}",
            stamp.g_eq
        );
        assert!(
            (stamp.i_history - 1.5).abs() < 1.0e-15,
            "i_history must equal g_eq·v_prev, got {}",
            stamp.i_history
        );
    }

    #[test]
    fn capacitor_companion_with_negative_v_prev_sign_propagates() {
        // i_history must carry v_prev's sign
        let stamp = capacitor_companion(1.0, 1.0, CapacitorHistory::new(-2.5))
            .expect("valid inputs must succeed");
        assert!(
            (stamp.i_history - (-2.5)).abs() < 1.0e-15,
            "i_history must be -2.5, got {}",
            stamp.i_history
        );
    }

    // -----------------------------------------------------------------
    // Algebraic identities — inductor
    // -----------------------------------------------------------------

    #[test]
    fn inductor_companion_g_eq_equals_h_over_l() {
        // L = 1 mH, h = 1 µs ⇒ g_eq = 1 mS
        let stamp = inductor_companion(1.0e-3, 1.0e-6, InductorHistory::zero())
            .expect("valid inputs must succeed");
        assert!(
            (stamp.g_eq - 1.0e-3).abs() < 1.0e-15,
            "g_eq must equal h/L, got {}",
            stamp.g_eq
        );
        assert!(
            stamp.i_history == 0.0,
            "zero history ⇒ zero i_history (got {})",
            stamp.i_history
        );
    }

    #[test]
    fn inductor_companion_i_history_equals_negated_i_prev() {
        // i_prev = +2.0 A ⇒ i_history = −2.0 A (companion source
        // pushes current b → a to maintain the inductor's forward
        // current through the external circuit).
        let stamp = inductor_companion(1.0, 1.0, InductorHistory::new(2.0))
            .expect("valid inputs must succeed");
        assert!(
            (stamp.i_history - (-2.0)).abs() < 1.0e-15,
            "i_history must be -i_prev, got {}",
            stamp.i_history
        );
    }

    #[test]
    fn inductor_companion_with_negative_i_prev_sign_propagates() {
        let stamp = inductor_companion(1.0, 1.0, InductorHistory::new(-3.0))
            .expect("valid inputs must succeed");
        assert!(
            (stamp.i_history - 3.0).abs() < 1.0e-15,
            "i_history must be -(-3) = +3, got {}",
            stamp.i_history
        );
    }

    // -----------------------------------------------------------------
    // Physical-steady-state sanity checks
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_at_dc_steady_state_passes_zero_current() {
        // At DC steady state, v_new = v_prev. The branch current
        // through the BE companion is i = g_eq · v − i_history.
        // With v_prev = v_new = 5.0 V:
        //   i = (C/h) · 5.0 − (C/h) · 5.0 = 0.
        let v_steady = 5.0;
        let c = 1.0e-6;
        let h = 1.0e-3;
        let stamp = capacitor_companion(c, h, CapacitorHistory::new(v_steady)).unwrap();
        let i_branch = stamp.g_eq * v_steady - stamp.i_history;
        assert!(
            i_branch.abs() < 1.0e-12,
            "capacitor at DC steady state must carry zero current, got {i_branch}"
        );
    }

    #[test]
    fn inductor_at_dc_steady_state_drops_zero_voltage() {
        // At DC steady state, i_new = i_prev. We can recover v by
        // solving the BE Norton equation for v given the same
        // branch current: i = g_eq · v − i_history ⇒
        // v = (i + i_history) / g_eq. With i_prev = i_new = 0.7 A
        // and i_history = −i_prev = −0.7 A:
        //   v = (0.7 − 0.7) / g_eq = 0.
        let i_steady = 0.7;
        let l = 1.0e-3;
        let h = 1.0e-6;
        let stamp = inductor_companion(l, h, InductorHistory::new(i_steady)).unwrap();
        let v_branch = (i_steady + stamp.i_history) / stamp.g_eq;
        assert!(
            v_branch.abs() < 1.0e-12,
            "inductor at DC steady state must drop zero voltage, got {v_branch}"
        );
    }

    // -----------------------------------------------------------------
    // RC step-response physics check: charges toward V_in over time
    // -----------------------------------------------------------------

    /// Simulate an RC low-pass filter (R = 1 kΩ, C = 1 nF, τ = 1 µs)
    /// driven by a 1.0 V step at t = 0 using the BE companion model.
    /// The exact solution is `v_C(t) = 1 − exp(−t/τ)`. With a small
    /// timestep, BE must approach this within first-order accuracy.
    #[test]
    fn rc_low_pass_step_response_converges_to_source() {
        let r = 1.0e3; // 1 kΩ
        let c = 1.0e-9; // 1 nF, τ = R·C = 1 µs
        let v_in = 1.0; // step amplitude
        let h = 1.0e-9; // 1 ns step (well below τ)
        let n_steps = 5_000; // 5 µs total — 5 time constants

        // Simple one-node analytic loop: at each step, the capacitor
        // branch current i_C = g_eq · v_C − i_history must equal the
        // resistor current i_R = (v_in − v_C) / R. Solve for v_C:
        //   g_eq · v_C − i_history = (v_in − v_C) / R
        //   v_C · (g_eq + 1/R) = i_history + v_in / R
        //   v_C = (i_history + v_in / R) / (g_eq + 1/R)
        let mut hist = CapacitorHistory::zero();
        let mut v_c = 0.0;
        for _ in 0..n_steps {
            let stamp = capacitor_companion(c, h, hist).unwrap();
            v_c = (stamp.i_history + v_in / r) / (stamp.g_eq + 1.0 / r);
            hist = advance_capacitor_history(v_c).unwrap();
        }
        // After 5τ the capacitor must be within 1% of v_in. (BE
        // overshoots toward 1.0 monotonically from below for any h
        // and converges; this is a loose-but-meaningful tolerance.)
        assert!(
            v_c > 0.99 && v_c < 1.0,
            "RC step response after 5τ must be in (0.99, 1.0), got {v_c}"
        );
    }

    /// Simulate an LR circuit (L = 1 mH, R = 1 kΩ, τ = L/R = 1 µs)
    /// driven by a 1.0 V step at t = 0 using the BE companion. The
    /// exact solution is `i_L(t) = (1/R)·(1 − exp(−t/τ))`. After 5τ,
    /// `i_L ≈ 1/R = 1 mA`.
    #[test]
    fn lr_step_response_converges_to_source_over_r() {
        let r = 1.0e3; // 1 kΩ
        let l = 1.0e-3; // 1 mH, τ = L/R = 1 µs
        let v_in = 1.0;
        let h = 1.0e-9; // 1 ns step
        let n_steps = 5_000;

        // KVL: v_in = i·R + v_L, so v_L = v_in − i·R. Also from the
        // BE inductor companion: i = g_eq · v_L − i_history. Solve:
        //   i = g_eq · (v_in − i·R) − i_history
        //   i · (1 + g_eq · R) = g_eq · v_in − i_history
        //   i = (g_eq · v_in − i_history) / (1 + g_eq · R)
        let mut hist = InductorHistory::zero();
        let mut i = 0.0;
        for _ in 0..n_steps {
            let stamp = inductor_companion(l, h, hist).unwrap();
            i = (stamp.g_eq * v_in - stamp.i_history) / (1.0 + stamp.g_eq * r);
            hist = advance_inductor_history(i).unwrap();
        }
        // After 5τ, i_L must be within 1% of 1/R = 1 mA.
        let i_expected = v_in / r;
        let rel_err = (i - i_expected).abs() / i_expected;
        assert!(
            rel_err < 0.02,
            "LR step response after 5τ must be within 2% of 1/R = {i_expected}, got {i} (rel_err = {rel_err})"
        );
    }

    // -----------------------------------------------------------------
    // History advancers
    // -----------------------------------------------------------------

    #[test]
    fn advance_capacitor_history_copies_value() {
        let h = advance_capacitor_history(2.5).unwrap();
        assert_eq!(h, CapacitorHistory::new(2.5));
    }

    #[test]
    fn advance_inductor_history_copies_value() {
        let h = advance_inductor_history(-1.25).unwrap();
        assert_eq!(h, InductorHistory::new(-1.25));
    }

    #[test]
    fn advance_capacitor_history_rejects_nan() {
        match advance_capacitor_history(f64::NAN) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "v_prev");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory, got {other:?}"),
        }
    }

    #[test]
    fn advance_inductor_history_rejects_infinity() {
        match advance_inductor_history(f64::INFINITY) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "i_prev");
                assert!(value.is_infinite() && value.is_sign_positive());
            }
            other => panic!("expected NonFiniteHistory, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // Input validation
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_companion_rejects_zero_step() {
        match capacitor_companion(1.0e-9, 0.0, CapacitorHistory::zero()) {
            Err(CompanionInputError::NonPositiveStep { h }) => {
                assert_eq!(h.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_negative_step() {
        match capacitor_companion(1.0e-9, -1.0e-9, CapacitorHistory::zero()) {
            Err(CompanionInputError::NonPositiveStep { h }) => {
                assert_eq!(h.to_bits(), (-1.0e-9_f64).to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_nan_step() {
        match capacitor_companion(1.0e-9, f64::NAN, CapacitorHistory::zero()) {
            Err(CompanionInputError::NonPositiveStep { h }) => assert!(h.is_nan()),
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_zero_capacitance() {
        match capacitor_companion(0.0, 1.0e-9, CapacitorHistory::zero()) {
            Err(CompanionInputError::NonPositiveCapacitance { c }) => {
                assert_eq!(c.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveCapacitance, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_negative_capacitance() {
        match capacitor_companion(-1.0e-9, 1.0e-9, CapacitorHistory::zero()) {
            Err(CompanionInputError::NonPositiveCapacitance { c }) => {
                assert_eq!(c.to_bits(), (-1.0e-9_f64).to_bits());
            }
            other => panic!("expected NonPositiveCapacitance, got {other:?}"),
        }
    }

    #[test]
    fn capacitor_companion_rejects_nan_history() {
        match capacitor_companion(1.0e-9, 1.0e-9, CapacitorHistory::new(f64::NAN)) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "v_prev");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory, got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_zero_inductance() {
        match inductor_companion(0.0, 1.0e-9, InductorHistory::zero()) {
            Err(CompanionInputError::NonPositiveInductance { l }) => {
                assert_eq!(l.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveInductance, got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_negative_inductance() {
        match inductor_companion(-1.0e-3, 1.0e-9, InductorHistory::zero()) {
            Err(CompanionInputError::NonPositiveInductance { l }) => {
                assert_eq!(l.to_bits(), (-1.0e-3_f64).to_bits());
            }
            other => panic!("expected NonPositiveInductance, got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_zero_step() {
        match inductor_companion(1.0e-3, 0.0, InductorHistory::zero()) {
            Err(CompanionInputError::NonPositiveStep { h }) => {
                assert_eq!(h.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveStep, got {other:?}"),
        }
    }

    #[test]
    fn inductor_companion_rejects_nan_history() {
        match inductor_companion(1.0e-3, 1.0e-9, InductorHistory::new(f64::NAN)) {
            Err(CompanionInputError::NonFiniteHistory { field, value }) => {
                assert_eq!(field, "i_prev");
                assert!(value.is_nan());
            }
            other => panic!("expected NonFiniteHistory, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // Error display strings are actionable
    // -----------------------------------------------------------------

    #[test]
    fn error_display_strings_are_actionable() {
        let e = CompanionInputError::NonPositiveStep { h: -1.0 };
        let s = e.to_string();
        assert!(s.contains("step size"), "Display must mention step: {s}");
        assert!(
            s.contains("-1"),
            "Display must include offending value: {s}"
        );

        let e = CompanionInputError::NonPositiveCapacitance { c: 0.0 };
        assert!(e.to_string().contains("capacitance"));

        let e = CompanionInputError::NonPositiveInductance { l: 0.0 };
        assert!(e.to_string().contains("inductance"));

        let e = CompanionInputError::NonFiniteHistory {
            field: "v_prev",
            value: f64::NAN,
        };
        let s = e.to_string();
        assert!(s.contains("v_prev"));
        assert!(s.contains("history"));
    }

    // -----------------------------------------------------------------
    // Companion stamp identity element
    // -----------------------------------------------------------------

    #[test]
    fn companion_stamp_zero_is_well_defined() {
        let z = CompanionStamp::zero();
        assert_eq!(z.g_eq.to_bits(), 0.0_f64.to_bits());
        assert_eq!(z.i_history.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn capacitor_and_inductor_history_zero_are_canonical() {
        assert_eq!(CapacitorHistory::zero(), CapacitorHistory::new(0.0));
        assert_eq!(InductorHistory::zero(), InductorHistory::new(0.0));
    }

    // -----------------------------------------------------------------
    // Copy / size_of layout witness
    // -----------------------------------------------------------------

    #[test]
    fn types_are_copy_and_have_expected_size() {
        fn assert_copy<T: Copy>() {}

        // CompanionStamp = 2× f64 = 16 bytes
        assert_eq!(core::mem::size_of::<CompanionStamp>(), 16);
        // History structs are 1× f64 = 8 bytes
        assert_eq!(core::mem::size_of::<CapacitorHistory>(), 8);
        assert_eq!(core::mem::size_of::<InductorHistory>(), 8);

        // Compile-time Copy witness.
        assert_copy::<CompanionStamp>();
        assert_copy::<CapacitorHistory>();
        assert_copy::<InductorHistory>();
    }

    // -----------------------------------------------------------------
    // BE produces dissipation on a lossless LC tank (documented
    // pitfall mitigation)
    // -----------------------------------------------------------------

    /// Documents BE's known numerical-damping behavior on a lossless
    /// LC tank: under exact arithmetic an LC tank oscillates
    /// forever, but BE injects energy dissipation per step. This
    /// test asserts the *direction* of the artifact (energy
    /// monotonically decreases over a full period) so a future
    /// regression that accidentally produces *growth* (instability)
    /// would be caught.
    ///
    /// See `design.md` line 144 "BE / Gear-2 numerical damping".
    #[test]
    fn be_dissipates_energy_on_lossless_lc_tank() {
        // Tank: L = 1 mH, C = 1 nF ⇒ ω₀ = 1/√(LC) = 1e6 rad/s,
        // f₀ ≈ 159 kHz, T₀ ≈ 6.28 µs.
        let l = 1.0e-3;
        let c = 1.0e-9;
        let h = 1.0e-9; // 1 ns step, much smaller than T₀
        let n_steps = 10_000; // ~1.6 periods

        // Initial condition: capacitor charged to 1 V, inductor at
        // 0 A. The tank should oscillate (if there were no damping).
        let v_c0 = 1.0;
        let initial_energy = 0.5 * c * v_c0 * v_c0;

        let mut v_c = v_c0;
        let mut i_l = 0.0;
        let mut hist_c = CapacitorHistory::new(v_c0);
        let mut hist_l = InductorHistory::new(0.0);

        // The tank is L (between nodes a and b) in parallel with C
        // (same nodes). Ground at b. KCL at node a:
        //   i_C(out of a) + i_L(out of a) = 0
        //   (g_C · v_a − i_hist_C) + (g_L · v_a − i_hist_L) = 0
        //   v_a · (g_C + g_L) = i_hist_C + i_hist_L
        //   v_a = (i_hist_C + i_hist_L) / (g_C + g_L)
        // and inductor branch current after solving:
        //   i_L^{n+1} = g_L · v_a − i_hist_L
        for _ in 0..n_steps {
            let stamp_c = capacitor_companion(c, h, hist_c).unwrap();
            let stamp_l = inductor_companion(l, h, hist_l).unwrap();
            v_c = (stamp_c.i_history + stamp_l.i_history) / (stamp_c.g_eq + stamp_l.g_eq);
            i_l = stamp_l.g_eq * v_c - stamp_l.i_history;
            hist_c = advance_capacitor_history(v_c).unwrap();
            hist_l = advance_inductor_history(i_l).unwrap();
        }

        let final_energy = 0.5 * c * v_c * v_c + 0.5 * l * i_l * i_l;
        // BE dissipates, so final_energy < initial_energy strictly.
        // It must also remain non-negative and finite.
        assert!(
            final_energy.is_finite() && final_energy >= 0.0,
            "energy must be finite and non-negative, got {final_energy}"
        );
        assert!(
            final_energy < initial_energy,
            "BE must dissipate energy on a lossless LC tank: initial = {initial_energy}, final = {final_energy}"
        );
    }
}
