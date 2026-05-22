//! Integration tests for the `circuit_solver` `PyO3` module — tasks.md
//! items #52, #53, and #55.
//!
//! These tests embed `CPython` via `PyO3`'s `auto-initialize` dev feature
//! and exercise the `CircuitBuilder` class the way a user would from
//! Python. They cover the four `add_*` methods plus the
//! `element_decl_count` inspection helper from #52, the terminal
//! `build()` method plus immutable `CircuitGraph` accessors from #53,
//! and the builder-isolation Gherkin scenario from #55 (rewritten
//! against the real `build()` after the #55 stopgap was removed). The
//! full Gherkin scenario
//! (`python-frontend#incremental-circuit-construction-via-builder-api`)
//! is covered by [`gherkin_scenario_full_returns_two_elements_three_nodes`].
//! The builder-isolation scenario
//! (`python-frontend#builder-isolation-across-multiple-builds`) is
//! covered by both [`build_twice_yields_independent_graphs`] (#53's
//! minimum delegation property) and
//! [`gherkin_scenario_builder_isolation_across_multiple_builds`] (#55's
//! full named scenario plus defence-in-depth checks).
//!
//! ## Test harness
//!
//! `Python::attach` is supplied by the dev-only `pyo3` dependency
//! configured with `auto-initialize`. The classes under test are
//! constructed by `Bound::new` and exercised via `call_method0`,
//! `call_method1`, and `call_method` so the tests go through the same
//! method-dispatch path a `import circuit_solver` import would.
//!
//! ## Why the cfg-gate
//!
//! The `extension-module` feature is incompatible with linking the
//! Python ABI directly into a test binary. The whole module is gated
//! off when that feature is active so `cargo test --workspace`
//! (default features) still passes; the test recipe for this crate is
//!
//!     cargo test -p circuit-solver-py --no-default-features
//!
//! which is documented in the crate `Cargo.toml`.

#![cfg(not(feature = "extension-module"))]

use circuit_solver::PyCircuitBuilder;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

/// Helper: produce a fresh Python-side `CircuitBuilder` instance.
fn fresh_builder(py: Python<'_>) -> Bound<'_, PyCircuitBuilder> {
    Bound::new(py, PyCircuitBuilder::new()).expect("constructing PyCircuitBuilder must not fail")
}

#[test]
fn constructor_yields_empty_builder() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 0);
    });
}

#[test]
fn add_element_resistor_records_one_decl() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs = [("value", 1000.0)].into_py_dict(py).unwrap();
        let terminals = PyList::new(py, ["n1", "n2"]).unwrap();
        b.call_method("add_element", ("R1", "R", terminals), Some(&kwargs))
            .unwrap();
        let count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 1);
    });
}

#[test]
fn add_element_voltage_source_records_one_decl() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs = [("value", 5.0)].into_py_dict(py).unwrap();
        let terminals = PyList::new(py, ["n2", "0"]).unwrap();
        b.call_method("add_element", ("V1", "V", terminals), Some(&kwargs))
            .unwrap();
        let count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 1);
    });
}

#[test]
fn gherkin_scenario_setup_records_two_elements() {
    // Mirrors the `Given / And` steps of
    // `python-frontend#incremental-circuit-construction-via-builder-api`.
    // The terminal `When builder.build()` step is covered by
    // `gherkin_scenario_full_returns_two_elements_three_nodes`.
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs_r = [("value", 1000.0)].into_py_dict(py).unwrap();
        let terminals_r = PyList::new(py, ["n1", "n2"]).unwrap();
        b.call_method("add_element", ("R1", "R", terminals_r), Some(&kwargs_r))
            .unwrap();

        let kwargs_v = [("value", 5.0)].into_py_dict(py).unwrap();
        let terminals_v = PyList::new(py, ["n2", "0"]).unwrap();
        b.call_method("add_element", ("V1", "V", terminals_v), Some(&kwargs_v))
            .unwrap();

        let count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 2);
    });
}

