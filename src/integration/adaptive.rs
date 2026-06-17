//! Adaptive timestep controller based on local truncation error (LTE).
//!
//! Accepts or rejects each proposed timestep and recommends the next step size.
//!
//! # Tolerance
//!
//! The acceptance criterion is `lte < rtol * ||x||∞ + atol`.
//! Defaults: `rtol = 1e-3`, `atol = 1e-6`.
//!
//! # Growth / shrink
//!
//! - LTE ≥ tol → reject, halve `h`.
//! - LTE < tol / 10 → accept, grow `h` by factor 1.5.
//! - Otherwise → accept, keep `h`.
//!
//! `h` is clamped to `[h_min, h_max]` at all times.
//!
//! After 5 consecutive rejections the controller returns `Err(IntegrationError)`.

use super::IntegrationError;

/// Outcome of a single `evaluate` call.
#[derive(Debug, Clone, PartialEq)]
pub enum ControllerOutcome {
    /// Step accepted. Payload is the recommended next step size.
    Accept(f64),
    /// Step rejected. Payload is the recommended next (halved) step size.
    Reject(f64),
}

/// Stateful adaptive timestep controller.
///
/// Call [`evaluate`][AdaptiveStepController::evaluate] after each proposed
/// integration step. The controller tracks consecutive rejections and errors
/// out after `max_consecutive_rejections`.
#[derive(Debug, Clone)]
pub struct AdaptiveStepController {
    /// Relative tolerance (default 1e-3).
    pub rtol: f64,
    /// Absolute tolerance (default 1e-6).
    pub atol: f64,
    /// Minimum allowed timestep (default 1e-15 s).
    pub h_min: f64,
    /// Maximum allowed timestep (default 1e-3 s).
    pub h_max: f64,
    /// Maximum consecutive rejections before returning `IntegrationError`.
    pub max_consecutive_rejections: usize,
    /// Current timestep (updated by `evaluate`).
    pub h: f64,
    /// Number of consecutive rejections so far.
    consecutive_rejections: usize,
}

impl Default for AdaptiveStepController {
    fn default() -> Self {
        Self::new(1e-3, 1e-6, 1e-15, 1e-3, 5, 1e-9)
    }
}

impl AdaptiveStepController {
    /// Create a controller with explicit parameters.
    ///
    /// # Parameters
    /// - `rtol` — relative tolerance
    /// - `atol` — absolute tolerance
    /// - `h_min` — minimum timestep
    /// - `h_max` — maximum timestep
    /// - `max_consecutive_rejections` — failure threshold
    /// - `h_initial` — initial timestep
    pub fn new(
        rtol: f64,
        atol: f64,
        h_min: f64,
        h_max: f64,
        max_consecutive_rejections: usize,
        h_initial: f64,
    ) -> Self {
        AdaptiveStepController {
            rtol,
            atol,
            h_min,
            h_max,
            max_consecutive_rejections,
            h: h_initial.clamp(h_min, h_max),
            consecutive_rejections: 0,
        }
    }

    /// Compute the tolerance at the current solution magnitude `x_inf_norm`.
    fn tol(&self, x_inf_norm: f64) -> f64 {
        self.rtol * x_inf_norm + self.atol
    }

    /// Evaluate whether a step is accepted or rejected.
    ///
    /// # Parameters
    /// - `t` — current time (for error reporting only)
    /// - `lte` — scalar LTE estimate (infinity-norm of per-node LTE vector)
    /// - `x_inf_norm` — infinity-norm of the current solution vector
    ///
    /// # Returns
    /// - `Ok(ControllerOutcome::Accept(next_h))` — step accepted
    /// - `Ok(ControllerOutcome::Reject(next_h))` — step rejected, retry with `next_h`
    /// - `Err(IntegrationError)` — 5 consecutive rejections
    ///
    /// # Errors
    ///
    /// Returns [`IntegrationError`] when the controller has rejected
    /// `max_consecutive_rejections` steps in a row.
    pub fn evaluate(
        &mut self,
        t: f64,
        lte: f64,
        x_inf_norm: f64,
    ) -> Result<ControllerOutcome, IntegrationError> {
        // NaN lte always rejects.
        let tol = self.tol(x_inf_norm);

        if lte < tol {
            // Accepted.
            self.consecutive_rejections = 0;
            let next_h = if lte < tol / 10.0 {
                // Well below tolerance: grow.
                (self.h * 1.5).clamp(self.h_min, self.h_max)
            } else {
                self.h
            };
            self.h = next_h;
            Ok(ControllerOutcome::Accept(next_h))
        } else {
            // Rejected.
            self.consecutive_rejections += 1;
            if self.consecutive_rejections >= self.max_consecutive_rejections {
                return Err(IntegrationError { t, lte, h: self.h });
            }
            let next_h = (self.h * 0.5).clamp(self.h_min, self.h_max);
            self.h = next_h;
            Ok(ControllerOutcome::Reject(next_h))
        }
    }

