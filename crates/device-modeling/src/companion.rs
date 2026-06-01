//! Reactive-element companion-model stamps (tasks.md #30).
//!
//! This module owns the per-timestep linearization of *reactive*
//! (energy-storing) two-terminal elements — `Capacitor` and `Inductor`
//! — under a chosen numerical integration method. The result is a
//! [`ReactiveCompanion`] stamp: a small-signal admittance matrix plus
//! a companion current vector, in the same terminal-local shape the
//! `LinearizedModel` of `stamp.rs` exposes for nonlinear devices. The
//! `numeric-solver` MNA assembler folds both kinds of stamp into the
//! global system uniformly (tasks.md #14).
//!
//! # Scope of this task (#30)
//!
//! Per `openspec/changes/circuit-solver-2026-05-21-v1-spec/tasks.md`
//! the slice landing in this task is the **Trapezoidal** companion
//! for the two reactive primitives recognised by the netlist-graph
//! context's [`ElementKind`][element_kind]:
//!
//! - `ElementKind::Capacitor { capacitance_farads }`,
//! - `ElementKind::Inductor { inductance_henries }`.
//!
//! The Backward-Euler companion (tasks.md #29) and Gear-2 BDF
//! companion (tasks.md #31) are *sibling* tasks; they will live in the
//! same module as additional constructors on these types so that the
//! transient control loop (tasks.md #33) selects by integration method
//! without learning a new stamp shape.
//!
//! [element_kind]: ../../../netlist-graph/src/element.rs
//!
//! # The Trapezoidal Rule (TR)
//!
//! TR is the second-order implicit linear multistep method that
//! averages Forward-Euler and Backward-Euler. For an autonomous ODE
//! `dx/dt = f(x, t)`, TR discretises as
//!
//! ```text
//! x_n − x_{n−1} = (h / 2) · ( f(x_n, t_n) + f(x_{n−1}, t_{n−1}) )
//! ```
//!
//! Applied to the two reactive primitives this yields a *Norton
//! equivalent* — a parallel conductance and an independent current
//! source whose values are computed from the previous step's state.
//! The same shape covers capacitor and inductor; only the formulas
//! that populate the conductance and current source differ. See the
//! `wiki/concepts/trapezoidal-rule.md` page for the textbook
//! derivation; the per-element form lives below.
//!
//! ## Capacitor companion (TR)
//!
//! Constitutive law: `i_C = C · d v_C / d t`, where `v_C = v_p − v_m`
//! is the voltage from the *positive* to the *negative* terminal.
//!
//! Trapezoidal discretisation:
//!
//! ```text
//! i_n + i_{n−1}   v_n − v_{n−1}
//! ───────────── = C · ─────────────────
//!       2                h
//! ```
//!
//! Rearrange to put `i_n` as an affine function of `v_n`:
//!
//! ```text
//! i_n = (2C / h) · v_n − [ (2C / h) · v_{n−1} + i_{n−1} ]
//!     = G_eq    · v_n − I_eq
//! ```
//!
//! with:
//!
//! - **Equivalent conductance** `G_eq = 2C / h`,
//! - **Companion current**       `I_eq = G_eq · v_{n−1} + i_{n−1}`.
//!
//! Stamped into MNA (terminals `[p, m]`, indices `0` and `1`):
//!
//! ```text
//! G_stamp = [ [  G_eq, −G_eq ],
//!             [ −G_eq,  G_eq ] ]
//!
//! I_companion = [ +I_eq, −I_eq ]
//! ```
//!
//! The current-vector sign convention follows MNA's RHS contribution:
//! `companion_current[k]` is the current flowing *out of* node-of-`k`
//! into the device companion model (so the positive terminal sources
//! `+I_eq` into the device and the negative terminal sinks the same
//! current).
//!
//! ## Inductor companion (TR)
//!
//! Constitutive law: `v_L = L · d i_L / d t`. The TR discretisation
//! mirrors the capacitor case with `(v, i)` and `(L, 1/C)` swapped,
//! producing a *Norton* form so the assembler sees the same
//! `(jacobian, companion_current)` shape:
//!
//! ```text
//! v_n + v_{n−1}      i_n − i_{n−1}
//! ───────────── = L · ─────────────────
//!       2                  h
//! ```
//!
//! Solving for `i_n`:
//!
//! ```text
//! i_n = (h / 2L) · v_n + [ (h / 2L) · v_{n−1} + i_{n−1} ]
//!     = G_eq    · v_n + I_eq_inductor
//! ```
//!
//! Note the **sign** on the companion current is opposite the
//! capacitor: the inductor's previous-step current adds to (not
//! subtracts from) the next-step current. To keep the stamp's RHS
//! sign convention identical across families we expose the inductor
//! companion as
//!
//! - **Equivalent conductance** `G_eq = h / (2L)`,
//! - **Companion current**       `I_eq = −[ G_eq · v_{n−1} + i_{n−1} ]`,
//!
//! and the assembler subtracts `companion_current[k]` from the RHS in
//! the same direction as for the capacitor (and for the diode and
//! BJT). The sign of `I_eq` carries the device polarity.
//!
//! The 2×2 conductance Jacobian has the same anti-symmetric pattern:
//! `[[+G_eq, −G_eq], [−G_eq, +G_eq]]`, expressing the fact that the
//! reactive element's terminal currents sum to zero (KCL is enforced
//! at the stamp level, not just at the assembler).
//!
//! # Terminal-local coordinates
//!
//! Reactive companions are expressed in *terminal-local* coordinates,
//! never in graph node identifiers. The two terminals are
//!
//! - **Index 0** — the *positive* terminal (`p`), which in the
//!   `netlist-graph` context is `Element.terminals[0]`,
//! - **Index 1** — the *negative* terminal (`m`), which is
//!   `Element.terminals[1]`.
//!
//! This boundary parallels the convention chosen for nonlinear
//! `LinearizedModel`s in `stamp.rs`: device-modeling never learns
//! about graph topology; the MNA assembler maps terminal indices to
//! `NodeId` through the `FlattenedStructure` (tasks.md #6).
//!
//! # ADR alignment
//!
//! - **ADR-0005 (closed-enum dispatch).** [`ReactiveCompanion`] is a
//!   closed enum with one variant per reactive primitive
//!   (`Capacitor`, `Inductor`). Adding a future element kind that
//!   requires a companion stamp (e.g. a coupled-inductor mutual term)
//!   forces every `match` site to be updated.
//! - **ADR-0010 (unstable v1 surface).** Every type here is part of
//!   the v1-unstable Rust API. Consumers must pin to exact versions.
//!
//! # Numerical robustness
//!
//! The trapezoidal stamp depends on `1 / h` (capacitor) and `h`
//! (inductor); for `h = 0` both produce a degenerate stamp.
//! Constructors return [`CompanionConstructionError`] on a
//! non-positive timestep or on a non-positive element value rather
//! than panicking, so callers can surface the contract violation as a
//! solver-level diagnostic without crashing.

