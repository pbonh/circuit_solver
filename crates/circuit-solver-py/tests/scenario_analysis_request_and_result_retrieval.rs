//! Scenario-level integration test for
//! `python-frontend#analysis-request-and-result-retrieval`.
//!
//! This file is the executable witness for the Gherkin scenario
//! inlined into kanban task `t_edf5defe`:
//!
//! ```gherkin
//! Given CircuitDesigner has built a CircuitGraph containing a resistive divider
//! When CircuitDesigner creates an AnalysisRequest for DC operating point
//! And CircuitDesigner submits the AnalysisRequest to the Simulator
//! Then the Simulator returns a Result object
//! And the Result contains node voltages accessible by node name
//! And the voltage at node "n1" is approximately 5 V within the tolerance envelope
//! ```
//!
//! The test exercises the **public** `circuit_solver` Python surface
//! end-to-end through the same `call_method*` dispatch path an `import
//! circuit_solver` user would take:
//!
//! 1. Construct a `CircuitBuilder` and build a resistive divider whose
//!    midpoint node is named `"n1"` (per the Gherkin step
//!    *"the voltage at node \"n1\" is approximately 5 V"*).
//! 2. Construct an `AnalysisRequest` with `analysis_type="dc-operating-point"`.
//! 3. Construct a `Simulator()` and call `submit(graph, request)`.
//! 4. Read `result.node_voltage("n1")` and assert it sits inside the
//!    ADR-0008 per-node `max(relative, absolute)` envelope around the
//!    analytic golden reference of 5 V.
//!
//! # Golden-reference choice
//!
//! Per the inlined glossary, *"Golden Reference — a trusted external
//! simulator against which results are compared."* The cross-cutting
//! ngspice conformance harness lives at tasks.md item #62 and is gated
//! on the full v1 stack; that harness is tracked on its own kanban
//! thread. For a **linear resistive** divider the analytic DC operating
//! point is closed-form and *exactly* what any industrial simulator
//! would converge to. We therefore use the analytic solution as the
//! golden reference for this scenario witness, matching the precedent
//! established by `analysis-orchestration`'s
//! `tests/scenario_linear_resistive_dc_operating_point.rs`.
//!
//! # Tolerance envelope (ADR-0008 row "DC")
//!
//! Per ADR-0008 the DC defaults are 1 % relative / 1 mV absolute, and
//! the pass criterion is `|err| ≤ max(rel · |ref|, abs)`. For the
//! 5 V midpoint, the relative term (50 mV) dominates and we beat it
//! by many decades against the analytic reference.
//!
//! # cfg-gate rationale
//!
//! `#![cfg(not(feature = "extension-module"))]` matches the sibling
//! test binaries: the `extension-module` Cargo feature is incompatible
//! with linking the Python ABI directly into a test binary. The whole
//! module is skipped under the default feature set; the dedicated
//! recipe for this crate is `cargo test -p circuit-solver-py
//! --no-default-features`.

#![cfg(not(feature = "extension-module"))]

use circuit_solver::{
    PyAnalysisRequest, PyAnalysisResult, PyCircuitBuilder, PyCircuitGraph, PySimulator,
};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

/// Relative tolerance for DC node voltages, per ADR-0008 row "DC".
const DC_V_REL: f64 = 0.01;
/// Absolute floor for DC node voltages, per ADR-0008 row "DC".
const DC_V_ABS: f64 = 1e-3;

/// ADR-0008 envelope predicate: `|actual − reference| ≤
/// max(rel · |reference|, abs)`.
fn within_envelope(actual: f64, reference: f64, rel: f64, abs: f64) -> bool {
    let bound = (rel * reference.abs()).max(abs);
    (actual - reference).abs() <= bound
}

