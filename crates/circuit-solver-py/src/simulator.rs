//! `PyO3` `Simulator` class — analysis-submission entry point.
//!
//! This module implements the wiring that satisfies the Gherkin
//! scenario `python-frontend#analysis-request-and-result-retrieval`:
//!
//! ```text
//! Given CircuitDesigner has built a CircuitGraph containing a resistive divider
//! When CircuitDesigner creates an AnalysisRequest for DC operating point
//! And CircuitDesigner submits the AnalysisRequest to the Simulator
//! Then the Simulator returns a Result object
//! And the Result contains node voltages accessible by node name
//! And the voltage at node "n1" is approximately 5 V within the tolerance envelope
//! ```
//!
//! It is the **submission seam** between the three frozen value
//! objects ([`crate::PyCircuitGraph`], [`crate::PyAnalysisRequest`],
//! [`crate::PyAnalysisResult`]) and the Rust-side analysis control
//! loops in `analysis-orchestration` / `numeric-solver`.
//!
//! # Scope
//!
//! Per the inlined task body, the scenario witness needs only the
//! **DC operating-point** path. The Gherkin step *"creates an
//! `AnalysisRequest` for DC operating point"* fixes
//! `analysis_type = "dc-operating-point"` on the request, so this
//! module's [`PySimulator::submit`] method dispatches on the
//! request's analysis-type slug and routes the DC variant through
//! [`analysis_orchestration::dc_analysis`].
//!
//! The other analysis types (`dc-sweep`, `ac-small-signal`,
//! `transient-time-domain`, `noise-spectral-density`,
//! `mixed-signal-cosim`) are recognised by [`PyAnalysisRequest`] but
//! not yet plumbed through here — they have their own scenarios on
//! the `python-frontend` capability and downstream Simulator-submission
//! tasks (one per analysis kind) will fill them in. Today,
//! non-DC analysis types raise `NotImplementedError` from
//! `submit()` so callers get a typed, actionable diagnostic rather
//! than a silent no-op.
//!
//! # Why a free-standing `Simulator` class
//!
//! The acceptance criterion *"An `AnalysisRequest` is constructed
//! from Python with analysis type, sweep parameters, and options;
//! submitting it returns a `Result` object."* requires a verb (`submit`)
//! that takes the value object and returns a result. We model that
//! verb as a method on a dedicated `Simulator` Python class. The
//! Python idiom is then:
//!
//! ```python
//! sim = circuit_solver.Simulator()
//! result = sim.submit(graph, request)
//! v_n1 = result.node_voltage("n1")
//! ```
//!
//! `Simulator` is stateless in v1 — no caching, no preferences. The
//! placeholder field reserves space for the operating-point cache
//! (tasks.md #26 / #40 auto-DC patterns) once a cross-analysis-type
//! call site needs it.
//!
//! # GIL release
//!
//! `analysis_orchestration::dc_analysis` is pure Rust with no Python
//! callbacks, so the native solver work runs inside a
//! [`pyo3::Python::detach`] block (the `PyO3` 0.28 spelling of the
//! prior `allow_threads` API; same semantics). This satisfies the
//! acceptance criterion *"The GIL is released during native solver
//! execution, allowing other Python threads to proceed while a
//! simulation runs."* for the DC path. tasks.md #59 covers the
//! cross-analysis-type rollout of the same pattern.
//!
//! # ADR alignment
//!
//! - **ADR-0001 — `PyO3` In-Process Binding with Immutable
//!   `CircuitGraph`.** `submit` borrows the immutable `CircuitGraph`
//!   (`&self` via the frozen pyclass) and never mutates it. Multiple
//!   submissions against the same graph are safe and independent —
//!   the Pass 1 flattening is recomputed each call (cheap relative
//!   to solve) so no cross-call state leaks.
//! - **ADR-0006 — Dual Convergence Criterion for Newton-Raphson.**
//!   The DC dispatch uses [`numeric_solver::NewtonRaphsonConfig::DC_DEFAULTS`]
//!   (delegated to from `dc_analysis`), which honors the dual
//!   update-norm + residue-norm criterion.
//! - **ADR-0008 — Per-Node max(Relative, Absolute) Tolerance
//!   Envelope.** The Gherkin step *"approximately 5 V within the
//!   tolerance envelope"* is the conformance posture; the analytic
//!   resistive-divider solution is the golden reference and the
//!   `max(rel · |ref|, abs)` envelope is enforced in the scenario
//!   witness test (`tests/scenario_analysis_request_and_result_retrieval.rs`).
//! - **ADR-0010 — Unstable Public Rust API.** No new public Rust
//!   `Simulator` type is exported from the workspace; the class lives
//!   only behind the `PyO3` boundary.

use std::collections::BTreeMap;

use application_frontend::FlattenedStructure;
use application_frontend::{AnalysisType, BranchId, NodeId};
use application_frontend::CircuitGraph;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

use application_frontend::{dc_analysis, DcAnalysisError, DcAnalysisRequest, OperatingPoint};
use application_frontend::flatten;

use crate::analysis_request::PyAnalysisRequest;
use crate::graph::PyCircuitGraph;
use crate::result::PyAnalysisResult;