use core::fmt;

/// Number of terminals on a two-terminal reactive element.
///
/// Matches the SPICE convention for `C` and `L` cards: terminal 0 is
/// the *positive* (top) terminal, terminal 1 is the *negative*
/// (bottom) terminal.
pub const REACTIVE_TERMINALS: usize = 2;

// ---------------------------------------------------------------------
// ReactiveState — previous-step state handed in by the transient loop
// ---------------------------------------------------------------------

/// State of a reactive element at the *previous* committed timestep,
/// required to build its next-step trapezoidal companion.
///
/// Sign convention matches MNA's branch orientation: `voltage_volts`
/// is `v_p − v_m` (positive minus negative terminal) and
/// `current_amperes` is the current flowing into the *positive*
/// terminal of the device companion model.
///
/// The transient control loop (tasks.md #33) is responsible for
/// caching this state per reactive element between timesteps. At
/// `t = 0` with no UIC the loop seeds both fields from the DC
/// operating-point solution; with UIC the user-supplied initial
/// conditions populate the seed.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ReactiveState {
    /// `v_{n−1} = v_p − v_m` at the previously accepted timestep.
    pub voltage_volts: f64,
    /// `i_{n−1}`, current into the positive terminal at the
    /// previously accepted timestep.
    pub current_amperes: f64,
}

impl ReactiveState {
    /// State of a freshly-initialised reactive element with zero
    /// initial conditions (`v = 0`, `i = 0`). Useful as a seed when
    /// no DC operating point exists and no UIC is supplied.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            voltage_volts: 0.0,
            current_amperes: 0.0,
        }
    }
}

// ---------------------------------------------------------------------
// Error type — surfaced rather than panicking on bad input
// ---------------------------------------------------------------------

