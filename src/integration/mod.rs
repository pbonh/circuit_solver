//! Numerical integration methods for transient circuit simulation.
//!
//! Provides BDF1 (Backward Euler) and BDF2 integrators, plus the
//! `AdaptiveStepController` that accepts or rejects each timestep based on
//! local truncation error.
//!
//! # Integration error
//!
//! [`IntegrationError`] is returned when the adaptive controller fails to
//! converge (5 consecutive rejections at the same timepoint).

pub mod adaptive;
pub mod bdf;

/// Error returned by the transient engine when a timestep cannot be integrated.
///
/// This occurs when the adaptive controller exceeds its consecutive-rejection
/// limit (default 5), meaning the LTE cannot be driven below tolerance.
#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationError {
    /// Time at which integration failed.
    pub t: f64,
    /// Local truncation error estimate at the failing step.
    pub lte: f64,
    /// Timestep that was attempted when the failure was detected.
    pub h: f64,
}

impl std::fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Integration failed at t = {:.6e}: LTE = {:.3e} > tolerance after 5 consecutive \
             rejections (h = {:.3e})",
            self.t, self.lte, self.h
        )
    }
}

pub use adaptive::AdaptiveStepController;
pub use bdf::{Bdf, BdfConfig, BdfOrder};
