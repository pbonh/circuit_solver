//! Companion-model surface types shared across implicit integration
//! methods.
//!
//! This module defines the *contract* between the implicit
//! integration methods ([`backward_euler`](super::backward_euler);
//! sibling `trapezoidal` and `gear2_bdf` modules land in tasks.md
//! #30 and #31) and the Pass-2 MNA assembler (tasks.md #14):
//!
//! - **Input** — the discretization step size `h` (seconds) and a
//!   per-element history struct ([`CapacitorHistory`] or
//!   [`InductorHistory`]) carrying the previous timestep's state.
//! - **Output** — a [`CompanionStamp`] containing the
//!   Norton-equivalent conductance (`g_eq`, siemens) and the history
//!   current source (`i_history`, amps) to stamp at the new
//!   timestep.
//!
//! Both reactive families share the same output shape so the MNA
//! assembler can fold them uniformly:
//!
//! ```text
//!   G[a, a] += g_eq;  G[a, b] -= g_eq;
//!   G[b, a] -= g_eq;  G[b, b] += g_eq;
//!   RHS[a]  -= i_history;
//!   RHS[b]  += i_history;
//! ```
//!
//! where `a` and `b` are the element's two terminals' `NodeId`s in
//! the flattened structure. The sign convention is: `i_history > 0`
//! represents conventional current flowing from terminal `a` to
//! terminal `b` through the companion model at `t = t_n` (i.e. the
//! current the element "remembers" pushing forward into the new
//! timestep).
//!
//! # Why a flat struct rather than a per-method enum
//!
//! Unlike `LinearizedModel` (which is an enum tagged by device family
//! because each family has a different terminal count and Jacobian
//! shape), every reactive element is a two-terminal Norton model
//! with a single scalar conductance and a single scalar history
//! current. There is no per-method or per-element shape variance to
//! capture in the type system, so a plain struct is the right
//! abstraction.
//!
//! # Sign and units conventions
//!
//! - Voltages: volts (V), absolute, relative to circuit ground.
//! - Currents: amps (A), conventional current direction `a → b`.
//! - Conductance: siemens (S), always non-negative for passive
//!   reactive elements at the discretization step sizes this design
//!   supports (`h > 0`, `C > 0`, `L > 0`).
//! - Time step: seconds (s), strictly positive — the [`backward_euler`]
//!   helpers reject `h <= 0` and non-finite `h` via
//!   [`CompanionInputError`](super::CompanionInputError).
//!
//! [`backward_euler`]: super::backward_euler

// -----------------------------------------------------------------------
// Companion stamp — the per-element Norton-equivalent output
// -----------------------------------------------------------------------

/// Per-timestep Norton-equivalent stamp for a single reactive
/// element.
///
/// This is the unit of communication from any implicit integration
/// method to the Pass-2 MNA assembler. The assembler stamps `g_eq`
/// into the conductance matrix at the element's two terminals and
/// `i_history` into the right-hand-side vector with the sign
/// convention documented at the module level.
///
/// # Field semantics
///
/// - `g_eq` — equivalent small-signal conductance contributed by the
///   reactive element at the *new* timestep. For Backward Euler,
///   capacitor `g_eq = C / h`, inductor `g_eq = h / L`.
/// - `i_history` — history current source (amps) carrying the
///   contribution of the previous timestep's state into the
///   right-hand-side vector at the new timestep. For Backward Euler,
///   capacitor `i_history = (C / h) · v_prev`, inductor
///   `i_history = i_prev`.
///
/// # Why `Copy + PartialEq`
///
/// The MNA assembler (tasks.md #14) iterates a per-element
/// `Vec<ElementStamp>` and unconditionally folds the contributions
/// into the global system; a `Copy` value-typed stamp keeps the
/// inner loop branch-free. `PartialEq` exists for unit tests that
/// compare expected vs. actual stamps; production code should not
/// compare floats for equality and should instead use the
/// convergence-criterion abstractions from
/// `circuit-solver-types::convergence` (tasks.md #2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompanionStamp {
    /// Equivalent conductance in siemens (S). Always finite and
    /// non-negative for valid reactive companion models with
    /// `h > 0`, `C > 0`, and `L > 0`.
    pub g_eq: f64,
    /// History current source in amps (A), with the conventional
    /// current direction `a → b` from terminal `a` to terminal `b`
    /// at `t = t_n`.
    pub i_history: f64,
}

