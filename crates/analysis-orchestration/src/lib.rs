//! `analysis-orchestration` — analysis control loops and the
//! Mixed-Signal Scheduler.
//!
//! This crate hosts the per-analysis driver loops (DC, AC, transient,
//! noise) and the [`MixedSignalScheduler`]
//! that orchestrates a continuous-time analog solver against an
//! event-driven digital simulator per ADR-0004 ("Optimistic Mixed-Signal
//! Synchronization via Shared Scheduler").
//!
//! At present the implemented surfaces are:
//!
//! - [`ac`] — AC small-signal control loop (tasks.md #25). Composes
//!   the AC sub-view extractor and the complex sparse-LU dispatch into
//!   a per-frequency driver that produces [`TransferFunction`]
//!   results. Covers scenarios
//!   `ac-small-signal#ac-analysis-with-pre-computed-operating-point`
//!   and `ac-small-signal#ac-analysis-on-purely-linear-circuit`.
//! - [`auto_dc_ac`] — Auto-DC AC composition (tasks.md #26). When no
//!   prior [`OperatingPoint`] is cached, runs [`dc_analysis`] first and
//!   then [`ac_analysis`] at the converged operating point, returning
//!   both halves in a single bundled result. Covers scenario
//!   `ac-small-signal#ac-analysis-without-prior-operating-point`.
//! - [`noise`] — Noise spectral-density control loop (tasks.md #37)
//!   plus the integrated-noise-over-bandwidth summary metric
//!   (tasks.md #39, [`integrated_noise`]).
//!   Linearizes at the DC operating point, builds noise transfer
//!   matrices at each frequency via the AC sub-view extractor, and
//!   computes the output-referred PSD by summing squared-magnitude
//!   transfer-function contributions weighted by per-source PSDs.
//!   Covers scenarios
//!   `noise-spectral-density#noise-analysis-on-a-resistive-circuit`,
//!   `noise-spectral-density#integrated-noise-over-bandwidth`,
//!   and
//!   `noise-spectral-density#noise-analysis-on-circuit-with-failed-operating-point`.
//! - [`sweep`] — Logarithmic frequency [`Sweep`][crate::LogSweep]
//!   generator (tasks.md #28). Produces the frequency vector used by
//!   [`ac_analysis`] for multi-decade Bode-style analyses. Covers
//!   scenario `ac-small-signal#ac-frequency-sweep-over-multiple-decades`.
//! - [`mixed_signal`] — Mixed-Signal Scheduler skeleton with the
//!   `optimistic-advance-with-correct-prediction` happy path. Sibling
//!   implementer tasks fill in mis-prediction rollback, boundary
//!   signal exchange interpolation, conformance, etc.
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod ac;
pub mod auto_dc_ac;
pub mod dc;
pub mod mixed_signal;
pub mod noise;
pub mod sweep;

pub use ac::{ac_analysis, AcAnalysisError, AcAnalysisRequest, AcAnalysisResult, TransferFunction};
pub use auto_dc_ac::{
    ac_analysis_with_auto_dc, AcWithAutoDcError, AcWithAutoDcRequest, AcWithAutoDcResult,
};
pub use dc::{
    dc_analysis, BranchCurrentSample, DcAnalysisError, DcAnalysisRequest, DcAnalysisResult,
    OperatingPoint,
};
pub use mixed_signal::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, MixedSignalScheduler, NextEventReport, SchedulerError, SchedulerOutcome,
};
pub use noise::{
    collect_noise_sources, integrated_noise, noise_analysis, DeviceNoiseContribution,
    IntegratedNoise, IntegratedNoiseError, IntegratedNoiseRequest, IntegrationBand,
    NoiseAnalysisData, NoiseAnalysisError, NoiseAnalysisRequest, NoiseAnalysisResult,
    NoiseInjection, SemiconductorNoiseSource,
};
pub use sweep::{LogSweep, LogSweepError};