/// Reasons a reactive companion constructor may refuse to build a
/// stamp.
///
/// Construction is a contract-checking step: the transient control
/// loop guarantees `h > 0` and the netlist parser guarantees
/// `C > 0`, `L > 0`. A failure here therefore indicates a bug
/// upstream (a degenerate timestep, a zero-valued reactive
/// component), not user input, and the loop surfaces it as a
/// diagnostic rather than crashing.
#[derive(Debug, Clone, PartialEq)]
pub enum CompanionConstructionError {
    /// Timestep `h` was not strictly positive (zero, negative, or
    /// non-finite). The trapezoidal stamp divides by `h` (capacitor)
    /// or multiplies by `h` (inductor); a non-positive timestep is
    /// always a contract violation by the transient loop.
    NonPositiveTimestep {
        /// The offending value as supplied.
        timestep_seconds: f64,
    },
    /// The reactive element value (`C` for a capacitor, `L` for an
    /// inductor) was not strictly positive. The netlist-graph
    /// builder is responsible for rejecting non-positive values at
    /// parse time; a violation here means an upstream invariant
    /// slipped through.
    NonPositiveElementValue {
        /// The two-character SPICE letter for the element family
        /// that failed (`"C"` for capacitor, `"L"` for inductor).
        family: &'static str,
        /// The offending value as supplied.
        value: f64,
    },
}

impl fmt::Display for CompanionConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveTimestep { timestep_seconds } => write!(
                f,
                "trapezoidal companion: non-positive timestep h = {timestep_seconds}; \
                 the transient loop must supply h > 0"
            ),
            Self::NonPositiveElementValue { family, value } => write!(
                f,
                "trapezoidal companion: non-positive {family} value {value}; \
                 the netlist must supply a strictly positive reactive value"
            ),
        }
    }
}

impl std::error::Error for CompanionConstructionError {}

// ---------------------------------------------------------------------
// CapacitorCompanion — Norton form, terminal-local
// ---------------------------------------------------------------------

/// Capacitor 2-terminal companion stamp.
///
/// Two-terminal Norton equivalent of a linear capacitor under a
/// chosen integration method. The MNA assembler stamps `jacobian`
/// into the conductance matrix and `companion_current` into the
/// right-hand-side, identically to the nonlinear-device
/// `DiodeLinearization` in `stamp.rs`.
///
/// See the module docstring for the trapezoidal derivation. Other
/// integration methods (Backward Euler — tasks.md #29, Gear-2 BDF —
/// tasks.md #31) populate the same struct shape via additional
/// constructor functions; only the formulas for `G_eq` and `I_eq`
/// differ between methods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CapacitorCompanion {
    /// 2×2 conductance Jacobian indexed `[positive, negative]`.
    ///
    /// The matrix has the anti-symmetric KCL-conserving pattern
    /// `[[+G, −G], [−G, +G]]` where `G = G_eq` is the equivalent
    /// conductance.
    pub jacobian: [[f64; REACTIVE_TERMINALS]; REACTIVE_TERMINALS],

    /// 2-vector of companion currents indexed `[positive, negative]`.
    ///
    /// Pattern is `[+I, −I]` where `I = I_eq` (positive into the
    /// positive-terminal node, equal magnitude out of the
    /// negative-terminal node).
    pub companion_current: [f64; REACTIVE_TERMINALS],
}

impl CapacitorCompanion {
    /// All-zero companion (no stamp contribution).
    ///
    /// Useful as the seed at `t = 0` with zero initial conditions
    /// and for property-based tests that need a neutral element.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            jacobian: [[0.0; REACTIVE_TERMINALS]; REACTIVE_TERMINALS],
            companion_current: [0.0; REACTIVE_TERMINALS],
        }
    }

    /// Equivalent conductance recovered from the jacobian's `[0][0]`
    /// entry.
    ///
    /// Provided so tests and the MNA assembler can recover `G_eq`
    /// without re-deriving it. By construction
    /// `jacobian[0][0] == G_eq` and `jacobian[0][1] == −G_eq`.
    #[must_use]
    pub fn equivalent_conductance(&self) -> f64 {
        self.jacobian[0][0]
    }

    /// Equivalent (Norton) companion current recovered from the
    /// `companion_current[0]` entry.
    ///
    /// By construction `companion_current[0] == +I_eq` and
    /// `companion_current[1] == −I_eq`.
    #[must_use]
    pub fn equivalent_current(&self) -> f64 {
        self.companion_current[0]
    }

    /// Build a capacitor companion under the **Trapezoidal Rule**
    /// (tasks.md #30).
    ///
    /// # Math
    ///
    /// `G_eq = 2C / h`,
    /// `I_eq = G_eq · v_{n−1} + i_{n−1}`.
    ///
    /// See the module docstring for the full derivation.
    ///
    /// # Arguments
    ///
    /// - `capacitance_farads` — the element value `C` from the
    ///   `netlist-graph::ElementKind::Capacitor` instance. Must be
    ///   strictly positive.
    /// - `timestep_seconds` — `h`, the proposed next-timestep length
    ///   in seconds. Must be strictly positive.
    /// - `state` — the previous-timestep state cached by the
    ///   transient control loop.
    ///
    /// # Errors
    ///
    /// - [`CompanionConstructionError::NonPositiveTimestep`] if
    ///   `timestep_seconds <= 0` or non-finite.
    /// - [`CompanionConstructionError::NonPositiveElementValue`] if
    ///   `capacitance_farads <= 0` or non-finite.
    pub fn trapezoidal(
        capacitance_farads: f64,
        timestep_seconds: f64,
        state: ReactiveState,
    ) -> Result<Self, CompanionConstructionError> {
        check_timestep(timestep_seconds)?;
        check_element_value("C", capacitance_farads)?;

        let g_eq = 2.0 * capacitance_farads / timestep_seconds;
        let i_eq = g_eq * state.voltage_volts + state.current_amperes;

        Ok(Self {
            jacobian: [[g_eq, -g_eq], [-g_eq, g_eq]],
            companion_current: [i_eq, -i_eq],
        })
    }
}

