//! Analysis-type discriminator.
//!
//! The v1 spec defines six analysis capabilities, each one a separate
//! `specs/*` document under the change manifest:
//!
//! - **DC operating point** (`dc-operating-point`),
//! - **DC sweep** (a parameterized iteration of DC operating points,
//!   carried as its own variant because the request shape and Result
//!   shape differ),
//! - **AC small-signal** (`ac-small-signal`),
//! - **Transient time-domain** (`transient-time-domain`),
//! - **Noise spectral density** (`noise-spectral-density`),
//! - **Mixed-signal co-simulation** (`mixed-signal-cosim`).
//!
//! `AnalysisType` is the closed enum that the analysis-orchestration
//! layer uses to dispatch on the requested capability. It is
//! intentionally separate from the `AnalysisRequest` parameter
//! envelopes (which carry the per-analysis numerics), so that
//! plumbing — selecting a control loop, choosing a `LinearSolver`
//! backend per ADR-0002, selecting an integration method — can route
//! purely on the tag without inspecting the full request.

use core::fmt;

/// The kind of analysis to perform.
///
/// `AnalysisType` is `Copy + Eq + Hash`, suitable for use as a key in
/// dispatch tables and as a tag on cached `OperatingPoint` /
/// `FlattenedStructure` artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnalysisType {
    /// A single DC operating-point solve.
    ///
    /// Drives the Newton-Raphson loop with the real-valued
    /// `russell_sparse` LU backend (ADR-0002). The Result is an
    /// `OperatingPoint` plus `ConvergenceStatus`.
    DcOperatingPoint,

    /// A DC sweep over a source parameter range.
    ///
    /// Yields one `OperatingPoint` per sweep point. Distinct from
    /// `DcOperatingPoint` because the Result is a vector addressable
    /// by sweep index.
    DcSweep,

    /// Small-signal AC analysis linearized at a precomputed
    /// `OperatingPoint`.
    ///
    /// Drives a complex-valued sparse-LU dispatch (`faer`, ADR-0002)
    /// at each frequency point. Yields a `TransferFunction`.
    AcSmallSignal,

    /// Transient time-domain analysis with one of the supported
    /// integration methods (Backward Euler, Trapezoidal, Gear-2 BDF).
    ///
    /// Drives the Newton-Raphson loop per timestep with adaptive LTE
    /// control. Yields a `Waveform` per observed node.
    Transient,

    /// Output-referred noise spectral-density analysis.
    ///
    /// Linearizes at an operating point, assembles per-device noise
    /// stamps (thermal, shot, flicker), and solves the noise transfer
    /// matrix at each frequency. Shares the `faer` complex backend
    /// with AC.
    Noise,

    /// Mixed-signal co-simulation against an external digital event
    /// simulator (Icarus Verilog or Verilator).
    ///
    /// Coordinated by the Mixed-Signal Scheduler (ADR-0004); yields a
    /// `MixedSignalResult` containing analog `Waveform`s and digital
    /// VCD event traces.
    MixedSignal,
}

impl AnalysisType {
    /// True iff this analysis requires the complex-valued
    /// (`faer`-backed, ADR-0002) sparse-LU dispatch. Pure-real
    /// analyses use `russell_sparse`.
    #[must_use]
    pub const fn requires_complex_solver(self) -> bool {
        matches!(self, Self::AcSmallSignal | Self::Noise)
    }

    /// True iff this analysis advances simulation time and therefore
    /// requires a `SimulationTime` interval and (potentially) timestep
    /// control. The non-time-advancing analyses (`DcOperatingPoint`,
    /// `DcSweep`, `AcSmallSignal`, `Noise`) solve at a single
    /// time-invariant or steady-state condition.
    #[must_use]
    pub const fn is_time_domain(self) -> bool {
        matches!(self, Self::Transient | Self::MixedSignal)
    }

    /// True iff this analysis must be linearized around an existing
    /// `OperatingPoint`. AC and Noise both linearize at a DC bias; if
    /// no cached operating point is available, the orchestrator
    /// auto-dispatches a `DcOperatingPoint` solve first (see tasks.md
    /// items #26 and #40, and `specs/ac-small-signal` /
    /// `specs/noise-spectral-density`).
    #[must_use]
    pub const fn needs_operating_point(self) -> bool {
        matches!(self, Self::AcSmallSignal | Self::Noise)
    }

    /// A stable slug suitable for logs, diagnostics, and Result tags.
    /// The slug matches the corresponding `specs/<slug>` directory
    /// where applicable.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::DcOperatingPoint => "dc-operating-point",
            Self::DcSweep => "dc-sweep",
            Self::AcSmallSignal => "ac-small-signal",
            Self::Transient => "transient-time-domain",
            Self::Noise => "noise-spectral-density",
            Self::MixedSignal => "mixed-signal-cosim",
        }
    }
}

impl fmt::Display for AnalysisType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.slug())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs_match_specs_directories() {
        assert_eq!(AnalysisType::DcOperatingPoint.slug(), "dc-operating-point");
        assert_eq!(AnalysisType::AcSmallSignal.slug(), "ac-small-signal");
        assert_eq!(AnalysisType::Transient.slug(), "transient-time-domain");
        assert_eq!(AnalysisType::Noise.slug(), "noise-spectral-density");
        assert_eq!(AnalysisType::MixedSignal.slug(), "mixed-signal-cosim");
    }

    #[test]
    fn ac_and_noise_use_complex_solver() {
        assert!(AnalysisType::AcSmallSignal.requires_complex_solver());
        assert!(AnalysisType::Noise.requires_complex_solver());
        assert!(!AnalysisType::DcOperatingPoint.requires_complex_solver());
        assert!(!AnalysisType::DcSweep.requires_complex_solver());
        assert!(!AnalysisType::Transient.requires_complex_solver());
        assert!(!AnalysisType::MixedSignal.requires_complex_solver());
    }

    #[test]
    fn time_domain_set_is_transient_and_mixed_signal() {
        assert!(AnalysisType::Transient.is_time_domain());
        assert!(AnalysisType::MixedSignal.is_time_domain());
        assert!(!AnalysisType::DcOperatingPoint.is_time_domain());
        assert!(!AnalysisType::DcSweep.is_time_domain());
        assert!(!AnalysisType::AcSmallSignal.is_time_domain());
        assert!(!AnalysisType::Noise.is_time_domain());
    }

    #[test]
    fn ac_and_noise_need_an_operating_point() {
        assert!(AnalysisType::AcSmallSignal.needs_operating_point());
        assert!(AnalysisType::Noise.needs_operating_point());
        assert!(!AnalysisType::DcOperatingPoint.needs_operating_point());
        assert!(!AnalysisType::Transient.needs_operating_point());
        assert!(!AnalysisType::MixedSignal.needs_operating_point());
    }

    #[test]
    fn display_matches_slug() {
        assert_eq!(
            format!("{}", AnalysisType::Transient),
            "transient-time-domain"
        );
    }
}
