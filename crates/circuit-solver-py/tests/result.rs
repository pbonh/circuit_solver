//! Integration tests for the `circuit_solver.Result` `PyO3` class —
//! tasks.md item #57 for `2026-05-21-v1-spec`.
//!
//! These tests embed `CPython` via `PyO3`'s `auto-initialize` dev
//! feature and exercise the `Result` class the way a user would from
//! Python: construct via `__new__` (which the downstream
//! `Simulator.run` entry point will eventually replace with a Rust
//! producer), read channels via by-name accessors, observe rejection
//! on malformed input.
//!
//! ## Why a separate test binary
//!
//! Integration-test binaries in a Cargo project are independent
//! compilation units, which keeps the per-file build cheap. The
//! existing `analysis_request.rs` test binary is large; splitting the
//! #57 surface into its own file mirrors the production module split
//! (`result.rs` ↔ `result.rs`) and keeps each file's reasoning local.
//!
//! ## Coverage map (Gherkin scenario steps lit up by these tests)
//!
//! Scenario `python-frontend#analysis-request-and-result-retrieval`:
//!
//! - *Then the `Simulator` returns a `Result` object* — exercised by
//!   [`construct_empty_result_is_well_formed`] (the well-formed value
//!   object exists and is reachable through the Python module).
//! - *And the `Result` contains node voltages accessible by node name*
//!   — exercised by [`node_voltage_lookup_by_name_returns_value`].
//! - *And the voltage at node "n1" is approximately 5 V within the
//!   tolerance envelope* — exercised by
//!   [`gherkin_step_voltage_at_n1_is_5v_within_tolerance`]. Tolerance
//!   here is the per-node `max(relative, absolute)` envelope per
//!   ADR-0008; for the resistive-divider witness the absolute floor
//!   (`1 mV`) is the dominant term.
//! - *And `CircuitDesigner` submits the `AnalysisRequest` to the
//!   `Simulator`* — out of scope for #57 (the submission entry point
//!   is a downstream task; #57 ships the value object only).
//!
//! Additional defence-in-depth tests cover by-name access for the
//! other three channels (branch currents, waveforms, transfer
//! functions), construction-time validation, and the immutability
//! posture (frozen pyclass cannot be mutated post-construction).
//!
//! ## Why the cfg-gate
//!
//! Same reasoning as `analysis_request.rs`: the test binary embeds
//! `libpython` via the dev-dependency `auto-initialize` feature, which
//! conflicts with the production `extension-module` feature. The
//! recipe `cargo test -p circuit-solver-py --no-default-features` is
//! what the workspace CI runs and what the integrator's P6 preflight
//! re-runs.

#![cfg(not(feature = "extension-module"))]

use circuit_solver::PyAnalysisResult;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

/// Probe-and-inject `numpy` site-packages so the embedded test
/// interpreter can `import numpy` even when the dev `auto-initialize`
/// `CPython` does not ship it.
///
/// ## Why this exists
///
/// Tasks.md #58 introduces the `numpy` Rust crate, which calls into
/// `import numpy` at first use to fetch the `NumPy` C-API capsule.
/// Under `cargo test -p circuit-solver-py --no-default-features`, `PyO3`
/// auto-initializes the host Python (`PYO3_PYTHON` or system `python3`).
/// That interpreter typically lacks `numpy`, which causes the
/// "Failed to access `NumPy` array API capsule … `ModuleNotFoundError`"
/// panic seen during initial bring-up.
///
/// To keep workspace preflights (`cargo test -p circuit-solver-py
/// --no-default-features` and `cargo test --workspace`) green without
/// pinning a global `PYTHONPATH` on the integrator's shell, this
/// helper:
///
/// 1. Tries `import numpy` directly. If it succeeds, returns (the
///    common case once an integrator has `pip install numpy`d on
///    their system Python, or once `PYTHONPATH` is exported).
/// 2. Otherwise, walks parent directories from `CARGO_MANIFEST_DIR`
///    and for each directory looks at every `.venv*` subdirectory
///    (e.g. `.venv`, `.venv-test`, `.venv-3.14`). For each candidate
///    it scans `<venv>/lib/pythonX.Y/site-packages` for a
///    matching-ABI `numpy/` and tries inserting it at the head of
///    `sys.path`. Only a numpy whose `pythonX.Y` matches the embedded
///    interpreter's `(major, minor)` will satisfy `import numpy`; the
///    helper iterates until one does or it exhausts candidates.
/// 3. If still no numpy, panics with a directive message pointing the
///    user at `uv venv .venv && uv pip install --python .venv/bin/python
///    numpy` from the workspace root, run against the same Python
///    binary that `cargo test` links against (see
///    `crates/circuit-solver-py/build.rs`).
///
/// Idempotent and re-entrant: safe to call from every test on every
/// thread. The fast path is a single `import numpy` check, which is
/// a cheap `sys.modules` dict lookup once numpy has been imported
/// once; subsequent calls do not re-walk the filesystem.
///
/// ## What this does *not* do
///
/// - It does not install numpy. It only locates an existing install.
/// - It does not affect the production wheel: the test gate
///   `#[cfg(not(feature = "extension-module"))]` means this code is
///   compiled only for `cargo test --no-default-features`.
fn setup_numpy_path(py: Python<'_>) {
    // Fast path: numpy already importable (cached or previously set up).
    // Subsequent calls from concurrent test threads sail through this
    // check without serializing on a global lock.
    if py.import("numpy").is_ok() {
        return;
    }
    let mut cur = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for _ in 0..6 {
        if try_inject_numpy_from_venvs(py, &cur).is_ok() {
            return;
        }
        if !cur.pop() {
            break;
        }
    }
    panic!(
        "numpy could not be imported by the embedded test interpreter, and no `.venv*` \
         directory with a matching-ABI numpy was found by walking up from {}. Either:\n\
         (a) install numpy on the active Python (`PYO3_PYTHON` or system `python3`), or\n\
         (b) create a workspace venv with numpy against the same Python the test binary \
         links against (see `build.rs` and `ldd target/debug/deps/result-*`):\n\
             `uv venv .venv-test --python /path/to/matching/python && \\\n\
              uv pip install --python .venv-test/bin/python numpy`\n\
         then re-run `cargo test -p circuit-solver-py --no-default-features`.",
        env!("CARGO_MANIFEST_DIR"),
    );
}

