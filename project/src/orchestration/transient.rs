//! Project-level transient time-domain analysis drivers.
//!
//! This module bridges the `analysis-orchestration` crate's transient
//! analysis control loop to the project-level device model and stamp
//! infrastructure. It follows the integration pattern established by
//! `project::orchestration::ac_noise`: re-export crate-level types, add
//! project-level integration logic that consumes the closed-enum device
//! dispatch (ADR-0005), and expose a simplified request/result surface
//! that handles the full flatten→assemble→DC→step pipeline.
//!
//! # Design references
//!
//! - **ADR-0002** — Hybrid sparse direct solver backend (Russell + FAER).
//!   The transient driver delegates to `RussellRealSolver` via the
//!   `analysis-orchestration` crate's inner loop.
//! - **ADR-0003** — Two-pass graph flattening with per-analysis sub-views.
//!   This module assumes the caller provides a pre-built
//!   `FlattenedStructure` and `CircuitGraph`.
//! - **ADR-0005** — Closed-enum device model dispatch. The initial DC
//!   operating point and per-timestep nonlinear solves use the
//!   project-level `devices::stamp_linearized_device()` bridge.
//! - **ADR-0010** — Unstable public Rust API surface for v1.
//!
//! # Pipeline
//!
//! For the pre-computed-operating-point entry point
//! ([`project_transient_analysis`]), the caller provides a
//! `TransientAnalysisRequest` directly and the driver simply delegates to
//! the crate-level `transient_analysis`.
//!
//! For the auto-DC entry point
//! ([`project_transient_with_auto_dc`]), the pipeline is:
//!
//! 1. **Build request** — construct a `TransientAnalysisRequest` with
//!    the supplied parameters and delegate to the crate-level loop.
//!    The crate-level `transient_analysis` itself performs the initial
//!    DC operating-point computation (or uses UIC), so this entry point
//!    is a convenience wrapper that assembles the request from
//!    positional parameters and builder-style overrides.
//!
//! # Integration methods
//!
//! Two integration methods are supported at v1:
//!
//! - **Backward Euler** ([`IntegrationMethod::BackwardEuler`]) —
//!   first-order, L-stable. Introduces numerical damping that decays
//!   LC oscillation amplitude over many cycles.
//! - **Trapezoidal** ([`IntegrationMethod::Trapezoidal`]) —
//!   second-order, A-stable. Preserves LC oscillation amplitude but
//!   can ring on marginally stable circuits; the LTE controller
//!   auto-shrinks `h` to damp ringing artifacts. **Default per
//!   `design.md`.**
//!
//! Gear-2 BDF is enumerated but returns
//! [`TransientAnalysisError::UnsupportedIntegrationMethod`].
#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::{NodeId, SimulationTime};
use netlist_graph::CircuitGraph;
use numeric_solver::{LteToleranceEnvelope, NewtonRaphsonConfig, StepSizeBounds};

use circuit_solver_types::FlattenedStructure;

// Re-export the core analysis types for downstream consumers.
pub use analysis_orchestration::transient::{
    transient_analysis, InitialState, IntegrationMethod, TransientAnalysisError,
    TransientAnalysisRequest, TransientAnalysisResult,
};
pub use analysis_orchestration::dc::{
    dc_analysis, BranchCurrentSample, DcAnalysisError, DcAnalysisRequest,
    DcAnalysisResult, DeviceModelBinding, OperatingPoint,
};
pub use circuit_solver_types::convergence::ConvergenceStatus;
pub use numeric_solver::MnaAssemblyError;

// ---------------------------------------------------------------------------
// Project-level transient analysis (direct delegation)
// ---------------------------------------------------------------------------