// ---------------------------------------------------------------------
// InductorCompanion — Norton form, terminal-local
// ---------------------------------------------------------------------

/// Inductor 2-terminal companion stamp.
///
/// Norton equivalent of a linear inductor under a chosen integration
/// method. Shape and sign convention are identical to
/// [`CapacitorCompanion`]; the polarity difference between the two
/// elements is folded into `companion_current` so the MNA assembler
/// treats them uniformly.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InductorCompanion {
    /// 2×2 conductance Jacobian indexed `[positive, negative]`.
    pub jacobian: [[f64; REACTIVE_TERMINALS]; REACTIVE_TERMINALS],
    /// 2-vector of companion currents indexed `[positive, negative]`.
    pub companion_current: [f64; REACTIVE_TERMINALS],
}

impl InductorCompanion {
    /// All-zero companion (no stamp contribution).
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            jacobian: [[0.0; REACTIVE_TERMINALS]; REACTIVE_TERMINALS],
            companion_current: [0.0; REACTIVE_TERMINALS],
        }
    }

    /// Equivalent conductance recovered from `jacobian[0][0]`.
    #[must_use]
    pub fn equivalent_conductance(&self) -> f64 {
        self.jacobian[0][0]
    }

    /// Equivalent (Norton) companion current recovered from
    /// `companion_current[0]`.
    #[must_use]
    pub fn equivalent_current(&self) -> f64 {
        self.companion_current[0]
    }

    /// Build an inductor companion under the **Trapezoidal Rule**
    /// (tasks.md #30).
    ///
    /// # Math
    ///
    /// `G_eq = h / (2L)`,
    /// `I_eq = −[ G_eq · v_{n−1} + i_{n−1} ]`.
    ///
    /// The sign on the companion current is the inverse of the
    /// capacitor's. See the module docstring.
    ///
    /// # Arguments
    ///
    /// - `inductance_henries` — the element value `L`. Must be
    ///   strictly positive.
    /// - `timestep_seconds` — `h`. Must be strictly positive.
    /// - `state` — the previous-timestep state.
    ///
    /// # Errors
    ///
    /// See [`CapacitorCompanion::trapezoidal`].
    pub fn trapezoidal(
        inductance_henries: f64,
        timestep_seconds: f64,
        state: ReactiveState,
    ) -> Result<Self, CompanionConstructionError> {
        check_timestep(timestep_seconds)?;
        check_element_value("L", inductance_henries)?;

        let g_eq = timestep_seconds / (2.0 * inductance_henries);
        let i_eq = -(g_eq * state.voltage_volts + state.current_amperes);

        Ok(Self {
            jacobian: [[g_eq, -g_eq], [-g_eq, g_eq]],
            companion_current: [i_eq, -i_eq],
        })
    }
}

// ---------------------------------------------------------------------
// ReactiveCompanion — family-tagged enum for the assembler
// ---------------------------------------------------------------------