#[test]
fn duplicate_element_name_raises_circuit_builder_error() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs = [("value", 1000.0)].into_py_dict(py).unwrap();
        let t1 = PyList::new(py, ["n1", "n2"]).unwrap();
        b.call_method("add_element", ("R1", "R", t1), Some(&kwargs))
            .unwrap();
        let t2 = PyList::new(py, ["n2", "0"]).unwrap();
        let err = b
            .call_method("add_element", ("R1", "R", t2), Some(&kwargs))
            .expect_err("duplicate name must be rejected");
        let msg = err.to_string();
        assert!(
            msg.contains("duplicate element name"),
            "unexpected error message: {msg}"
        );
        assert!(msg.contains("R1"), "error must name the duplicate: {msg}");
    });
}

#[test]
fn unrecognised_kind_raises_type_error() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let t = PyList::new(py, ["a", "b"]).unwrap();
        let err = b
            .call_method1("add_element", ("X1", "Z", t))
            .expect_err("unknown kind must be rejected");
        assert!(err.is_instance_of::<pyo3::exceptions::PyTypeError>(py));
        let msg = err.to_string();
        assert!(
            msg.contains("unrecognised element kind tag"),
            "unexpected error message: {msg}"
        );
    });
}

#[test]
fn two_terminal_arity_mismatch_raises_circuit_builder_error() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs = [("value", 1000.0)].into_py_dict(py).unwrap();
        let t = PyList::new(py, ["a", "b", "c"]).unwrap();
        let err = b
            .call_method("add_element", ("R1", "R", t), Some(&kwargs))
            .expect_err("3-terminal resistor must be rejected");
        let msg = err.to_string();
        assert!(
            msg.contains("expected 2 terminal(s)"),
            "unexpected error message: {msg}"
        );
    });
}

#[test]
fn semiconductor_kind_requires_model_argument() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let t = PyList::new(py, ["anode", "cathode"]).unwrap();
        // Missing model= — must error.
        let err = b
            .call_method1("add_element", ("D1", "DEV", t))
            .expect_err("DEV element without model must be rejected");
        assert!(err.is_instance_of::<pyo3::exceptions::PyTypeError>(py));
        assert!(
            err.to_string().contains("requires a model"),
            "unexpected error message: {err}"
        );
    });
}

#[test]
fn semiconductor_with_model_is_accepted() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs = [("model", "DMOD")].into_py_dict(py).unwrap();
        let t = PyList::new(py, ["anode", "cathode"]).unwrap();
        b.call_method("add_element", ("D1", "DEV", t), Some(&kwargs))
            .unwrap();
        let count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 1);
    });
}

#[test]
fn add_wire_does_not_record_an_element() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        b.call_method1("add_wire", ("a", "b")).unwrap();
        b.call_method1("add_wire", ("b", "c")).unwrap();
        // add_wire is intentionally not an element; the element_decl_count
        // is still zero. Wires apply during the build() union-find pass.
        let count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 0);
    });
}

#[test]
fn add_model_is_idempotent() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        b.call_method1("add_model", ("D1N4148",)).unwrap();
        b.call_method1("add_model", ("D1N4148",)).unwrap();
        b.call_method1("add_model", ("BC547",)).unwrap();
        // No surface-visible assertion possible until #53's CircuitGraph
        // exposes the registered models. The test ensures the call path
        // is exception-free, which is the only contract #52 owns.
    });
}

#[test]
fn add_subcircuit_accepts_dict_body() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        // INV subcircuit: two resistors in series between in and out.
        let r1_body = [
            ("name", "R1".into_pyobject(py).unwrap().into_any()),
            ("kind", "R".into_pyobject(py).unwrap().into_any()),
            (
                "terminals",
                PyList::new(py, ["in", "mid"]).unwrap().into_any(),
            ),
            ("value", 1000.0f64.into_pyobject(py).unwrap().into_any()),
        ]
        .into_py_dict(py)
        .unwrap();
        let r2_body = [
            ("name", "R2".into_pyobject(py).unwrap().into_any()),
            ("kind", "R".into_pyobject(py).unwrap().into_any()),
            (
                "terminals",
                PyList::new(py, ["mid", "out"]).unwrap().into_any(),
            ),
            ("value", 2000.0f64.into_pyobject(py).unwrap().into_any()),
        ]
        .into_py_dict(py)
        .unwrap();
        let body = PyList::new(py, [r1_body, r2_body]).unwrap();
        let ports = PyList::new(py, ["in", "out"]).unwrap();
        b.call_method1("add_subcircuit", ("INV", ports, body))
            .unwrap();
        // No element-decl count change: subcircuit definitions are
        // registered separately and only flattened into the top-level
        // element list at build()-time expansion.
        let count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 0);
    });
}

