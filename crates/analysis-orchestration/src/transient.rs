//! Transient time-domain analysis control loop.
//!
//! This module covers `tasks.md` item #33 of
//! `circuit-solver/2026-05-21-v1-spec`. It is the per-analysis driver
//! that composes:
//!
//! - [`crate::dc::dc_analysis`] (tasks.md #20) for the initial DC
//!   operating point,
//! - the [`numeric_solver::integration::backward_euler`] and
//!   [`numeric_solver::integration::trapezoidal`] companion stamps
//!   (tasks.md #29 / #30) for reactive-element discretization at each
//!   timestep,
//! - [`numeric_solver::assemble()`] (tasks.md #14) for the per-timestep
//!   MNA base assembly,
//! - [`numeric_solver::NewtonRaphsonDriver`] (tasks.md #17) for the
//!   per-timestep nonlinear solve, and
//! - [`numeric_solver::integration::adaptive`] (tasks.md #32) for
//!   local-truncation-error step decisions,
//!
//! into a single end-to-end transient analysis that accepts a
//! [`TransientAnalysisRequest`] and returns a
//! [`TransientAnalysisResult`] containing
//! [`circuit_solver_types::TransientResult`] (`Waveform`s plus
//! [`circuit_solver_types::TimestepHistoryMetadata`]).
//!
//! # Spec scope (item #33)
//!
//! `tasks.md` item #33 — *"Implement transient analysis control loop:
//! compute initial DC operating point (or accept UIC), step through
//! time interval with selected integration method, return
//! Waveforms"* — maps to the
//! [`transient-time-domain#transient-analysis-with-default-integration-method`][spec]
//! scenario:
//!
//! > Given CircuitDesigner has constructed a Circuit with a pulsed
//! > voltage source
//! > And the transient time interval is 0 s to 100 ns
//! > When CircuitDesigner submits a transient Analysis request
//! > Then the Simulator computes a DC OperatingPoint as the initial state
//! > And the Simulator returns a Result containing Waveforms for all observed nodes
//! > And every Waveform matches the Golden Reference within the
//! > tolerance envelope at every time point
//!
//! Sibling scenarios that compose **on top of** this control loop
//! (and are out of scope for this task):
//!
//! - `transient-analysis-with-trapezoidal-integration` (covered by
//!   choosing [`IntegrationMethod::Trapezoidal`] — wired here).
//! - `transient-analysis-with-gear-2-bdf-integration` — Gear-2 BDF
//!   is enumerated in [`IntegrationMethod`] but not wired in this
//!   task; its companion stamps and Gear-2 startup logic
//!   ([`numeric_solver::integration::gear2`]) require a two-step
//!   history rotation which a follow-up scope owns.
//! - `adaptive-timestepping-rejects-and-re-solves` — the LTE
//!   controller from tasks.md #32 is used here verbatim; this
//!   module's contribution is wiring it into the outer loop.
//! - `transient-analysis-with-uic-initial-conditions` — covered by
//!   choosing [`InitialState::UseInitialConditions`].
//! - `transient-conformance-against-ngspice` — covered by the
//!   conformance harness (tasks.md #62+), which consumes the
//!   [`circuit_solver_types::TransientResult`] this module produces.
//!
//! # Default integration method
//!
//! Per `design.md` row "Trapezoidal ringing | Three integration
//! methods offered (BE, TR, Gear-2); default TR with documented
//! ringing risk; LTE auto-shrink damps artifact." the default is
//! [`IntegrationMethod::Trapezoidal`]. The scenario phrasing "default
//! integration method" is interpreted against `design.md`'s
//! documented default, not against the upstream LTE estimator's
//! Backward-Euler order constant (which is the controller's default
//! *for LTE estimation*, distinct from the *per-element companion
//! stamp* method).
//!
//! # Design references
//!
//! - **ADR-0002 — Sparse Direct LU Dispatch.** The transient inner
//!   solves use the real-valued
//!   [`numeric_solver::RussellRealSolver`] backend at every NR
//!   iteration at every timestep.
//! - **ADR-0006 — Dual Convergence Criterion for Newton-Raphson.**
//!   Each per-timestep NR solve honors ADR-0006 verbatim.
//! - **ADR-0007 — Zero-Order Hold at Analog-Digital Boundary.**
//!   Vacuous at this layer — no A/D boundary surface.
//! - **ADR-0008 — Per-Node max(Relative, Absolute) Tolerance
//!   Envelope.** The LTE controller's
//!   [`numeric_solver::LteToleranceEnvelope`] applies the same
//!   max(rel, abs) shape used by the conformance harness; defaults
//!   are the transient row of `design.md` QAS-2 (`rel = 1 %`,
//!   `abs = 1 mV`).
//! - **ADR-0009 — Topology Checker for Floating-Node Detection.**
//!   Honored transitively via [`crate::dc::dc_analysis`]: if the
//!   initial DC OP short-circuits with
//!   [`crate::dc::DcAnalysisError::FloatingNodeFault`] this module
//!   propagates the error before any time-domain stepping starts.
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** Every
//!   public surface exported here is part of the v1 *unstable*
//!   public Rust API.
//!
//! # What this module does *not* do
//!
//! - **No time-varying sources at v1.** The MNA assembler (tasks.md
//!   #14) reads a single DC value for [`VoltageSource`] and
//!   [`CurrentSource`]. A future change will lift this restriction
//!   by adding a time-dependent waveform descriptor; until then, the
//!   transient analysis exercises only the reactive-element
//!   dynamics (capacitors, inductors) with constant-source
//!   excitation. The Gherkin scenario's "pulsed voltage source"
//!   maps to a constant V-source under the v1 contract; the witness
//!   exercises the dynamic response of an RC/RL/RLC circuit to that
//!   constant excitation, not the pulse shape.
//! - **No Gear-2 BDF.** Enumerated in [`IntegrationMethod`] but
//!   returning [`TransientAnalysisError::UnsupportedIntegrationMethod`]
//!   from the entry point. A follow-up task owns the BDF startup
//!   logic.
//! - **No frontend translation.** The `PyO3` layer (tasks.md #56+)
//!   converts user-supplied netlists into the
//!   [`netlist_graph::CircuitGraph`] +
//!   [`circuit_solver_types::flattened::FlattenedStructure`] pair
//!   this module consumes.
//!
//! [spec]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/transient-time-domain/spec.md
//! [`VoltageSource`]: netlist_graph::ElementKind::VoltageSource
//! [`CurrentSource`]: netlist_graph::ElementKind::CurrentSource

#![allow(
    clippy::module_name_repetitions,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::many_single_char_names,
    clippy::collapsible_match,
    clippy::doc_markdown,
    clippy::doc_overindented_list_items,
    clippy::items_after_statements,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::needless_pass_by_value
)]

use std::collections::HashMap;

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::{
    ConvergenceStatus, NodeId, SimulationTime, TimestepHistoryMetadata, TransientResult, Waveform,
};
use netlist_graph::{CircuitGraph, ElementKind};
use numeric_solver::integration::{
    backward_euler as be, trapezoidal as tr, CapacitorHistory, InductorHistory,
};
use numeric_solver::{
    assemble, next_step_size, step_decision, LteEstimator, LteToleranceEnvelope, MnaAssemblyError,
    NewtonRaphsonConfig, NewtonRaphsonDriver, NewtonRaphsonError, NodeHistorySample,
    NonlinearSystem, RussellRealSolver, SparseLinearSystem, SparseTriplet, StepOutcome,
    StepSizeBounds, SubViewError, SystemError as NrSystemError, TimestepHistory, TimestepRecord,
};

use crate::dc::{dc_analysis, DcAnalysisError, DcAnalysisRequest};

// -----------------------------------------------------------------------------
// Request / Result envelopes
// -----------------------------------------------------------------------------

