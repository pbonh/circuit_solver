//! Analysis orchestration — project-level driver integration.
//!
//! This module hosts the project-level analysis drivers that bridge the
//! `analysis-orchestration` crate's control loops (DC, AC, noise,
//! transient, sweep) to the project's device model / stamp
//! infrastructure (ADR-0005 closed-enum dispatch).
//!
//! # Architecture
//!
//! The `analysis-orchestration` crate implements the core per-frequency
//! analysis loops using the `numeric-solver` crate's MNA system and
//! solver dispatch. The `project` crate adds a layer on top that:
//!
//! 1. **Consumes project-level types** — `DeviceModel`, `LinearizedModel`,
//!    `OperatingPoint` — via the closed-enum dispatch pattern (ADR-0005).
//! 2. **Stamps linearized devices** into the MNA system using
//!    [`crate::devices::stamp_linearized_device`], which performs an
//!    exhaustive `match` on [`LinearizedModel`] and folds each device's
//!    Jacobian and companion current into the `StampInterface` contract.
//! 3. **Delegates to the crate-level analysis loops** — `ac_analysis`,
//!    `noise_analysis`, etc. — passing the assembled MNA system.
//!
//! # Design references
//!
//! - **ADR-0002** — Hybrid sparse direct solver backend (Russell + FAER).
//!   `numeric.StampInterface` as shared contract.
//! - **ADR-0003** — Two-pass graph flattening with per-analysis sub-views.
//! - **ADR-0005** — Closed-enum device model dispatch.
//! - **ADR-0010** — Unstable public Rust API surface for v1.
//!
//! # What this module does *not* do
//!
//! - No auto-DC dispatch; that lives in `analysis-orchestration::auto_dc_ac`
//!   and `analysis-orchestration::noise::noise_analysis_with_auto_dc`.
//! - No frequency sweep generation; that lives in
//!   `analysis-orchestration::sweep::LogSweep`.
//! - No device noise model computation; that lives in
//!   `device-modeling::noise` and is wired through
//!   `analysis-orchestration::noise::SemiconductorNoiseSource`.

pub mod ac_noise;

// Re-export the analysis-orchestration crate's core types so downstream
// consumers can use them without a direct dependency.
pub use analysis_orchestration::ac::{
    ac_analysis, AcAnalysisError, AcAnalysisRequest, AcAnalysisResult, TransferFunction,
};
pub use analysis_orchestration::noise::{
    collect_noise_sources, integrated_noise, noise_analysis,
    noise_analysis_with_auto_dc, DeviceNoiseContribution, IntegratedNoise,
    IntegratedNoiseError, IntegratedNoiseRequest, IntegrationBand, NoiseAnalysisData,
    NoiseAnalysisError, NoiseAnalysisRequest, NoiseAnalysisResult,
    NoiseAnalysisWithAutoDcError, NoiseAnalysisWithAutoDcRequest,
    NoiseAnalysisWithAutoDcResult, NoiseInjection, SemiconductorNoiseSource,
};
