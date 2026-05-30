//! Scenario-level integration test for
//! `python-frontend#incremental-circuit-construction-via-builder-api`.
//!
//! This file is the executable witness for the Gherkin scenario inlined
//! into kanban task `t_1442b70e`. It exercises the **public** `PyO3`
//! surface of the `circuit_solver` module end-to-end the way a Python
//! user would, exactly as written in the scenario:
//!
//! ```gherkin
//! Given PythonDeveloper imports the circuit_solver module
//! When  PythonDeveloper creates a CircuitBuilder and adds a resistor
//!       "R1" between nodes "n1" and "n2" with value 1 kΩ
//! And   PythonDeveloper adds a voltage source "V1" between nodes "n2"
//!       and "0" with value 5 V
//! And   PythonDeveloper calls builder.build()
//! Then  the returned object is an immutable CircuitGraph
//! And   the CircuitGraph contains two elements and three nodes
//! ```
//!
//! Sibling unit/integration tests inside
//! `crates/circuit-solver-py/tests/circuit_builder.rs` already pin the
//! finer-grained surface contracts (per-method success / failure paths,
//! subcircuit-expansion error propagation, defence-in-depth checks on
//! builder isolation). This file is intentionally narrower and
//! load-bearing for **this** scenario only: it consumes solely the
//! public `circuit_solver` crate exports (`PyCircuitBuilder`,
//! `PyCircuitGraph`, `CircuitBuilderError`) through the same
//! `call_method*` dispatch path an `import circuit_solver` import would
//! use, so a future refactor that breaks the v1 Python surface fails
//! here loudly.
//!
//! # Glossary terms exercised
//!
//! Per the task body's inlined glossary (verbatim, not paraphrased):
//!
//! - `Circuit` — the top-level object representing a netlist and its
//!   associated models. *Realised* as the `CircuitGraph` value the
//!   `CircuitBuilder.build()` call returns.
//! - `Netlist` — the textual or programmatic circuit description.
//!   *Realised* by the sequence of `add_element` calls the scenario
//!   issues.
//!
//! The scenario does not exercise `Simulator`, `Analysis`, `Result`,
//! `OperatingPoint`, `Waveform`, `TransferFunction`, `SmallSignal`,
//! `LargeSignal`, `Sweep`, `Convergence`, `Golden Reference`, or
//! `Conformance` — those terms are owned by the sibling per-scenario
//! tasks in the `python-frontend` capability spec
//! (`analysis-request-and-result-retrieval`,
//! `zero-copy-numpy-result-arrays`, `gil-release-during-simulation`,
//! `spice-netlist-file-parsing`, `error-on-malformed-netlist`) and by
//! the four solver-side capabilities (`dc-operating-point`,
//! `ac-small-signal`, `transient-time-domain`,
//! `noise-spectral-density`).
//!
//! # ADRs honoured
//!
//! - **ADR-0001** (`PyO3` in-process binding with immutable
//!   `CircuitGraph`). The scenario's *"the returned object is an
//!   immutable `CircuitGraph`"* Then-clause is satisfied structurally
//!   by `#[pyclass(frozen)]` on
//!   `crate::graph::PyCircuitGraph` (no `&mut self` `#[pymethods]`
//!   compile against a `frozen` class). This test reads the Python
//!   type name back through `get_type().name()` to confirm the class
//!   identity, and additionally asserts that *invoking each `add_*`
//!   trap method* on the returned graph raises
//!   `ImmutableHandleError` with a message that names the attempted
//!   method and cites the immutability invariant — the direct
//!   behavioural realisation of task #54's contract (merged in
//!   `ebf976c`) at the scenario layer. Prior to task #54, the
//!   scenario relied on `getattr`-absence for each `add_*`
//!   method as a defence-in-depth proxy; the trap-method
//!   approach now provides a stronger, direct observable signal
//!   that mutation is rejected.
//! - **ADR-0010** (Unstable Public Rust API Surface for v1). This
//!   witness uses only the names re-exported from
//!   `crate::{PyCircuitBuilder, PyCircuitGraph, CircuitBuilderError}`,
//!   pinning the v1 Python surface for this scenario. Any rename or
//!   removal of those names without coordinated scenario-witness
//!   updates breaks this test loudly, which is the intended
//!   v1-stability signal.
//! - **ADR-0006**, **ADR-0007**, **ADR-0008**, **ADR-0009** are listed
//!   on the task body but are *vacuously honoured* by this scenario:
//!   it constructs structure only, with no Newton-Raphson iteration
//!   (ADR-0006), no analog-digital boundary exchange (ADR-0007), no
//!   golden-reference comparison (ADR-0008), and no topology
//!   classification (ADR-0009).
//!
//! # `cfg`-gate rationale
//!
//! `#![cfg(not(feature = "extension-module"))]` is identical to the
//! gate on the sibling `circuit_builder.rs` test binary: the
//! `extension-module` feature is incompatible with linking the Python
//! ABI directly into a test binary. The whole module is skipped under
//! the default feature set, so workspace `cargo test --workspace`
//! still passes. The dedicated recipe for this crate is:
//!
//! ```text
//! cargo test -p circuit-solver-py --no-default-features
//! ```