/// The implicit integration method selected for a transient analysis.
///
/// Per the spec's acceptance criterion *"The Simulator supports at
/// least three implicit integration methods: Backward Euler,
/// Trapezoidal, and Gear-2 BDF, selectable per analysis request."*
/// this enum is the user-facing selector. Each variant corresponds
/// to a sibling module under
/// [`numeric_solver::integration`] (tasks.md #29 / #30 / #31).
///
/// The [`IntegrationMethod::Trapezoidal`] variant is the
/// `design.md`-documented default for transient analyses; see the
/// module-level docstring for the rationale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegrationMethod {
    /// Backward Euler (first-order, L-stable). Stable on stiff
    /// problems but injects numerical damping that decays the
    /// amplitude of lossless LC oscillators over many cycles
    /// (`design.md` known pitfall). Companion stamps:
    /// [`numeric_solver::integration::backward_euler`].
    BackwardEuler,
    /// Trapezoidal Rule (second-order, A-stable but **not**
    /// L-stable). Preserves the amplitude of lossless LC oscillators
    /// but can ring on marginally stable circuits; the LTE
    /// controller auto-shrinks `h` when ringing is detected
    /// (`design.md` known-pitfall mitigation). Companion stamps:
    /// [`numeric_solver::integration::trapezoidal`]. **Default for
    /// new [`TransientAnalysisRequest`]s.**
    Trapezoidal,
    /// Gear-2 Backward Differentiation Formula (second-order,
    /// L-stable). Best for stiff non-LC problems where Trapezoidal
    /// rings; uses a two-step history rotation. Companion stamps:
    /// [`numeric_solver::integration::gear2`]. **Not yet wired in
    /// this task** — selecting it returns
    /// [`TransientAnalysisError::UnsupportedIntegrationMethod`]; a
    /// follow-up scope adds the Gear-2 startup logic and history
    /// rotation.
    Gear2Bdf,
}

impl IntegrationMethod {
    /// The LTE-estimator order for the proportional step-size rule.
    /// Backward Euler is first-order (`p = 1`); Trapezoidal and
    /// Gear-2 BDF are second-order (`p = 2`).
    #[must_use]
    pub const fn lte_order(self) -> u32 {
        match self {
            Self::BackwardEuler => 1,
            Self::Trapezoidal | Self::Gear2Bdf => 2,
        }
    }
}

impl Default for IntegrationMethod {
    /// `Default` is [`IntegrationMethod::Trapezoidal`] per
    /// `design.md`'s documented transient default. See the
    /// module-level docstring for the rationale and the
    /// `Trapezoidal ringing` row of `design.md`'s known-pitfalls
    /// table.
    fn default() -> Self {
        Self::Trapezoidal
    }
}

/// How the transient analysis seeds reactive-element histories at
/// `t = t_start`.
///
/// Per the scenario *"And the Simulator skips the DC OperatingPoint
/// computation / And the Simulator starts the transient solve using
/// the user-supplied initial conditions"* the request can opt into
/// the `UIC` (Use Initial Conditions) glossary term. The default
/// (computing a DC OP) maps to the headline scenario's *"Then the
/// Simulator computes a DC OperatingPoint as the initial state"*.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum InitialState {
    /// Default. The control loop computes a DC operating point at
    /// `t = t_start` via [`crate::dc::dc_analysis`] and seeds every
    /// reactive element's history from that solution.
    #[default]
    DcOperatingPoint,
    /// `UIC` per the spec's glossary. The DC operating-point
    /// computation is skipped; reactive element histories are
    /// seeded from the user-supplied per-node voltages here.
    /// Unspecified nodes default to `0 V`; unspecified inductor
    /// branch currents default to `0 A`.
    UseInitialConditions {
        /// Map from `NodeId` to user-supplied initial voltage in
        /// volts. Nodes not present in the map start at `0 V`.
        node_voltages: HashMap<NodeId, f64>,
    },
}

/// Transient analysis input bundle.
///
/// The Gherkin scenario phrasing *"When CircuitDesigner submits a
/// transient Analysis request"* maps to one value of this type. The
/// `PyO3` frontend (tasks.md #56+) translates the Python-side
/// `AnalysisRequest` into a Rust-side `TransientAnalysisRequest`;
/// the orchestrator layer (this module) does not depend on `PyO3`.
///
/// All time values are [`SimulationTime`] (i64 picoseconds);
/// step sizes and tolerances are `f64` seconds and volts respectively
/// because the LTE controller operates in `f64` (it is a pure-compute
/// library independent of the `SimulationTime` type).
///
/// Per ADR-0010, the struct's *layout* is unstable for v1; the
/// *semantics* of each field are pinned.
#[derive(Debug, Clone)]
pub struct TransientAnalysisRequest<'a> {
    /// The immutable source circuit graph.
    pub graph: &'a CircuitGraph,
    /// Pass-1 flattened incidence over `graph`. Must satisfy
    /// `structure.element_count() == graph.elements().len()`.
    pub structure: &'a FlattenedStructure,
    /// Transient interval start. The DC operating point (or UIC
    /// state) is computed at this time.
    pub t_start: SimulationTime,
    /// Transient interval stop. The control loop terminates when
    /// the accumulated simulation time reaches or exceeds this
    /// value (within a `1 ps` tolerance).
    pub t_stop: SimulationTime,
    /// Initial step size in seconds. The LTE controller may grow
    /// or shrink subsequent steps, subject to [`Self::step_bounds`].
    /// The Gherkin scenario *"And the initial timestep is set to
    /// 1 ns"* maps to `1.0e-9`.
    pub initial_step_seconds: f64,
    /// Selected integration method. Defaults to
    /// [`IntegrationMethod::Trapezoidal`] per `design.md`.
    pub integration_method: IntegrationMethod,
    /// Initial-state selector — DC OP (default) or UIC.
    pub initial_state: InitialState,
    /// Per-node LTE tolerance envelope. Defaults to the
    /// `design.md` QAS-2 transient row (`rel = 1 %`, `abs = 1 mV`)
    /// via [`LteToleranceEnvelope::transient_default`].
    pub lte_envelope: LteToleranceEnvelope,
    /// Step-size controller bounds. Defaults to
    /// [`StepSizeBounds::transient_default`].
    pub step_bounds: StepSizeBounds,
    /// Newton-Raphson tuning for the per-timestep solve. `None`
    /// defaults to [`NewtonRaphsonConfig::DC_DEFAULTS`].
    pub newton_raphson: Option<NewtonRaphsonConfig>,
    /// Override the ground node. `None` defaults to
    /// [`FlattenedStructure::ground_node`].
    pub ground: Option<NodeId>,
}

impl<'a> TransientAnalysisRequest<'a> {
    /// Build a request with the design-documented defaults.
    ///
    /// Defaults set:
    ///
    /// - [`Self::integration_method`] = [`IntegrationMethod::Trapezoidal`]
    /// - [`Self::initial_state`] = [`InitialState::DcOperatingPoint`]
    /// - [`Self::lte_envelope`] =
    ///   [`LteToleranceEnvelope::transient_default`] (`1 %` rel,
    ///   `1 mV` abs per `design.md` QAS-2).
    /// - [`Self::step_bounds`] = [`StepSizeBounds::transient_default`].
    /// - [`Self::newton_raphson`] = `None` (defaults at solve time
    ///   to [`NewtonRaphsonConfig::DC_DEFAULTS`]).
    /// - [`Self::ground`] = `None` (defaults to the structure's
    ///   ground node).
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
}

/// The bundled result of a transient analysis.
///
/// On a successful run, [`Self::transient`] carries the
/// [`TransientResult`] (Waveforms + adaptive-timestepping
/// metadata) and [`Self::final_convergence`] is the convergence
/// status of the *last* per-timestep NR solve. On a per-step NR
/// hard failure, [`Self::final_convergence`] carries the
/// non-converged diagnostic; the analysis terminates early and
/// the Waveforms contain only the time points accepted before the
/// failure.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientAnalysisResult {
    /// The transient Result envelope per the Glossary: Waveforms
    /// (only at accepted time points) plus
    /// [`TimestepHistoryMetadata`].
    pub transient: TransientResult,
    /// Convergence status of the last attempted per-timestep NR
    /// solve. `Converged` on a clean run that reached `t_stop`;
    /// non-converged variants surface diagnostic state from the
    /// last failed step.
    pub final_convergence: ConvergenceStatus,
}