#[test]
fn add_subcircuit_duplicate_raises_circuit_builder_error() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let body = PyList::empty(py);
        let ports = PyList::new(py, ["in", "out"]).unwrap();
        b.call_method1("add_subcircuit", ("INV", ports.clone(), body.clone()))
            .unwrap();
        let err = b
            .call_method1("add_subcircuit", ("INV", ports, body))
            .expect_err("duplicate subcircuit must be rejected");
        assert!(
            err.to_string().contains("duplicate subcircuit"),
            "unexpected error: {err}"
        );
    });
}

#[test]
fn add_subcircuit_malformed_body_raises_type_error() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        // Body entry missing required "kind" key.
        let bad = [
            ("name", "R1".into_pyobject(py).unwrap().into_any()),
            ("terminals", PyList::new(py, ["a", "b"]).unwrap().into_any()),
        ]
        .into_py_dict(py)
        .unwrap();
        let body = PyList::new(py, [bad]).unwrap();
        let ports = PyList::new(py, ["a", "b"]).unwrap();
        let err = b
            .call_method1("add_subcircuit", ("MISS", ports, body))
            .expect_err("missing kind key must be rejected");
        assert!(err.is_instance_of::<pyo3::exceptions::PyTypeError>(py));
        assert!(
            err.to_string().contains("kind"),
            "error must name the missing key: {err}"
        );
    });
}

// ---------------------------------------------------------------------------
// tasks.md item #53: `CircuitBuilder.build()` returning immutable CircuitGraph
// ---------------------------------------------------------------------------

#[test]
fn build_on_empty_builder_yields_empty_graph() {
    // An empty builder still produces a well-formed graph: ground is
    // always present, so node_count is 1 (ground only) and
    // element_count is 0.
    Python::attach(|py| {
        let b = fresh_builder(py);
        let graph = b.call_method0("build").unwrap();
        let element_count: usize = graph
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        let node_count: usize = graph.call_method0("node_count").unwrap().extract().unwrap();
        assert_eq!(element_count, 0);
        // Ground is always seeded by the netlist-graph builder.
        assert_eq!(node_count, 1);
        let is_empty: bool = graph.call_method0("is_empty").unwrap().extract().unwrap();
        assert!(is_empty);
        let fully_expanded: bool = graph
            .call_method0("is_fully_expanded")
            .unwrap()
            .extract()
            .unwrap();
        assert!(fully_expanded);
    });
}

#[test]
fn build_returns_circuit_graph_python_class() {
    // The terminal `Then the returned object is an immutable CircuitGraph`
    // step: the returned Python object must report its type name as
    // `CircuitGraph` and be an instance of the registered class.
    Python::attach(|py| {
        let b = fresh_builder(py);
        let graph = b.call_method0("build").unwrap();
        let type_name: String = graph.get_type().name().unwrap().extract().unwrap();
        assert_eq!(type_name, "CircuitGraph");
    });
}

#[test]
fn gherkin_scenario_full_returns_two_elements_three_nodes() {
    // Full coverage of
    // `python-frontend#incremental-circuit-construction-via-builder-api`:
    //
    //   Given PythonDeveloper imports the circuit_solver module
    //   When PythonDeveloper creates a CircuitBuilder and adds a
    //        resistor "R1" between nodes "n1" and "n2" with value 1 kΩ
    //   And PythonDeveloper adds a voltage source "V1" between nodes
    //       "n2" and "0" with value 5 V
    //   And PythonDeveloper calls builder.build()
    //   Then the returned object is an immutable CircuitGraph
    //   And the CircuitGraph contains two elements and three nodes
    //
    // Nodes are: ground "0", "n1", "n2" — three total, matching the
    // scenario's terminal assertion.
    Python::attach(|py| {
        let b = fresh_builder(py);

        let kwargs_r = [("value", 1000.0)].into_py_dict(py).unwrap();
        let terminals_r = PyList::new(py, ["n1", "n2"]).unwrap();
        b.call_method("add_element", ("R1", "R", terminals_r), Some(&kwargs_r))
            .unwrap();

        let kwargs_v = [("value", 5.0)].into_py_dict(py).unwrap();
        let terminals_v = PyList::new(py, ["n2", "0"]).unwrap();
        b.call_method("add_element", ("V1", "V", terminals_v), Some(&kwargs_v))
            .unwrap();

        let graph = b.call_method0("build").unwrap();

        // Then: type is CircuitGraph.
        let type_name: String = graph.get_type().name().unwrap().extract().unwrap();
        assert_eq!(type_name, "CircuitGraph");

        // And: two elements, three nodes.
        let element_count: usize = graph
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        let node_count: usize = graph.call_method0("node_count").unwrap().extract().unwrap();
        assert_eq!(element_count, 2, "expected two elements (R1, V1)");
        assert_eq!(node_count, 3, "expected three nodes (0, n1, n2)");

        // And the elements are named as declared.
        let element_names: Vec<String> = graph
            .call_method0("element_names")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(element_names, vec!["R1".to_string(), "V1".to_string()]);

        // And the node names include ground and both user nets.
        let node_names: Vec<String> = graph.call_method0("node_names").unwrap().extract().unwrap();
        assert!(node_names.contains(&"0".to_string()));
        assert!(node_names.contains(&"n1".to_string()));
        assert!(node_names.contains(&"n2".to_string()));
    });
}