/// Try every `.venv*` subdirectory of `dir`; for each one whose
/// `lib/pythonX.Y/site-packages/numpy` exists, prepend that
/// `site-packages` to `sys.path` and attempt `import numpy`. Returns
/// `Ok(())` on the first venv whose ABI matches.
fn try_inject_numpy_from_venvs(py: Python<'_>, dir: &std::path::Path) -> Result<(), ()> {
    let entries = std::fs::read_dir(dir).map_err(|_| ())?;
    for entry in entries.flatten() {
        let p = entry.path();
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !p.is_dir() || !name.starts_with(".venv") {
            continue;
        }
        if let Some(site) = find_site_packages_with_numpy(&p) {
            let sys = py.import("sys").expect("import sys");
            let path: Bound<'_, PyAny> = sys.getattr("path").expect("sys.path");
            // Insert at head; if this venv has the wrong ABI, the
            // import will fail and we'll undo by popping at index 0.
            path.call_method1("insert", (0, site.to_string_lossy().into_owned()))
                .expect("insert into sys.path");
            if py.import("numpy").is_ok() {
                return Ok(());
            }
            // Mismatched ABI — pop our entry off and try the next.
            path.call_method1("pop", (0,)).expect("pop from sys.path");
        }
    }
    Err(())
}

/// Return the `site-packages` path inside `<venv>/lib/pythonX.Y/`
/// that contains a `numpy/` directory, if any. Used by
/// [`setup_numpy_path`] to locate a pre-installed numpy without
/// hard-coding a Python version.
fn find_site_packages_with_numpy(venv: &std::path::Path) -> Option<std::path::PathBuf> {
    let lib = venv.join("lib");
    let entries = std::fs::read_dir(&lib).ok()?;
    for entry in entries.flatten() {
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }
        let site = p.join("site-packages");
        if site.join("numpy").is_dir() {
            return Some(site);
        }
    }
    None
}

/// Tolerance envelope from ADR-0008 for the
/// `python-frontend#analysis-request-and-result-retrieval` resistive
/// divider witness. ADR-0008 specifies a per-node `max(relative,
/// absolute)` envelope; this constant is the absolute term (1 mV) —
/// for a 5 V node the absolute floor dominates the 1 % relative term
/// only marginally (50 mV relative vs 1 mV absolute → relative wins),
/// but in this synthetic witness the value is exact so any floor will
/// hold. We pick the looser of the two to avoid bit-exact-equality
/// fragility if downstream tasks (#62 conformance) inject ULP-level
/// drift.
const ADR_0008_ABSOLUTE_TOLERANCE_VOLTS: f64 = 1e-3;

/// Build a `dict[str, float]` for a `node_voltages` or
/// `branch_currents` arg. The verbose ceremony exists because `PyO3`
/// 0.28's `PyDict::new`/`set_item` requires explicit `IntoPyObject`
/// conversion for heterogeneous (str/float) entries.
fn make_scalar_map<'py>(py: Python<'py>, entries: &[(&str, f64)]) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (key, value) in entries {
        dict.set_item(*key, *value)?;
    }
    Ok(dict)
}

/// Build a `dict[str, (list[float], list[float])]` for a `waveforms`
/// arg.
fn make_waveform_map<'py>(
    py: Python<'py>,
    entries: &[(&str, &[f64], &[f64])],
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (key, times, values) in entries {
        let t = PyList::new(py, times.iter().copied())?;
        let v = PyList::new(py, values.iter().copied())?;
        let tuple = PyTuple::new(py, [t.into_any().unbind(), v.into_any().unbind()])?;
        dict.set_item(*key, tuple)?;
    }
    Ok(dict)
}

/// Build a `dict[str, (list[float], list[float], list[float])]` for a
/// `transfer_functions` arg.
/// Each entry: (name, frequencies, magnitudes, phases). Hoisted out
/// of the function signature so `clippy::type_complexity` stays clean.
type TfEntry<'a> = (&'a str, &'a [f64], &'a [f64], &'a [f64]);

fn make_tf_map<'py>(py: Python<'py>, entries: &[TfEntry<'_>]) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (key, freqs, mag, phase) in entries {
        let f = PyList::new(py, freqs.iter().copied())?;
        let m = PyList::new(py, mag.iter().copied())?;
        let p = PyList::new(py, phase.iter().copied())?;
        let tuple = PyTuple::new(
            py,
            [
                f.into_any().unbind(),
                m.into_any().unbind(),
                p.into_any().unbind(),
            ],
        )?;
        dict.set_item(*key, tuple)?;
    }
    Ok(dict)
}