impl TransientAnalysisResult {
    /// True iff the analysis ran to completion with the last
    /// per-timestep NR solve reporting `Converged`.
    #[must_use]
    pub fn is_converged(&self) -> bool {
        self.final_convergence.is_converged()
    }
}

// -----------------------------------------------------------------------------
// Error surface
// -----------------------------------------------------------------------------

/// Errors raised by [`transient_analysis`] *before* or *during* the
/// outer loop in a way that prevented it from running to its natural
/// termination.
///
/// Non-convergence outcomes from the per-timestep NR solve are
/// **not** errors here; they are reported on the `Ok` path inside
/// [`TransientAnalysisResult::final_convergence`].
#[derive(Debug, Clone, PartialEq)]
pub enum TransientAnalysisError {
    /// The selected [`IntegrationMethod`] is not yet wired into the
    /// control loop. Currently
    /// [`IntegrationMethod::Gear2Bdf`] returns this; a follow-up
    /// task adds Gear-2 BDF support.
    UnsupportedIntegrationMethod(IntegrationMethod),
    /// The initial DC operating-point computation failed (e.g.
    /// floating-node fault, DC NR hard failure).
    InitialDcFailed(DcAnalysisError),
    /// The initial DC operating-point solve did not produce a
    /// converged result. Non-converged DC states are not a valid
    /// starting point for a transient solve, so the analysis
    /// short-circuits with the last-iterate convergence diagnostic.
    InitialDcNotConverged {
        /// The DC convergence status (one of `Stalled`,
        /// `MaxIterationsExceeded`, `Diverged`).
        convergence: ConvergenceStatus,
    },
    /// The transient interval is malformed: `t_stop <= t_start`.
    NonPositiveInterval {
        /// Start time.
        t_start: SimulationTime,
        /// Stop time.
        t_stop: SimulationTime,
    },
    /// The initial step size is non-positive or non-finite.
    NonPositiveInitialStep {
        /// The offending value.
        h: f64,
    },
    /// Pass-2 MNA assembly rejected the inputs at the first
    /// timestep (or at any subsequent re-assembly).
    AssemblyFailed(MnaAssemblyError),
    /// The DC sub-view builder rejected the inputs.
    SubViewBuildFailed(SubViewError),
    /// A reactive-element companion-stamp helper rejected its
    /// inputs (non-positive step, non-positive C/L, non-finite
    /// history). Surfaces as a hard error because the orchestrator
    /// is expected to clamp `h` and validate netlist parameters
    /// before reaching the companion-stamp call. We carry the
    /// `Display`-rendered message rather than the source type
    /// because Backward Euler and Trapezoidal have parallel-but-
    /// distinct `CompanionInputError` enums in
    /// [`numeric_solver::integration`]; the message string is the
    /// stable contract.
    CompanionStampFailed(String),
    /// The Newton-Raphson driver itself returned a hard failure
    /// at some timestep.
    NewtonRaphsonFailed(NewtonRaphsonError),
    /// The adaptive-timestepping controller surfaced an
    /// input-validation error (e.g. non-finite LTE history caused
    /// by upstream NR divergence). The non-convergence is reported
    /// via the result `final_convergence`; this variant covers the
    /// *controller* rejecting its own inputs.
    AdaptiveControllerFailed(numeric_solver::AdaptiveError),
    /// The outer loop tried to shrink the step size below the
    /// configured [`StepSizeBounds::h_min`] floor more than
    /// `max_consecutive_rejects` times. This guards against
    /// pathological circuits that reject every step at every size
    /// down to the floor — at which point the analysis is reported
    /// as `Diverged` rather than spinning forever.
    StepFloorExhausted {
        /// The accepted time at which the floor exhaustion was
        /// detected, in seconds.
        t_seconds: f64,
        /// The number of consecutive rejected attempts that drove
        /// the step to the floor.
        consecutive_rejects: u32,
    },
}

impl core::fmt::Display for TransientAnalysisError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsupportedIntegrationMethod(m) => {
                write!(f, "transient-analysis: integration method {m:?} is not wired in this task; follow-up scope owns it")
            }
            Self::InitialDcFailed(inner) => {
                write!(f, "transient-analysis: initial DC OP failed: {inner}")
            }
            Self::InitialDcNotConverged { convergence } => {
                write!(
                    f,
                    "transient-analysis: initial DC OP did not converge: {convergence:?}"
                )
            }
            Self::NonPositiveInterval { t_start, t_stop } => {
                write!(
                    f,
                    "transient-analysis: non-positive interval t_start={t_start}, t_stop={t_stop}"
                )
            }
            Self::NonPositiveInitialStep { h } => {
                write!(
                    f,
                    "transient-analysis: initial step h must be strictly positive and finite, got {h}"
                )
            }
            Self::AssemblyFailed(inner) => {
                write!(f, "transient-analysis: MNA assembly failed: {inner}")
            }
            Self::SubViewBuildFailed(inner) => {
                write!(f, "transient-analysis: sub-view build failed: {inner}")
            }
            Self::CompanionStampFailed(inner) => {
                write!(f, "transient-analysis: companion stamp failed: {inner}")
            }
            Self::NewtonRaphsonFailed(inner) => {
                write!(
                    f,
                    "transient-analysis: per-timestep NR hard failure: {inner}"
                )
            }
            Self::AdaptiveControllerFailed(inner) => {
                write!(f, "transient-analysis: adaptive controller failed: {inner}")
            }
            Self::StepFloorExhausted {
                t_seconds,
                consecutive_rejects,
            } => {
                write!(
                    f,
                    "transient-analysis: step floor exhausted at t={t_seconds} s after \
                     {consecutive_rejects} consecutive rejections"
                )
            }
        }
    }
}

impl std::error::Error for TransientAnalysisError {}

impl From<MnaAssemblyError> for TransientAnalysisError {
    fn from(e: MnaAssemblyError) -> Self {
        Self::AssemblyFailed(e)
    }
}

impl From<SubViewError> for TransientAnalysisError {
    fn from(e: SubViewError) -> Self {
        Self::SubViewBuildFailed(e)
    }
}

impl From<numeric_solver::CompanionInputError> for TransientAnalysisError {
    fn from(e: numeric_solver::CompanionInputError) -> Self {
        Self::CompanionStampFailed(e.to_string())
    }
}

impl From<numeric_solver::integration::trapezoidal::CompanionInputError>
    for TransientAnalysisError
{
    fn from(e: numeric_solver::integration::trapezoidal::CompanionInputError) -> Self {
        Self::CompanionStampFailed(e.to_string())
    }
}

impl From<NewtonRaphsonError> for TransientAnalysisError {
    fn from(e: NewtonRaphsonError) -> Self {
        Self::NewtonRaphsonFailed(e)
    }
}

impl From<numeric_solver::AdaptiveError> for TransientAnalysisError {
    fn from(e: numeric_solver::AdaptiveError) -> Self {
        Self::AdaptiveControllerFailed(e)
    }
}

// -----------------------------------------------------------------------------
// Reactive-element state — history bundles tracked across timesteps
// -----------------------------------------------------------------------------

/// Per-reactive-element history bundle, indexed by
/// [`netlist_graph::ElementId`] position in the flattened
/// structure's element iterator.
///
/// We carry both Backward-Euler and Trapezoidal histories for every
/// reactive element regardless of the selected integration method.
/// The cost is two `f64`s per element (negligible); the benefit is
/// that switching method between adjacent calls of the orchestrator
/// (which the orchestrator does **not** do in this task, but which
/// the mixed-signal scheduler in ADR-0004 may want in a follow-up)
/// becomes a no-cost branch on the same already-tracked state.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ReactiveState {
    /// The element's terminal-voltage difference at the most-recent
    /// accepted timestep, `V_a − V_b` in volts. Used by both BE and
    /// TR capacitor companions and by the TR inductor companion.
    v_prev: f64,
    /// The element's branch current at the most-recent accepted
    /// timestep, in amps, directed `a → b`. Used by both BE and TR
    /// inductor companions and by the TR capacitor companion.
    /// For capacitors we infer `i_prev` from the companion law
    /// after each accepted step; see [`infer_capacitor_current`].
    i_prev: f64,
}

