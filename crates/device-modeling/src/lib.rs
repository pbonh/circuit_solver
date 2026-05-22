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
//!   (Diode), #10 (BJT), and #11–#13 (MOSFET levels).
//!
//! # Stability
//!
//! Per [ADR-0010](../../wiki/decisions/0010-unstable-public-rust-api-surface-for-v1.md)
//! the public API surface is **unstable** at v1.0.0. Consumers must
//! pin to exact versions until a future stabilization ADR.

#![deny(missing_docs)]

pub mod model;
pub mod params;
pub mod stamp;

pub use model::{DeviceFamily, DeviceModel};
pub use params::{
    BJTParams, BJTPolarity, DiodeParams, MOSFETParams, MosBSIM3v3Params, MosBSIM4Params,
    MosLevel1Params, MosPolarity,
};
pub use stamp::{
    linearize_bjt, linearize_diode, linearize_mosfet, BJTLinearization, DiodeLinearization,
    LinearizedModel, MOSFETLinearization, OperatingPoint, OperatingPointFamilyMismatch,
    BJT_TERMINALS, DIODE_MAX_EXP_ARG, DIODE_TERMINALS, MOSFET_TERMINALS,
};