/// Construct a fresh Python-side `Result` via the class object (so the
/// dispatch path matches `import circuit_solver`).
///
/// Calls [`setup_numpy_path`] internally because the `Result`
/// constructor now (tasks.md #58) converts waveform / transfer-function
/// `Vec<f64>` payloads to `Py<PyArray1<f64>>` via the `numpy` crate,
/// which `import numpy`'s on first use to fetch the C-API capsule.
/// All tests reach the constructor via this helper, so a single setup
/// call here covers every test path.
fn fresh_result<'py>(
    py: Python<'py>,
    node_voltages: Option<Bound<'py, PyAny>>,
    branch_currents: Option<Bound<'py, PyAny>>,
    waveforms: Option<Bound<'py, PyAny>>,
    transfer_functions: Option<Bound<'py, PyAny>>,
) -> PyResult<Bound<'py, PyAnalysisResult>> {
    setup_numpy_path(py);
    let kwargs = PyDict::new(py);
    if let Some(v) = node_voltages {
        kwargs.set_item("node_voltages", v)?;
    }
    if let Some(v) = branch_currents {
        kwargs.set_item("branch_currents", v)?;
    }
    if let Some(v) = waveforms {
        kwargs.set_item("waveforms", v)?;
    }
    if let Some(v) = transfer_functions {
        kwargs.set_item("transfer_functions", v)?;
    }
    let cls = py.get_type::<PyAnalysisResult>();
    let obj = cls.call((), Some(&kwargs))?;
    obj.cast_into::<PyAnalysisResult>().map_err(PyErr::from)
}

// -- happy-path construction ------------------------------------------------

#[test]
fn construct_empty_result_is_well_formed() {
    Python::attach(|py| {
        let result = fresh_result(py, None, None, None, None)
            .expect("constructing an empty Result must not fail");

        let is_empty: bool = result.call_method0("is_empty").unwrap().extract().unwrap();
        assert!(
            is_empty,
            "a Result constructed with no channels must report empty"
        );

        let node_names: Vec<String> = result
            .call_method0("node_names")
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            node_names.is_empty(),
            "empty Result must list zero node names"
        );
    });
}

#[test]
fn node_voltage_lookup_by_name_returns_value() {
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("n1", 5.0), ("n2", 2.5)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, Some(voltages), None, None, None)
            .expect("constructing a DC Result must not fail");

        let v_n1: f64 = result
            .call_method1("node_voltage", ("n1",))
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            (v_n1 - 5.0).abs() < ADR_0008_ABSOLUTE_TOLERANCE_VOLTS,
            "node n1 voltage must be 5 V within the ADR-0008 envelope"
        );

        let v_n2: f64 = result
            .call_method1("node_voltage", ("n2",))
            .unwrap()
            .extract()
            .unwrap();
        assert!((v_n2 - 2.5).abs() < ADR_0008_ABSOLUTE_TOLERANCE_VOLTS);
    });
}

/// Gherkin scenario `python-frontend#analysis-request-and-result-retrieval`,
/// the final two `Then` lines:
///
/// > And the Result contains node voltages accessible by node name
/// > And the voltage at node "n1" is approximately 5 V within the
/// > tolerance envelope
///
/// This test constructs a Result that simulates the output of the
/// resistive-divider scenario, then witnesses the by-name accessor
/// returning the expected value within ADR-0008's envelope.
/// Construction of the Rust-side producer (`Simulator.run`) is the
/// downstream task; this test is the witness that the consumer side
/// (the by-name access pattern) is implementable.
#[test]
fn gherkin_step_voltage_at_n1_is_5v_within_tolerance() {
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("n1", 5.0), ("n2", 2.5), ("0", 0.0)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, Some(voltages), None, None, None).unwrap();

        let v_n1: f64 = result
            .call_method1("node_voltage", ("n1",))
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            (v_n1 - 5.0).abs() < ADR_0008_ABSOLUTE_TOLERANCE_VOLTS,
            "Gherkin 'voltage at node \"n1\" approximately 5 V within \
             the tolerance envelope' must hold"
        );

        let names: Vec<String> = result
            .call_method0("node_names")
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            names.contains(&"n1".to_string()),
            "Gherkin 'Result contains node voltages accessible by node \
             name' requires n1 to be listed"
        );
    });
}

#[test]
fn node_voltage_lookup_for_unknown_node_raises_key_error() {
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("n1", 5.0)]).unwrap().into_any();
        let result = fresh_result(py, Some(voltages), None, None, None).unwrap();

        let err = result
            .call_method1("node_voltage", ("doesnotexist",))
            .expect_err("unknown node name must raise");
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyKeyError>(py),
            "missing node lookup must surface as KeyError, got {err:?}"
        );
        let msg = err.to_string();
        assert!(
            msg.contains("doesnotexist"),
            "KeyError must mention the missing name; got {msg}"
        );
    });
}

// -- branch currents --------------------------------------------------------

