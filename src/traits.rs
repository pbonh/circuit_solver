//! DeviceModel trait: the open-ended interface that every circuit element
//! implements so the Newton-Raphson assembler can stamp it without knowing the
//! concrete type.

use crate::{MnaMatrix, VarMap};

/// Open-ended device model interface.
///
/// Implementors must be dyn-safe (no associated types or generic methods).
///
/// Stamping convention
/// -------------------
/// Node indices used inside `stamp_linear` / `stamp_nonlinear` are obtained
/// from a [`VarMap`]:
/// - Ground rows/cols are handled by callers passing `None` to the low-level
///   stamping helpers; this trait simply provides the node names.
/// - `stamp_nonlinear` receives the current solution vector so the device can
///   evaluate its operating point.
pub trait DeviceModel {
    /// Return the ordered list of terminal net names for this device.
    ///
    /// The first terminal is usually the positive / drain node; order follows
    /// SPICE conventions for each device type.
    fn terminals(&self) -> Vec<String>;

    /// Stamp the **linear** (operating-point-independent) part of this device
    /// into `matrix`.
    ///
    /// For fully linear elements this is the complete stamp.  For nonlinear
    /// devices this is typically a no-op (all work done in `stamp_nonlinear`).
    fn stamp_linear(&self, matrix: &mut MnaMatrix, var_map: &VarMap);

    /// Stamp the **nonlinear** (operating-point-dependent) contribution at the
    /// current solution vector `solution`.
    ///
    /// For linear elements this simply delegates to `stamp_linear`.  Nonlinear
    /// devices re-evaluate their conductances / currents at the current
    /// operating point and stamp the Jacobian and companion-current entries.
    fn stamp_nonlinear(
        &self,
        matrix: &mut MnaMatrix,
        var_map: &VarMap,
        solution: &[f64],
    );

    /// Returns `true` if this device's I-V characteristic is smooth (analytic
    /// first derivative everywhere).
    ///
    /// Linear elements (Resistor, Capacitor, Inductor, independent sources)
    /// are smooth.  Nonlinear elements with piecewise-linear clamping or
    /// discontinuous derivatives (Diode, MOSFET Level 1) return `false`.
    fn is_smooth(&self) -> bool;

    /// Update the companion-model timestep `h` (seconds) before stamping.
    ///
    /// Called at the start of every transient timestep so reactive elements
    /// (Capacitor, Inductor) can store the current `h` for use in
    /// `stamp_linear` / `stamp_nonlinear`. Non-reactive elements use the
    /// provided default no-op.
    fn set_timestep(&mut self, _h: f64) {}

    /// Advance the companion-model history state after an accepted timestep.
    ///
    /// Called after each accepted timestep. Reactive elements extract their
    /// previous voltage (`Capacitor::v_prev`) or previous branch current
    /// (`Inductor::i_prev`) from `solution` so the next step's companion stamp
    /// is correct. Non-reactive elements use the provided default no-op.
    fn advance_state(&mut self, _solution: &[f64], _var_map: &VarMap) {}

    /// Set the current simulation time `t` (seconds) before each MNA assembly.
    ///
    /// Called by the transient driver at the start of each step so time-varying
    /// sources (e.g. PWL voltage sources) can evaluate their waveform at `t`.
    /// Non-time-varying elements use the provided default no-op.
    fn set_time(&mut self, _t: f64) {}
}