/// Build a resistive voltage divider through the Python `CircuitBuilder`:
///
/// ```text
///   V1 (10 V, n_top → 0) → R1 (1 kΩ, n_top → n1) → R2 (1 kΩ, n1 → 0)
/// ```
///
/// Analytic golden reference: `V(n_top) = 10 V`, `V(n1) = 5 V`,
/// `V(0) = 0`.
///
/// The midpoint is named **`n1`** verbatim because the Gherkin scenario
/// asserts on that exact node name (*"the voltage at node \"n1\" is
/// approximately 5 V"*).
fn build_resistive_divider(py: Python<'_>) -> Bound<'_, PyCircuitGraph> {
    let builder = Bound::new(py, PyCircuitBuilder::new())
        .expect("constructing PyCircuitBuilder must succeed in Python::attach scope");

    // V1: 10 V source between n_top and 0 (SPICE ground).
    let kwargs_v = [("value", 10.0)]
        .into_py_dict(py)
        .expect("kwargs dict for V1 must succeed");
    let terms_v = PyList::new(py, ["n_top", "0"]).expect("terminal list for V1 must succeed");
    builder
        .call_method("add_element", ("V1", "V", terms_v), Some(&kwargs_v))
        .expect("add_element(V1) must succeed");

    // R1: 1 kΩ between n_top and n1.
    let kwargs_r1 = [("value", 1_000.0)]
        .into_py_dict(py)
        .expect("kwargs dict for R1 must succeed");
    let terms_r1 = PyList::new(py, ["n_top", "n1"]).expect("terminal list for R1 must succeed");
    builder
        .call_method("add_element", ("R1", "R", terms_r1), Some(&kwargs_r1))
        .expect("add_element(R1) must succeed");

    // R2: 1 kΩ between n1 and 0.
    let kwargs_r2 = [("value", 1_000.0)]
        .into_py_dict(py)
        .expect("kwargs dict for R2 must succeed");
    let terms_r2 = PyList::new(py, ["n1", "0"]).expect("terminal list for R2 must succeed");
    builder
        .call_method("add_element", ("R2", "R", terms_r2), Some(&kwargs_r2))
        .expect("add_element(R2) must succeed");

    let graph_any = builder
        .call_method0("build")
        .expect("builder.build() must succeed on a well-formed divider");
    graph_any
        .cast_into::<PyCircuitGraph>()
        .expect("build() must return CircuitGraph")
}

/// The canonical end-to-end scenario witness.
///
/// Walks the Gherkin `Given` / `When` / `And` / `Then` steps in order,
/// with comments quoting each step verbatim. Each assertion maps to
/// exactly one Then-clause.
#[test]
fn scenario_analysis_request_and_result_retrieval() {
    Python::attach(|py| {
        // Given CircuitDesigner has built a CircuitGraph containing a
        //       resistive divider.
        let graph = build_resistive_divider(py);

        // When CircuitDesigner creates an AnalysisRequest for DC
        //      operating point.
        let request = Bound::new(
            py,
            PyAnalysisRequest::new("dc-operating-point", None, None, None)
                .expect("constructing AnalysisRequest must succeed"),
        )
        .expect("Bound::new(PyAnalysisRequest) must succeed");

        // And CircuitDesigner submits the AnalysisRequest to the
        //     Simulator.
        let sim = Bound::new(py, PySimulator::new()).expect("Bound::new(PySimulator) must succeed");
        let result_any = sim
            .call_method("submit", (graph.clone(), request.clone()), None)
            .expect("Simulator.submit must succeed for a converged linear DC analysis");

        // Then the Simulator returns a Result object.
        //
        // Two independent observations pin "Result object":
        //
        //   1. The Python type name reported by `type(result).__name__`
        //      must be `"Result"` — confirms class identity.
        //   2. The object must downcast to `PyAnalysisResult` — confirms
        //      Rust-side type identity.
        let type_name: String = result_any
            .get_type()
            .name()
            .expect("type(result).__name__ readable")
            .extract()
            .expect("type name extracts as String");
        assert_eq!(
            type_name, "Result",
            "Then-clause requires submit() to return a Result object; got {type_name}"
        );
        let result = result_any
            .cast_into::<PyAnalysisResult>()
            .expect("submit() must return Result");

        // And the Result contains node voltages accessible by node
        //     name.
        //
        // We probe through the Python surface (call_method) rather
        // than the Rust-side accessor so the scenario witness is
        // exercising the same code path a Python user would.
        let node_names_any = result
            .call_method0("node_names")
            .expect("result.node_names() must succeed");
        let node_names: Vec<String> = node_names_any
            .extract()
            .expect("node_names() must extract as a sequence of strings");
        assert!(
            node_names.contains(&"n1".to_string()),
            "Then-clause requires node voltages accessible by node \
             name; node \"n1\" missing from {node_names:?}"
        );
        assert!(
            node_names.contains(&"n_top".to_string()),
            "node \"n_top\" missing from {node_names:?}"
        );

        // And the voltage at node "n1" is approximately 5 V within
        //     the tolerance envelope.
        //
        // ADR-0008 envelope: `|actual − reference| ≤ max(rel · |ref|,
        // abs)` with DC defaults (rel=1 %, abs=1 mV). For the 5 V
        // midpoint the relative term (50 mV) dominates; we beat it by
        // many decades against the analytic reference.
        let v_n1: f64 = result
            .call_method1("node_voltage", ("n1",))
            .expect("result.node_voltage(\"n1\") must succeed")
            .extract()
            .expect("node_voltage extracts as f64");
        assert!(
            within_envelope(v_n1, 5.0, DC_V_REL, DC_V_ABS),
            "DC voltage at n1 = {v_n1} V violates the ADR-0008 \
             envelope around the analytic golden reference 5 V \
             (rel={DC_V_REL}, abs={DC_V_ABS} V)"
        );
    });
}

/// Defence-in-depth: a second submission against the same Simulator
/// instance produces a structurally equivalent Result. The Simulator
/// is stateless in v1; this test pins that property so a future
/// caching refactor cannot silently introduce cross-submission state
/// leakage without breaking a witness.
#[test]
fn simulator_is_stateless_across_submissions() {
    Python::attach(|py| {
        let graph = build_resistive_divider(py);
        let request = Bound::new(
            py,
            PyAnalysisRequest::new("dc-operating-point", None, None, None)
                .expect("constructing AnalysisRequest must succeed"),
        )
        .expect("Bound::new(PyAnalysisRequest) must succeed");
        let sim = Bound::new(py, PySimulator::new()).expect("Bound::new(PySimulator) must succeed");

        let v_first: f64 = {
            let r = sim
                .call_method("submit", (graph.clone(), request.clone()), None)
                .expect("submit 1 must succeed");
            r.call_method1("node_voltage", ("n1",))
                .expect("node_voltage 1 must succeed")
                .extract()
                .expect("extract f64 1")
        };
        let v_second: f64 = {
            let r = sim
                .call_method("submit", (graph.clone(), request.clone()), None)
                .expect("submit 2 must succeed");
            r.call_method1("node_voltage", ("n1",))
                .expect("node_voltage 2 must succeed")
                .extract()
                .expect("extract f64 2")
        };
        assert!(
            (v_first - v_second).abs() < 1e-12,
            "Stateless Simulator must produce bit-identical results \
             across submissions; got v1={v_first}, v2={v_second}"
        );
    });
}

/// Defence-in-depth: the Result also exposes the source-branch current
/// for V1 keyed by element name. This pins the
/// branch-currents-by-element-name projection so future refactors
/// cannot drop the channel without breaking a witness.
#[test]
fn result_exposes_branch_current_by_element_name() {
    Python::attach(|py| {
        let graph = build_resistive_divider(py);
        let request = Bound::new(
            py,
            PyAnalysisRequest::new("dc-operating-point", None, None, None)
                .expect("constructing AnalysisRequest must succeed"),
        )
        .expect("Bound::new(PyAnalysisRequest) must succeed");
        let sim = Bound::new(py, PySimulator::new()).expect("Bound::new(PySimulator) must succeed");

        let r = sim
            .call_method("submit", (graph.clone(), request.clone()), None)
            .expect("submit must succeed");

        let branch_names_any = r
            .call_method0("branch_names")
            .expect("branch_names() must succeed");
        let branch_names: Vec<String> = branch_names_any
            .extract()
            .expect("branch_names() extracts as Vec<String>");
        assert!(
            branch_names.contains(&"V1".to_string()),
            "branch_currents must include V1; got {branch_names:?}"
        );

        // Analytic: i_V1 = 10 V / (1 kΩ + 1 kΩ) = 5 mA. The sign
        // depends on the assembler stamping convention; the
        // magnitude is the testable invariant per
        // `analysis-orchestration/tests/scenario_linear_resistive_dc_operating_point.rs`.
        let i_v1: f64 = r
            .call_method1("branch_current", ("V1",))
            .expect("branch_current(V1) must succeed")
            .extract()
            .expect("branch_current extracts as f64");
        let i_v1_mag = i_v1.abs();
        // Loose check: within 1 % of 5 mA, abs floor 1 µA.
        let reference = 5e-3;
        let bound = (0.01_f64 * reference).max(1e-6);
        assert!(
            (i_v1_mag - reference).abs() <= bound,
            "Branch current magnitude through V1 = {i_v1_mag} A \
             violates the ADR-0008 DC current envelope around \
             reference {reference} A"
        );
    });
}

/// The Simulator must surface a `NotImplementedError` for analysis
/// types that have not yet been wired through. This pins the v1
/// failure-mode contract so callers can detect missing dispatchers
/// rather than silently getting an empty Result.
#[test]
fn submit_for_unimplemented_analysis_type_raises_not_implemented() {
    Python::attach(|py| {
        let graph = build_resistive_divider(py);

        // Build the AC sweep tuple via Python so the f64/usize
        // literals don't need explicit type annotations on
        // `into_pyobject` — going through `PyTuple::new` keeps the
        // test surface clean.
        let sweep = pyo3::types::PyTuple::new(
            py,
            [
                pyo3::IntoPyObject::into_pyobject(1.0_f64, py)
                    .expect("f64 to py")
                    .into_any()
                    .unbind(),
                pyo3::IntoPyObject::into_pyobject(1.0e6_f64, py)
                    .expect("f64 to py")
                    .into_any()
                    .unbind(),
                pyo3::IntoPyObject::into_pyobject(10_usize, py)
                    .expect("usize to py")
                    .into_any()
                    .unbind(),
                pyo3::IntoPyObject::into_pyobject("log", py)
                    .expect("str to py")
                    .into_any()
                    .unbind(),
            ],
        )
        .expect("sweep tuple construction must succeed");

        let request = Bound::new(
            py,
            PyAnalysisRequest::new("ac-small-signal", Some(&sweep.into_any()), None, None)
                .expect("AC AnalysisRequest must construct"),
        )
        .expect("Bound::new(PyAnalysisRequest) must succeed");
        let sim = Bound::new(py, PySimulator::new()).expect("Bound::new(PySimulator) must succeed");

        let err = sim
            .call_method("submit", (graph, request), None)
            .expect_err("AC submit must raise NotImplementedError in v1 since only DC is wired");
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyNotImplementedError>(py),
            "AC submit must raise NotImplementedError; got: {err}"
        );
    });
}