#[test]
fn branch_current_lookup_by_name_returns_value() {
    Python::attach(|py| {
        let currents = make_scalar_map(py, &[("V1", -1.5e-3), ("I1", 2.0e-3)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, None, Some(currents), None, None).unwrap();

        let i_v1: f64 = result
            .call_method1("branch_current", ("V1",))
            .unwrap()
            .extract()
            .unwrap();
        assert!((i_v1 - (-1.5e-3)).abs() < 1e-9);

        let names: Vec<String> = result
            .call_method0("branch_names")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(names, vec!["I1".to_string(), "V1".to_string()]);
    });
}

#[test]
fn branch_current_lookup_for_unknown_element_raises_key_error() {
    Python::attach(|py| {
        let currents = make_scalar_map(py, &[("V1", -1.5e-3)]).unwrap().into_any();
        let result = fresh_result(py, None, Some(currents), None, None).unwrap();

        let err = result
            .call_method1("branch_current", ("R99",))
            .expect_err("unknown branch element must raise");
        assert!(err.is_instance_of::<pyo3::exceptions::PyKeyError>(py));
    });
}

/// Assert that `obj` is a `numpy.ndarray` of `dtype=float64`. Used by
/// the tasks.md #58 zero-copy tests to verify the array-returning
/// accessors return ndarrays, not Python lists.
fn expect_float64_ndarray(obj: &Bound<'_, PyAny>, field: &str) {
    let py = obj.py();
    let numpy = py.import("numpy").expect("numpy must be importable");
    let ndarray = numpy.getattr("ndarray").unwrap();
    assert!(
        obj.is_instance(&ndarray).unwrap(),
        "{field}: expected numpy.ndarray, got {}",
        obj.get_type().name().unwrap()
    );
    let dtype = obj.getattr("dtype").unwrap();
    let kind: String = dtype.getattr("kind").unwrap().extract().unwrap();
    let itemsize: usize = dtype.getattr("itemsize").unwrap().extract().unwrap();
    assert_eq!(
        (kind.as_str(), itemsize),
        ("f", 8),
        "{field}: expected dtype float64 (kind='f', itemsize=8), got kind={kind:?} itemsize={itemsize}"
    );
}

// -- waveforms --------------------------------------------------------------

#[test]
fn waveform_lookup_returns_parallel_time_and_value_lists() {
    Python::attach(|py| {
        let times = [0.0, 1e-9, 2e-9, 3e-9];
        let values = [0.0, 1.65, 3.3, 1.65];
        let waveforms = make_waveform_map(py, &[("n1", &times, &values)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, None, None, Some(waveforms), None).unwrap();

        let pair = result.call_method1("waveform", ("n1",)).unwrap();
        let pair_tuple = pair.cast::<PyTuple>().unwrap();
        assert_eq!(pair_tuple.len(), 2);

        // tasks.md #58: the inner elements must be `numpy.ndarray` of
        // `dtype=float64`, not `list[float]` as in the pre-#58 surface.
        // See `expect_float64_ndarray` for the assertion vocabulary.
        let times_arr = pair_tuple.get_item(0).unwrap();
        let values_arr = pair_tuple.get_item(1).unwrap();
        expect_float64_ndarray(&times_arr, "waveform.times");
        expect_float64_ndarray(&values_arr, "waveform.values");

        let returned_times: Vec<f64> = times_arr.extract().unwrap();
        let returned_values: Vec<f64> = values_arr.extract().unwrap();
        assert_eq!(returned_times, times.to_vec());
        assert_eq!(returned_values, values.to_vec());
        assert_eq!(
            returned_times.len(),
            returned_values.len(),
            "waveform parallel arrays must share a length"
        );
    });
}

#[test]
fn waveform_with_mismatched_array_lengths_is_rejected() {
    Python::attach(|py| {
        let times = [0.0, 1e-9, 2e-9];
        let values = [0.0, 1.65];
        let waveforms = make_waveform_map(py, &[("n1", &times, &values)])
            .unwrap()
            .into_any();
        let err = fresh_result(py, None, None, Some(waveforms), None)
            .expect_err("mismatched waveform arrays must raise");
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyValueError>(py),
            "mismatched lengths must surface as ValueError, got {err:?}"
        );
    });
}

#[test]
fn waveform_lookup_for_unknown_node_raises_key_error() {
    Python::attach(|py| {
        let waveforms = make_waveform_map(py, &[("n1", &[0.0, 1.0], &[2.0, 3.0])])
            .unwrap()
            .into_any();
        let result = fresh_result(py, None, None, Some(waveforms), None).unwrap();
        let err = result
            .call_method1("waveform", ("n2",))
            .expect_err("unknown waveform node must raise");
        assert!(err.is_instance_of::<pyo3::exceptions::PyKeyError>(py));
    });
}

// -- transfer functions -----------------------------------------------------

#[test]
fn transfer_function_lookup_returns_parallel_freq_mag_phase_lists() {
    Python::attach(|py| {
        let freqs = [1.0, 10.0, 100.0, 1000.0];
        let mag = [0.0, -3.0, -20.0, -40.0];
        let phase = [0.0, -45.0, -90.0, -90.0];
        let tfs = make_tf_map(py, &[("vout", &freqs, &mag, &phase)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, None, None, None, Some(tfs)).unwrap();

        let triple = result.call_method1("transfer_function", ("vout",)).unwrap();
        let triple_tuple = triple.cast::<PyTuple>().unwrap();
        assert_eq!(triple_tuple.len(), 3);

        // tasks.md #58: each inner element must be a `numpy.ndarray`
        // of `dtype=float64`.
        let f_arr = triple_tuple.get_item(0).unwrap();
        let m_arr = triple_tuple.get_item(1).unwrap();
        let p_arr = triple_tuple.get_item(2).unwrap();
        expect_float64_ndarray(&f_arr, "transfer_function.frequencies_hz");
        expect_float64_ndarray(&m_arr, "transfer_function.magnitude_db");
        expect_float64_ndarray(&p_arr, "transfer_function.phase_degrees");

        let f: Vec<f64> = f_arr.extract().unwrap();
        let m: Vec<f64> = m_arr.extract().unwrap();
        let p: Vec<f64> = p_arr.extract().unwrap();
        assert_eq!(f, freqs.to_vec());
        assert_eq!(m, mag.to_vec());
        assert_eq!(p, phase.to_vec());
    });
}

#[test]
fn transfer_function_with_mismatched_lengths_is_rejected() {
    Python::attach(|py| {
        let freqs = [1.0, 10.0, 100.0];
        let mag = [0.0, -3.0];
        let phase = [0.0, -45.0, -90.0];
        let tfs = make_tf_map(py, &[("vout", &freqs, &mag, &phase)])
            .unwrap()
            .into_any();
        let err = fresh_result(py, None, None, None, Some(tfs))
            .expect_err("mismatched transfer-function arrays must raise");
        assert!(err.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    });
}

// -- validation --------------------------------------------------------------

#[test]
fn non_finite_voltage_is_rejected() {
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("n1", f64::NAN)]).unwrap().into_any();
        let err = fresh_result(py, Some(voltages), None, None, None)
            .expect_err("NaN voltage must be rejected at construction");
        assert!(err.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    });
}

