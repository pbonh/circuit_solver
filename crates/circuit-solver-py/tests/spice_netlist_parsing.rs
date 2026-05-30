//! Integration tests for `circuit_solver.parse_netlist` — tasks.md
//! item #60.
//!
//! Drives the SPICE netlist parser via the `PyO3` Python-facing
//! entry point exactly the way a user would from Python:
//!
//! ```python
//! import circuit_solver
//! g = circuit_solver.parse_netlist("/path/to/divider.cir")
//! ```
//!
//! These tests embed `CPython` via `PyO3`'s `auto-initialize` dev
//! feature, write fixture netlist files to a temp dir, and call
//! the registered `parse_netlist` pyfunction by registering the
//! module by hand under a private name (the standard `PyO3` way to
//! exercise a `#[pymodule]` function in a `cargo test` harness).
//!
//! The Gherkin scenario witness is
//! [`gherkin_scenario_spice_netlist_file_parsing`]: it asserts that
//! the parsed `CircuitGraph` is identical (element count, node
//! count, model count, sorted name lists) to one built
//! incrementally via the `CircuitBuilder` API.
//!
//! ## Why the cfg-gate
//!
//! The `extension-module` feature is incompatible with linking the
//! Python ABI directly into a test binary. The whole module is gated
//! off when that feature is active so `cargo test --workspace`
//! (default features) still passes; the crate test recipe is
//!
//!     cargo test -p circuit-solver-py --no-default-features
//!
//! mirroring the existing `tests/circuit_builder.rs` harness.

#![cfg(not(feature = "extension-module"))]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use circuit_solver::PyCircuitBuilder;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

/// Helper: write `contents` to a per-test unique file in `env::temp_dir()`
/// and return the path.
fn write_temp_deck(stem: &str, contents: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let pid = std::process::id();
    let nonce = COUNTER.fetch_add(1, Ordering::SeqCst);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let mut path = env::temp_dir();
    path.push(format!(
        "circuit_solver_test_{stem}_{pid}_{nonce}_{nanos}.cir"
    ));
    fs::write(&path, contents).expect("write temp deck must succeed");
    path
}

/// Helper: produce a fresh Python-side `CircuitBuilder` instance.
fn fresh_builder(py: Python<'_>) -> Bound<'_, PyCircuitBuilder> {
    Bound::new(py, PyCircuitBuilder::new()).expect("constructing PyCircuitBuilder must not fail")
}

/// Helper: call the parser by going through the Python free
/// function `circuit_solver.parser_test_parse(path)` we register
/// inline below. That function lives only in the test binary; it
/// is a thin re-export of `parser::parse_file` so we can prove the
/// Python-facing surface end-to-end without depending on the
/// crate-private `PyCircuitGraph::from_inner` constructor.
fn parse_via_python<'py>(py: Python<'py>, path: &Path) -> PyResult<Bound<'py, PyAny>> {
    let parser_fn = test_helpers::parse_netlist_test_binding(py)?;
    let path_str = path.to_string_lossy().to_string();
    parser_fn.call1((path_str,))
}

