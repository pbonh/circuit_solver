//! `LinearizedModel` stamp surface — placeholder for tasks.md #8.
//!
//! This module exists so that the closed-enum
//! [`DeviceModel`](crate::model::DeviceModel) from
//! tasks.md #7 can be referenced by downstream crates today; the
//! actual stamp / Jacobian methods land with tasks.md #8 behind a
//! single `match` on the enum, with per-family bodies arriving in
//! tasks.md #9 (Diode), #10 (BJT), #11–#13 (MOSFET levels).
//!
//! Keeping the surface here (rather than in
//! [`crate::model`]) lets task #8 grow the `LinearizedModel`
//! struct and the dispatch function without re-shuffling the type
//! definitions emitted by task #7.

// Intentionally empty at item #7. See module docstring above.
