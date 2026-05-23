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
//! - [`mod@dc_sweep`] — DC Sweep control loop (tasks.md #21). Wraps
//!   [`dc_analysis`] in an outer loop over a voltage-source value
//!   range and returns one [`OperatingPoint`] per sweep point,
//!   addressable by sweep index. Covers scenario
//!   `dc-operating-point#dc-sweep-over-a-voltage-source`.
//! - [`noise`] — Noise spectral-density control loop (tasks.md #37),
//!   its auto-DC entry point (tasks.md #40), plus the
//!   integrated-noise-over-bandwidth summary metric
//!   (tasks.md #39, [`integrated_noise`]).
//!   Linearizes at the DC operating point, builds noise transfer
//!   matrices at each frequency via the AC sub-view extractor, and
//!   computes the output-referred PSD by summing squared-magnitude
//!   transfer-function contributions weighted by per-source PSDs.
//!   The [`noise::noise_analysis_with_auto_dc`] convenience composes
//!   the internal DC dispatch + noise loop so callers without a
//!   pre-computed `OperatingPoint` (see [`dc::OperatingPoint`]) can run
//!   noise analysis directly from a `(graph, structure)` pair.
//!   Covers scenarios
//!   `noise-spectral-density#noise-analysis-on-a-resistive-circuit`,
//!   `noise-spectral-density#integrated-noise-over-bandwidth`,
//!   `noise-spectral-density#noise-analysis-on-circuit-with-failed-operating-point`,
//!   and
//!   `noise-spectral-density#noise-analysis-without-prior-operating-point`.
//! - [`sweep`] — Logarithmic frequency [`Sweep`][crate::LogSweep]
//!   generator (tasks.md #28). Produces the frequency vector used by
//!   [`ac_analysis`] for multi-decade Bode-style analyses. Covers
//!   scenario `ac-small-signal#ac-frequency-sweep-over-multiple-decades`.
//! - [`mixed_signal`] — Mixed-Signal Scheduler skeleton with the
//!   `optimistic-advance-with-correct-prediction` happy path. Sibling
//!   implementer tasks fill in mis-prediction rollback, boundary
//!   signal exchange interpolation, conformance, etc.
//! - [`boundary_exchanger`] — Analog↔digital boundary signal exchanger
//!   with the **zero-order hold** default per ADR-0007 (tasks.md item
//!   #45). Routes named boundary values between the analog solver and
//!   the digital simulator at every synchronization point, holding
//!   the last accepted value constant until the event time. Linear
//!   interpolation opt-in is reserved for tasks.md item #46.
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod ac;
pub mod auto_dc_ac;
pub mod boundary_exchanger;
pub mod dc;
pub mod dc_sweep;
pub mod mixed_signal;
pub mod noise;
pub mod sweep;
pub mod transient;

pub use ac::{ac_analysis, AcAnalysisError, AcAnalysisRequest, AcAnalysisResult, TransferFunction};
pub use auto_dc_ac::{
    ac_analysis_with_auto_dc, AcWithAutoDcError, AcWithAutoDcRequest, AcWithAutoDcResult,
};
pub use boundary_exchanger::{
    AnalogValueProvider, BoundaryExchangePacket, BoundaryExchangerError, BoundaryInterpolationMode,
    BoundarySignalExchanger, DigitalValueProvider,
};
pub use dc::{
    dc_analysis, BranchCurrentSample, DcAnalysisError, DcAnalysisRequest, DcAnalysisResult,
    DeviceModelBinding, OperatingPoint,
};
pub use dc_sweep::{dc_sweep, DcSweepError, DcSweepPoint, DcSweepRequest, DcSweepResult};
pub use mixed_signal::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, MixedSignalScheduler, NextEventReport, SchedulerError, SchedulerOutcome,
};
pub use noise::{
    collect_noise_sources, integrated_noise, noise_analysis, noise_analysis_with_auto_dc,
    DeviceNoiseContribution, IntegratedNoise, IntegratedNoiseError, IntegratedNoiseRequest,
    IntegrationBand, NoiseAnalysisData, NoiseAnalysisError, NoiseAnalysisRequest,
    NoiseAnalysisResult, NoiseAnalysisWithAutoDcError, NoiseAnalysisWithAutoDcRequest,
    NoiseAnalysisWithAutoDcResult, NoiseInjection, SemiconductorNoiseSource,
};
pub use sweep::{LogSweep, LogSweepError};
pub use transient::{
    transient_analysis, InitialState, IntegrationMethod, TransientAnalysisError,
    TransientAnalysisRequest, TransientAnalysisResult,
};