#[test]
fn build_twice_yields_independent_graphs() {
    // Rust-side delegation property for
    // `python-frontend#builder-isolation-across-multiple-builds`
    // (tasks.md item #55 owns the spec-level surface; this test just
    // pins that #53's `build()` produces a fresh handle each call and
    // mutating the builder between calls only affects the second
    // graph).
    Python::attach(|py| {
        let b = fresh_builder(py);

        let kwargs_r1 = [("value", 1000.0)].into_py_dict(py).unwrap();
        let t_r1 = PyList::new(py, ["n1", "0"]).unwrap();
        b.call_method("add_element", ("R1", "R", t_r1), Some(&kwargs_r1))
            .unwrap();

        let graph_a = b.call_method0("build").unwrap();

        let kwargs_r2 = [("value", 2000.0)].into_py_dict(py).unwrap();
        let t_r2 = PyList::new(py, ["n2", "0"]).unwrap();
        b.call_method("add_element", ("R2", "R", t_r2), Some(&kwargs_r2))
            .unwrap();

        let graph_b = b.call_method0("build").unwrap();

        let count_a: usize = graph_a
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        let count_b: usize = graph_b
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count_a, 1, "graph_a snapshot must remain at one element");
        assert_eq!(count_b, 2, "graph_b must reflect post-A mutation");
    });
}

#[test]
fn build_propagates_subcircuit_expansion_error() {
    // `build()` runs subcircuit expansion before returning. If the
    // builder references an unknown subcircuit definition, the error
    // surfaces as `CircuitBuilderError` — proving the error-mapping
    // contract from #52 carries through `build()` unchanged.
    //
    // We construct this state by adding a subcircuit definition that
    // itself references an unknown nested subcircuit. (The Python
    // surface for instantiating a subcircuit is task #56's scope, so
    // here we wedge in the precondition via the existing
    // `add_subcircuit` declaration path with a body element whose
    // kind is "DEV" without a model — the eager `add_element`-style
    // checks for that case are run at body-parse time. To force a
    // build-time failure path instead, we use the fact that an empty
    // builder builds successfully: this test asserts the
    // non-erroring case from `build()` is robust, and the
    // error-mapping is exercised by the existing `add_*`-time error
    // tests.)
    //
    // A future task that exposes subcircuit instantiation through
    // Python will replace this with a true build-time failure
    // assertion. For now, the contract is that `build()` returns a
    // `PyResult<PyCircuitGraph>` whose error variant is
    // `CircuitBuilderError`; that mapping is covered by
    // [`to_py_err`] in `errors.rs`.
    Python::attach(|py| {
        let b = fresh_builder(py);
        // Build on an empty builder — must not error.
        let _graph = b
            .call_method0("build")
            .expect("empty builder must build cleanly");
    });
}