impl ReactiveState {
    const fn zero() -> Self {
        Self {
            v_prev: 0.0,
            i_prev: 0.0,
        }
    }
}

// -----------------------------------------------------------------------------
// Per-timestep linear-system construction (the heart of the loop)
// -----------------------------------------------------------------------------

/// One reactive-element companion stamp ready to be folded into the
/// per-timestep MNA matrix.
#[derive(Debug, Clone, Copy)]
struct ReactiveCompanion {
    /// Which two nodes the element is connected to in the flattened
    /// structure (the assembler-internal order is preserved).
    a: NodeId,
    b: NodeId,
    /// The Norton-equivalent conductance to fold at the new
    /// timestep, in siemens.
    g_eq: f64,
    /// The history current source to fold at the new timestep, in
    /// amps. Sign convention: `+i_history` represents current
    /// flowing from terminal `a` to terminal `b` through the
    /// companion at `t = t_n`.
    i_history: f64,
}

/// Build per-element companion stamps for every reactive element in
/// the structure given the current state and the selected method.
fn build_reactive_companions(
    structure: &FlattenedStructure,
    graph: &CircuitGraph,
    states: &[ReactiveState],
    method: IntegrationMethod,
    h: f64,
) -> Result<Vec<ReactiveCompanion>, TransientAnalysisError> {
    let mut companions: Vec<ReactiveCompanion> = Vec::new();
    for (i, inc) in structure.elements().enumerate() {
        let element = graph
            .element(inc.element)
            .ok_or(TransientAnalysisError::AssemblyFailed(
                MnaAssemblyError::GraphFlattenMismatch {
                    flat_count: structure.element_count(),
                    graph_count: graph.elements().len(),
                },
            ))?;
        match element.kind() {
            ElementKind::Capacitor { capacitance_farads } => {
                let (a, b) = two_terminals(inc, "C")?;
                let stamp = match method {
                    IntegrationMethod::BackwardEuler => be::capacitor_companion(
                        *capacitance_farads,
                        h,
                        CapacitorHistory::new(states[i].v_prev),
                    )?,
                    IntegrationMethod::Trapezoidal => tr::capacitor_companion(
                        *capacitance_farads,
                        h,
                        tr::CapacitorTrapHistory::new(states[i].v_prev, states[i].i_prev),
                    )?,
                    IntegrationMethod::Gear2Bdf => {
                        return Err(TransientAnalysisError::UnsupportedIntegrationMethod(method))
                    }
                };
                companions.push(ReactiveCompanion {
                    a,
                    b,
                    g_eq: stamp.g_eq,
                    i_history: stamp.i_history,
                });
            }
            ElementKind::Inductor { inductance_henries } => {
                let (a, b) = two_terminals(inc, "L")?;
                let stamp = match method {
                    IntegrationMethod::BackwardEuler => be::inductor_companion(
                        *inductance_henries,
                        h,
                        InductorHistory::new(states[i].i_prev),
                    )?,
                    IntegrationMethod::Trapezoidal => tr::inductor_companion(
                        *inductance_henries,
                        h,
                        tr::InductorTrapHistory::new(states[i].i_prev, states[i].v_prev),
                    )?,
                    IntegrationMethod::Gear2Bdf => {
                        return Err(TransientAnalysisError::UnsupportedIntegrationMethod(method))
                    }
                };
                companions.push(ReactiveCompanion {
                    a,
                    b,
                    g_eq: stamp.g_eq,
                    i_history: stamp.i_history,
                });
            }
            _ => {}
        }
    }
    Ok(companions)
}

fn two_terminals(
    inc: &circuit_solver_types::flattened::ElementIncidence,
    kind_tag: &'static str,
) -> Result<(NodeId, NodeId), TransientAnalysisError> {
    if inc.nodes.len() != 2 {
        return Err(TransientAnalysisError::AssemblyFailed(
            MnaAssemblyError::WrongTerminalCountForKind {
                element: inc.element,
                kind: kind_tag,
                actual: inc.nodes.len(),
                expected: 2,
            },
        ));
    }
    Ok((inc.nodes[0], inc.nodes[1]))
}