#[test]
fn infinite_voltage_is_rejected() {
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("n1", f64::INFINITY)])
            .unwrap()
            .into_any();
        let err = fresh_result(py, Some(voltages), None, None, None)
            .expect_err("infinite voltage must be rejected");
        assert!(err.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    });
}

#[test]
fn non_finite_waveform_sample_is_rejected() {
    Python::attach(|py| {
        let waveforms = make_waveform_map(py, &[("n1", &[0.0, 1e-9], &[0.0, f64::NAN])])
            .unwrap()
            .into_any();
        let err = fresh_result(py, None, None, Some(waveforms), None)
            .expect_err("NaN waveform sample must be rejected");
        assert!(err.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    });
}

#[test]
fn non_mapping_node_voltages_is_rejected() {
    Python::attach(|py| {
        // Pass a list instead of a dict — PyMapping downcast must fail.
        let bad = PyList::new(py, [1.0_f64, 2.0, 3.0]).unwrap().into_any();
        let err = fresh_result(py, Some(bad), None, None, None)
            .expect_err("non-mapping node_voltages must be rejected");
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyTypeError>(py),
            "non-mapping must surface as TypeError, got {err:?}"
        );
    });
}

#[test]
fn empty_name_is_rejected() {
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("", 5.0)]).unwrap().into_any();
        let err = fresh_result(py, Some(voltages), None, None, None)
            .expect_err("empty name must be rejected");
        assert!(err.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    });
}

// -- immutability posture ----------------------------------------------------

/// `#[pyclass(frozen)]` rules out reassigning attributes from Python.
/// We don't ship setter methods, so the only way to mutate would be
/// attribute assignment; `CPython` rejects that on frozen pyclasses with
/// `AttributeError`. This test pins that behaviour so a future
/// refactor that accidentally drops `frozen` is caught immediately.
#[test]
fn result_is_frozen_and_rejects_attribute_assignment() {
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("n1", 5.0)]).unwrap().into_any();
        let result = fresh_result(py, Some(voltages), None, None, None).unwrap();

        let err = result
            .setattr("node_voltages", py.None())
            .expect_err("frozen pyclass must reject attribute assignment from Python");
        // CPython's frozen pyclass surfaces this as AttributeError.
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyAttributeError>(py),
            "frozen pyclass write must raise AttributeError, got {err:?}"
        );
    });
}

#[test]
fn repr_renders_channel_sizes() {
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("n1", 5.0), ("n2", 2.5)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, Some(voltages), None, None, None).unwrap();
        let repr = result.repr().unwrap().extract::<String>().unwrap();
        assert!(
            repr.contains("nodes=2"),
            "repr must include nodes count; got {repr}"
        );
        assert!(repr.contains("branches=0"));
        assert!(repr.contains("waveforms=0"));
        assert!(repr.contains("transfer_functions=0"));
    });
}

// -- name listings stability -------------------------------------------------

/// The four listing accessors must return names in a deterministic
/// (sorted) order so callers can compare snapshots across runs without
/// flake. The `BTreeMap` backing store gives us this for free; this
/// test pins it as a contract.
#[test]
fn node_names_are_returned_in_sorted_order() {
    Python::attach(|py| {
        // Insert in scrambled order; expect sorted on read.
        let voltages = make_scalar_map(py, &[("z", 1.0), ("a", 2.0), ("m", 3.0)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, Some(voltages), None, None, None).unwrap();
        let names: Vec<String> = result
            .call_method0("node_names")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            names,
            vec!["a".to_string(), "m".to_string(), "z".to_string()],
            "node_names must be returned in lexicographic order for run-to-run stability"
        );
    });
}

#[test]
fn all_four_channels_can_coexist_in_one_result() {
    // Defence in depth: the four channels are independent — a Result
    // can carry DC voltages, DC currents, a transient waveform, and
    // an AC transfer function simultaneously without crosstalk.
    // Downstream `Simulator.run` shapes (e.g., a mixed-signal run)
    // will exercise this.
    Python::attach(|py| {
        let voltages = make_scalar_map(py, &[("n1", 5.0)]).unwrap().into_any();
        let currents = make_scalar_map(py, &[("V1", 1e-3)]).unwrap().into_any();
        let waveforms = make_waveform_map(py, &[("n1", &[0.0, 1e-9], &[5.0, 5.0])])
            .unwrap()
            .into_any();
        let tfs = make_tf_map(py, &[("n1", &[1.0, 10.0], &[0.0, -3.0], &[0.0, -45.0])])
            .unwrap()
            .into_any();

        let result = fresh_result(
            py,
            Some(voltages),
            Some(currents),
            Some(waveforms),
            Some(tfs),
        )
        .unwrap();

        let v: f64 = result
            .call_method1("node_voltage", ("n1",))
            .unwrap()
            .extract()
            .unwrap();
        assert!((v - 5.0).abs() < 1e-12);

        let i: f64 = result
            .call_method1("branch_current", ("V1",))
            .unwrap()
            .extract()
            .unwrap();
        assert!((i - 1e-3).abs() < 1e-15);

        let wf = result.call_method1("waveform", ("n1",)).unwrap();
        assert_eq!(wf.cast::<PyTuple>().unwrap().len(), 2);

        let tf = result.call_method1("transfer_function", ("n1",)).unwrap();
        assert_eq!(tf.cast::<PyTuple>().unwrap().len(), 3);

        let is_empty: bool = result.call_method0("is_empty").unwrap().extract().unwrap();
        assert!(!is_empty, "fully-populated Result must not report empty");
    });
}

