//! Device models — project-level integration of the device-modeling crate.
//!
//! This module hosts the project-level `devices.DeviceModel` contract
//! (ADR-0005) and the stamp evaluator that bridges
//! `device-modeling::LinearizedModel` with the numeric solver's
//! `StampInterface` (ADR-0002).
//!
//! # Architecture
//!
//! ```text
//! DeviceModel  ──linearize()──►  LinearizedModel
//!        │                            │
//!        │  (ADR-0005 closed enum)     │  terminal-local Jacobian
//!        │                             │  + companion current
//!        ▼                             ▼
//!   stamp_linearized_device()
//!        │
//!        ▼
//!   StampInterface (MnaMatrix)
//! ```
//!
//! The stamp evaluator performs exhaustive `match` on `LinearizedModel`
//! to learn stamp dimensions, then folds each terminal-local
//! contribution into the global MNA system via `StampInterface`
//! methods — matching the closed-enum discipline of ADR-0005.
//!
//! # Design references
//!
//! - **ADR-0002** — Hybrid sparse direct solver backend (Russell + FAER).
//!   Ratifies `numeric.StampInterface` as a shared contract.
//! - **ADR-0005** — Closed-enum device model dispatch. No `dyn`,
//!   exhaustive `match` on `DeviceModel` / `LinearizedModel`.
//! - **ADR-0010** — Unstable public Rust API surface for v1.

pub mod model;
pub mod mosfet;
pub mod stamp;

pub use model::{DeviceModel, LinearizedModel, OperatingPoint};
pub use stamp::{stamp_linearized_device, StampDeviceError};