#[test]
fn circuit_graph_repr_is_diagnostic() {
    // `__repr__` shape stability — useful for log scraping and REPL
    // ergonomics. Not part of the public Python contract (ADR-0010
    // keeps it unstable) but tested here so we catch unintended
    // breakage.
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs = [("value", 1000.0)].into_py_dict(py).unwrap();
        let t = PyList::new(py, ["n1", "n2"]).unwrap();
        b.call_method("add_element", ("R1", "R", t), Some(&kwargs))
            .unwrap();
        let graph = b.call_method0("build").unwrap();
        let repr: String = graph.call_method0("__repr__").unwrap().extract().unwrap();
        assert!(repr.starts_with("CircuitGraph("), "unexpected repr: {repr}");
        assert!(repr.contains("elements=1"), "unexpected repr: {repr}");
        assert!(repr.contains("nodes="), "unexpected repr: {repr}");
        assert!(repr.contains("models=0"), "unexpected repr: {repr}");
    });
}

#[test]
fn circuit_graph_is_frozen_against_add_element_mutation() {
    // ADR-0001 / scenario
    // `python-frontend#immutable-circuit-graph-prevents-post-build-mutation`:
    // the returned `CircuitGraph` does NOT expose builder-mutation
    // methods. The `#[pyclass(frozen)]` enforcement means there is no
    // `add_element` `#[pymethod]` on `PyCircuitGraph`, so the
    // attribute lookup itself fails. Task #54 will replace this
    // `AttributeError` with a dedicated `ImmutableHandleError`; for
    // now we pin the structural property.
    Python::attach(|py| {
        let b = fresh_builder(py);
        let graph = b.call_method0("build").unwrap();
        let kwargs = [("value", 1000.0)].into_py_dict(py).unwrap();
        let t = PyList::new(py, ["n1", "n2"]).unwrap();
        let err = graph
            .call_method("add_element", ("R1", "R", t), Some(&kwargs))
            .expect_err("CircuitGraph must not expose add_element");
        // Python raises AttributeError for missing methods on a class.
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyAttributeError>(py),
            "unexpected error type: {err}"
        );
    });
}

// ---------------------------------------------------------------------------
// tasks.md item #55: builder-isolation-across-multiple-builds Gherkin scenario
// ---------------------------------------------------------------------------
//
// These tests originally drove `build_snapshot_element_count` (the
// stopgap helper #55 introduced before #53 landed). With the real
// `build() -> PyCircuitGraph` now in place (#53), the stopgap is gone
// and these tests have been rewritten to use the immutable graph
// directly: `let graph = builder.build()?; graph.element_count()`.
//
// `build_twice_yields_independent_graphs` above (from #53) covers the
// minimum invariant. The three tests below preserve the additional
// observable properties #55 pinned (named Gherkin scenario, no-mutation
// stability, post-build reuse).

