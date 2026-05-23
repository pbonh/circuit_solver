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
fn fresh_result<'py>(
    py: Python<'py>,
    node_voltages: Option<Bound<'py, PyAny>>,
    branch_currents: Option<Bound<'py, PyAny>>,
    waveforms: Option<Bound<'py, PyAny>>,
    transfer_functions: Option<Bound<'py, PyAny>>,
) -> PyResult<Bound<'py, PyAnalysisResult>> {
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

        let returned_times: Vec<f64> = pair_tuple.get_item(0).unwrap().extract().unwrap();
        let returned_values: Vec<f64> = pair_tuple.get_item(1).unwrap().extract().unwrap();
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

        let f: Vec<f64> = triple_tuple.get_item(0).unwrap().extract().unwrap();
        let m: Vec<f64> = triple_tuple.get_item(1).unwrap().extract().unwrap();
        let p: Vec<f64> = triple_tuple.get_item(2).unwrap().extract().unwrap();
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
