//! Scenario-level integration test for
//! `python-frontend#immutable-circuit-graph-prevents-post-build-mutation`.
//!
//! This file is the executable witness for the Gherkin scenario inlined
//! into kanban task `t_0ed71e96`. It exercises the **public** `PyO3`
//! surface of the `circuit_solver` module end-to-end the way a Python
//! user would, exactly as written in the scenario:
//!
//! ```gherkin
//! Given CircuitDesigner has built a CircuitGraph via the builder API
//! When  CircuitDesigner attempts to call an add-element method on the CircuitGraph
//! Then  a Python exception of type "ImmutableHandleError" is raised
//! And   the CircuitGraph remains unchanged
//! ```
//!
//! Sibling unit/integration tests inside
//! `crates/circuit-solver-py/tests/circuit_builder.rs` already pin the
//! finer-grained surface contracts that get this scenario over the
//! line:
//!
//! - `circuit_graph_is_frozen_against_add_element_mutation` — establishes
//!   that `CircuitGraph.add_element` raises `ImmutableHandleError` with
//!   a message naming the attempted method and explaining the rejection
//!   (tasks.md #54);
//! - `circuit_graph_traps_every_builder_mutation_method_with_immutable_handle_error`
//!   — defence-in-depth check that *every* `add_*` method
//!   (`add_element`, `add_wire`, `add_model`, `add_subcircuit`) traps
//!   with the same exception class;
//! - the existing `gherkin_scenario_immutable_circuit_graph_prevents_post_build_mutation`
//!   inside `circuit_builder.rs` — written alongside #54 to exercise
//!   the scenario end-to-end against the trap surface as it landed.
//!
//! This file is intentionally narrower and load-bearing for **this**
//! scenario only: it consumes solely the public `circuit_solver` crate
//! exports (`PyCircuitBuilder`, `ImmutableHandleError`) through the
//! same `call_method*` dispatch path an `import circuit_solver` import
//! would use, so a future refactor that breaks the v1 Python surface
//! fails here loudly.
//!
//! The witness mirrors the file layout of
//! `scenario_incremental_circuit_construction.rs` (`t_1442b70e`) so the
//! per-scenario witness corpus is uniform across the
//! `python-frontend` capability.
//!
//! # Glossary terms exercised
//!
//! Per the task body's inlined glossary (verbatim, not paraphrased):
//!
//! - `Circuit` — the top-level object representing a netlist and its
//!   associated models. *Realised* as the `CircuitGraph` value the
//!   `CircuitBuilder.build()` call returns. The whole point of this
//!   scenario is that once realised, the `Circuit` is immutable.
//! - `Netlist` — the textual or programmatic circuit description.
//!   *Realised* by the single `add_element` call the Given step issues
//!   to give the graph non-trivial structure to compare against.
//!
//! The scenario does not exercise `Simulator`, `Analysis`, `Result`,
//! `OperatingPoint`, `Waveform`, `TransferFunction`, `SmallSignal`,
//! `LargeSignal`, `Sweep`, `Convergence`, `Golden Reference`, or
//! `Conformance` — those terms are owned by sibling per-scenario tasks
//! and by the four solver-side capabilities.
//!
//! # ADRs honoured
//!
//! - **ADR-0001** (`PyO3` in-process binding with immutable
//!   `CircuitGraph`). The scenario's Then-clause that demands an
//!   `ImmutableHandleError` is the *behavioural* manifestation of
//!   ADR-0001's structural `#[pyclass(frozen)]` guarantee: builder
//!   mutators on a built `CircuitGraph` must be visibly rejected to
//!   the Python caller, not silently swallowed and not deferred to a
//!   generic `AttributeError`. This test pins both the rejection
//!   *and* the post-rejection invariant ("remains unchanged").
//! - **ADR-0010** (Unstable Public Rust API Surface for v1). This
//!   witness uses only the names re-exported from
//!   `crate::{PyCircuitBuilder, ImmutableHandleError}`, pinning the
//!   v1 Python surface for this scenario. Any rename or removal of
//!   those names without coordinated scenario-witness updates breaks
//!   this test loudly, which is the intended v1-stability signal.
//! - **ADR-0006**, **ADR-0007**, **ADR-0008**, **ADR-0009** are listed
//!   on the task body but are *vacuously honoured* by this scenario:
//!   it constructs structure only, with no Newton-Raphson iteration
//!   (ADR-0006), no analog-digital boundary exchange (ADR-0007), no
//!   golden-reference comparison (ADR-0008), and no topology
//!   classification (ADR-0009).
//!
//! # `cfg`-gate rationale
//!
//! The `extension-module` feature is incompatible with linking the
//! Python ABI directly into a test binary. The whole module is gated
//! off when that feature is active so `cargo test --workspace`
//! (default features) still passes; the test recipe for this crate is
//!
//!     cargo test -p circuit-solver-py --no-default-features
//!
//! mirroring the recipe documented in
//! `crates/circuit-solver-py/Cargo.toml` and called out as load-bearing
//! by the `t_a425f12a` reviewer notes carried forward through `t_1d312132`.

