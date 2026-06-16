//! `device-modeling` — closed-enum [`model::DeviceModel`] and Jacobian stamps,
//! plus the open [`traits::DeviceModel`] trait for Newton-Raphson dispatch.
//!
//! This crate owns the electrical behavior of nonlinear and linear
//! devices for the circuit-solver workspace. It supplies linearized
//! [`Stamp`](crate::stamp)s to the `numeric-solver` context at
//! each Newton-Raphson iterate.
//!
//! # Closed-enum dispatch (ADR-0005)
//!
//! [`model::DeviceModel`] is a Rust enum with one variant per in-scope model
//! family ([`model::DeviceModel::Diode`], [`model::DeviceModel::BJT`],
//! [`model::DeviceModel::MOSFET`]). Each variant owns its
//! `ModelParameters` payload **inline** — no `Box`,
//! no `dyn`, no string-keyed registry. Stamp evaluation and Jacobian
//! computation dispatch through `match` on the enum, producing
//! zero-cost monomorphized code per
//! [ADR-0005](../../wiki/decisions/0005-closed-enum-device-model-dispatch.md).
//!
//! Adding a new variant is a deliberate compile-time breaking change
//! — every `match` arm must be updated, which is exactly the property
//! the closed enum exists to guarantee.
//!
//! # Trait-based dispatch (US-011)
//!
//! [`traits::DeviceModel`] is the open-ended `dyn`-safe trait used when
//! the Newton-Raphson engine needs to hold a heterogeneous mix of device
//! types without a closed variant list. Both dispatch styles coexist:
//! the enum is the hot-path zero-cost path; the trait is the extension
//! point for tests, future user-defined models, and uniform NR iteration.
//!
//! Supporting types:
//!
//! - [`MnaMatrix`] — mutable view over the MNA matrix + RHS passed to
//!   stamp methods.
//! - [`VarMap`] — maps [`circuit_solver_types::NodeId`] /
//!   [`circuit_solver_types::BranchId`] to integer MNA row/column offsets.
//!
//! # Module map
//!
//! - [`params`] — per-family `ModelParameters` payload structs
//!   (`DiodeParams`, `BJTParams`, `MOSFETParams`). `MOSFETParams` is
//!   itself a closed enum over MOS levels (Level-1, `BSIM3v3`, BSIM4)
//!   per the design slice at `openspec/changes/circuit-solver-2026-05-21-v1-spec/design.md`.
//! - [`model`] — the top-level closed-enum device model, its family
//!   discriminator [`DeviceFamily`], and convenience accessors.
//! - [`stamp`] — the `LinearizedModel` stamp + Jacobian surface
//!   introduced in tasks.md #8. Defines the [`stamp::LinearizedModel`]
//!   response type, the [`stamp::OperatingPoint`] request type, and
//!   the [`model::DeviceModel::linearize`] dispatch entry
//!   point. Per-family equation bodies land in tasks.md #9
//!   (Diode), #10 (BJT), #11–#13 (MOSFET levels). As of the
//!   merged state of this slice the Diode arm
//!   ([`stamp::linearize_diode`]), BJT arm
//!   ([`stamp::linearize_bjt`]), MOSFET Level-1 arm
//!   ([`stamp::linearize_mosfet_level1`]), MOSFET `BSIM3v3` arm
//!   ([`bsim3v3::linearize_bsim3v3`]), and MOSFET BSIM4 arm
//!   ([`stamp::linearize_mosfet_bsim4`]) are all real
//!   implementations dispatching through the closed-enum `match`;
//!   no arm is a placeholder at this point.
//! - [`mna_matrix`] — [`MnaMatrix`]: mutable view over the flat MNA
//!   matrix and RHS vector, passed to [`traits::DeviceModel`] stamp methods.
//! - [`var_map`] — [`VarMap`]: maps node/branch identifiers to MNA
//!   row/column indices.
//! - [`traits`] — [`traits::DeviceModel`]: the `dyn`-safe trait that
//!   both linear and nonlinear devices implement for uniform NR stamping.
//!
//! # Stability
//!
//! Per [ADR-0010](../../wiki/decisions/0010-unstable-public-rust-api-surface-for-v1.md)
//! the public API surface is **unstable** at v1.0.0. Consumers must
//! pin to exact versions until a future stabilization ADR.

#![deny(missing_docs)]

pub mod bsim3v3;
pub mod companion;
pub mod mna_matrix;
pub mod model;
pub mod noise;
pub mod params;
pub mod stamp;
pub mod traits;
pub mod var_map;

pub use bsim3v3::linearize_bsim3v3;
pub use companion::{
    CapacitorCompanion, CompanionConstructionError, InductorCompanion, ReactiveCompanion,
    ReactiveState, REACTIVE_TERMINALS,
};
pub use model::{DeviceFamily, DeviceModel};
pub use noise::{
    noise_stamp_bjt, noise_stamp_diode, noise_stamp_mosfet, resistor_thermal_noise, BJTNoiseStamp,
    BJTOperatingState, DeviceNoiseStamp, DeviceOperatingState, DiodeNoiseStamp,
    DiodeOperatingState, MosfetNoiseStamp, MosfetOperatingState, NoiseMechanism, NoiseSource,
    BOLTZMANN_J_PER_K, ELEMENTARY_CHARGE_C, ROOM_TEMPERATURE_K,
};
pub use params::{
    BJTParams, BJTPolarity, DiodeParams, MOSFETParams, MosBSIM3v3Params, MosBSIM4Params,
    MosLevel1Params, MosPolarity,
};
pub use mna_matrix::MnaMatrix;
pub use stamp::{
    linearize_bjt, linearize_diode, linearize_mosfet, linearize_mosfet_bsim4,
    linearize_mosfet_level1, BJTLinearization, DiodeLinearization, LinearizedModel,
    MOSFETLinearization, OperatingPoint, OperatingPointFamilyMismatch, BJT_TERMINALS,
    DIODE_MAX_EXP_ARG, DIODE_TERMINALS, MOSFET_TERMINALS,
};
pub use var_map::VarMap;
