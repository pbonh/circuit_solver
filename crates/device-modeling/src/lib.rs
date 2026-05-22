//! `device-modeling` — closed-enum [`DeviceModel`] and Jacobian stamps.
//!
//! This crate owns the electrical behavior of nonlinear and linear
//! devices for the circuit-solver workspace. It supplies linearized
//! [`Stamp`](crate::stamp)s to the `numeric-solver` context at
//! each Newton-Raphson iterate.
//!
//! # Closed-enum dispatch (ADR-0005)
//!
//! [`DeviceModel`] is a Rust enum with one variant per in-scope model
//! family ([`DeviceModel::Diode`], [`DeviceModel::BJT`],
//! [`DeviceModel::MOSFET`]). Each variant owns its
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
//! # Module map
//!
//! - [`params`] — per-family `ModelParameters` payload structs
//!   (`DiodeParams`, `BJTParams`, `MOSFETParams`). `MOSFETParams` is
//!   itself a closed enum over MOS levels (Level-1, `BSIM3v3`, BSIM4)
//!   per the design slice at `openspec/changes/circuit-solver-2026-05-21-v1-spec/design.md`.
//! - [`model`] — the top-level [`DeviceModel`] enum, its family
//!   discriminator [`DeviceFamily`], and convenience accessors.
//! - [`stamp`] — the `LinearizedModel` stamp + Jacobian surface
//!   introduced in tasks.md #8. Defines the [`stamp::LinearizedModel`]
//!   response type, the [`stamp::OperatingPoint`] request type, and
//!   the [`DeviceModel::linearize`](crate::DeviceModel) dispatch entry
//!   point. Per-family equation bodies land in tasks.md #9
//!   (Diode), #10 (BJT), #11–#13 (MOSFET levels). As of tasks.md #12
//!   and #13 the MOSFET `BSIM3v3` arm
//!   ([`bsim3v3::linearize_bsim3v3`]) and BSIM4 arm
//!   ([`stamp::linearize_mosfet_bsim4`]) are implemented; the Diode,
//!   BJT, and Level-1 arms are still zero placeholders awaiting
//!   their respective tasks.
//!
//! # Stability
//!
//! Per [ADR-0010](../../wiki/decisions/0010-unstable-public-rust-api-surface-for-v1.md)
//! the public API surface is **unstable** at v1.0.0. Consumers must
//! pin to exact versions until a future stabilization ADR.

#![deny(missing_docs)]

pub mod bsim3v3;
pub mod companion;
pub mod model;
pub mod noise;
pub mod params;
pub mod stamp;

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
pub use stamp::{
    linearize_bjt, linearize_diode, linearize_mosfet, linearize_mosfet_bsim4, BJTLinearization,
    DiodeLinearization, LinearizedModel, MOSFETLinearization, OperatingPoint,
    OperatingPointFamilyMismatch, BJT_TERMINALS, DIODE_MAX_EXP_ARG, DIODE_TERMINALS,
    MOSFET_TERMINALS,
};