impl CompanionStamp {
    /// All-zero stamp — the well-defined steady-state of a reactive
    /// element with zero capacitance / infinite inductance, useful
    /// as a sentinel and as the initial condition before any
    /// reactive element exists.
    ///
    /// Note that `g_eq = 0` is *not* the typical state of any
    /// reactive element during a transient solve — capacitors and
    /// inductors always contribute a finite `g_eq` at every timestep.
    /// The all-zero stamp exists as a type-level identity element,
    /// not a physically common case.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            g_eq: 0.0,
            i_history: 0.0,
        }
    }
}

// -----------------------------------------------------------------------
// History structs — per-element memory carried across timesteps
// -----------------------------------------------------------------------

/// Capacitor state at the most recent accepted timestep.
///
/// Backward Euler (and Trapezoidal, Gear-2 BDF) need only the
/// previous timestep's terminal-voltage difference to advance the
/// capacitor companion model. The capacitance value `C` is a
/// netlist parameter, not history.
///
/// # Field semantics
///
/// - `v_prev` — the *terminal-voltage difference* `V_a − V_b` at
///   `t = t_n` (volts). Sign convention: positive when terminal `a`
///   is the higher-potential terminal.
///
/// # Initial conditions
///
/// At the start of a transient solve, a capacitor's `v_prev` is
/// either:
///
/// 1. The DC operating-point solution at `t = 0` (default), or
/// 2. The user-supplied `UIC` initial-condition voltage when the
///    analysis request opts into UIC (per
///    `transient-time-domain#transient-analysis-with-uic-initial-conditions`).
///
/// The transient analysis control loop (tasks.md #33) decides which.
/// This module accepts either via [`CapacitorHistory::new`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CapacitorHistory {
    /// Terminal-voltage difference `V_a − V_b` at the previous
    /// accepted timestep, in volts.
    pub v_prev: f64,
}

impl CapacitorHistory {
    /// Construct a capacitor history from a single
    /// terminal-voltage difference.
    #[must_use]
    pub const fn new(v_prev: f64) -> Self {
        Self { v_prev }
    }

    /// History at `t = 0` with no prior solve — `v_prev = 0`.
    ///
    /// Used when the transient control loop initializes a capacitor
    /// from a DC operating point that has not yet been computed, or
    /// when UIC explicitly leaves the capacitor unset (an unusual
    /// case — most UIC requests specify the capacitor's initial
    /// voltage).
    #[must_use]
    pub const fn zero() -> Self {
        Self { v_prev: 0.0 }
    }
}

/// Inductor state at the most recent accepted timestep.
///
/// Backward Euler needs only the previous timestep's branch current
/// `i_prev` to advance the inductor companion model. The inductance
/// value `L` is a netlist parameter, not history.
///
/// # Field semantics
///
/// - `i_prev` — the inductor's *branch current* at `t = t_n` (amps),
///   directed from terminal `a` to terminal `b` per the conventional
///   current direction.
///
/// # MNA branch augmentation note
///
/// In MNA, an inductor's branch current is represented as an extra
/// state variable (an MNA branch augmentation row), not derived from
/// node voltages. The transient control loop (tasks.md #33) reads
/// this branch-current solution from the previous timestep's
/// solution vector and feeds it back into [`InductorHistory::new`]
/// for the next timestep's companion stamp.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InductorHistory {
    /// Branch current at the previous accepted timestep, in amps,
    /// directed from terminal `a` to terminal `b`.
    pub i_prev: f64,
}

impl InductorHistory {
    /// Construct an inductor history from a single branch current.
    #[must_use]
    pub const fn new(i_prev: f64) -> Self {
        Self { i_prev }
    }

    /// History at `t = 0` with no prior solve — `i_prev = 0`.
    ///
    /// Used when the transient control loop initializes an inductor
    /// from a DC operating point or when UIC explicitly leaves the
    /// inductor unset.
    #[must_use]
    pub const fn zero() -> Self {
        Self { i_prev: 0.0 }
    }
}