/// Family-tagged companion stamp returned to the MNA assembler.
///
/// The assembler matches on the variant to learn the element family,
/// then folds `jacobian` and `companion_current` into the global MNA
/// system. The match is exhaustive (ADR-0005) — adding a future
/// reactive primitive (e.g. a coupled-inductor mutual term) forces
/// every site to be updated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReactiveCompanion {
    /// Capacitor 2-terminal companion.
    Capacitor(CapacitorCompanion),
    /// Inductor 2-terminal companion.
    Inductor(InductorCompanion),
}

impl ReactiveCompanion {
    /// Terminal count contributed by this stamp.
    ///
    /// Both reactive primitives are two-terminal — this accessor
    /// exists so the assembler's stamp loop can be written without
    /// hard-coding the constant.
    #[must_use]
    pub const fn terminal_count(&self) -> usize {
        match self {
            Self::Capacitor(_) | Self::Inductor(_) => REACTIVE_TERMINALS,
        }
    }

    /// The 2×2 conductance Jacobian, irrespective of family.
    #[must_use]
    pub fn jacobian(&self) -> [[f64; REACTIVE_TERMINALS]; REACTIVE_TERMINALS] {
        match self {
            Self::Capacitor(c) => c.jacobian,
            Self::Inductor(l) => l.jacobian,
        }
    }

    /// The 2-vector companion current, irrespective of family.
    #[must_use]
    pub fn companion_current(&self) -> [f64; REACTIVE_TERMINALS] {
        match self {
            Self::Capacitor(c) => c.companion_current,
            Self::Inductor(l) => l.companion_current,
        }
    }
}

// ---------------------------------------------------------------------
// Internal validators
// ---------------------------------------------------------------------

fn check_timestep(timestep_seconds: f64) -> Result<(), CompanionConstructionError> {
    if !timestep_seconds.is_finite() || timestep_seconds <= 0.0 {
        Err(CompanionConstructionError::NonPositiveTimestep { timestep_seconds })
    } else {
        Ok(())
    }
}