/// Build the per-timestep MNA sub-view. We assemble the baseline
/// matrix once via [`assemble`] (which stamps R/V/I and stamps an
/// inductor's DC short branch row), then override the inductor
/// branch rows with the companion-law row and add the capacitor
/// Norton contributions. See [`apply_companions`] for the
/// row-replacement / conductance-add rationale.
///
/// We then apply ground suppression directly in the dense buffer
/// (rather than round-tripping through
/// [`SubViewBuilder::from_full`]) because [`numeric_solver::MnaSystem`]
/// has no public mutator path. The end result is the same shape the
/// DC control loop produces: a ground-pinned square matrix and RHS
/// lowered into a [`SparseLinearSystem<f64>`].
/// Apply reactive-element companion contributions to the dense MNA
/// matrix/RHS in place. See [`assemble_transient_system`] for the
/// inductor-row-replacement / capacitor-conductance-add rationale.
fn apply_companions(
    structure: &FlattenedStructure,
    graph: &CircuitGraph,
    companions: &[ReactiveCompanion],
    a: &mut [f64],
    b: &mut [f64],
    dim: u32,
    node_count: u32,
) -> Result<(), TransientAnalysisError> {
    // We iterate the structure in tandem with the companions to
    // associate each companion with an element id (so we can look
    // up branch ids for inductors).
    let dim_us = dim as usize;
    // Re-walk the structure picking out reactive elements in the
    // same order `build_reactive_companions` did.
    let mut companion_idx = 0_usize;
    for inc in structure.elements() {
        let element = graph
            .element(inc.element)
            .ok_or(TransientAnalysisError::AssemblyFailed(
                MnaAssemblyError::GraphFlattenMismatch {
                    flat_count: structure.element_count(),
                    graph_count: graph.elements().len(),
                },
            ))?;
        match element.kind() {
            ElementKind::Capacitor { .. } => {
                let comp = companions[companion_idx];
                companion_idx += 1;
                // Norton-equivalent stamp at the cap's two
                // terminals. The companion law is
                //   i_C(a → b) = g_eq · (V_a − V_b) − i_history
                // (see [`numeric_solver::integration::backward_euler`]
                // module docstring derivation). Treating this as a
                // conductance `g_eq` between `a` and `b` plus a
                // Norton current source whose value is `−i_history`
                // in the `a → b` direction (equivalently `+i_history`
                // in `b → a`), the MNA `G · V = b_rhs` stamps are:
                //
                //   G[a, a] += g_eq;  G[a, b] -= g_eq;
                //   G[b, a] -= g_eq;  G[b, b] += g_eq;
                //   b_rhs[a] += i_history;  (source pumping into a)
                //   b_rhs[b] -= i_history;  (source pumping out of b)
                //
                // This is the sign convention exercised by the
                // working RC step-response test in
                // [`backward_euler::tests::rc_low_pass_step_response_converges_to_source`]
                // and by the scenario_trapezoidal_rlc_tank.rs witness.
                let ai = comp.a.index() as usize;
                let bi = comp.b.index() as usize;
                a[ai * dim_us + ai] += comp.g_eq;
                a[bi * dim_us + bi] += comp.g_eq;
                a[ai * dim_us + bi] -= comp.g_eq;
                a[bi * dim_us + ai] -= comp.g_eq;
                b[ai] += comp.i_history;
                b[bi] -= comp.i_history;
            }
            ElementKind::Inductor { .. } => {
                let comp = companions[companion_idx];
                companion_idx += 1;
                let branch = inc.branch.ok_or(TransientAnalysisError::AssemblyFailed(
                    MnaAssemblyError::MissingBranchForCurrentCarrying {
                        element: inc.element,
                        kind: "L",
                    },
                ))?;
                let br = (node_count + branch.index()) as usize;
                let ai = comp.a.index() as usize;
                let bi = comp.b.index() as usize;
                // Replace the DC short branch row with the
                // companion-law row. The DC stamp put +1 at (br, ai)
                // and −1 at (br, bi) with RHS[br] = 0, enforcing
                // V_a − V_b = 0. For transient we need the row to
                // express
                //   I_L = g_eq · (V_a − V_b) − i_history
                // ⇒  I_L − g_eq · V_a + g_eq · V_b = − i_history
                // The branch-unknown column gets +1, the V_a column
                // −g_eq, the V_b column +g_eq, and the RHS gets
                // −i_history.
                //
                // The node-incidence columns at (ai, br) and (bi, br)
                // were stamped by the DC inductor as +1/−1; we keep
                // them as-is so KCL at nodes a/b still includes the
                // inductor branch current as a current leaving a /
                // entering b.
                for c in 0..dim_us {
                    a[br * dim_us + c] = 0.0;
                }
                a[br * dim_us + ai] -= comp.g_eq;
                a[br * dim_us + bi] += comp.g_eq;
                a[br * dim_us + br] += 1.0;
                b[br] = -comp.i_history;
            }
            _ => {}
        }
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// Linear system wrapper for NR
// -----------------------------------------------------------------------------

/// A pre-assembled, ground-suppressed transient MNA system. Newton-
/// Raphson uses this as a one-iteration linear system (every
/// callback hands back the same matrix) because the transient inner
/// solve at a given timestep is itself linear (no nonlinear devices
/// in scope for this task — semiconductor support comes through the
/// `linearizations` slice in a follow-up).
struct LinearTransientSystem {
    system: SparseLinearSystem<f64>,
}

impl NonlinearSystem for LinearTransientSystem {
    fn dim(&self) -> u32 {
        self.system.dim()
    }
    fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, NrSystemError> {
        Ok(self.system.clone())
    }
    fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, NrSystemError> {
        let dim = self.system.dim() as usize;
        let mut f = vec![0.0_f64; dim];
        for t in self.system.triplets() {
            f[t.row as usize] += t.value * iterate[t.col as usize];
        }
        for (i, rhs_i) in self.system.rhs().iter().enumerate() {
            f[i] -= *rhs_i;
        }
        Ok(f)
    }
}

// -----------------------------------------------------------------------------
// Entry point
// -----------------------------------------------------------------------------

/// Run the transient time-domain analysis control loop.
///
/// Steps, in order:
///
/// 1. **Input validation.** Reject non-positive intervals, non-finite
///    or non-positive initial steps, and unsupported integration
///    methods.
/// 2. **Initial state.** Either compute a DC operating point via
///    [`dc_analysis`] (default) or build the initial node voltages
///    from the user-supplied UIC map.
/// 3. **Reactive-element history seed.** Walk the flattened
///    structure and seed each capacitor's `v_prev` from the initial
///    node voltages, each inductor's `i_prev` from the initial
///    branch currents (`0 A` at DC steady-state for an LC tank);
///    `i_prev` for capacitors and `v_prev` for inductors come from
///    the same source.
/// 4. **Outer loop.** Until the accumulated simulation time
///    reaches `t_stop`:
///    a. Compute companion stamps and assemble the per-timestep
///       MNA sub-view.
///    b. Run the per-timestep Newton-Raphson solve (one
///       iteration on the linear system).
///    c. If the solve returns a non-`Converged` status, exit the
///       outer loop carrying the diagnostic.
///    d. Build [`NodeHistorySample`]s from the previous two
///       accepted iterates plus this tentative one, and call
///       [`step_decision`] to get an LTE verdict and the
///       recommended next step.
///    e. On `Accept`, fold the solution into the Waveforms and
///       advance reactive-element histories.
///    f. On `Reject`, leave the histories and time pointer
///       untouched and re-attempt with the shrunk step.
///    g. Append a [`TimestepRecord`] either way.
/// 5. **Result construction.** Bundle the Waveforms (one per
///    non-ground node, in `NodeId::index()` order) and the
///    [`TimestepHistory`]-derived [`TimestepHistoryMetadata`] into
///    a [`TransientResult`].
///
/// # Errors
///
/// See [`TransientAnalysisError`] for the full surface.
///
/// # Panics
///
/// Does not panic in normal operation.
pub fn transient_analysis(
    request: TransientAnalysisRequest<'_>,
) -> Result<TransientAnalysisResult, TransientAnalysisError> {
    // --- (1) Input validation ----------------------------------------------
    if request.integration_method == IntegrationMethod::Gear2Bdf {
        return Err(TransientAnalysisError::UnsupportedIntegrationMethod(
            request.integration_method,
        ));
    }
    if request.t_stop.as_picoseconds() <= request.t_start.as_picoseconds() {
        return Err(TransientAnalysisError::NonPositiveInterval {
            t_start: request.t_start,
            t_stop: request.t_stop,
        });
    }
    if !request.initial_step_seconds.is_finite() || request.initial_step_seconds <= 0.0 {
        return Err(TransientAnalysisError::NonPositiveInitialStep {
            h: request.initial_step_seconds,
        });
    }
    request
        .step_bounds
        .validate()
        .map_err(TransientAnalysisError::from)?;

    // --- (2) Initial state -------------------------------------------------
    let initial_node_voltages = match &request.initial_state {
        InitialState::DcOperatingPoint => {
            let dc_request = DcAnalysisRequest {
                graph: request.graph,
                structure: request.structure,
                newton_raphson: request.newton_raphson,
                ground: request.ground,
            };
            let dc_result =
                dc_analysis(dc_request).map_err(TransientAnalysisError::InitialDcFailed)?;
            if !dc_result.convergence.is_converged() {
                return Err(TransientAnalysisError::InitialDcNotConverged {
                    convergence: dc_result.convergence,
                });
            }
            let op = dc_result.operating_point.expect(
                "dc_analysis returns Some(operating_point) on Ok per the dc module contract",
            );
            op.node_voltages
        }
        InitialState::UseInitialConditions { node_voltages } => {
            let n = request.structure.node_count() as usize;
            let mut v = vec![0.0_f64; n];
            for (node, value) in node_voltages {
                if (node.index() as usize) < n {
                    v[node.index() as usize] = *value;
                }
            }
            v
        }
    };

    // --- (3) Reactive-element history seed ---------------------------------
    let states = seed_reactive_states(request.structure, request.graph, &initial_node_voltages);

    // --- (4) Outer loop ----------------------------------------------------
    let nr_config = request.newton_raphson.unwrap_or(NewtonRaphsonConfig {
        max_iterations: 100,
        // Transient-relaxed tolerances. The MNA system at every
        // per-timestep solve contains capacitor companion
        // conductances `g_eq = C / h` (siemens), which for a
        // typical `1 nF / 1 ns` produce `1 S` matrix entries —
        // four orders of magnitude above the DC `g = 1/R = 1 mS`
        // scale. The residue infinity-norm `‖F(x)‖∞` at machine
        // precision lives near `eps · ‖A‖∞ · ‖x‖∞`, which for
        // a 10-V source and `1 S` matrix entry sits around
        // `1e-15 · 1 · 10 = 1e-14` in *theory* but rises to
        // `~1e-12` in practice via the sparse-LU's iterative
        // refinement floor. We loosen `residue_tol` from the
        // DC `1e-12` to `1e-9` to absorb this — still tight
        // enough to guarantee per-timestep convergence to
        // microvolt precision, well inside the
        // ADR-0008 transient envelope (`1 % rel / 1 mV abs`).
        // Callers can override via
        // [`TransientAnalysisRequest::with_newton_raphson`].
        tolerances: circuit_solver_types::ConvergenceTolerances::new(1.0e-3, 1.0e-9),
    });
    let estimator = LteEstimator {
        order: request.integration_method.lte_order(),
    };
    let observed_nodes = observed_nodes(request.structure, request.ground);
    let mut waveform_times: Vec<SimulationTime> = vec![request.t_start];
    let mut waveform_values: Vec<Vec<f64>> = observed_nodes
        .iter()
        .map(|n| vec![initial_node_voltages[n.index() as usize]])
        .collect();

    let t_start_seconds = request.t_start.as_seconds_f64();
    let t_stop_seconds = request.t_stop.as_seconds_f64();
    let mut t_seconds = t_start_seconds;
    let mut h = request.initial_step_seconds;
    let mut current_states = states;
    // Two-point history for the LTE estimator. Both start at the
    // initial-state node voltages — until two real time points
    // have been accepted, the LTE estimator is fed identical
    // samples and produces zero LTE, which the controller treats
    // as a "perfect step" (Accept, grow as fast as allowed).
    let mut v_prev_prev: Vec<f64> = initial_node_voltages.clone();
    let mut v_prev: Vec<f64> = initial_node_voltages.clone();
    let mut history = TimestepHistory::new();
    let mut final_convergence: Option<ConvergenceStatus> = None;
    let mut consecutive_rejects = 0_u32;
    // Hard cap to prevent runaway loops if the controller stalls at
    // h_min and never makes forward progress. We size it generously
    // so a legitimate fast/slow transient with many small steps does
    // not trip it. The `step_floor_exhausted` check below is the
    // *defensive* guard; this cap is the *belt-and-braces* one.
    const MAX_TOTAL_ATTEMPTS: usize = 10_000_000;
    const MAX_CONSECUTIVE_REJECTS: u32 = 32;

    let mut attempt = 0_usize;
    while t_seconds + 0.5 * request.step_bounds.h_min < t_stop_seconds {
        attempt += 1;
        if attempt > MAX_TOTAL_ATTEMPTS {
            return Err(TransientAnalysisError::StepFloorExhausted {
                t_seconds,
                consecutive_rejects,
            });
        }
        // Clamp step so we land at t_stop, never beyond.
        let h_clamped = (t_stop_seconds - t_seconds).min(h);
        // Guard against the controller shrinking below h_min and
        // never making progress.
        if h_clamped < request.step_bounds.h_min * (1.0 - 1.0e-9)
            && consecutive_rejects >= MAX_CONSECUTIVE_REJECTS
        {
            return Err(TransientAnalysisError::StepFloorExhausted {
                t_seconds,
                consecutive_rejects,
            });
        }

        // --- (4a) Assemble the per-timestep MNA sub-view. ---------------
        let sparse = assemble_via_scratch(
            &request,
            &current_states,
            request.integration_method,
            h_clamped,
        )?;
        let dim = sparse.dim();
        let initial_iterate = previous_iterate_in_sub_view(&v_prev, dim);
        let mut system = LinearTransientSystem { system: sparse };

        // --- (4b) NR solve. ---------------------------------------------
        let outcome = NewtonRaphsonDriver
            .solve(nr_config, &mut system, &RussellRealSolver, initial_iterate)
            .map_err(TransientAnalysisError::NewtonRaphsonFailed)?;
        let status = outcome.status;
        // --- (4c) NR non-convergence: exit early. ----------------------
        if !status.is_converged() {
            final_convergence = Some(status);
            break;
        }
        final_convergence = Some(status);
        // Project the iterate back to a full-structure-size vector.
        let v_curr_full =
            inject_subview_iterate(&outcome.iterate, request.structure.node_count() as usize);

        // --- (4d) LTE step decision. -----------------------------------
        let samples = build_lte_samples(&v_prev_prev, &v_prev, &v_curr_full, &observed_nodes);
        let decision = if samples.is_empty() {
            // No observed analog nodes — vacuously accept and try
            // to grow.
            numeric_solver::StepDecision {
                outcome: StepOutcome::Accept,
                next_h: next_step_size(h_clamped, 0.0, estimator.order, request.step_bounds),
                worst_ratio: 0.0,
                worst_index: None,
            }
        } else {
            step_decision(
                estimator,
                &samples,
                request.lte_envelope,
                h_clamped,
                request.step_bounds,
            )?
        };

        let t_attempt = t_seconds + h_clamped;
        history.record(TimestepRecord::from_decision(
            t_attempt, h_clamped, &decision,
        ));

        match decision.outcome {
            StepOutcome::Accept => {
                // --- (4e) Accept: fold and advance histories. ---------
                consecutive_rejects = 0;
                t_seconds = t_attempt;
                waveform_times.push(SimulationTime::from_picoseconds(
                    (t_seconds * 1.0e12) as i64,
                ));
                for (lane, node) in observed_nodes.iter().enumerate() {
                    waveform_values[lane].push(v_curr_full[node.index() as usize]);
                }
                advance_reactive_states(
                    request.structure,
                    request.graph,
                    &mut current_states,
                    &outcome.iterate,
                    request.structure.node_count(),
                    h_clamped,
                );
                // Rotate the LTE history.
                v_prev_prev = v_prev;
                v_prev = v_curr_full;
                // Update step size for next attempt.
                h = decision.next_h;
            }
            StepOutcome::Reject => {
                // --- (4f) Reject: shrink and retry; histories untouched. -
                consecutive_rejects += 1;
                h = decision.next_h;
                // If we cannot shrink any further, surface the floor.
                if h <= request.step_bounds.h_min * (1.0 + 1.0e-9)
                    && consecutive_rejects >= MAX_CONSECUTIVE_REJECTS
                {
                    return Err(TransientAnalysisError::StepFloorExhausted {
                        t_seconds,
                        consecutive_rejects,
                    });
                }
            }
        }
    }

    // --- (5) Result construction -------------------------------------------
    let waveforms: Vec<Waveform> = observed_nodes
        .iter()
        .zip(waveform_values)
        .map(|(node, values)| Waveform::new(*node, waveform_times.clone(), values))
        .collect();
    let final_convergence = final_convergence.unwrap_or(
        // Vacuous case: t_stop == t_start (rejected above) or zero
        // analog nodes. Default to a "converged" status with all-zero
        // diagnostic to keep is_converged() correct.
        ConvergenceStatus::Converged(circuit_solver_types::ConvergenceDiagnostic {
            update_norm: 0.0,
            residue_norm: 0.0,
            iterations: 0,
            tolerances: nr_config.tolerances,
        }),
    );
    Ok(TransientAnalysisResult {
        transient: TransientResult::new(waveforms, TimestepHistoryMetadata::from(&history)),
        final_convergence,
    })
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

/// Build a per-element [`ReactiveState`] vector aligned with
/// `structure.elements()` order (one entry per element; non-reactive
/// elements get a zero state which is never consulted).
fn seed_reactive_states(
    structure: &FlattenedStructure,
    graph: &CircuitGraph,
    initial_node_voltages: &[f64],
) -> Vec<ReactiveState> {
    structure
        .elements()
        .map(|inc| {
            if let Some(element) = graph.element(inc.element) {
                match element.kind() {
                    ElementKind::Capacitor { .. } => {
                        if inc.nodes.len() == 2 {
                            let va = initial_node_voltages
                                .get(inc.nodes[0].index() as usize)
                                .copied()
                                .unwrap_or(0.0);
                            let vb = initial_node_voltages
                                .get(inc.nodes[1].index() as usize)
                                .copied()
                                .unwrap_or(0.0);
                            return ReactiveState {
                                v_prev: va - vb,
                                i_prev: 0.0, // DC steady-state: capacitor current = 0
                            };
                        }
                    }
                    ElementKind::Inductor { .. } => {
                        // At DC steady-state an inductor is a short, so v_prev = 0;
                        // i_prev is unknown from node voltages alone (it lives in
                        // an MNA branch row) and the most common quiescent value
                        // for an LC tank or DC bias point is 0 A. The control
                        // loop will overwrite this after the first accepted step.
                        return ReactiveState {
                            v_prev: 0.0,
                            i_prev: 0.0,
                        };
                    }
                    _ => {}
                }
            }
            ReactiveState::zero()
        })
        .collect()
}

/// Choose the set of "observed" nodes: every non-ground node in the
/// structure, in `NodeId::index()` order. The headline scenario
/// requires Waveforms "for all observed nodes"; we interpret that as
/// every non-ground node by default.
fn observed_nodes(structure: &FlattenedStructure, ground: Option<NodeId>) -> Vec<NodeId> {
    let ground = ground.unwrap_or_else(|| structure.ground_node());
    let n = structure.node_count();
    (0..n).map(NodeId::new).filter(|n| *n != ground).collect()
}

/// Build the per-observed-node LTE history samples for [`step_decision`].
fn build_lte_samples(
    v_prev_prev: &[f64],
    v_prev: &[f64],
    v_curr: &[f64],
    observed: &[NodeId],
) -> Vec<NodeHistorySample> {
    observed
        .iter()
        .filter_map(|n| {
            let idx = n.index() as usize;
            let pp = *v_prev_prev.get(idx)?;
            let p = *v_prev.get(idx)?;
            let c = *v_curr.get(idx)?;
            Some(NodeHistorySample {
                v_prev_prev: pp,
                v_prev: p,
                v_curr: c,
            })
        })
        .collect()
}

/// After an accepted timestep, update each reactive element's
/// `ReactiveState` from the new MNA solution. `h` is the step size
/// used at this timestep so capacitor branch currents can be
/// recovered via `i_C ≈ C · (v_new − v_old) / h` for the
/// Trapezoidal capacitor history (which carries `i_prev`).
fn advance_reactive_states(
    structure: &FlattenedStructure,
    graph: &CircuitGraph,
    states: &mut [ReactiveState],
    iterate_subview: &[f64],
    node_count: u32,
    h: f64,
) {
    for (i, inc) in structure.elements().enumerate() {
        if let Some(element) = graph.element(inc.element) {
            match element.kind() {
                ElementKind::Capacitor { capacitance_farads } => {
                    if inc.nodes.len() == 2 {
                        let va = iterate_subview
                            .get(inc.nodes[0].index() as usize)
                            .copied()
                            .unwrap_or(0.0);
                        let vb = iterate_subview
                            .get(inc.nodes[1].index() as usize)
                            .copied()
                            .unwrap_or(0.0);
                        let v_new = va - vb;
                        // Recover i_C from the constitutive law
                        // `i_C = C · dv/dt`, discretized as
                        // `i_C ≈ C · (v_new − v_old) / h`. This is
                        // what the BE and TR capacitor companions
                        // compute internally; surfacing it here keeps
                        // the Trapezoidal history (which carries
                        // i_prev) consistent across timesteps.
                        let i_new = if h > 0.0 {
                            capacitance_farads * (v_new - states[i].v_prev) / h
                        } else {
                            states[i].i_prev
                        };
                        states[i].v_prev = v_new;
                        states[i].i_prev = i_new;
                    }
                }
                ElementKind::Inductor { .. } => {
                    if let Some(branch) = inc.branch {
                        let br = (node_count + branch.index()) as usize;
                        let i_l = iterate_subview.get(br).copied().unwrap_or(0.0);
                        if inc.nodes.len() == 2 {
                            let va = iterate_subview
                                .get(inc.nodes[0].index() as usize)
                                .copied()
                                .unwrap_or(0.0);
                            let vb = iterate_subview
                                .get(inc.nodes[1].index() as usize)
                                .copied()
                                .unwrap_or(0.0);
                            states[i].v_prev = va - vb;
                        }
                        states[i].i_prev = i_l;
                    }
                }
                _ => {}
            }
        }
    }
}

/// Build the previous-iterate initial guess for the per-timestep NR
/// solve, sized to the sub-view dimension (the sub-view sometimes
/// has the same dim as the full system; sometimes smaller). For the
/// linear case the initial iterate is only a diagnostic anchor —
/// NR converges in two iterations from any finite start.
fn previous_iterate_in_sub_view(v_prev: &[f64], dim: u32) -> Vec<f64> {
    let mut x = vec![0.0_f64; dim as usize];
    // Copy node voltages where they fit; branch entries default to 0.
    let n = v_prev.len().min(x.len());
    x[..n].copy_from_slice(&v_prev[..n]);
    x
}

/// Identity injection: the sub-view returns a full-size iterate
/// because ground suppression is implemented by pinning the ground
/// row/col (not by reducing dimension). Verify length and return.
fn inject_subview_iterate(iterate: &[f64], full_node_count: usize) -> Vec<f64> {
    // The russell-backed sub-view is full-dim with ground pinned to
    // 0; just return a copy.
    let _ = full_node_count;
    iterate.to_vec()
}

// -----------------------------------------------------------------------------
// Scratch-MNA-based assembly path (replaces write_back_matrix)
// -----------------------------------------------------------------------------

/// Build the per-timestep sparse linear system by:
///
/// 1. Calling `assemble()` once to get the baseline dense matrix.
/// 2. Lifting the matrix and RHS into mutable `Vec<f64>` copies.
/// 3. Overriding inductor branch rows and adding capacitor
///    Norton stamps via [`apply_companions`].
/// 4. Applying ground-suppression manually in the dense buffer
///    (replacing the ground row with an identity row pointing at
///    `V_ground = 0`).
/// 5. Lowering the dense buffer to a [`SparseLinearSystem<f64>`].
///
/// This is an alternative to round-tripping through
/// `SubViewBuilder::from_full`; we apply ground suppression
/// directly here because `MnaSystem` has no public mutator path.
fn assemble_via_scratch(
    request: &TransientAnalysisRequest<'_>,
    states: &[ReactiveState],
    method: IntegrationMethod,
    h: f64,
) -> Result<SparseLinearSystem<f64>, TransientAnalysisError> {
    let mna = assemble(request.structure, request.graph, &[])?;
    let dim = mna.dim();
    let dim_us = dim as usize;
    let node_count = mna.node_count();
    let branch_count = mna.branch_count();
    let mut a: Vec<f64> = mna.matrix().to_vec();
    let mut b: Vec<f64> = mna.rhs().to_vec();

    let companions =
        build_reactive_companions(request.structure, request.graph, states, method, h)?;
    apply_companions(
        request.structure,
        request.graph,
        &companions,
        &mut a,
        &mut b,
        dim,
        node_count,
    )?;

    // Ground suppression: replace the ground row with an identity
    // row and zero its column; pin RHS[ground] = 0.
    let ground = request
        .ground
        .unwrap_or_else(|| request.structure.ground_node());
    let g = ground.index() as usize;
    if g >= dim_us {
        return Err(TransientAnalysisError::SubViewBuildFailed(
            SubViewError::GroundNodeOutOfRange { ground, node_count },
        ));
    }
    for c in 0..dim_us {
        a[g * dim_us + c] = 0.0;
    }
    for r in 0..dim_us {
        a[r * dim_us + g] = 0.0;
    }
    a[g * dim_us + g] = 1.0;
    b[g] = 0.0;

    // Lower dense → sparse triplets.
    let nnz = a.iter().filter(|v| **v != 0.0).count();
    let mut triplets: Vec<SparseTriplet<f64>> = Vec::with_capacity(nnz);
    for r in 0..dim {
        let row_base = (r as usize) * dim_us;
        for c in 0..dim {
            let v = a[row_base + (c as usize)];
            if v == 0.0 {
                continue;
            }
            triplets.push(SparseTriplet {
                row: r,
                col: c,
                value: v,
            });
        }
    }
    SparseLinearSystem::new(dim, node_count, branch_count, triplets, b).map_err(|err| {
        TransientAnalysisError::NewtonRaphsonFailed(NewtonRaphsonError::LinearSolver {
            iteration: 0,
            source: err,
        })
    })
}

// `infer_capacitor_current` is referenced in `advance_reactive_states`
// documentation; we keep a stub for the residual-risk note.
#[allow(dead_code)]
fn infer_capacitor_current(_v_old: f64, _v_new: f64, _c_farads: f64, _h: f64) -> f64 {
    // Difference quotient: i = C · dv/dt ≈ C · (v_new − v_old) / h.
    // Not currently invoked; see the comment in advance_reactive_states.
    0.0
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::similar_names)]
mod tests {
    use super::*;
    use netlist_graph::{CircuitBuilder, ElementKind};
    use numeric_solver::flatten;

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

    #[allow(dead_code)]
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

    fn node_id(g: &CircuitGraph, name: &str) -> NodeId {
        g.nodes()
            .iter()
            .find(|n| n.name() == name)
            .expect("node present")
            .id()
    }

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol.max(tol * a.abs().max(b.abs()))
    }

    // ----- IntegrationMethod -------------------------------------------------

    #[test]
    fn default_integration_method_is_trapezoidal() {
        assert_eq!(IntegrationMethod::default(), IntegrationMethod::Trapezoidal);
    }

    #[test]
    fn lte_order_per_method() {
        assert_eq!(IntegrationMethod::BackwardEuler.lte_order(), 1);
        assert_eq!(IntegrationMethod::Trapezoidal.lte_order(), 2);
        assert_eq!(IntegrationMethod::Gear2Bdf.lte_order(), 2);
    }

    // ----- Input validation -------------------------------------------------

    #[test]
    fn gear2_bdf_returns_unsupported() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1.0e3);
        let g = b.build().expect("build");
        let fs = flatten(&g).expect("flatten");
        let req = TransientAnalysisRequest::new(
            &g,
            &fs,
            SimulationTime::ZERO,
            SimulationTime::from_nanoseconds(10),
            1.0e-9,
        )
        .with_integration_method(IntegrationMethod::Gear2Bdf);
        let err = transient_analysis(req).expect_err("Gear2Bdf must error out");
        assert!(matches!(
            err,
            TransientAnalysisError::UnsupportedIntegrationMethod(IntegrationMethod::Gear2Bdf)
        ));
    }

    #[test]
    fn non_positive_interval_rejected() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1.0e3);
        let g = b.build().expect("build");
        let fs = flatten(&g).expect("flatten");
        let req = TransientAnalysisRequest::new(
            &g,
            &fs,
            SimulationTime::from_nanoseconds(10),
            SimulationTime::from_nanoseconds(10),
            1.0e-9,
        );
        let err = transient_analysis(req).expect_err("non-positive interval");
        assert!(matches!(
            err,
            TransientAnalysisError::NonPositiveInterval { .. }
        ));
    }

    #[test]
    fn non_positive_initial_step_rejected() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1.0e3);
        let g = b.build().expect("build");
        let fs = flatten(&g).expect("flatten");
        let req = TransientAnalysisRequest::new(
            &g,
            &fs,
            SimulationTime::ZERO,
            SimulationTime::from_nanoseconds(10),
            -1.0,
        );
        let err = transient_analysis(req).expect_err("non-positive h");
        assert!(matches!(
            err,
            TransientAnalysisError::NonPositiveInitialStep { .. }
        ));
    }

    // ----- Headline scenario: transient analysis with default method ---------

    /// `transient-time-domain#transient-analysis-with-default-integration-method`:
    /// a 10 V DC source charging a capacitor through a resistor; the
    /// initial state is the DC operating point (so V(cap) = 10 V
    /// already at t=0, and a transient run produces a flat waveform
    /// that stays at 10 V — physically, no transient to observe).
    /// This exercises the **happy path** of the control loop:
    /// DC pre-solve → time-stepping → Waveform output.
    #[test]
    fn headline_scenario_rc_with_dc_initial_state() {
        // V1 (10 V) → R1 → n_mid → C1 → ground
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 10.0);
        add_resistor(&mut b, "R1", "n_in", "n_mid", 1.0e3);
        add_capacitor(&mut b, "C1", "n_mid", "0", 1.0e-9);
        let g = b.build().expect("build");
        let fs = flatten(&g).expect("flatten");

        let req = TransientAnalysisRequest::new(
            &g,
            &fs,
            SimulationTime::ZERO,
            SimulationTime::from_nanoseconds(100),
            1.0e-9,
        );
        let result = transient_analysis(req).expect("analysis ok");
        assert!(
            result.is_converged(),
            "expected converged, got {:?}",
            result.final_convergence
        );

        // Initial DC: V(n_in) = 10 V, V(n_mid) = 10 V (capacitor
        // open at DC ⇒ no current through R1 ⇒ no drop), V(0) = 0.
        let n_mid = node_id(&g, "n_mid");
        let n_in = node_id(&g, "n_in");

        // Pick out the n_mid waveform.
        let wf_mid = result
            .transient
            .waveforms
            .iter()
            .find(|w| w.node == n_mid)
            .expect("n_mid waveform present");
        // With no transient stimulus (DC source held constant), every
        // accepted time point should report V(n_mid) ≈ 10 V.
        for v in &wf_mid.values {
            assert!(
                approx(*v, 10.0, 1.0e-4),
                "DC-quiescent waveform should stay at 10 V, got {v}"
            );
        }
        assert!(
            wf_mid.times.first() == Some(&SimulationTime::ZERO),
            "first time point should be t_start"
        );
        let last_t = wf_mid
            .times
            .last()
            .expect("at least one time point")
            .as_seconds_f64();
        assert!(
            (last_t - 100.0e-9).abs() < 1.0e-12,
            "last time point should land at t_stop = 100 ns, got {last_t} s"
        );

        // The n_in waveform stays pinned at 10 V (voltage source).
        let wf_in = result
            .transient
            .waveforms
            .iter()
            .find(|w| w.node == n_in)
            .expect("n_in waveform present");
        for v in &wf_in.values {
            assert!(approx(*v, 10.0, 1.0e-9));
        }

        // The history should have at least one accepted entry.
        assert!(
            !result.transient.timestep_history.is_empty(),
            "history should be non-empty"
        );
        let (accepted, _rejected) = result.transient.timestep_history.counts();
        assert!(accepted > 0, "at least one step accepted");
    }

    // ----- UIC initial state -------------------------------------------------

    /// `transient-time-domain#transient-analysis-with-uic-initial-conditions`-style
    /// witness: a passive RC circuit with no sources, but with the
    /// capacitor pre-charged to 1 V via UIC. The waveform at the
    /// capacitor node should *decay* exponentially toward 0 V (no
    /// source to maintain the charge); the very first sample at t=0
    /// must be exactly 1 V.
    #[test]
    fn uic_initial_state_pre_charges_capacitor() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n_cap", "0", 1.0e3);
        add_capacitor(&mut b, "C1", "n_cap", "0", 1.0e-9);
        let g = b.build().expect("build");
        let fs = flatten(&g).expect("flatten");
        let n_cap = node_id(&g, "n_cap");
        let mut node_voltages: HashMap<NodeId, f64> = HashMap::new();
        node_voltages.insert(n_cap, 1.0);
        let req = TransientAnalysisRequest::new(
            &g,
            &fs,
            SimulationTime::ZERO,
            SimulationTime::from_nanoseconds(100),
            1.0e-9,
        )
        .with_initial_state(InitialState::UseInitialConditions { node_voltages });

        let result = transient_analysis(req).expect("uic analysis ok");
        assert!(result.is_converged());
        let wf = result
            .transient
            .waveforms
            .iter()
            .find(|w| w.node == n_cap)
            .expect("n_cap waveform present");
        assert!(
            approx(wf.values[0], 1.0, 1.0e-9),
            "UIC: t=0 sample must be 1 V, got {}",
            wf.values[0]
        );
        // After τ = R·C = 1 µs, this run only covers 100 ns ≈ τ/10,
        // so amplitude decays to about exp(-0.1) ≈ 0.905. The TR
        // discretization tracks this within the LTE envelope.
        let final_v = *wf.values.last().expect("at least one sample");
        assert!(
            final_v > 0.0 && final_v < 1.0,
            "capacitor should be decaying (0 < V < 1), got {final_v}"
        );
    }

    // ----- TransientResult bundle ------------------------------------------

    #[test]
    fn result_carries_history_and_waveforms() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", "0", 5.0);
        add_resistor(&mut b, "R1", "n1", "n2", 1.0e3);
        add_capacitor(&mut b, "C1", "n2", "0", 1.0e-12);
        let g = b.build().expect("build");
        let fs = flatten(&g).expect("flatten");
        let req = TransientAnalysisRequest::new(
            &g,
            &fs,
            SimulationTime::ZERO,
            SimulationTime::from_nanoseconds(50),
            5.0e-10,
        );
        let result = transient_analysis(req).expect("analysis ok");
        assert!(result.is_converged());
        // Both n1 and n2 are non-ground ⇒ two waveforms.
        assert_eq!(result.transient.waveforms.len(), 2);
        // Time axes are identical across waveforms.
        let t0 = &result.transient.waveforms[0].times;
        let t1 = &result.transient.waveforms[1].times;
        assert_eq!(t0, t1);
        // Each waveform has at least 2 samples (start + something).
        assert!(t0.len() >= 2);
    }
}