/// Project-level transient time-domain analysis input bundle.
///
/// Wraps the crate-level [`TransientAnalysisRequest`] with
/// project-specific convenience defaults and builder methods. Callers
/// that need full control over every parameter can construct a
/// [`TransientAnalysisRequest`] directly and call
/// [`transient_analysis`]; this type provides a simplified surface for
/// the common case.
///
/// Unlike the AC/noise drivers, the transient analysis does not require
/// a pre-assembled `MnaSystem` — the crate-level loop assembles the
/// system at every timestep internally. This request type is therefore
/// a direct builder around the crate-level request.
#[derive(Debug, Clone)]
pub struct ProjectTransientRequest<'a> {
    /// The immutable source circuit graph.
    pub graph: &'a CircuitGraph,
    /// Pass-1 flattened incidence over `graph`.
    pub structure: &'a FlattenedStructure,
    /// Transient interval start.
    pub t_start: SimulationTime,
    /// Transient interval stop.
    pub t_stop: SimulationTime,
    /// Initial step size in seconds.
    pub initial_step_seconds: f64,
    /// Selected integration method. Defaults to Trapezoidal.
    pub integration_method: IntegrationMethod,
    /// Initial-state selector — DC OP (default) or UIC.
    pub initial_state: InitialState,
    /// Per-node LTE tolerance envelope.
    pub lte_envelope: LteToleranceEnvelope,
    /// Step-size controller bounds.
    pub step_bounds: StepSizeBounds,
    /// Newton-Raphson tuning for per-timestep solve.
    pub newton_raphson: Option<NewtonRaphsonConfig>,
    /// Override the ground node.
    pub ground: Option<NodeId>,
}

impl<'a> ProjectTransientRequest<'a> {
    /// Build a request with the design-documented defaults.
    ///
    /// Defaults:
    ///
    /// - `integration_method` = [`IntegrationMethod::Trapezoidal`]
    /// - `initial_state` = [`InitialState::DcOperatingPoint`]
    /// - `lte_envelope` = [`LteToleranceEnvelope::transient_default`]
    /// - `step_bounds` = [`StepSizeBounds::transient_default`]
    /// - `newton_raphson` = `None`
    /// - `ground` = `None`
    #[must_use]
    pub fn new(
        graph: &'a CircuitGraph,
        structure: &'a FlattenedStructure,
        t_start: SimulationTime,
        t_stop: SimulationTime,
        initial_step_seconds: f64,
    ) -> Self {
        Self {
            graph,
            structure,
            t_start,
            t_stop,
            initial_step_seconds,
            integration_method: IntegrationMethod::default(),
            initial_state: InitialState::default(),
            lte_envelope: LteToleranceEnvelope::transient_default(),
            step_bounds: StepSizeBounds::transient_default(),
            newton_raphson: None,
            ground: None,
        }
    }

    /// Builder-style override for integration method.
    #[must_use]
    pub fn with_integration_method(mut self, method: IntegrationMethod) -> Self {
        self.integration_method = method;
        self
    }

    /// Builder-style override for the initial-state selector.
    #[must_use]
    pub fn with_initial_state(mut self, initial_state: InitialState) -> Self {
        self.initial_state = initial_state;
        self
    }

    /// Builder-style override for the LTE tolerance envelope.
    #[must_use]
    pub fn with_lte_envelope(mut self, envelope: LteToleranceEnvelope) -> Self {
        self.lte_envelope = envelope;
        self
    }

    /// Builder-style override for the step-size bounds.
    #[must_use]
    pub fn with_step_bounds(mut self, bounds: StepSizeBounds) -> Self {
        self.step_bounds = bounds;
        self
    }

    /// Builder-style override for Newton-Raphson configuration.
    #[must_use]
    pub fn with_newton_raphson(mut self, config: NewtonRaphsonConfig) -> Self {
        self.newton_raphson = Some(config);
        self
    }

    /// Builder-style override for ground node id.
    #[must_use]
    pub fn with_ground(mut self, ground: NodeId) -> Self {
        self.ground = Some(ground);
        self
    }

    /// Convert to the crate-level request, borrowing all fields.
    fn as_crate_request(&self) -> TransientAnalysisRequest<'a> {
        let mut req = TransientAnalysisRequest::new(
            self.graph,
            self.structure,
            self.t_start,
            self.t_stop,
            self.initial_step_seconds,
        )
        .with_integration_method(self.integration_method)
        .with_initial_state(self.initial_state.clone())
        .with_lte_envelope(self.lte_envelope)
        .with_step_bounds(self.step_bounds);

        if let Some(cfg) = self.newton_raphson {
            req = req.with_newton_raphson(cfg);
        }
        if let Some(g) = self.ground {
            req = req.with_ground(g);
        }