fn check_element_value(family: &'static str, value: f64) -> Result<(), CompanionConstructionError> {
    if !value.is_finite() || value <= 0.0 {
        Err(CompanionConstructionError::NonPositiveElementValue { family, value })
    } else {
        Ok(())
    }
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // Cross-cutting invariants (apply to both reactive families).
    // -----------------------------------------------------------------

    #[test]
    fn reactive_terminals_is_two() {
        assert_eq!(REACTIVE_TERMINALS, 2);
    }

    #[test]
    fn zero_companions_have_no_contribution() {
        let c = CapacitorCompanion::zero();
        assert!(c.jacobian.iter().flatten().all(|x| *x == 0.0));
        assert!(c.companion_current.iter().all(|x| *x == 0.0));

        let l = InductorCompanion::zero();
        assert!(l.jacobian.iter().flatten().all(|x| *x == 0.0));
        assert!(l.companion_current.iter().all(|x| *x == 0.0));
    }

    // -----------------------------------------------------------------
    // CapacitorCompanion — Trapezoidal math.
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_trapezoidal_g_eq_matches_2c_over_h() {
        // C = 1 nF, h = 1 ns => G_eq = 2 · 1e−9 / 1e−9 = 2 S
        let c = CapacitorCompanion::trapezoidal(1e-9, 1e-9, ReactiveState::zero()).unwrap();
        let g = c.equivalent_conductance();
        assert!((g - 2.0).abs() < 1e-12, "expected G_eq = 2 S, got {g}");
    }

    #[test]
    fn capacitor_trapezoidal_jacobian_is_kcl_conserving() {
        // Each row must sum to zero (KCL at the stamp level).
        let c = CapacitorCompanion::trapezoidal(2.5e-9, 0.5e-9, ReactiveState::zero()).unwrap();
        let row0 = c.jacobian[0][0] + c.jacobian[0][1];
        let row1 = c.jacobian[1][0] + c.jacobian[1][1];
        assert!(row0.abs() < 1e-12, "row 0 sum = {row0}, expected 0");
        assert!(row1.abs() < 1e-12, "row 1 sum = {row1}, expected 0");
    }

    #[test]
    fn capacitor_trapezoidal_companion_current_is_anti_symmetric() {
        // The two terminals see equal and opposite companion current.
        let c = CapacitorCompanion::trapezoidal(
            1e-12,
            1e-9,
            ReactiveState {
                voltage_volts: 1.2,
                current_amperes: 3.4e-3,
            },
        )
        .unwrap();
        assert!(
            (c.companion_current[0] + c.companion_current[1]).abs() < 1e-15,
            "expected anti-symmetry, got {:?}",
            c.companion_current
        );
    }

    #[test]
    fn capacitor_trapezoidal_zero_initial_state_zero_companion() {
        // With v_{n−1} = 0 and i_{n−1} = 0, the companion current
        // must be zero — there is no stored energy to project forward.
        let c = CapacitorCompanion::trapezoidal(1e-9, 1e-9, ReactiveState::zero()).unwrap();
        assert!(c.companion_current.iter().all(|x| *x == 0.0));
        // But the jacobian still carries G_eq (a capacitor's
        // small-signal admittance is non-zero even at rest).
        assert!(c.jacobian[0][0] > 0.0);
    }

    #[test]
    fn capacitor_trapezoidal_companion_current_formula() {
        // Closed-form check: I_eq = G_eq · v_{n−1} + i_{n−1}.
        let c_val = 4e-9;
        let h = 2e-9;
        let v_prev = 0.75;
        let i_prev = 1.5e-3;
        let c = CapacitorCompanion::trapezoidal(
            c_val,
            h,
            ReactiveState {
                voltage_volts: v_prev,
                current_amperes: i_prev,
            },
        )
        .unwrap();
        let g_expected = 2.0 * c_val / h;
        let i_expected = g_expected * v_prev + i_prev;
        assert!(
            (c.equivalent_conductance() - g_expected).abs() < 1e-12,
            "G_eq mismatch: got {}, expected {g_expected}",
            c.equivalent_conductance()
        );
        assert!(
            (c.equivalent_current() - i_expected).abs() < 1e-9 * i_expected.abs().max(1.0),
            "I_eq mismatch: got {}, expected {i_expected}",
            c.equivalent_current()
        );
    }

    // -----------------------------------------------------------------
    // CapacitorCompanion — DC limit & monotonicity.
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_trapezoidal_g_eq_grows_unboundedly_as_h_shrinks() {
        // Halving the timestep doubles G_eq — this is the property
        // that gives TR its O(h^3) per-step truncation error: the
        // discrete operator approaches the continuous derivative.
        let g_h1 = CapacitorCompanion::trapezoidal(1e-9, 1e-9, ReactiveState::zero())
            .unwrap()
            .equivalent_conductance();
        let g_h2 = CapacitorCompanion::trapezoidal(1e-9, 0.5e-9, ReactiveState::zero())
            .unwrap()
            .equivalent_conductance();
        assert!(
            (g_h2 - 2.0 * g_h1).abs() < 1e-9,
            "expected G(h/2) = 2·G(h); got G(h)={g_h1}, G(h/2)={g_h2}"
        );
    }

    // -----------------------------------------------------------------
    // InductorCompanion — Trapezoidal math.
    // -----------------------------------------------------------------

    #[test]
    fn inductor_trapezoidal_g_eq_matches_h_over_2l() {
        // L = 1 nH, h = 1 ns => G_eq = 1e−9 / (2 · 1e−9) = 0.5 S
        let l = InductorCompanion::trapezoidal(1e-9, 1e-9, ReactiveState::zero()).unwrap();
        let g = l.equivalent_conductance();
        assert!((g - 0.5).abs() < 1e-12, "expected G_eq = 0.5 S, got {g}");
    }

    #[test]
    fn inductor_trapezoidal_jacobian_is_kcl_conserving() {
        let l = InductorCompanion::trapezoidal(2.5e-9, 0.5e-9, ReactiveState::zero()).unwrap();
        let row0 = l.jacobian[0][0] + l.jacobian[0][1];
        let row1 = l.jacobian[1][0] + l.jacobian[1][1];
        assert!(row0.abs() < 1e-12);
        assert!(row1.abs() < 1e-12);
    }

    #[test]
    fn inductor_trapezoidal_companion_current_has_opposite_polarity_to_capacitor() {
        // Same state, same h: the inductor's I_eq is the negative
        // of the analogous capacitor expression (sign convention
        // documented in the module docstring).
        let v_prev = 1.0;
        let i_prev = 1e-3;
        let h = 1e-9;
        let c_val = 1e-9;
        let l_val = 1e-9;

        let c = CapacitorCompanion::trapezoidal(
            c_val,
            h,
            ReactiveState {
                voltage_volts: v_prev,
                current_amperes: i_prev,
            },
        )
        .unwrap();
        let l = InductorCompanion::trapezoidal(
            l_val,
            h,
            ReactiveState {
                voltage_volts: v_prev,
                current_amperes: i_prev,
            },
        )
        .unwrap();

        // Same sign of v_prev and i_prev produces opposite-signed
        // companion currents (capacitor positive, inductor negative
        // by the docstring's chosen convention).
        assert!(
            c.equivalent_current() > 0.0,
            "capacitor I_eq should be positive; got {}",
            c.equivalent_current()
        );
        assert!(
            l.equivalent_current() < 0.0,
            "inductor I_eq should be negative; got {}",
            l.equivalent_current()
        );
    }

    #[test]
    fn inductor_trapezoidal_companion_current_formula() {
        let l_val = 4e-9;
        let h = 2e-9;
        let v_prev = 0.75;
        let i_prev = 1.5e-3;
        let l = InductorCompanion::trapezoidal(
            l_val,
            h,
            ReactiveState {
                voltage_volts: v_prev,
                current_amperes: i_prev,
            },
        )
        .unwrap();
        let g_expected = h / (2.0 * l_val);
        let i_expected = -(g_expected * v_prev + i_prev);
        assert!((l.equivalent_conductance() - g_expected).abs() < 1e-12);
        assert!((l.equivalent_current() - i_expected).abs() < 1e-12);
    }

    #[test]
    fn inductor_trapezoidal_g_eq_shrinks_as_h_shrinks() {
        // Inductor admittance is h/(2L) — *halves* when h halves.
        // The duality with the capacitor case (where G doubles) is
        // why an inductor's stamp is "stiff" in the opposite sense:
        // small h makes the inductor look open-circuit, large h
        // makes it look short-circuit.
        let g_h1 = InductorCompanion::trapezoidal(1e-9, 1e-9, ReactiveState::zero())
            .unwrap()
            .equivalent_conductance();
        let g_h2 = InductorCompanion::trapezoidal(1e-9, 0.5e-9, ReactiveState::zero())
            .unwrap()
            .equivalent_conductance();
        assert!(
            (g_h2 - 0.5 * g_h1).abs() < 1e-12,
            "expected G(h/2) = G(h)/2; got G(h)={g_h1}, G(h/2)={g_h2}"
        );
    }

    // -----------------------------------------------------------------
    // ReactiveCompanion enum — uniform access for the MNA assembler.
    // -----------------------------------------------------------------

    #[test]
    fn reactive_companion_terminal_count_is_two_for_both_families() {
        let c = ReactiveCompanion::Capacitor(CapacitorCompanion::zero());
        let l = ReactiveCompanion::Inductor(InductorCompanion::zero());
        assert_eq!(c.terminal_count(), 2);
        assert_eq!(l.terminal_count(), 2);
    }

    #[test]
    fn reactive_companion_accessors_are_pass_through() {
        let inner = CapacitorCompanion::trapezoidal(1e-9, 1e-9, ReactiveState::zero()).unwrap();
        let wrapped = ReactiveCompanion::Capacitor(inner);
        // Compare bit-for-bit (the wrapped enum just re-exposes the
        // inner companion verbatim — no arithmetic happens — so an
        // exact bit comparison is the right test here. Using
        // `to_bits` sidesteps the `float_cmp` clippy lint that
        // (correctly) flags float-array `assert_eq!` in general.)
        assert!(jacobians_bit_equal(wrapped.jacobian(), inner.jacobian));
        assert!(currents_bit_equal(
            wrapped.companion_current(),
            inner.companion_current
        ));

        let inner_l = InductorCompanion::trapezoidal(1e-9, 1e-9, ReactiveState::zero()).unwrap();
        let wrapped_l = ReactiveCompanion::Inductor(inner_l);
        assert!(jacobians_bit_equal(wrapped_l.jacobian(), inner_l.jacobian));
        assert!(currents_bit_equal(
            wrapped_l.companion_current(),
            inner_l.companion_current
        ));
    }

    fn jacobians_bit_equal(
        a: [[f64; REACTIVE_TERMINALS]; REACTIVE_TERMINALS],
        b: [[f64; REACTIVE_TERMINALS]; REACTIVE_TERMINALS],
    ) -> bool {
        a.iter().zip(b.iter()).all(|(ra, rb)| {
            ra.iter()
                .zip(rb.iter())
                .all(|(x, y)| x.to_bits() == y.to_bits())
        })
    }

    fn currents_bit_equal(a: [f64; REACTIVE_TERMINALS], b: [f64; REACTIVE_TERMINALS]) -> bool {
        a.iter()
            .zip(b.iter())
            .all(|(x, y)| x.to_bits() == y.to_bits())
    }

    // -----------------------------------------------------------------
    // RLC-tank no-damping property: the TR stamp's signature
    // numerical-damping behavior is that on a lossless LC tank the
    // amplitude is preserved step-to-step (modulo O(h^3) LTE). We
    // can witness this at the stamp level by checking that the
    // *companion-current update* for a freshly-discharged capacitor
    // and a freshly-energized inductor exactly trades energy — the
    // tank's stored energy E = (1/2) C v^2 + (1/2) L i^2 is
    // *invariant under the TR map* up to O(h^3). At h=0 the map is
    // exact; we can only verify the leading-order invariant here.
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_trapezoidal_recovers_dc_stamp_as_h_grows_relative_to_rc() {
        // Sanity: as h grows without bound the capacitor's G_eq
        // (= 2C/h) → 0, i.e. the capacitor looks like an open
        // circuit. This is the textbook DC limit of a capacitor.
        let c = CapacitorCompanion::trapezoidal(1e-12, 1.0, ReactiveState::zero()).unwrap();
        assert!(
            c.equivalent_conductance() < 1e-11,
            "expected near-zero G_eq for large h/C, got {}",
            c.equivalent_conductance()
        );
    }

    #[test]
    fn inductor_trapezoidal_recovers_dc_stamp_as_h_grows_relative_to_l() {
        // Sanity: as h grows the inductor's G_eq (= h/2L) grows
        // without bound, i.e. the inductor looks like a short
        // circuit. This is the textbook DC limit of an inductor.
        let l = InductorCompanion::trapezoidal(1e-12, 1.0, ReactiveState::zero()).unwrap();
        assert!(
            l.equivalent_conductance() > 1e10,
            "expected large G_eq for large h/L, got {}",
            l.equivalent_conductance()
        );
    }

    // -----------------------------------------------------------------
    // Error paths — non-positive timestep / element value.
    // -----------------------------------------------------------------

    #[test]
    fn capacitor_rejects_zero_timestep() {
        let err = CapacitorCompanion::trapezoidal(1e-9, 0.0, ReactiveState::zero()).unwrap_err();
        assert!(matches!(
            err,
            CompanionConstructionError::NonPositiveTimestep { .. }
        ));
    }

    #[test]
    fn capacitor_rejects_negative_timestep() {
        let err = CapacitorCompanion::trapezoidal(1e-9, -1e-12, ReactiveState::zero()).unwrap_err();
        assert!(matches!(
            err,
            CompanionConstructionError::NonPositiveTimestep { .. }
        ));
    }

    #[test]
    fn capacitor_rejects_nan_timestep() {
        let err =
            CapacitorCompanion::trapezoidal(1e-9, f64::NAN, ReactiveState::zero()).unwrap_err();
        assert!(matches!(
            err,
            CompanionConstructionError::NonPositiveTimestep { .. }
        ));
    }

    #[test]
    fn capacitor_rejects_zero_capacitance() {
        let err = CapacitorCompanion::trapezoidal(0.0, 1e-9, ReactiveState::zero()).unwrap_err();
        assert!(matches!(
            err,
            CompanionConstructionError::NonPositiveElementValue { family: "C", .. }
        ));
    }

    #[test]
    fn inductor_rejects_zero_timestep() {
        let err = InductorCompanion::trapezoidal(1e-9, 0.0, ReactiveState::zero()).unwrap_err();
        assert!(matches!(
            err,
            CompanionConstructionError::NonPositiveTimestep { .. }
        ));
    }

    #[test]
    fn inductor_rejects_negative_inductance() {
        let err = InductorCompanion::trapezoidal(-1e-9, 1e-9, ReactiveState::zero()).unwrap_err();
        assert!(matches!(
            err,
            CompanionConstructionError::NonPositiveElementValue { family: "L", .. }
        ));
    }

    // -----------------------------------------------------------------
    // Error display strings name the offender, so a transient-loop
    // diagnostic can point at the bad input.
    // -----------------------------------------------------------------

    #[test]
    fn error_display_mentions_offending_timestep() {
        let err = CapacitorCompanion::trapezoidal(1e-9, -1e-12, ReactiveState::zero()).unwrap_err();
        let s = format!("{err}");
        assert!(s.contains("-0.000000000001") || s.contains("-1e-12"));
    }

    #[test]
    fn error_display_mentions_offending_family_for_inductor() {
        let err = InductorCompanion::trapezoidal(-1e-9, 1e-9, ReactiveState::zero()).unwrap_err();
        let s = format!("{err}");
        assert!(
            s.contains('L'),
            "expected error to name family 'L', got: {s}"
        );
    }
}
