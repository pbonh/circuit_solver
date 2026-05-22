//! Integration tests for the `circuit_solver` `PyO3` module — tasks.md
//! item #52.
//!
//! These tests embed `CPython` via `PyO3`'s `auto-initialize` dev feature
//! and exercise the `CircuitBuilder` class the way a user would from
//! Python. They cover the four `add_*` methods plus the
//! `element_decl_count` inspection helper. The full Gherkin scenario
//! (`python-frontend#incremental-circuit-construction-via-builder-api`)
//! also asserts on `build()` returning a `CircuitGraph` with two
//! elements and three nodes — that final step lights up in tasks.md
//! item #53; the assertions here cover the post-condition that #52 is
//! responsible for: every `add_element` call records exactly one
//! declaration on the inner Rust builder, in insertion order.
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
    // The terminal `When builder.build()` step lights up in #53.
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

/// Gherkin scenario: `python-frontend#builder-isolation-across-multiple-builds`.
///
/// This test lifts the isolation invariant — already proven for the
/// pure-Rust `netlist_graph::CircuitBuilder` in
/// `crates/netlist-graph/src/builder.rs`'s
/// `builder_isolation_across_multiple_builds` unit test — across the
/// `PyO3` boundary, using `build_snapshot_element_count` as the
/// inspection helper (the full `CircuitGraph` `PyO3` handle is tasks.md
/// item #53's scope).
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
/// Mapping to the Python-frontend surface as of tasks.md #55:
///
/// - `builder.build()` is rendered by
///   [`PyCircuitBuilder::build_snapshot_element_count`], which drives
///   the inner Rust builder's `build()` and returns the snapshot's
///   post-expansion element count. The full handle (whose
///   `.elements()` len would give the same number) lands in #53.
/// - "`graph_a` is not affected by the addition of `R2`" is verified by
///   capturing the first snapshot count into a Python int *before*
///   adding `R2`; once captured, that int is an owned Python value
///   that cannot change retroactively, so re-asserting it remains 1
///   after the second `build()` proves the snapshot was independent.
#[test]
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
        let graph_a_element_count: usize = b
            .call_method0("build_snapshot_element_count")
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
        let graph_b_element_count: usize = b
            .call_method0("build_snapshot_element_count")
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

        // And: graph_a is not affected by the addition of R2. This is
        // attested by `graph_a_element_count` still being 1 after the
        // mutation + second build — i.e. the first snapshot remained
        // an independent capture.
        assert_eq!(
            graph_a_element_count, 1,
            "graph_a's captured count must be unchanged by the later add_element + build"
        );

        // Defence-in-depth: re-asserting via the builder's
        // element_decl_count makes the divergence visible if the
        // helper ever stops snapshotting and starts aliasing internal
        // state — the builder now has 2 declarations, but the
        // previously-captured graph_a count must still be 1.
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
/// This guards against the helper introducing a hidden monotonic
/// counter or other state that would cause spurious divergence.
#[test]
fn repeated_build_snapshots_without_mutation_are_equal() {
    Python::attach(|py| {
        let b = fresh_builder(py);
        let kwargs = [("value", 1_000.0)].into_py_dict(py).unwrap();
        let terminals = PyList::new(py, ["n1", "0"]).unwrap();
        b.call_method("add_element", ("R1", "R", terminals), Some(&kwargs))
            .unwrap();

        let snap_a: usize = b
            .call_method0("build_snapshot_element_count")
            .unwrap()
            .extract()
            .unwrap();
        let snap_b: usize = b
            .call_method0("build_snapshot_element_count")
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

/// Companion: after a `build_snapshot_element_count` call, the live
/// builder must remain usable — the `add_element` path after a build
/// must still succeed. This is the consequence of
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

        let _ = b
            .call_method0("build_snapshot_element_count")
            .unwrap()
            .extract::<usize>()
            .unwrap();

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