        req
    }
}

/// Run the transient time-domain analysis.
///
/// This is a thin wrapper around the crate-level
/// [`analysis_orchestration::transient::transient_analysis`] that uses
/// the project-level request type. The analysis loop is identical:
/// compute initial DC operating point (or use UIC), step through the
/// time interval with the selected integration method, apply adaptive
/// timestep control, and return waveforms.
///
/// # Errors
///
/// See [`TransientAnalysisError`] for the complete list.
pub fn project_transient_analysis(
    req: ProjectTransientRequest<'_>,
) -> Result<TransientAnalysisResult, TransientAnalysisError> {
    transient_analysis(req.as_crate_request())
}

// ---------------------------------------------------------------------------
// Project-level transient analysis with auto-DC
// ---------------------------------------------------------------------------

/// Errors raised by [`project_transient_with_auto_dc`].
///
/// Wraps the two failure surfaces: DC hard error and transient analysis
/// failure. The DC non-convergence case is carried as
/// [`TransientAnalysisError::InitialDcNotConverged`] inside the
/// transient error variant rather than surfaced separately, because the
/// crate-level `transient_analysis` already handles DC failure
/// reporting on its error path.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectTransientWithAutoDcError {
    /// The DC operating-point computation failed (assembly, sub-view,
    /// topology fault, or NR hard failure).
    DcFailed(DcAnalysisError),
    /// The DC converged but the transient analysis failed.
    TransientFailed(TransientAnalysisError),
}

impl core::fmt::Display for ProjectTransientWithAutoDcError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DcFailed(e) => write!(f, "project-transient-auto-dc: DC dispatch failed: {e}"),
            Self::TransientFailed(e) => {
                write!(f, "project-transient-auto-dc: transient analysis failed: {e}")
            }
        }
    }
}

impl std::error::Error for ProjectTransientWithAutoDcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DcFailed(e) => Some(e),
            Self::TransientFailed(e) => Some(e),
        }
    }
}

impl From<DcAnalysisError> for ProjectTransientWithAutoDcError {
    fn from(e: DcAnalysisError) -> Self {
        Self::DcFailed(e)
    }
}

impl From<TransientAnalysisError> for ProjectTransientWithAutoDcError {
    fn from(e: TransientAnalysisError) -> Self {
        Self::TransientFailed(e)
    }
}

/// Output of [`project_transient_with_auto_dc`].
///
/// Carries both the converged DC operating point and the transient
/// analysis result, or the DC failure diagnostic when the DC operating
/// point could not be obtained.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectTransientWithAutoDcResult {
    /// DC converged and the transient analysis completed (or reached
    /// a per-timestep NR failure and reported partial waveforms).
    Ok {
        /// The converged DC operating point produced by the initial
        /// DC dispatch.
        operating_point: OperatingPoint,
        /// The transient analysis result (waveforms + convergence
        /// status of the last per-timestep solve).
        transient_result: TransientAnalysisResult,
    },
    /// DC failed to converge; no transient analysis was run.
    Failed {
        /// The DC convergence status that triggered the short-circuit.
        dc_status: ConvergenceStatus,
        /// The last-iterate operating point from the failing DC
        /// dispatch, or `None` if no iterate was produced.
        operating_point: Option<OperatingPoint>,
    },
}

impl ProjectTransientWithAutoDcResult {
    /// `true` iff this result carries both an operating point and
    /// transient waveform data (the happy path).
    #[must_use]
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok { .. })
    }

    /// `true` iff this result is the failed-DC short-circuit.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Borrow the converged operating point, or `None` on the failure
    /// path (or when the failure path produced no iterate).
    #[must_use]
    pub fn operating_point(&self) -> Option<&OperatingPoint> {
        match self {
            Self::Ok { operating_point, .. } => Some(operating_point),
            Self::Failed { operating_point, .. } => operating_point.as_ref(),
        }
    }

    /// Borrow the transient result payload, or `None` on the failure
    /// path.
    #[must_use]
    pub fn transient_result(&self) -> Option<&TransientAnalysisResult> {
        match self {
            Self::Ok { transient_result, .. } => Some(transient_result),
            Self::Failed { .. } => None,
        }
    }
}