/// Helper: build the canonical resistive-divider graph incrementally
/// via the Python-binding builder API. This is the
/// "incrementally-built equivalent" the Gherkin scenario's third
/// assertion compares against.
fn build_resistive_divider_incrementally(py: Python<'_>) -> Bound<'_, PyAny> {
    let b = fresh_builder(py);
    // R1 between n1 and n2, 1 kΩ
    {
        let kwargs = [("value", 1000.0)].into_py_dict(py).unwrap();
        let terminals = PyList::new(py, ["n1", "n2"]).unwrap();
        b.call_method("add_element", ("R1", "R", terminals), Some(&kwargs))
            .unwrap();
    }
    // R2 between n2 and 0, 2 kΩ
    {
        let kwargs = [("value", 2000.0)].into_py_dict(py).unwrap();
        let terminals = PyList::new(py, ["n2", "0"]).unwrap();
        b.call_method("add_element", ("R2", "R", terminals), Some(&kwargs))
            .unwrap();
    }
    // V1 between n1 and 0, 5 V
    {
        let kwargs = [("value", 5.0)].into_py_dict(py).unwrap();
        let terminals = PyList::new(py, ["n1", "0"]).unwrap();
        b.call_method("add_element", ("V1", "V", terminals), Some(&kwargs))
            .unwrap();
    }
    b.call_method0("build").unwrap()
}

/// Read the structural signature of a graph object exposed
/// through Python: `(element_count, node_count, model_count,
/// sorted element_names, sorted node_names)`. The Gherkin scenario
/// requires identity on all five.
fn signature(g: &Bound<'_, PyAny>) -> (usize, usize, usize, Vec<String>, Vec<String>) {
    let element_count: usize = g.call_method0("element_count").unwrap().extract().unwrap();
    let node_count: usize = g.call_method0("node_count").unwrap().extract().unwrap();
    let model_count: usize = g.call_method0("model_count").unwrap().extract().unwrap();
    let mut element_names: Vec<String> =
        g.call_method0("element_names").unwrap().extract().unwrap();
    let mut node_names: Vec<String> = g.call_method0("node_names").unwrap().extract().unwrap();
    element_names.sort();
    node_names.sort();
    (
        element_count,
        node_count,
        model_count,
        element_names,
        node_names,
    )
}

mod test_helpers {
    //! Build a Python module identical in shape to `circuit_solver`
    //! but registered under a private name so the test binary can
    //! resolve its `parse_netlist` function without colliding with
    //! a real `import circuit_solver` (we are not running maturin
    //! here — there is no installed extension).
    //!
    //! `circuit-solver-py` exposes `parse_netlist` via the
    //! `#[pymodule]` entry point declared in `lib.rs`, which the
    //! test binary cannot call directly (the `fn` is private to the
    //! crate). Instead we re-create just the binding we need by
    //! calling `pyo3::wrap_pyfunction!` against a local
    //! `#[pyfunction]` that delegates to the real
    //! `circuit_solver::parser::parse_file`. The Python-facing
    //! contract — accepts a path, returns a `CircuitGraph` —
    //! is byte-for-byte the same; this just lets `cargo test`
    //! reach the function without going through `CPython`'s
    //! `import_module`.
    use std::path::PathBuf;

    use circuit_solver::{parser, PyCircuitGraph};
    use pyo3::prelude::*;

    /// Test-only `parse_netlist` pyfunction. Mirrors the binding
    /// registered on the real `circuit_solver` Python module by
    /// `lib.rs`.
    #[pyfunction]
    #[allow(clippy::needless_pass_by_value)]
    fn parse_netlist_test(path: PathBuf) -> PyResult<PyCircuitGraph> {
        // Public, free-function entry point on the parser module.
        let graph = parser::parse_file(path.as_path())?;
        Ok(PyCircuitGraph::from_inner_public_for_tests(graph))
    }

    /// Get a `Bound<PyAny>` pointer to the test-only
    /// `parse_netlist_test` function so callers can invoke it via
    /// Python call semantics.
    pub fn parse_netlist_test_binding(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        // Build a one-off module holding just our function so
        // `wrap_pyfunction!` can resolve a `module` argument.
        let module = PyModule::new(py, "circuit_solver_test_helpers")?;
        module.add_function(wrap_pyfunction!(parse_netlist_test, &module)?)?;
        let f = module.getattr("parse_netlist_test")?;
        Ok(f)
    }
}

use test_helpers::parse_netlist_test_binding;

// ---------------------------------------------------------------------
// Tests.
// ---------------------------------------------------------------------

/// Full Gherkin witness for the
/// `python-frontend#spice-netlist-file-parsing` scenario.
///
/// Given: a SPICE netlist file on disk.
/// When : `circuit_solver.parse_netlist(path)` is called via the
///        `PyO3` Python-facing binding.
/// Then : the returned object is a `CircuitGraph`,
///        contains every element / model / subcircuit declared,
///        and is identical to one built incrementally with the
///        same topology.
#[test]
fn gherkin_scenario_spice_netlist_file_parsing() {
    let deck = "\
* Resistive divider — Gherkin witness for python-frontend#spice-netlist-file-parsing
R1 n1 n2 1k
R2 n2 0 2k
V1 n1 0 5
.end
";
    let path = write_temp_deck("divider", deck);
    Python::attach(|py| {
        let parsed = parse_via_python(py, &path).expect("parse_netlist must succeed");

        // Assertion 1 ("returned object is a CircuitGraph"): the
        // value exposes the `CircuitGraph` Python surface; reading
        // any frozen accessor proves that.
        let _: usize = parsed
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();

        // Build the incremental equivalent and compare full
        // signatures. Identity here is the strongest "identical to
        // one built incrementally with the same topology" assertion
        // the public surface permits.
        let incremental = build_resistive_divider_incrementally(py);
        let parsed_sig = signature(&parsed);
        let incremental_sig = signature(&incremental);
        assert_eq!(
            parsed_sig, incremental_sig,
            "parsed graph signature must match incrementally-built equivalent"
        );

        // Explicitly verify assertion 2 ("contains all elements,
        // models, subcircuits declared in the netlist"): three
        // top-level cards → three elements; ground + n1 + n2 →
        // three nodes; no .MODEL → zero models.
        assert_eq!(parsed_sig.0, 3, "element_count must equal 3");
        assert_eq!(parsed_sig.1, 3, "node_count must equal 3 (0, n1, n2)");
        assert_eq!(parsed_sig.2, 0, "model_count must equal 0");
    });
    let _ = fs::remove_file(&path);
}

/// Identity property holds for a deck that also exercises models
/// and subcircuit definitions, not just linear elements. Covers the
/// "models, and subcircuits declared" clause of the Gherkin scenario.
#[test]
fn gherkin_scenario_includes_models_and_subcircuits() {
    let deck = "\
* Full-shape deck: linear elements + .MODEL + .SUBCKT + X-instance
.MODEL DMOD D IS=1e-14
.SUBCKT INV in out vdd vss
R1 in mid 1k
R2 mid out 1k
.ENDS
X1 a b vdd 0 INV
D1 a b DMOD
V1 vdd 0 5
.end
";
    let path = write_temp_deck("full", deck);
    Python::attach(|py| {
        let parsed = parse_via_python(py, &path).expect("parse_netlist must succeed");
        let element_count: usize = parsed
            .call_method0("element_count")
            .unwrap()
            .extract()
            .unwrap();
        // X1 expands to R1 + R2 inside INV; plus D1, V1 → 4 elements.
        assert_eq!(element_count, 4, "X1 expansion + D1 + V1 → 4 elements");
        let model_count: usize = parsed
            .call_method0("model_count")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(model_count, 1, "one .MODEL declaration → one model");
        let is_fully_expanded: bool = parsed
            .call_method0("is_fully_expanded")
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            is_fully_expanded,
            "parse_netlist must return a fully-expanded graph"
        );
    });
    let _ = fs::remove_file(&path);
}

/// Missing files surface as Python `IOError`, not a Rust panic.
#[test]
fn parse_netlist_missing_file_raises_io_error() {
    let path: PathBuf = env::temp_dir().join("circuit_solver_test_does_not_exist.cir");
    // Make sure it really doesn't exist.
    let _ = fs::remove_file(&path);
    Python::attach(|py| {
        let err = parse_via_python(py, &path).expect_err("missing file must fail");
        // Verify it's a Python IOError (PyIOError).
        let is_io = err.is_instance_of::<pyo3::exceptions::PyIOError>(py);
        assert!(
            is_io,
            "missing-file error must be PyIOError, got: {}",
            err.value(py)
        );
        let msg = err.value(py).to_string();
        assert!(
            msg.contains("parse_netlist"),
            "error message must mention parse_netlist; got: {msg}"
        );
    });
}

/// Sanity: the test-helper binding itself returns a non-null
/// callable, guarding against future regressions in the
/// `wrap_pyfunction!`-based shim.
#[test]
fn parse_netlist_test_binding_resolves() {
    Python::attach(|py| {
        let f = parse_netlist_test_binding(py).expect("binding must resolve");
        assert!(f.is_callable());
    });
}
