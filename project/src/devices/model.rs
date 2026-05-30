//! Device model re-exports from the `device-modeling` crate.
//!
//! This module provides the project-level `devices.DeviceModel` contract
//! (ADR-0005) by re-exporting the core device-modeling types that the
//! stamp evaluator and MNA assembler consume.
//!
//! The closed-enum dispatch pattern (ADR-0005) requires every consumer
//! to `match` exhaustively on `DeviceModel` and `LinearizedModel`;
//! re-exporting them here keeps the project crate's public surface
//! self-contained without requiring downstream code to depend on
//! `device-modeling` directly.

// Re-export the closed-enum device model and its linearization types.
pub use device_modeling::model::{DeviceModel, DeviceFamily};

// Re-export the linearization input (OperatingPoint) and output
// (LinearizedModel) types from the stamp module.
pub use device_modeling::stamp::{
    LinearizedModel, OperatingPoint, OperatingPointFamilyMismatch,
    DiodeLinearization, BJTLinearization, MOSFETLinearization,
    DIODE_TERMINALS, BJT_TERMINALS, MOSFET_TERMINALS,
};

// Re-export the per-family linearization functions for direct use
// in conformance tests and the stamp evaluator.
pub use device_modeling::stamp::{linearize_diode, linearize_bjt, linearize_mosfet};

// Re-export the per-level MOSFET linearization helpers and the
// BSIM3v3 standalone function for direct use in conformance tests.
pub use device_modeling::stamp::{linearize_mosfet_level1, linearize_mosfet_bsim4};
pub use device_modeling::linearize_bsim3v3;

// Re-export parameter types needed to construct DeviceModel instances.
pub use device_modeling::params::{
    DiodeParams, BJTParams, MOSFETParams,
    BJTPolarity,
    MosLevel1Params, MosBSIM3v3Params, MosBSIM4Params, MosPolarity,
};