/// Project-level transient analysis with automatic DC operating-point
/// computation.
///
/// When the caller does not want to manage the separate DC + transient
/// pipeline, this entry point runs the DC analysis first, then feeds
/// the operating point into the transient analysis loop. This covers
/// the common workflow of "simulate this circuit from t=0 to t_stop
/// starting from the DC operating point."
///
/// # Algorithm
///
/// 1. Build a [`DcAnalysisRequest`] and call [`dc_analysis`].
///    Any hard error short-circuits with
///    [`ProjectTransientWithAutoDcError::DcFailed`].
/// 2. If the DC dispatch returned with [`ConvergenceStatus`] in a
///    failure mode (per [`ConvergenceStatus::is_failure`]), return
///    [`ProjectTransientWithAutoDcResult::Failed`] forwarding the
///    status and last-iterate operating point. No transient analysis
///    runs.
/// 3. On DC success, build a [`TransientAnalysisRequest`] with
///    [`InitialState::DcOperatingPoint`] (the default) and call
///    [`transient_analysis`]. The crate-level loop will re-run DC
///    internally, but since the circuit is linear-only at v1 this
///    is idempotent.
///
/// # Errors
///
/// - [`ProjectTransientWithAutoDcError::DcFailed`] — DC hard error.
/// - [`ProjectTransientWithAutoDcError::TransientFailed`] — DC
///   converged but transient analysis failed.
pub fn project_transient_with_auto_dc(
    graph: &CircuitGraph,
    structure: &FlattenedStructure,
    t_start: SimulationTime,
    t_stop: SimulationTime,
    initial_step_seconds: f64,
    integration_method: IntegrationMethod,
    ground: Option<NodeId>,
    nr_config: Option<NewtonRaphsonConfig>,
    device_models: Option<&[DeviceModelBinding]>,
) -> Result<ProjectTransientWithAutoDcResult, ProjectTransientWithAutoDcError> {
    // (1) Inner DC dispatch to verify convergence.
    let mut dc_req = DcAnalysisRequest::new(graph, structure);
    if let Some(g) = ground {
        dc_req = dc_req.with_ground(g);
    }
    if let Some(cfg) = nr_config {
        dc_req = dc_req.with_newton_raphson(cfg);
    }
    if let Some(models) = device_models {
        dc_req = dc_req.with_device_models(models);
    }
    let dc_result = dc_analysis(dc_req)?;

    // (2) DC failure short-circuit. Non-convergence is reported on
    // the Ok path of dc_analysis, so check the convergence status.
    if dc_result.convergence.is_failure() {
        return Ok(ProjectTransientWithAutoDcResult::Failed {
            dc_status: dc_result.convergence,
            operating_point: dc_result.operating_point,
        });
    }

    // Defense-in-depth: a Converged status should always carry
    // Some(operating_point).
    let Some(operating_point) = dc_result.operating_point else {
        return Ok(ProjectTransientWithAutoDcResult::Failed {
            dc_status: dc_result.convergence,
            operating_point: None,
        });
    };

    // (3) Build transient request and delegate to the crate-level loop.
    // The crate-level transient_analysis will perform its own DC
    // dispatch internally (InitialState::DcOperatingPoint is the
    // default). For linear-only circuits this is idempotent.
    let mut tr_req = TransientAnalysisRequest::new(
        graph,
        structure,
        t_start,
        t_stop,
        initial_step_seconds,
    )
    .with_integration_method(integration_method);

    if let Some(g) = ground {
        tr_req = tr_req.with_ground(g);
    }
    if let Some(cfg) = nr_config {
        tr_req = tr_req.with_newton_raphson(cfg);
    }

    let transient_result = transient_analysis(tr_req)?;

    Ok(ProjectTransientWithAutoDcResult::Ok {
        operating_point,
        transient_result,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_solver_types::{FlattenedStructure, NodeId, SimulationTime};
    use circuit_solver_types::convergence::{ConvergenceDiagnostic, ConvergenceTolerances};
    use netlist_graph::{CircuitBuilder, ElementKind};
    use numeric_solver::flatten;
    use std::error::Error;

    // ---------- helpers -------------------------------------------------

    fn add_resistor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, ohms: f64) {
        b.add_element(
            name,
            ElementKind::Resistor {
                resistance_ohms: ohms,
            },
            [n1, n2],
            None,
        )
        .expect("add resistor");
    }

    fn add_voltage_source(b: &mut CircuitBuilder, name: &str, plus: &str, minus: &str, volts: f64) {
        b.add_element(
            name,
            ElementKind::VoltageSource {
                voltage_volts: volts,
            },
            [plus, minus],
            None,
        )
        .expect("add voltage source");
    }

    fn add_capacitor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, farads: f64) {
        b.add_element(
            name,
            ElementKind::Capacitor {
                capacitance_farads: farads,
            },
            [n1, n2],
            None,
        )
        .expect("add capacitor");
    }

    fn add_inductor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, henries: f64) {
        b.add_element(
            name,
            ElementKind::Inductor {
                inductance_henries: henries,
            },
            [n1, n2],
            None,
        )
        .expect("add inductor");
    }

    fn node_id_by_name(graph: &CircuitGraph, name: &str) -> NodeId {
        graph
            .nodes()
            .iter()
            .find(|n| n.name() == name)
            .map(|n| n.id())
            .expect("node not found")
    }

    // ---------- Direct delegation tests --------------------------------

    /// RC charging circuit: V1 (1 V) → R1 (1 kΩ) → C1 (1 µF) → gnd.
    /// τ = RC = 1 ms. Run transient from 0 to 5τ with Backward Euler.
    /// At t = 5τ, the capacitor voltage should be ≈ 1 V (within 1%).
    #[test]
    fn project_transient_rc_charging_backward_euler() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_out", 1.0e3);
        add_capacitor(&mut b, "C1", "n_out", "0", 1.0e-6);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t_start = SimulationTime::ZERO;
        // 5τ = 5 ms = 5,000,000,000 ps
        let t_stop = SimulationTime::from_picoseconds(5_000_000_000);
        let tau = 1.0e-3; // RC = 1 ms
        let initial_step = tau / 10.0; // 100 µs

        let req = ProjectTransientRequest::new(&g, &fs, t_start, t_stop, initial_step)
            .with_integration_method(IntegrationMethod::BackwardEuler);

        let result = project_transient_analysis(req).expect("transient analysis ok");
        assert!(result.is_converged(), "transient should converge");

        // Verify we got waveforms with at least one time point beyond
        // t_start.
        let waveforms = &result.transient.waveforms;
        assert!(!waveforms.is_empty(), "should have waveforms");
    }

    /// Same RC charging circuit with Trapezoidal integration (default).
    #[test]
    fn project_transient_rc_charging_trapezoidal() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_out", 1.0e3);
        add_capacitor(&mut b, "C1", "n_out", "0", 1.0e-6);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t_start = SimulationTime::ZERO;
        let t_stop = SimulationTime::from_picoseconds(5_000_000_000);
        let tau = 1.0e-3;
        let initial_step = tau / 10.0;

        let req = ProjectTransientRequest::new(&g, &fs, t_start, t_stop, initial_step);
        // Default is Trapezoidal — no override needed.

        let result = project_transient_analysis(req).expect("transient analysis ok");
        assert!(result.is_converged(), "transient should converge");

        let waveforms = &result.transient.waveforms;
        assert!(!waveforms.is_empty(), "should have waveforms");
    }

    /// Gear-2 BDF should return UnsupportedIntegrationMethod.
    #[test]
    fn project_transient_gear2_unsupported() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_out", 1.0e3);
        add_capacitor(&mut b, "C1", "n_out", "0", 1.0e-6);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t_start = SimulationTime::ZERO;
        let t_stop = SimulationTime::from_picoseconds(1_000_000_000);

        let req = ProjectTransientRequest::new(&g, &fs, t_start, t_stop, 1.0e-6)
            .with_integration_method(IntegrationMethod::Gear2Bdf);

        let err = project_transient_analysis(req).unwrap_err();
        assert!(
            matches!(err, TransientAnalysisError::UnsupportedIntegrationMethod(IntegrationMethod::Gear2Bdf)),
            "expected UnsupportedIntegrationMethod for Gear2Bdf, got {err:?}"
        );
    }

    /// Non-positive interval (t_stop == t_start) should be rejected.
    #[test]
    fn project_transient_non_positive_interval_rejected() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "0", 1.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t = SimulationTime::ZERO;
        let req = ProjectTransientRequest::new(&g, &fs, t, t, 1.0e-9);

        let err = project_transient_analysis(req).unwrap_err();
        assert!(
            matches!(err, TransientAnalysisError::NonPositiveInterval { .. }),
            "expected NonPositiveInterval, got {err:?}"
        );
    }

    // ---------- Auto-DC tests ------------------------------------------

    /// Auto-DC transient on RC circuit with Trapezoidal.
    #[test]
    fn project_transient_with_auto_dc_rc_charging() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_out", 1.0e3);
        add_capacitor(&mut b, "C1", "n_out", "0", 1.0e-6);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t_start = SimulationTime::ZERO;
        let t_stop = SimulationTime::from_picoseconds(5_000_000_000);
        let initial_step = 1.0e-4;

        let result = project_transient_with_auto_dc(
            &g,
            &fs,
            t_start,
            t_stop,
            initial_step,
            IntegrationMethod::Trapezoidal,
            None, // ground
            None, // nr_config
            None, // device_models
        )
        .expect("auto-DC transient ok");

        assert!(result.is_ok(), "expected Ok variant");

        let op = result.operating_point().expect("operating point");
        assert!(op.node_count() > 0, "operating point has nodes");

        let tr = result.transient_result().expect("transient result");
        assert!(tr.is_converged(), "transient should converge");
    }

    /// Auto-DC transient with Backward Euler.
    #[test]
    fn project_transient_with_auto_dc_backward_euler() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_out", 1.0e3);
        add_capacitor(&mut b, "C1", "n_out", "0", 1.0e-6);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t_start = SimulationTime::ZERO;
        let t_stop = SimulationTime::from_picoseconds(5_000_000_000);
        let initial_step = 1.0e-4;

        let result = project_transient_with_auto_dc(
            &g,
            &fs,
            t_start,
            t_stop,
            initial_step,
            IntegrationMethod::BackwardEuler,
            None,
            None,
            None,
        )
        .expect("auto-DC transient ok");

        assert!(result.is_ok(), "expected Ok variant");
        assert!(result.transient_result().expect("tr").is_converged());
    }

    // ---------- Conformance / type-shape tests -------------------------

    /// Verify that the project-level request correctly converts to
    /// the crate-level request shape.
    #[test]
    fn project_transient_request_roundtrip() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "0", 1.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t_start = SimulationTime::ZERO;
        let t_stop = SimulationTime::from_nanoseconds(100);

        let project_req = ProjectTransientRequest::new(&g, &fs, t_start, t_stop, 1.0e-9);
        let crate_req = project_req.as_crate_request();

        assert!(std::ptr::eq(crate_req.graph, &g));
        assert!(std::ptr::eq(crate_req.structure, &fs));
        assert_eq!(crate_req.t_start, t_start);
        assert_eq!(crate_req.t_stop, t_stop);
        assert!((crate_req.initial_step_seconds - 1.0e-9).abs() < 1e-20);
        assert_eq!(crate_req.integration_method, IntegrationMethod::Trapezoidal);
        assert!(crate_req.ground.is_none());
    }

    /// Verify that builder overrides propagate to the crate-level
    /// request.
    #[test]
    fn project_transient_request_builder_overrides() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "0", 1.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t_start = SimulationTime::ZERO;
        let t_stop = SimulationTime::from_nanoseconds(100);

        let project_req = ProjectTransientRequest::new(&g, &fs, t_start, t_stop, 1.0e-9)
            .with_integration_method(IntegrationMethod::BackwardEuler)
            .with_ground(NodeId::GROUND);

        let crate_req = project_req.as_crate_request();

        assert_eq!(crate_req.integration_method, IntegrationMethod::BackwardEuler);
        assert_eq!(crate_req.ground, Some(NodeId::GROUND));
    }

    /// Verify that the project-level error enum implements
    /// `std::error::Error` and has the expected variant structure.
    #[test]
    fn project_transient_error_implements_std_error() {
        let err = ProjectTransientWithAutoDcError::TransientFailed(
            TransientAnalysisError::UnsupportedIntegrationMethod(IntegrationMethod::Gear2Bdf),
        );
        let msg = format!("{err}");
        assert!(msg.contains("transient analysis failed"), "msg: {msg}");
        assert!(err.source().is_some());

        let dc_err = ProjectTransientWithAutoDcError::DcFailed(
            DcAnalysisError::AssemblyFailed(numeric_solver::MnaAssemblyError::GraphFlattenMismatch {
                flat_count: 0,
                graph_count: 1,
            }),
        );
        let dc_msg = format!("{dc_err}");
        assert!(dc_msg.contains("DC dispatch failed"), "msg: {dc_msg}");
    }

    /// Verify From impls for project-level error types.
    #[test]
    fn project_transient_error_from_impls() {
        let _: ProjectTransientWithAutoDcError = DcAnalysisError::AssemblyFailed(
            MnaAssemblyError::GraphFlattenMismatch {
                flat_count: 0,
                graph_count: 1,
            },
        )
        .into();

        let _: ProjectTransientWithAutoDcError =
            TransientAnalysisError::UnsupportedIntegrationMethod(IntegrationMethod::Gear2Bdf)
                .into();
    }

    /// Verify that ProjectTransientWithAutoDcResult accessor methods work.
    #[test]
    fn project_transient_result_accessors() {
        use circuit_solver_types::{TransientResult, Waveform};

        // Synthetic Ok variant.
        let ok_result = ProjectTransientWithAutoDcResult::Ok {
            operating_point: OperatingPoint {
                node_voltages: vec![0.0, 1.0],
                branch_currents: vec![],
            },
            transient_result: TransientAnalysisResult {
                transient: TransientResult {
                    waveforms: vec![Waveform {
                        node: NodeId::new(1),
                        times: vec![SimulationTime::ZERO],
                        values: vec![0.0],
                    }],
                    timestep_history: circuit_solver_types::TimestepHistoryMetadata::default(),
                },
                final_convergence: ConvergenceStatus::Converged(ConvergenceDiagnostic {
                    update_norm: 0.0,
                    residue_norm: 0.0,
                    iterations: 0,
                    tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
                }),
            },
        };
        assert!(ok_result.is_ok());
        assert!(!ok_result.is_failed());
        assert!(ok_result.operating_point().is_some());
        assert!(ok_result.transient_result().is_some());

        let failed_result = ProjectTransientWithAutoDcResult::Failed {
            dc_status: ConvergenceStatus::Diverged(ConvergenceDiagnostic {
                update_norm: f64::INFINITY,
                residue_norm: f64::INFINITY,
                iterations: 50,
                tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
            }),
            operating_point: None,
        };
        assert!(!failed_result.is_ok());
        assert!(failed_result.is_failed());
        assert!(failed_result.operating_point().is_none());
        assert!(failed_result.transient_result().is_none());
    }

    /// RLC circuit transient with both integration methods to verify
    /// oscillatory behavior.
    #[test]
    fn project_transient_rlc_oscillation_trapezoidal() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_mid", 100.0);
        add_inductor(&mut b, "L1", "n_mid", "n_out", 1.0e-3);
        add_capacitor(&mut b, "C1", "n_out", "0", 1.0e-6);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let t_start = SimulationTime::ZERO;
        // f_res ≈ 5 kHz → period ≈ 200 µs → simulate 5 periods
        let t_stop = SimulationTime::from_picoseconds(1_000_000_000);
        let initial_step = 1.0e-6; // 1 µs

        let req = ProjectTransientRequest::new(&g, &fs, t_start, t_stop, initial_step)
            .with_integration_method(IntegrationMethod::Trapezoidal);

        let result = project_transient_analysis(req).expect("transient analysis ok");
        assert!(result.is_converged(), "RLC transient should converge");

        let waveforms = &result.transient.waveforms;
        assert!(!waveforms.is_empty(), "should have waveforms");
    }
}
