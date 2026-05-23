//! Integration tests for the
//! `python-frontend#error-on-malformed-netlist` Gherkin scenario —
//! tasks.md item #61.
//!
//! Drives `circuit_solver.parse_netlist` through the `PyO3`
//! Python-facing surface exactly the way a Python caller would:
//!
//! ```python
//! import circuit_solver
//! try:
//!     circuit_solver.parse_netlist("/path/to/bad.cir")
//! except circuit_solver.NetlistParseError as exc:
//!     # exc message identifies the line number and the unrecognized token
//!     ...
//! ```
//!
//! The Gherkin scenario:
//!
//! ```text
//! Given CircuitDesigner has a SPICE netlist file with an unrecognized
//!       device letter
//! When  CircuitDesigner calls circuit_solver.parse_netlist(path)
//! Then  a Python exception of type "NetlistParseError" is raised
//! And   the exception message identifies the line number and the
//!       unrecognized token
//! ```
//!
//! All four lines of the scenario are asserted by
//! [`gherkin_scenario_error_on_malformed_netlist`]:
//!
//! - The *Given* is set up by writing a fixture deck containing the
//!   card `Z1 a b 1k` (`Z` is not a recognised SPICE device letter).
//! - The *When* is the call to the test-registered
//!   `parse_netlist_test` pyfunction, which is a byte-for-byte twin
//!   of the real `circuit_solver.parse_netlist` (the test binary
//!   cannot reach the private `#[pyfunction]` in `lib.rs` directly,
//!   so we re-register an identical free function under a private
//!   module the same way `spice_netlist_parsing.rs` does).
//! - The first *Then* is asserted by `err.is_instance_of::<NetlistParseError>(py)`.
//! - The second *Then* is asserted by substring-matching on the
//!   exception message: it must contain both the offending token
//!   (`Z1`) and the source line number (`line 2`, since SPICE
//!   convention treats line 1 as the deck title).
//!
//! ## Why the cfg-gate
//!
//! Same as the sibling `spice_netlist_parsing.rs` harness: the
//! `extension-module` feature is incompatible with linking the
//! Python ABI directly into a test binary. The whole module is
//! gated off when that feature is active so
//! `cargo test --workspace` (default features) still passes; the
//! crate test recipe is
//!
//!     cargo test -p circuit-solver-py --no-default-features

#![cfg(not(feature = "extension-module"))]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use circuit_solver::NetlistParseError;
use pyo3::prelude::*;

/// Helper: write `contents` to a per-test unique file in
/// `env::temp_dir()` and return the path. Mirrors the helper in
/// `spice_netlist_parsing.rs` so the two harnesses share a fixture
/// shape.
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

/// Call the parser through the test-only Python binding the way a
/// Python caller would. Returns the raw `PyAny` result so the
/// caller can either extract a graph or read the error.
fn parse_via_python<'py>(py: Python<'py>, path: &Path) -> PyResult<Bound<'py, PyAny>> {
    let parser_fn = test_helpers::parse_netlist_test_binding(py)?;
    let path_str = path.to_string_lossy().to_string();
    parser_fn.call1((path_str,))
}

mod test_helpers {
    //! Test-only re-registration of `parse_netlist` under a private
    //! module. The real `#[pyfunction]` in `lib.rs` is crate-private
    //! and cannot be reached from the test binary; we re-wrap the
    //! public `parser::parse_file` entry point so we can exercise
    //! the binding from `cargo test`. See
    //! `spice_netlist_parsing.rs::test_helpers` for the canonical
    //! pattern.
    use std::path::PathBuf;

    use circuit_solver::{parser, PyCircuitGraph};
    use pyo3::prelude::*;

    #[pyfunction]
    #[allow(clippy::needless_pass_by_value)]
    fn parse_netlist_test(path: PathBuf) -> PyResult<PyCircuitGraph> {
        let graph = parser::parse_file(path.as_path())?;
        Ok(PyCircuitGraph::from_inner_public_for_tests(graph))
    }

    pub fn parse_netlist_test_binding(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        let module = PyModule::new(py, "circuit_solver_test_error_helpers")?;
        module.add_function(wrap_pyfunction!(parse_netlist_test, &module)?)?;
        let f = module.getattr("parse_netlist_test")?;
        Ok(f)
    }
}