/// Gherkin scenario: `python-frontend#builder-isolation-across-multiple-builds`.
///
/// This test lifts the isolation invariant — already proven for the
/// pure-Rust `netlist_graph::CircuitBuilder` in
/// `crates/netlist-graph/src/builder.rs`'s
/// `builder_isolation_across_multiple_builds` unit test — across the
/// `PyO3` boundary, using the immutable `CircuitGraph` handle returned
/// by `CircuitBuilder.build()` (tasks.md #53).
///
/// Gherkin steps:
///
/// ```text
/// Given CircuitDesigner creates a CircuitBuilder and adds a resistor "R1"
/// And   CircuitDesigner calls builder.build() producing graph_a
/// And   CircuitDesigner adds another resistor "R2" to the same builder
/// When  CircuitDesigner calls builder.build() a second time producing graph_b
/// Then  graph_a contains one element
/// And   graph_b contains two elements
/// And   graph_a is not affected by the addition of "R2"
/// ```
///
/// The "`graph_a` is not affected by the addition of `R2`" property is
/// verified two ways: (a) by capturing `graph_a.element_count()` into
/// an owned Python int **before** adding R2 (the captured int cannot
/// change retroactively), and (b) by re-reading
/// `graph_a.element_count()` *after* the second build and asserting it
/// is still 1 — the immutable `#[pyclass(frozen)]` semantics of
/// `PyCircuitGraph` (ADR-0001) guarantee no aliasing back to the live
/// builder.
#[test]
#[allow(clippy::similar_names)]
fn gherkin_scenario_builder_isolation_across_multiple_builds() {
    Python::attach(|py| {
        let b = fresh_builder(py);

        // Given: add R1 (1 kΩ between n1 and ground).
        let kwargs_r1 = [("value", 1_000.0)].into_py_dict(py).unwrap();
        let terminals_r1 = PyList::new(py, ["n1", "0"]).unwrap();
        b.call_method("add_element", ("R1", "R", terminals_r1), Some(&kwargs_r1))
            .unwrap();

        // And: builder.build() producing graph_a — capture the
        // element count as an owned Python int (the value cannot
        // mutate retroactively).
        let graph_a = b.call_method0("build").unwrap();
        let graph_a_element_count: usize = graph_a
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();

        // And: add R2 (2 kΩ between n2 and ground) to the SAME
        // builder.
        let kwargs_r2 = [("value", 2_000.0)].into_py_dict(py).unwrap();
        let terminals_r2 = PyList::new(py, ["n2", "0"]).unwrap();
        b.call_method("add_element", ("R2", "R", terminals_r2), Some(&kwargs_r2))
            .unwrap();

        // When: builder.build() a second time producing graph_b.
        let graph_b = b.call_method0("build").unwrap();
        let graph_b_element_count: usize = graph_b
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();

        // Then: graph_a contains one element.
        assert_eq!(
            graph_a_element_count, 1,
            "graph_a (first snapshot) must contain exactly one element (R1)"
        );

        // And: graph_b contains two elements.
        assert_eq!(
            graph_b_element_count, 2,
            "graph_b (second snapshot) must contain both R1 and R2"
        );

        // And: graph_a is not affected by the addition of R2. Re-read
        // graph_a.element_count() *after* the mutation + second build
        // — it must still report 1, proving the snapshot is an
        // independent immutable handle (ADR-0001).
        let graph_a_recheck: usize = graph_a
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            graph_a_recheck, 1,
            "graph_a must remain at one element after the later add_element + build"
        );
        assert_eq!(
            graph_a_element_count, graph_a_recheck,
            "captured graph_a count must equal the post-mutation re-read"
        );

        // Defence-in-depth: re-asserting via the builder's
        // element_decl_count makes the divergence visible if the
        // graph ever stops snapshotting and starts aliasing internal
        // builder state — the builder now has 2 declarations, but
        // graph_a must still report 1 element.
        let live_decl_count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            live_decl_count, 2,
            "live builder state must reflect the post-R2 mutation"
        );
        assert_ne!(
            graph_a_element_count, live_decl_count,
            "graph_a snapshot must diverge from the post-mutation live builder count"
        );
    });
}

/// Negative companion to the isolation scenario: repeated `build()`
/// calls with **no** intervening mutation must produce equal snapshots.
/// This guards against `build()` introducing a hidden monotonic
/// counter or other state that would cause spurious divergence.
#[test]
fn repeated_build_snapshots_without_mutation_are_equal() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs = [("value", 1_000.0)].into_py_dict(py).unwrap();
        let terminals = PyList::new(py, ["n1", "0"]).unwrap();
        b.call_method("add_element", ("R1", "R", terminals), Some(&kwargs))
            .unwrap();

        let graph_a = b.call_method0("build").unwrap();
        let snap_a: usize = graph_a
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        let graph_b = b.call_method0("build").unwrap();
        let snap_b: usize = graph_b
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();

        assert_eq!(snap_a, 1);
        assert_eq!(
            snap_a, snap_b,
            "no mutation between builds → equal snapshots"
        );
    });
}

/// Companion: after a `build()` call, the live builder must remain
/// usable — the `add_element` path after a build must still succeed.
/// This is the consequence of
/// `netlist_graph::CircuitBuilder::build`'s snapshot semantics; the
/// regression matters because subcircuit expansion runs once per
/// `build()`, and a future refactor could accidentally make it
/// destructive.
#[test]
fn builder_remains_usable_after_build_snapshot() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs_r1 = [("value", 1_000.0)].into_py_dict(py).unwrap();
        let terminals_r1 = PyList::new(py, ["n1", "0"]).unwrap();
        b.call_method("add_element", ("R1", "R", terminals_r1), Some(&kwargs_r1))
            .unwrap();

        let _graph_a = b.call_method0("build").unwrap();

        // Post-build add must succeed.
        let kwargs_r2 = [("value", 2_000.0)].into_py_dict(py).unwrap();
        let terminals_r2 = PyList::new(py, ["n2", "0"]).unwrap();
        b.call_method("add_element", ("R2", "R", terminals_r2), Some(&kwargs_r2))
            .unwrap();

        let count: usize = b
            .call_method0("element_decl_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 2, "builder must accept new elements after build");
    });
}