// -- tasks.md #58 / scenario python-frontend#zero-copy-numpy-result-arrays
// ----------------------------------------------------------------------------
//
// Gherkin (verbatim from
// `openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/python-frontend/spec.md`):
//
//     Scenario: Zero-copy NumPy result arrays
//     Given CircuitDesigner has obtained a Result from a transient Analysis
//     When PythonDeveloper accesses the Waveform array for node "n1"
//     Then the returned object is a NumPy ndarray of dtype float64
//     And the array's underlying buffer is a view into Rust-owned memory
//         (no copy is performed)
//     And the array length equals the number of time points in the Result
//
// The three tests below each light up one of the three "Then" lines.
// They use the same resistive-divider witness shape as the
// `analysis-request-and-result-retrieval` scenario, extended with a
// transient waveform on node "n1" (the scenario's "obtained a Result
// from a transient Analysis" precondition).

/// Witness for the Gherkin step
/// *"the returned object is a `NumPy` ndarray of dtype float64"*.
///
/// Constructs a transient-shaped `Result` (node "n1" carrying both a
/// DC voltage and a time-domain waveform) and asserts that
/// `result.waveform("n1")` returns a 2-tuple whose elements are
/// `numpy.ndarray` instances with `dtype=float64`. See
/// [`expect_float64_ndarray`] for the assertion vocabulary.
#[test]
fn gherkin_step_waveform_array_is_float64_ndarray() {
    Python::attach(|py| {
        // Mirror the resistive-divider witness from the sibling scenario.
        let voltages = make_scalar_map(py, &[("n1", 5.0)]).unwrap().into_any();
        let times = [0.0, 1e-9, 2e-9, 3e-9, 4e-9];
        let values = [5.0, 5.0, 5.0, 5.0, 5.0]; // DC operating point, sampled.
        let waveforms = make_waveform_map(py, &[("n1", &times, &values)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, Some(voltages), None, Some(waveforms), None).unwrap();

        let pair = result.call_method1("waveform", ("n1",)).unwrap();
        let pair_tuple = pair.cast::<PyTuple>().unwrap();
        let times_arr = pair_tuple.get_item(0).unwrap();
        let values_arr = pair_tuple.get_item(1).unwrap();

        expect_float64_ndarray(&times_arr, "waveform.times");
        expect_float64_ndarray(&values_arr, "waveform.values");
    });
}

/// Witness for the Gherkin step
/// *"the array's underlying buffer is a view into Rust-owned memory
/// (no copy is performed)"*.
///
/// **What "no copy" means concretely.** The waveform/transfer-function
/// channels are stored inside the `Result` as `Py<PyArray1<f64>>` —
/// refcounted handles to a `NumPy` ndarray that owns Rust-allocated
/// heap memory (transferred at `__new__` time via
/// [`numpy::PyArray1::from_vec`]). The accessor returns
/// `times.clone_ref(py)` and `values.clone_ref(py)` — a refcount
/// increment, not a buffer duplication.
///
/// The observable consequence: two successive accesses of the same
/// node name return ndarrays that share their underlying buffer, so
/// `numpy.shares_memory(a, b)` is `True`, and `a.ctypes.data` ==
/// `b.ctypes.data` (the buffer pointers are identical). A copying
/// accessor would yield disjoint pointers each call.
#[test]
fn gherkin_step_waveform_buffer_is_shared_across_accesses() {
    Python::attach(|py| {
        let times = [0.0, 1e-9, 2e-9, 3e-9];
        let values = [1.65, 3.3, 1.65, 0.0];
        let waveforms = make_waveform_map(py, &[("n1", &times, &values)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, None, None, Some(waveforms), None).unwrap();

        let a = result.call_method1("waveform", ("n1",)).unwrap();
        let b = result.call_method1("waveform", ("n1",)).unwrap();
        let a_tuple = a.cast::<PyTuple>().unwrap();
        let b_tuple = b.cast::<PyTuple>().unwrap();
        let a_values = a_tuple.get_item(1).unwrap();
        let b_values = b_tuple.get_item(1).unwrap();

        // 1. NumPy's own "do these two arrays alias?" predicate.
        let numpy = py.import("numpy").unwrap();
        let shares_memory = numpy
            .getattr("shares_memory")
            .unwrap()
            .call1((&a_values, &b_values))
            .unwrap()
            .extract::<bool>()
            .unwrap();
        assert!(
            shares_memory,
            "two successive accessor calls must return ndarray views over the same buffer \
             (zero-copy); numpy.shares_memory returned False"
        );

        // 2. Belt-and-suspenders: the raw `ctypes.data` pointers must
        //    be identical. If `from_vec` copied on each accessor, the
        //    allocations would be at different addresses.
        let a_addr: usize = a_values
            .getattr("ctypes")
            .unwrap()
            .getattr("data")
            .unwrap()
            .extract()
            .unwrap();
        let b_addr: usize = b_values
            .getattr("ctypes")
            .unwrap()
            .getattr("data")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            a_addr, b_addr,
            "underlying buffer addresses must match across accessor calls; \
             got a=0x{a_addr:x}, b=0x{b_addr:x}"
        );

        // 3. Round-trip the contents to confirm we're not just sharing
        //    a sentinel — the buffer truly carries the input values.
        let returned_values: Vec<f64> = a_values.extract().unwrap();
        assert_eq!(returned_values, values.to_vec());
    });
}

/// Witness for the Gherkin step
/// *"the array length equals the number of time points in the
/// Result"*.
///
/// The waveform's `len()` (Python-side) and the constructed `times`
/// slice length must agree. The matching `values` array shares that
/// length per the construction invariant enforced in
/// [`crate::result::parse_waveform_map`].
#[test]
fn gherkin_step_waveform_array_length_equals_time_point_count() {
    Python::attach(|py| {
        // Pick a non-trivial point count so the assertion is meaningful
        // even if the test is mechanically inspected (e.g. 4 is a
        // common off-by-one trap).
        let times = [0.0, 0.25e-9, 0.5e-9, 0.75e-9, 1.0e-9, 1.25e-9, 1.5e-9];
        let values = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_len = times.len();
        let waveforms = make_waveform_map(py, &[("n1", &times, &values)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, None, None, Some(waveforms), None).unwrap();

        let pair = result.call_method1("waveform", ("n1",)).unwrap();
        let pair_tuple = pair.cast::<PyTuple>().unwrap();
        let times_arr = pair_tuple.get_item(0).unwrap();
        let values_arr = pair_tuple.get_item(1).unwrap();

        let times_len: usize = times_arr.len().unwrap();
        let values_len: usize = values_arr.len().unwrap();
        assert_eq!(
            times_len, expected_len,
            "waveform.times length must equal the number of time points in the Result"
        );
        assert_eq!(
            values_len, expected_len,
            "waveform.values length must equal the number of time points in the Result"
        );
    });
}

/// Defence in depth: the transfer-function accessor returns three
/// `numpy.ndarray` views over Rust-owned memory.
///
/// Scenario `python-frontend#zero-copy-numpy-result-arrays` calls out
/// waveforms explicitly, but the acceptance criterion *"Result arrays
/// (node voltages, branch currents, Waveforms, `TransferFunctions`) are
/// `NumPy`-compatible views into Rust-owned memory with zero-copy
/// semantics"* covers all four channels. This test asserts the
/// transfer-function side of that promise the same way
/// [`gherkin_step_waveform_buffer_is_shared_across_accesses`] does for
/// the waveform side.
#[test]
fn transfer_function_arrays_are_zero_copy_numpy_ndarrays() {
    Python::attach(|py| {
        let freqs = [1.0, 10.0, 100.0, 1000.0, 10000.0];
        let mag = [0.0, -3.0, -20.0, -40.0, -60.0];
        let phase = [0.0, -45.0, -90.0, -90.0, -90.0];
        let tfs = make_tf_map(py, &[("vout", &freqs, &mag, &phase)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, None, None, None, Some(tfs)).unwrap();

        let a = result.call_method1("transfer_function", ("vout",)).unwrap();
        let b = result.call_method1("transfer_function", ("vout",)).unwrap();
        let a_tuple = a.cast::<PyTuple>().unwrap();
        let b_tuple = b.cast::<PyTuple>().unwrap();

        let numpy = py.import("numpy").unwrap();
        for (axis, idx) in [
            ("frequencies_hz", 0),
            ("magnitude_db", 1),
            ("phase_degrees", 2),
        ] {
            let aa = a_tuple.get_item(idx).unwrap();
            let bb = b_tuple.get_item(idx).unwrap();
            expect_float64_ndarray(&aa, &format!("transfer_function.{axis}"));
            let shares: bool = numpy
                .getattr("shares_memory")
                .unwrap()
                .call1((&aa, &bb))
                .unwrap()
                .extract()
                .unwrap();
            assert!(
                shares,
                "transfer_function.{axis}: ndarray buffer must be shared across accessor calls"
            );
        }

        // Length agrees with the input axis length on all three arrays.
        let expected_len = freqs.len();
        for (axis, idx) in [
            ("frequencies_hz", 0),
            ("magnitude_db", 1),
            ("phase_degrees", 2),
        ] {
            let arr_len: usize = a_tuple.get_item(idx).unwrap().len().unwrap();
            assert_eq!(
                arr_len, expected_len,
                "transfer_function.{axis} length must equal the frequency-sweep size"
            );
        }
    });
}

// -- tasks.md #26 / scenario frontend-contract#results-zero-copy-numpy
// ----------------------------------------------------------------------------
//
// Gherkin (from
// `proposals/2026-05-28-multidomain-solver-architecture/specs/frontend-contract/spec.md`):
//
//   Scenario: results-zero-copy-numpy
//     Given a DC analysis Result containing node voltages and branch currents
//      When the CircuitDesigner accesses the node voltages as a NumPy array
//       And accesses the branch currents as a NumPy array
//      Then the returned arrays are numpy.ndarray of dtype float64
//       And successive calls return handles to the same underlying buffer
//       And array[i] corresponds to the value for node_names()[i] / branch_names()[i]
// ----------------------------------------------------------------------------

/// Assert that `node_voltages_array()` returns a `numpy.ndarray` of
/// `dtype=float64` whose values are in sorted node-name order and
/// correspond to the by-name scalar channel.
///
/// This test covers the node-voltage arm of scenario
/// `frontend-contract#results-zero-copy-numpy` (task #26).
#[test]
fn node_voltages_array_is_zero_copy_numpy_ndarray() {
    Python::attach(|py| {
        let nv = make_scalar_map(py, &[("n0", 0.0), ("n1", 5.0), ("n2", 3.3)])
            .unwrap()
            .into_any();
        let bc = make_scalar_map(py, &[("V1", 1e-3)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, Some(nv), Some(bc), None, None).unwrap();

        // 1. The accessor returns a numpy.ndarray of dtype float64.
        let arr1 = result.call_method0("node_voltages_array").unwrap();
        expect_float64_ndarray(&arr1, "node_voltages_array");

        // 2. Successive calls return handles to the same buffer
        //    (zero-copy: refcount bump, not a fresh allocation).
        let arr2 = result.call_method0("node_voltages_array").unwrap();
        let numpy = py.import("numpy").unwrap();
        let shares_memory: bool = numpy
            .getattr("shares_memory")
            .unwrap()
            .call1((&arr1, &arr2))
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            shares_memory,
            "two successive node_voltages_array() calls must return ndarray views \
             over the same buffer (zero-copy); numpy.shares_memory returned False"
        );

        // Belt-and-suspenders: raw data pointers must be identical.
        // Use a Python one-liner to extract the data pointer from
        // __array_interface__["data"][0] to avoid pyo3 type-chain issues.
        let get_data_ptr = |py: Python<'_>, arr: &Bound<'_, PyAny>| -> isize {
            let code = c"lambda a: a.__array_interface__['data'][0]";
            let getter = py.eval(code, None, None).unwrap();
            getter.call1((arr,)).unwrap().extract().unwrap()
        };
        let ptr1 = get_data_ptr(py, &arr1);
        let ptr2 = get_data_ptr(py, &arr2);
        assert_eq!(
            ptr1, ptr2,
            "node_voltages_array: data pointers must match across calls"
        );

        // 3. array[i] corresponds to node_names()[i] (sorted order).
        let node_names: Vec<String> = result
            .call_method0("node_names")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(node_names, vec!["n0", "n1", "n2"], "node_names must be sorted");

        let values: Vec<f64> = arr1.extract().unwrap();
        assert_eq!(values.len(), 3, "node_voltages_array must have 3 entries");
        assert_eq!(values[0], 0.0, "node_voltages_array[0] == voltage at n0");
        assert!(
            (values[1] - 5.0).abs() < ADR_0008_ABSOLUTE_TOLERANCE_VOLTS,
            "node_voltages_array[1] == voltage at n1 (≈5 V)"
        );
        assert!(
            (values[2] - 3.3).abs() < ADR_0008_ABSOLUTE_TOLERANCE_VOLTS,
            "node_voltages_array[2] == voltage at n2 (≈3.3 V)"
        );

        // Cross-check: array[i] == node_voltage(node_names()[i]).
        for (i, name) in node_names.iter().enumerate() {
            let by_name: f64 = result
                .call_method1("node_voltage", (name,))
                .unwrap()
                .extract()
                .unwrap();
            let diff = (values[i] - by_name).abs();
            assert!(
                diff < 1e-15,
                "node_voltages_array[{}] must equal node_voltage({:?}): \
                 array={}, by_name={}, diff={}",
                i, name, values[i], by_name, diff
            );
        }
    });
}

/// Assert that `branch_currents_array()` returns a `numpy.ndarray` of
/// `dtype=float64` whose values are in sorted branch-name order and
/// correspond to the by-name scalar channel.
///
/// This test covers the branch-current arm of scenario
/// `frontend-contract#results-zero-copy-numpy` (task #26).
#[test]
fn branch_currents_array_is_zero_copy_numpy_ndarray() {
    Python::attach(|py| {
        let nv = make_scalar_map(py, &[("n1", 5.0)])
            .unwrap()
            .into_any();
        let bc = make_scalar_map(py, &[("I1", 0.0), ("V1", 1e-3), ("V2", -2e-3)])
            .unwrap()
            .into_any();
        let result = fresh_result(py, Some(nv), Some(bc), None, None).unwrap();

        // 1. dtype=float64 numpy.ndarray.
        let arr1 = result.call_method0("branch_currents_array").unwrap();
        expect_float64_ndarray(&arr1, "branch_currents_array");

        // 2. Zero-copy: successive calls share the same buffer.
        let arr2 = result.call_method0("branch_currents_array").unwrap();
        let numpy = py.import("numpy").unwrap();
        let shares_memory: bool = numpy
            .getattr("shares_memory")
            .unwrap()
            .call1((&arr1, &arr2))
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            shares_memory,
            "two successive branch_currents_array() calls must return ndarray views \
             over the same buffer (zero-copy); numpy.shares_memory returned False"
        );

        // 3. array[i] corresponds to branch_names()[i] (sorted order).
        let branch_names: Vec<String> = result
            .call_method0("branch_names")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            branch_names, vec!["I1", "V1", "V2"],
            "branch_names must be sorted"
        );

        let values: Vec<f64> = arr1.extract().unwrap();
        assert_eq!(values.len(), 3, "branch_currents_array must have 3 entries");
        assert_eq!(values[0], 0.0, "branch_currents_array[0] == current through I1");
        assert!(
            (values[1] - 1e-3).abs() < 1e-15,
            "branch_currents_array[1] == current through V1 (≈1 mA)"
        );
        assert!(
            (values[2] - (-2e-3)).abs() < 1e-15,
            "branch_currents_array[2] == current through V2 (≈−2 mA)"
        );

        // Cross-check: array[i] == branch_current(branch_names()[i]).
        for (i, name) in branch_names.iter().enumerate() {
            let by_name: f64 = result
                .call_method1("branch_current", (name,))
                .unwrap()
                .extract()
                .unwrap();
            let diff = (values[i] - by_name).abs();
            assert!(
                diff < 1e-15,
                "branch_currents_array[{}] must equal branch_current({:?}): \
                 array={}, by_name={}, diff={}",
                i, name, values[i], by_name, diff
            );
        }
    });
}

/// Empty result: both scalar array accessors return 0-length
/// `numpy.ndarray(dtype=float64)`.
///
/// Edge case for `frontend-contract#results-zero-copy-numpy`: the
/// zero-copy promise still holds (the handles share a buffer, just a
/// zero-length one).
#[test]
fn empty_result_scalar_arrays_are_zero_length_float64_ndarrays() {
    Python::attach(|py| {
        let result = fresh_result(py, None, None, None, None).unwrap();

        let nv_arr = result.call_method0("node_voltages_array").unwrap();
        expect_float64_ndarray(&nv_arr, "empty node_voltages_array");
        assert_eq!(nv_arr.len().unwrap(), 0, "empty node_voltages_array must have length 0");

        let bc_arr = result.call_method0("branch_currents_array").unwrap();
        expect_float64_ndarray(&bc_arr, "empty branch_currents_array");
        assert_eq!(bc_arr.len().unwrap(), 0, "empty branch_currents_array must have length 0");
    });
}
