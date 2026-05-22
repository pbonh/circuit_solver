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