// ---------------------------------------------------------------------
// Tests.
// ---------------------------------------------------------------------

/// Full Gherkin witness for the
/// `python-frontend#error-on-malformed-netlist` scenario.
///
/// Given: a SPICE netlist file on disk containing an unrecognized
///        device letter (`Z`).
/// When : `circuit_solver.parse_netlist(path)` is called via the
///        `PyO3` Python-facing binding.
/// Then : a Python exception of type `NetlistParseError` is raised,
///        and the exception message identifies the line number
///        (`line 2`) and the unrecognized token (`Z1`).
#[test]
fn gherkin_scenario_error_on_malformed_netlist() {
    // Line 1: SPICE title (always skipped by the parser).
    // Line 2: the offending card. `Z` is not in {R, C, L, V, I, D, Q, M, X}.
    let deck = "\
* Malformed deck — Gherkin witness for python-frontend#error-on-malformed-netlist
Z1 a b 1k
V1 a 0 5
.end
";
    let path = write_temp_deck("malformed", deck);
    Python::attach(|py| {
        let err = parse_via_python(py, &path).expect_err("parse_netlist must fail");

        // First Then: the exception type is `NetlistParseError`.
        assert!(
            err.is_instance_of::<NetlistParseError>(py),
            "exception must be circuit_solver.NetlistParseError; got: {}",
            err.value(py)
        );

        // Second Then: the message identifies both the line number
        // and the unrecognized token. Substring-match rather than
        // exact-match so future cosmetic message tweaks don't break
        // the witness; the contract is content, not formatting.
        let msg = err.value(py).to_string();
        assert!(
            msg.contains("line 2"),
            "exception message must identify the line number ('line 2'); got: {msg}"
        );
        assert!(
            msg.contains("Z1"),
            "exception message must identify the unrecognized token ('Z1'); got: {msg}"
        );
    });
    let _ = fs::remove_file(&path);
}

/// Line-number tracking is honest even when the offending card is
/// not on line 2 (the easy default). This deck pushes the bad card
/// to line 5 via intervening comments and a valid element, and
/// asserts the exception message says `line 5`. Defends against a
/// regression where the line number was hard-coded or off-by-one.
#[test]
fn netlist_parse_error_reports_correct_line_number_when_card_is_deeper() {
    // Line 1: title.
    // Line 2: comment.
    // Line 3: blank.
    // Line 4: another comment.
    // Line 5: offending card.
    let deck = "\
* deeper-card title line
* a comment

* another comment
W1 c d 2k
";
    let path = write_temp_deck("deep_malformed", deck);
    Python::attach(|py| {
        let err = parse_via_python(py, &path).expect_err("parse_netlist must fail");
        assert!(
            err.is_instance_of::<NetlistParseError>(py),
            "must be NetlistParseError; got: {}",
            err.value(py)
        );
        let msg = err.value(py).to_string();
        assert!(
            msg.contains("line 5"),
            "exception message must identify the offending line ('line 5'); got: {msg}"
        );
        assert!(
            msg.contains("W1"),
            "exception message must identify the unrecognized token ('W1'); got: {msg}"
        );
    });
    let _ = fs::remove_file(&path);
}

/// The exception class is also exposed on the `circuit_solver`
/// Python module as a regular attribute — so Python callers can
/// write `except circuit_solver.NetlistParseError:`. This guards
/// the registration in `lib.rs::circuit_solver` against accidental
/// removal. The test asserts only on the Rust-side exception type
/// existing and being instantiable (the `lib.rs` registration is
/// what makes it importable from Python; the cargo-test binary
/// itself doesn't `import circuit_solver` since there is no
/// installed extension, so this is the strongest check available
/// inside the test harness).
#[test]
fn netlist_parse_error_type_is_constructible_from_rust() {
    Python::attach(|py| {
        let exc_type = py.get_type::<NetlistParseError>();
        let name: String = exc_type
            .getattr("__name__")
            .unwrap()
            .extract()
            .expect("__name__ extract");
        assert_eq!(
            name, "NetlistParseError",
            "exception class must be exposed under the documented Python name"
        );
    });
}