/// Python class: `circuit_solver.Simulator`.
///
/// Stateless v1 entry point that consumes a [`PyCircuitGraph`] and a
/// [`PyAnalysisRequest`] and produces a [`PyAnalysisResult`]. The
/// class itself is `#[pyclass(frozen)]` so multiple submissions can
/// share a single Simulator instance without surfacing
/// `&mut self`-only methods.
#[pyclass(name = "Simulator", module = "circuit_solver", frozen)]
#[derive(Default)]
pub struct PySimulator {
    // Intentionally empty in v1. The downstream cross-analysis
    // operating-point cache (auto-DC for AC / noise, tasks.md #26 /
    // #40) will land a `Mutex<Option<OperatingPoint>>` here once
    // those analysis types come online. Until then the simulator is
    // a pure dispatcher.
}

#[pymethods]
impl PySimulator {
    /// Construct a fresh stateless `Simulator`.
    #[new]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Submit an [`AnalysisRequest`](crate::PyAnalysisRequest) against
    /// a [`CircuitGraph`](crate::PyCircuitGraph) and return a
    /// [`Result`](crate::PyAnalysisResult).
    ///
    /// # Arguments
    ///
    /// - `graph` — the immutable circuit graph produced by
    ///   `CircuitBuilder.build()` or `circuit_solver.parse_netlist`.
    /// - `request` — the immutable analysis request produced by
    ///   `AnalysisRequest(...)`.
    ///
    /// # Returns
    ///
    /// A [`PyAnalysisResult`] whose populated channels depend on the
    /// analysis type:
    ///
    /// - **`dc-operating-point`** — `node_voltages` (keyed by node
    ///   name) and `branch_currents` (keyed by element name) are
    ///   populated; the `waveforms` and `transfer_functions`
    ///   channels are empty.
    ///
    /// # Errors
    ///
    /// - `NotImplementedError` if the request's `analysis_type` is
    ///   any of `dc-sweep`, `ac-small-signal`,
    ///   `transient-time-domain`, `noise-spectral-density`, or
    ///   `mixed-signal-cosim`. These have their own scenarios and
    ///   downstream submission tasks; see the module-level docs.
    /// - `ValueError` if Pass 1 flattening of the input
    ///   [`CircuitGraph`] fails (structurally impossible for graphs
    ///   produced by `CircuitBuilder.build()`, but mapped to a typed
    ///   error rather than a panic for forward compatibility with
    ///   `parse_netlist`).
    /// - `RuntimeError` if the DC analysis produced no operating
    ///   point at all (the analysis-orchestration layer signals
    ///   that as a non-convergence outcome; the message carries the
    ///   `ConvergenceStatus` variant). The Gherkin scenario for
    ///   convergence failure is on the `dc-operating-point` capability
    ///   (`dc-operating-point-convergence-failure`) and is owned by
    ///   tasks.md #22 — for the `python-frontend` capability we
    ///   surface the failure as a typed `RuntimeError` so the v1
    ///   Python contract is "submission either returns a populated
    ///   Result or raises".
    pub fn submit(
        &self,
        py: Python<'_>,
        graph: &PyCircuitGraph,
        request: &PyAnalysisRequest,
    ) -> PyResult<PyAnalysisResult> {
        match request.kind() {
            AnalysisType::DcOperatingPoint => dispatch_dc(py, graph),
            other => Err(pyo3::exceptions::PyNotImplementedError::new_err(format!(
                "Simulator.submit does not yet dispatch analysis_type {:?}; \
                 only 'dc-operating-point' is wired in this release. The \
                 sibling python-frontend scenarios cover the other analysis \
                 types on their own kanban tasks.",
                other.slug()
            ))),
        }
    }

    /// Short diagnostic representation.
    ///
    /// Shape: `Simulator()`. Stable enough for log scraping but not
    /// part of the public contract; ADR-0010 keeps the `__repr__`
    /// surface unstable.
    //
    // The `&self` receiver is required by PyO3's pymethod dispatch
    // (instance methods bind to `self`), even though the body is
    // constant-only. Suppress the `unused_self` lint locally rather
    // than reshaping the method into an associated function, which
    // would not register as a Python instance method.
    #[allow(clippy::unused_self)]
    fn __repr__(&self) -> &'static str {
        "Simulator()"
    }
}