    /// Reset the consecutive-rejection counter without changing `h`.
    pub fn reset_rejection_count(&mut self) {
        self.consecutive_rejections = 0;
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn controller() -> AdaptiveStepController {
        // rtol=1e-3, atol=1e-6, h_min=1e-15, h_max=1e-3, max_rej=5, h_initial=1e-9
        AdaptiveStepController::default()
    }

    // tol at x_inf_norm=1.0: 1e-3 * 1 + 1e-6 = ~1.001e-3
    // lte = tol/2 → accept
    #[test]
    fn accept_when_lte_below_tol() {
        let mut ctrl = controller();
        let tol = ctrl.tol(1.0);
        let lte = tol / 2.0;
        let result = ctrl.evaluate(0.0, lte, 1.0).expect("should not error");
        assert!(matches!(result, ControllerOutcome::Accept(_)));
    }

    // lte = 2*tol → reject
    #[test]
    fn reject_when_lte_above_tol() {
        let mut ctrl = controller();
        let tol = ctrl.tol(1.0);
        let lte = 2.0 * tol;
        let result = ctrl.evaluate(0.0, lte, 1.0).expect("should not error after 1 rejection");
        assert!(matches!(result, ControllerOutcome::Reject(_)));
    }

    // h halved on rejection
    #[test]
    fn h_halved_on_rejection() {
        let mut ctrl = controller();
        let h_before = ctrl.h;
        let tol = ctrl.tol(1.0);
        let _ = ctrl.evaluate(0.0, 2.0 * tol, 1.0);
        assert!((ctrl.h - h_before / 2.0).abs() < 1e-30);
    }

    // h grows by 1.5x when lte < tol/10
    #[test]
    fn h_grows_when_lte_well_below_tol() {
        let mut ctrl = controller();
        let h_before = ctrl.h;
        let tol = ctrl.tol(1.0);
        let lte = tol / 20.0; // < tol/10
        let _ = ctrl.evaluate(0.0, lte, 1.0);
        assert!((ctrl.h - (h_before * 1.5).min(ctrl.h_max)).abs() < 1e-30);
    }

    // After 5 consecutive rejections → IntegrationError
    #[test]
    fn five_consecutive_rejections_returns_error() {
        let mut ctrl = controller();
        let tol = ctrl.tol(1.0);
        let lte = 2.0 * tol;
        for i in 0..4 {
            let r = ctrl.evaluate(0.0, lte, 1.0);
            assert!(r.is_ok(), "rejection {} should not yet error", i + 1);
        }
        let r = ctrl.evaluate(0.0, lte, 1.0);
        assert!(r.is_err(), "5th consecutive rejection should return error");
    }

    // Counter resets on accept
    #[test]
    fn rejection_counter_resets_on_accept() {
        let mut ctrl = controller();
        let tol = ctrl.tol(1.0);
        // 4 rejections
        for _ in 0..4 {
            let _ = ctrl.evaluate(0.0, 2.0 * tol, 1.0);
        }
        // accept
        let _ = ctrl.evaluate(0.0, tol / 2.0, 1.0);
        // should be able to reject 4 more without error
        for _ in 0..4 {
            let r = ctrl.evaluate(0.0, 2.0 * tol, 1.0);
            assert!(r.is_ok());
        }
    }

    // h clamped to h_max
    #[test]
    fn h_clamped_to_h_max() {
        let mut ctrl = controller();
        ctrl.h = ctrl.h_max * 0.9;
        let tol = ctrl.tol(1.0);
        // lte << tol/10 → want to grow, but should clamp
        let _ = ctrl.evaluate(0.0, tol / 1000.0, 1.0);
        assert!(ctrl.h <= ctrl.h_max + 1e-30);
    }

    // h clamped to h_min
    #[test]
    fn h_clamped_to_h_min() {
        let mut ctrl = AdaptiveStepController::new(1e-3, 1e-6, 1e-15, 1e-3, 5, 2e-15);
        let tol = ctrl.tol(1.0);
        // Force rejection → halve → should clamp to h_min
        let _ = ctrl.evaluate(0.0, 2.0 * tol, 1.0);
        assert!(ctrl.h >= ctrl.h_min);
    }

    // NaN lte → reject
    #[test]
    fn nan_lte_rejects() {
        let mut ctrl = controller();
        let result = ctrl.evaluate(0.0, f64::NAN, 1.0);
        // NaN < tol is false, so it goes to the rejection branch
        assert!(result.is_ok()); // first rejection is OK (not error)
        if let Ok(outcome) = result {
            assert!(matches!(outcome, ControllerOutcome::Reject(_)));
        }
    }
}