#![cfg(not(feature = "extension-module"))]

use circuit_solver::{ImmutableHandleError, PyCircuitBuilder};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

/// Helper: produce a fresh Python-side `CircuitBuilder` instance the
/// way `circuit_solver.CircuitBuilder()` would from Python user code.
///
/// Going through `Bound::new` exercises the same `PyO3` allocator
/// path the `#[pymodule] circuit_solver` registration makes available
/// to `import circuit_solver; circuit_solver.CircuitBuilder()`.
fn fresh_builder(py: Python<'_>) -> Bound<'_, PyCircuitBuilder> {
    Bound::new(py, PyCircuitBuilder::new()).expect(
        "constructing PyCircuitBuilder via Bound::new must succeed in a Python::attach scope",
    )
}

/// Five-observation snapshot of a `PyCircuitGraph`'s public Python
/// surface. Captures the data the And-clause "the `CircuitGraph`
/// remains unchanged" must preserve across a rejected builder-mutation
/// call: three counts (element / node / model) plus two identity
/// lists (element names, node names).
///
/// Construction reads the snapshot through the same `call_method0`
/// dispatch path a Python caller would use, so any future signature
/// drift on `PyCircuitGraph`'s read-side accessors fails here.
#[derive(Debug, PartialEq, Eq)]
struct GraphSnapshot {
    element_count: usize,
    node_count: usize,
    model_count: usize,
    element_names: Vec<String>,
    node_names: Vec<String>,
}

impl GraphSnapshot {
    fn capture(graph: &Bound<'_, PyAny>) -> Self {
        let element_count: usize = graph
            .call_method0("element_count")
            .expect("element_count() must be callable on the built graph")
            .extract()
            .expect("element_count() must return usize");
        let node_count: usize = graph
            .call_method0("node_count")
            .expect("node_count() must be callable on the built graph")
            .extract()
            .expect("node_count() must return usize");
        let model_count: usize = graph
            .call_method0("model_count")
            .expect("model_count() must be callable on the built graph")
            .extract()
            .expect("model_count() must return usize");
        let element_names: Vec<String> = graph
            .call_method0("element_names")
            .expect("element_names() must be callable on the built graph")
            .extract()
            .expect("element_names() must return list[str]");
        let node_names: Vec<String> = graph
            .call_method0("node_names")
            .expect("node_names() must be callable on the built graph")
            .extract()
            .expect("node_names() must return list[str]");
        Self {
            element_count,
            node_count,
            model_count,
            element_names,
            node_names,
        }
    }
}

/// Scenario witness for
/// `python-frontend#immutable-circuit-graph-prevents-post-build-mutation`.
///
/// This test is the single canonical end-to-end execution of the
/// scenario as written; if it fails, the spec scenario is no longer
/// satisfied by the trunk codebase. The body walks the Gherkin
/// `Given` / `When` / `Then` / `And` steps in order, with comments
/// quoting each step verbatim. Each assertion maps to exactly one
/// Then- or And-clause.
///
/// The "remains unchanged" And-clause is verified by snapshotting the
/// graph's observable state (`element_count`, `node_count`,
/// `model_count`, `element_names`, `node_names`) before the rejected
/// mutation attempt and asserting pointwise equality after it. The
/// graph is given non-trivial pre-build content (a single resistor)
/// so the snapshot has structure to compare against — an empty graph
/// would satisfy "remains unchanged" trivially.
#[test]
fn scenario_immutable_circuit_graph_prevents_post_build_mutation() {
    Python::attach(|py| {
        // Given CircuitDesigner has built a CircuitGraph via the
        //       builder API.
        //
        // We populate the builder with a single resistor R1 (1 kΩ
        // between n1 and the SPICE ground reference "0") before
        // calling `build()`. This gives the resulting `CircuitGraph`
        // a non-trivial structural fingerprint that the And-clause
        // ("the CircuitGraph remains unchanged") can compare against.
        let builder = fresh_builder(py);

        let kwargs_r1 = [("value", 1_000.0)]
            .into_py_dict(py)
            .expect("kwargs dict construction must succeed");
        let terminals_r1 =
            PyList::new(py, ["n1", "0"]).expect("terminal list construction must succeed");
        builder
            .call_method("add_element", ("R1", "R", terminals_r1), Some(&kwargs_r1))
            .expect("add_element(R1, R, [n1, 0], value=1000.0) must succeed pre-build");

        let graph = builder
            .call_method0("build")
            .expect("builder.build() must succeed on a well-formed builder");

        // Snapshot the observable state *before* the rejected
        // mutation via the five-observation `GraphSnapshot` helper.
        let before = GraphSnapshot::capture(&graph);

        // Sanity-check the snapshot itself: the Given step put one
        // resistor between n1 and "0", so we expect one element, two
        // nodes, zero models, and element name "R1". If these fail,
        // the test is wrong, not the scenario.
        assert_eq!(
            before.element_count, 1,
            "pre-mutation snapshot must record exactly the one R1 element from the Given step"
        );
        assert_eq!(
            before.node_count, 2,
            "pre-mutation snapshot must record exactly the two nodes (n1, 0) from the Given step"
        );
        assert_eq!(
            before.model_count, 0,
            "pre-mutation snapshot must record zero models (none declared)"
        );
        assert_eq!(
            before.element_names,
            vec!["R1".to_string()],
            "pre-mutation snapshot must record element name R1"
        );

        // When CircuitDesigner attempts to call an add-element method
        //      on the CircuitGraph.
        //
        // We mirror the same Python call shape the builder uses
        // (`add_element(name, kind, terminals, value=...)`) so the
        // trap surface on `PyCircuitGraph` cannot escape detection by
        // a signature mismatch — the call must be dispatched to the
        // graph's trap method, which raises `ImmutableHandleError`.
        let kwargs_r2 = [("value", 2_000.0)]
            .into_py_dict(py)
            .expect("kwargs dict construction must succeed");
        let terminals_r2 =
            PyList::new(py, ["n1", "n2"]).expect("terminal list construction must succeed");
        let err = graph
            .call_method("add_element", ("R2", "R", terminals_r2), Some(&kwargs_r2))
            .expect_err(
                "add_element on a built CircuitGraph must raise; \
                 the immutable-handle guarantee is the whole point of this scenario",
            );

        // Then a Python exception of type "ImmutableHandleError" is
        //      raised.
        //
        // The scenario quotes the exception name in double quotes,
        // which we interpret as "the Python exception class registered
        // by the `circuit_solver` module as `ImmutableHandleError`" —
        // the same class re-exported from `crate::ImmutableHandleError`
        // (see `crates/circuit-solver-py/src/errors.rs`).
        assert!(
            err.is_instance_of::<ImmutableHandleError>(py),
            "expected ImmutableHandleError per scenario Then-clause; got: {err}"
        );

        // And the CircuitGraph remains unchanged.
        //
        // Recapture the snapshot and assert pointwise equality.
        // Drift here would indicate the trap method partially applied
        // the mutation before raising — the failure mode the
        // And-clause is designed to catch.
        let after = GraphSnapshot::capture(&graph);
        assert_eq!(
            after, before,
            "And-clause: CircuitGraph observable state must be unchanged after rejected add_element call"
        );
    });
}

/// Defence-in-depth: the And-clause "the `CircuitGraph` remains
/// unchanged" must hold even when the Given step leaves the graph
/// completely empty. An empty-graph case is the structural floor of
/// the immutable-handle invariant — if a buggy trap method partially
/// mutated even an empty graph, this test would catch it before the
/// main scenario witness above could.
#[test]
fn empty_graph_remains_unchanged_after_rejected_add_element() {
    Python::attach(|py| {
        // Given an empty built CircuitGraph (no pre-build content).
        let builder = fresh_builder(py);
        let graph = builder
            .call_method0("build")
            .expect("builder.build() on an empty builder must succeed");

        let before_element_count: usize = graph
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        let before_node_count: usize = graph.call_method0("node_count").unwrap().extract().unwrap();
        assert_eq!(before_element_count, 0);
        // Ground node "0" is always seeded by the netlist-graph
        // builder even on an empty builder; see the sibling
        // `build_on_empty_builder_yields_empty_graph` test in
        // `circuit_builder.rs`.
        assert_eq!(before_node_count, 1);

        // When add_element is called on the empty graph.
        let kwargs = [("value", 1_000.0)].into_py_dict(py).unwrap();
        let terminals = PyList::new(py, ["n1", "n2"]).unwrap();
        let err = graph
            .call_method("add_element", ("R1", "R", terminals), Some(&kwargs))
            .expect_err("add_element on an empty built CircuitGraph must still raise");

        // Then ImmutableHandleError is raised.
        assert!(
            err.is_instance_of::<ImmutableHandleError>(py),
            "expected ImmutableHandleError on empty graph; got: {err}"
        );

        // And the (still-empty) graph remains unchanged.
        let after_element_count: usize = graph
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        let after_node_count: usize = graph.call_method0("node_count").unwrap().extract().unwrap();
        assert_eq!(
            after_element_count, 0,
            "rejected mutation must not introduce an element on an empty graph"
        );
        assert_eq!(
            after_node_count, 1,
            "rejected mutation must not introduce a node on an empty graph (ground-only invariant)"
        );
    });
}

/// Defence-in-depth: re-running the scenario must yield the same
/// observable state every time. Running the scenario three times
/// within one test process confirms there is no hidden per-run state
/// leak in `PyCircuitBuilder`'s constructor, in the trap method on
/// `PyCircuitGraph`, or in the `#[pymodule]` registration path for
/// `ImmutableHandleError`.
#[test]
fn scenario_is_deterministic_across_repeated_runs() {
    for run in 1..=3 {
        Python::attach(|py| {
            let builder = fresh_builder(py);
            let kwargs_r1 = [("value", 1_000.0)].into_py_dict(py).unwrap();
            let terminals_r1 = PyList::new(py, ["n1", "0"]).unwrap();
            builder
                .call_method("add_element", ("R1", "R", terminals_r1), Some(&kwargs_r1))
                .unwrap();
            let graph = builder.call_method0("build").unwrap();

            let kwargs_r2 = [("value", 2_000.0)].into_py_dict(py).unwrap();
            let terminals_r2 = PyList::new(py, ["n1", "n2"]).unwrap();
            let err = graph
                .call_method("add_element", ("R2", "R", terminals_r2), Some(&kwargs_r2))
                .expect_err("add_element on built graph must raise");
            assert!(
                err.is_instance_of::<ImmutableHandleError>(py),
                "run {run}: expected ImmutableHandleError"
            );

            let element_count: usize = graph
                .call_method0("element_count")
                .unwrap()
                .extract()
                .unwrap();
            let node_count: usize = graph.call_method0("node_count").unwrap().extract().unwrap();
            assert_eq!(
                element_count, 1,
                "run {run}: element_count must remain 1 across repeated scenario runs"
            );
            assert_eq!(
                node_count, 2,
                "run {run}: node_count must remain 2 across repeated scenario runs"
            );
        });
    }
}