/// Dispatch a DC operating-point analysis: flatten Pass 1, run the
/// Newton-Raphson driver under [`Python::detach`], and project
/// the resulting [`OperatingPoint`] into a name-keyed
/// [`PyAnalysisResult`].
fn dispatch_dc(py: Python<'_>, graph: &PyCircuitGraph) -> PyResult<PyAnalysisResult> {
    let inner_graph: &CircuitGraph = graph.as_inner();

    // Pass 1 — flatten incidence. Cheap relative to solve; no GIL
    // release needed because the operation is bounded by the
    // graph's element count and the Python side has nothing to
    // gain from concurrent progress here.
    let structure: FlattenedStructure = flatten(inner_graph).map_err(|e| {
        PyValueError::new_err(format!("CircuitGraph rejected by Pass 1 flattening: {e}"))
    })?;

    // Pass 2 + Newton-Raphson — heavy native work. Drop the GIL so
    // other Python threads can progress concurrently per the
    // python-frontend acceptance criterion. `Python::detach` is the
    // PyO3 0.28 spelling of the prior `allow_threads` API; the
    // semantics are identical (the closure runs without the GIL
    // held; the `Python` token is consumed for the duration).
    let dc_result = py.detach(|| {
        dc_analysis(DcAnalysisRequest {
            graph: inner_graph,
            structure: &structure,
            newton_raphson: None,
            ground: None,
            device_models: None,
            enable_gmin_fallback: true,
        })
    });

    let dc_result = dc_result.map_err(|e| map_dc_error(&e))?;

    let op = dc_result.operating_point.ok_or_else(|| {
        PyRuntimeError::new_err(format!(
            "DC analysis produced no operating point ({:?})",
            dc_result.convergence
        ))
    })?;

    let node_voltages = project_node_voltages(inner_graph, &op);
    let branch_currents = project_branch_currents(inner_graph, &structure, &op);

    Ok(Python::try_attach(|py| {
        PyAnalysisResult::from_channels(
            py,
            node_voltages,
            branch_currents,
            BTreeMap::new(),
            BTreeMap::new(),
        )
    })
    .expect("GIL is held after Python::detach returns"))
}

/// Project the [`OperatingPoint::node_voltages`] vector (indexed by
/// [`NodeId::index`]) into a name-keyed map suitable for
/// [`PyAnalysisResult::from_channels`]. Ground is included under its
/// canonical net name (`"0"` by default) so callers that introspect
/// the channel see a stable, complete view.
fn project_node_voltages(graph: &CircuitGraph, op: &OperatingPoint) -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    for node in graph.nodes() {
        let id: NodeId = node.id();
        if let Some(v) = op.voltage_at(id) {
            out.insert(node.name().to_string(), v);
        }
    }
    out
}

/// Project the [`OperatingPoint::branch_currents`] samples (indexed by
/// [`BranchId`]) into an element-name-keyed map. The branch → element
/// mapping comes from the [`FlattenedStructure`]: each
/// [`circuit_solver_types::flattened::ElementIncidence`] either owns
/// a `Some(branch)` (current-carrying element — voltage source,
/// inductor) or has no branch (resistor, capacitor, current source).
/// Only the current-carrying elements appear in the projected map,
/// keyed by their netlist names (e.g. `"V1"`).
fn project_branch_currents(
    graph: &CircuitGraph,
    structure: &FlattenedStructure,
    op: &OperatingPoint,
) -> BTreeMap<String, f64> {
    // Build a BranchId → element name lookup once.
    let mut branch_to_name: BTreeMap<u32, String> = BTreeMap::new();
    for incidence in structure.elements() {
        if let Some(branch) = incidence.branch {
            let element_id = incidence.element;
            if let Some(element) = graph.elements().get(element_id.index() as usize) {
                branch_to_name.insert(branch.index(), element.name().as_str().to_string());
            }
        }
    }

    let mut out = BTreeMap::new();
    for sample in &op.branch_currents {
        let branch: BranchId = sample.branch;
        if let Some(name) = branch_to_name.get(&branch.index()) {
            out.insert(name.clone(), sample.current_amperes);
        }
    }
    out
}

/// Map a [`DcAnalysisError`] to a Python exception. The variants are
/// flattened to either `ValueError` (input rejected before/during
/// assembly) or `RuntimeError` (solver internal failure) — the
/// Python boundary does not yet model the full Rust error taxonomy
/// (ADR-0010 keeps the v1 surface narrow); the diagnostic text
/// preserves the original variant name for log-scraping debuggers.
///
/// The match arms enumerate every current variant by name (rather
/// than catching the remainder with `_`) so adding a new variant to
/// [`DcAnalysisError`] in the future causes an exhaustiveness error
/// here, forcing a deliberate choice about which Python exception
/// class the new variant should map to.
fn map_dc_error(e: &DcAnalysisError) -> PyErr {
    match e {
        DcAnalysisError::AssemblyFailed(_)
        | DcAnalysisError::SubViewBuildFailed(_)
        | DcAnalysisError::FloatingNodeFault { .. } => {
            PyValueError::new_err(format!("DC analysis rejected input: {e:?}"))
        }
        DcAnalysisError::NewtonRaphsonFailed(_) => {
            PyRuntimeError::new_err(format!("DC analysis failed: {e:?}"))
        }
        DcAnalysisError::GminHomotopyFailed(_) => {
            // Hard failure inside the Gmin-stepping homotopy driver
            // (schedule validation, dim mismatch, or a non-convergence
            // outcome the driver could not lift). Non-convergence on
            // the user-facing surface (NR or homotopy not finding a
            // solution) flows through the `Ok(DcAnalysisResult)` path
            // and never reaches here; this arm is for the driver's
            // own pre-loop / hard-error surface.
            PyRuntimeError::new_err(format!("DC homotopy failed: {e:?}"))
        }
    }
}