#![cfg(not(feature = "extension-module"))]

use circuit_solver::{ImmutableHandleError, PyCircuitBuilder};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

/// Helper: produce a fresh Python-side `CircuitBuilder` instance the
/// way `circuit_solver.CircuitBuilder()` would from Python user code.
///
/// Going through `Bound::new` exercises the same `PyO3` allocator path
/// `Python::with_gil(|py| { let b = PyCircuitBuilder::new(); ... })`
/// would; the `#[pymethods]` constructor on `PyCircuitBuilder`
/// (registered via `#[pymodule] circuit_solver`) is reachable from
/// `import circuit_solver; circuit_solver.CircuitBuilder()` and from
/// this helper alike.
fn fresh_builder(py: Python<'_>) -> Bound<'_, PyCircuitBuilder> {
    Bound::new(py, PyCircuitBuilder::new()).expect(
        "constructing PyCircuitBuilder via Bound::new must succeed in a Python::attach scope",
    )
}

/// Scenario witness for
/// `python-frontend#incremental-circuit-construction-via-builder-api`.
///
/// This test is the single canonical end-to-end execution of the
/// scenario as written; if it fails, the spec scenario is no longer
/// satisfied by the trunk codebase. The body walks the Gherkin
/// `Given` / `When` / `And` / `Then` steps in order, with comments
/// quoting each step verbatim. Each assertion maps to exactly one
/// Then-clause.
#[test]
fn scenario_incremental_circuit_construction_via_builder_api() {
    Python::attach(|py| {
        // Given PythonDeveloper imports the circuit_solver module.
        //
        // In the Rust-side test harness, importing the module is
        // equivalent to bringing the re-exported pyclass into scope
        // (PyO3 registers the same Python type either way: the
        // `#[pymodule] fn circuit_solver` registers `CircuitBuilder`
        // and `CircuitGraph` against the interpreter, and the
        // `use circuit_solver::PyCircuitBuilder;` line above makes the
        // Rust binding to the same pyclass available to the test).
        // Asserting the binding is reachable would amount to asserting
        // `use` succeeded; instead we directly invoke the constructor
        // via `Bound::new` and treat the absence of a panic as the
        // realisation of the Given step.
        let builder = fresh_builder(py);

        // When PythonDeveloper creates a CircuitBuilder and adds a
        //      resistor "R1" between nodes "n1" and "n2" with value
        //      1 kΩ.
        //
        // `value=1000.0` is 1 kΩ in SI units; the scenario's "1 kΩ"
        // text is a human-friendly rendering of the float a Python
        // caller would actually pass. `terminals=["n1", "n2"]` is the
        // ordered terminal list the `PyCircuitBuilder.add_element`
        // signature expects. The `kind="R"` discriminator maps to
        // `netlist_graph::ElementKind::R` inside the binding.
        let kwargs_r = [("value", 1000.0)]
            .into_py_dict(py)
            .expect("kwargs dict construction must succeed");
        let terminals_r =
            PyList::new(py, ["n1", "n2"]).expect("terminal list construction must succeed");
        builder
            .call_method("add_element", ("R1", "R", terminals_r), Some(&kwargs_r))
            .expect("add_element(R1, R, [n1, n2], value=1000.0) must succeed");

        // And PythonDeveloper adds a voltage source "V1" between
        //     nodes "n2" and "0" with value 5 V.
        //
        // Node "0" is the conventional SPICE ground reference. The
        // `kind="V"` discriminator maps to
        // `netlist_graph::ElementKind::V` inside the binding.
        let kwargs_v = [("value", 5.0)]
            .into_py_dict(py)
            .expect("kwargs dict construction must succeed");
        let terminals_v =
            PyList::new(py, ["n2", "0"]).expect("terminal list construction must succeed");
        builder
            .call_method("add_element", ("V1", "V", terminals_v), Some(&kwargs_v))
            .expect("add_element(V1, V, [n2, 0], value=5.0) must succeed");

        // And PythonDeveloper calls builder.build().
        //
        // `build()` runs subcircuit expansion and returns a fresh
        // immutable `CircuitGraph` handle, per ADR-0001 and tasks.md
        // item #53.
        let graph = builder
            .call_method0("build")
            .expect("builder.build() must succeed on a well-formed builder");

        // Then the returned object is an immutable CircuitGraph.
        //
        // Three independent observations pin "immutable CircuitGraph":
        //
        //   1. The Python type name reported by `type(graph).__name__`
        //      must be `"CircuitGraph"` — this confirms the class
        //      identity.
        //   2. The class is `#[pyclass(frozen)]` (verified
        //      structurally; the `Bound<PyAny>` we hold cannot
        //      acquire a `&mut self` borrow at the PyO3 layer).
        //   3. As a direct behavioural check, *invoking* each `add_*`
        //      trap method (`add_element`, `add_wire`, `add_model`,
        //      `add_subcircuit`) on the returned graph must raise
        //      `ImmutableHandleError`, and the error message must
        //      name the attempted method and cite the immutability
        //      invariant (the string `"immutable"` or the governing
        //      ADR id `"ADR-0001"`). Task #54 (merged in `ebf976c`)
        //      added these trap methods specifically to make this
        //      scenario assertion possible; before #54 the scenario
        //      had to settle for `getattr`-absence as a defence-in-
        //      depth proxy.
        let type_name: String = graph
            .get_type()
            .name()
            .expect("type().__name__ must be readable")
            .extract()
            .expect("type().__name__ must extract as String");
        assert_eq!(
            type_name, "CircuitGraph",
            "Then-clause requires the returned object to be a CircuitGraph; got {type_name}"
        );

        for forbidden in ["add_element", "add_wire", "add_model", "add_subcircuit"] {
            // Call the trap method with no positional or keyword
            // arguments; #54 made the four `add_*` methods on
            // `CircuitGraph` `#[pyo3(signature = (*_args, **_kwargs))]`
            // unconditional raisers, so the empty-arg call is
            // sufficient to exercise the trap. We do *not* care
            // whether real-shaped arguments would also raise; the
            // contract is "any call raises", and the simplest call
            // we can construct is the empty one.
            let err = graph
                .call_method0(forbidden)
                .expect_err("immutable CircuitGraph trap method must raise on call, not return Ok");
            assert!(
                err.is_instance_of::<ImmutableHandleError>(py),
                "immutable CircuitGraph mutator '{forbidden}' must raise \
                 ImmutableHandleError; got: {err}"
            );
            let msg = err.to_string();
            assert!(
                msg.contains(forbidden),
                "ImmutableHandleError raised by '{forbidden}' must name the \
                 attempted method in its message; got: {msg}"
            );
            assert!(
                msg.contains("immutable") || msg.contains("ADR-0001"),
                "ImmutableHandleError raised by '{forbidden}' must explain why \
                 mutation is rejected (mention 'immutable' or 'ADR-0001'); got: {msg}"
            );
        }

        // And the CircuitGraph contains two elements and three nodes.
        //
        // "Two elements" — R1 and V1, accessible by name via
        // `element_names()`.
        //
        // "Three nodes" — ground "0", "n1", "n2". The PyO3 binding
        // delegates `node_count()` to
        // `netlist_graph::CircuitGraph::node_count`, which counts
        // distinct node identifiers including the SPICE ground
        // reference per the netlist-graph implementation merged in
        // task `t_b98bc22c`.
        let element_count: usize = graph
            .call_method0("element_count")
            .expect("element_count() must be callable")
            .extract()
            .expect("element_count() must return usize");
        assert_eq!(
            element_count, 2,
            "Then-clause requires two elements (R1, V1); got {element_count}"
        );

        let node_count: usize = graph
            .call_method0("node_count")
            .expect("node_count() must be callable")
            .extract()
            .expect("node_count() must return usize");
        assert_eq!(
            node_count, 3,
            "Then-clause requires three nodes (0, n1, n2); got {node_count}"
        );

        // Pin the *identities* of the elements and nodes too. Counts
        // alone could be satisfied by a buggy aliasing or duplicate
        // path; checking the names confirms the netlist topology was
        // recorded as written.
        let element_names: Vec<String> = graph
            .call_method0("element_names")
            .expect("element_names() must be callable")
            .extract()
            .expect("element_names() must return list[str]");
        assert_eq!(
            element_names,
            vec!["R1".to_string(), "V1".to_string()],
            "element names must be the two declared by the scenario, in declaration order"
        );

        let node_names: Vec<String> = graph
            .call_method0("node_names")
            .expect("node_names() must be callable")
            .extract()
            .expect("node_names() must return list[str]");
        for expected in ["0", "n1", "n2"] {
            assert!(
                node_names.contains(&expected.to_string()),
                "node_names must contain the scenario's node '{expected}'; got {node_names:?}"
            );
        }
        assert_eq!(
            node_names.len(),
            3,
            "node_names must contain exactly the three scenario nodes; got {node_names:?}"
        );
    });
}

/// Defence-in-depth: re-running the scenario must yield the same
/// observable state every time. Multiple `Python::attach` scopes can
/// share one embedded interpreter under `auto-initialize`; running the
/// scenario twice within one test process confirms there is no hidden
/// per-run state leak in `PyCircuitBuilder`'s constructor or in the
/// `#[pymodule]` registration path.
#[test]
fn scenario_is_deterministic_across_repeated_runs() {
    for run in 1..=3 {
        Python::attach(|py| {
            let builder = fresh_builder(py);

            let kwargs_r = [("value", 1000.0)].into_py_dict(py).unwrap();
            let terminals_r = PyList::new(py, ["n1", "n2"]).unwrap();
            builder
                .call_method("add_element", ("R1", "R", terminals_r), Some(&kwargs_r))
                .unwrap();

            let kwargs_v = [("value", 5.0)].into_py_dict(py).unwrap();
            let terminals_v = PyList::new(py, ["n2", "0"]).unwrap();
            builder
                .call_method("add_element", ("V1", "V", terminals_v), Some(&kwargs_v))
                .unwrap();

            let graph = builder.call_method0("build").unwrap();

            let element_count: usize = graph
                .call_method0("element_count")
                .unwrap()
                .extract()
                .unwrap();
            let node_count: usize = graph.call_method0("node_count").unwrap().extract().unwrap();

            assert_eq!(
                element_count, 2,
                "run {run}: element_count must remain 2 across repeated scenario runs"
            );
            assert_eq!(
                node_count, 3,
                "run {run}: node_count must remain 3 across repeated scenario runs"
            );
        });
    }
}

/// Defence-in-depth: explicitly verify the immutable-handle property
/// at the Python boundary by attempting (and failing) a `setattr` on
/// the returned `CircuitGraph`. The main scenario test already
/// asserts the canonical `ImmutableHandleError` raised by the `add_*`
/// trap methods (#54 contract); this `setattr` check covers the
/// orthogonal attribute-assignment axis of immutability, which is
/// owned by `#[pyclass(frozen)]` rather than by the trap methods.
///
/// Note: `#[pyclass(frozen)]` causes `setattr` to fail with
/// `AttributeError: attribute '<x>' of 'CircuitGraph' objects is not
/// writable` (or a similar message depending on `PyO3` version). We do
/// not pin the exact message; we only require that the setattr fails.
#[test]
fn returned_graph_rejects_python_setattr() {
    Python::attach(|py| {
        let builder = fresh_builder(py);

        let kwargs_r = [("value", 1000.0)].into_py_dict(py).unwrap();
        let terminals_r = PyList::new(py, ["n1", "n2"]).unwrap();
        builder
            .call_method("add_element", ("R1", "R", terminals_r), Some(&kwargs_r))
            .unwrap();
        let kwargs_v = [("value", 5.0)].into_py_dict(py).unwrap();
        let terminals_v = PyList::new(py, ["n2", "0"]).unwrap();
        builder
            .call_method("add_element", ("V1", "V", terminals_v), Some(&kwargs_v))
            .unwrap();

        let graph = builder.call_method0("build").unwrap();

        let setattr_result = graph.setattr("inner", py.None());
        assert!(
            setattr_result.is_err(),
            "Python-side setattr on an immutable CircuitGraph must fail; got {setattr_result:?}"
        );
    });
}
