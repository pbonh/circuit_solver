//! Integration tests for the `circuit_solver.AnalysisRequest` `PyO3`
//! class — tasks.md item #56 for `2026-05-21-v1-spec`.
//!
//! These tests embed `CPython` via `PyO3`'s `auto-initialize` dev
//! feature and exercise the `AnalysisRequest` class the way a user
//! would from Python: construct via `__new__`, read fields via
//! attribute access, observe rejection on malformed input.
//!
//! ## Why a separate test binary
//!
//! Integration-test binaries in a Cargo project are independent
//! compilation units, which keeps the per-file build cheap. The
//! existing `circuit_builder.rs` test binary is large; splitting the
//! #56 surface into its own file mirrors the production module split
//! (`builder.rs` ↔ `circuit_builder.rs`, `analysis_request.rs` ↔
//! `analysis_request.rs`) and keeps each file's reasoning local.
//!
//! ## Coverage map (Gherkin scenario steps lit up by these tests)
//!
//! Scenario `python-frontend#analysis-request-and-result-retrieval`:
//!
//! - *When `CircuitDesigner` creates an `AnalysisRequest` for DC operating
//!   point* — exercised by
//!   [`construct_dc_op_request_records_default_fields`] and
//!   [`gherkin_step_create_dc_op_analysis_request`].
//! - *And `CircuitDesigner` submits the `AnalysisRequest` to the
//!   `Simulator`* — out of scope for #56 (Simulator submission is a
//!   downstream task; #56 ships the value object only).
//! - *Then the `Simulator` returns a `Result` object* — out of scope for
//!   #56 (`Result` is tasks.md #57).
//!
//! The defence-in-depth tests below also cover the four-field surface
//! described in the task body: `analysis_type`, sweep parameters,
//! integration method, and boundary interpolation per ADR-0007.
//!
//! ## Why the cfg-gate
//!
//! Same reasoning as `circuit_builder.rs`: the test binary embeds
//! `libpython` via the dev-dependency `auto-initialize` feature, which
//! conflicts with the production `extension-module` feature. The
//! recipe `cargo test -p circuit-solver-py --no-default-features` is
//! what the workspace CI runs and what the integrator's P6 preflight
//! re-runs.

#![cfg(not(feature = "extension-module"))]

use circuit_solver::PyAnalysisRequest;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

/// Build a heterogeneous `(start, stop, points, scale)` Python tuple
/// for use as the `sweep` argument. `PyO3` 0.28's `PyTuple::new` takes
/// an `IntoIterator` of *homogeneous* items, so heterogeneous tuples
/// must be assembled by first converting each value to a `PyObject`.
fn make_sweep<'py>(
    py: Python<'py>,
    start: f64,
    stop: f64,
    points: i64,
    scale: &str,
) -> PyResult<Bound<'py, PyTuple>> {
    let items = [
        start.into_pyobject(py)?.into_any().unbind(),
        stop.into_pyobject(py)?.into_any().unbind(),
        points.into_pyobject(py)?.into_any().unbind(),
        scale.into_pyobject(py)?.into_any().unbind(),
    ];
    PyTuple::new(py, items)
}

/// Build a 3-element tuple — used to exercise the "wrong arity"
/// rejection path.
fn make_short_sweep(
    py: Python<'_>,
    start: f64,
    stop: f64,
    points: i64,
) -> PyResult<Bound<'_, PyTuple>> {
    let items = [
        start.into_pyobject(py)?.into_any().unbind(),
        stop.into_pyobject(py)?.into_any().unbind(),
        points.into_pyobject(py)?.into_any().unbind(),
    ];
    PyTuple::new(py, items)
}

/// Construct a fresh Python-side `AnalysisRequest` via the class
/// object (so the dispatch path matches `import circuit_solver`).
fn fresh_request<'py>(
    py: Python<'py>,
    analysis_type: &str,
    sweep: Option<Bound<'py, PyAny>>,
    integration_method: Option<&str>,
    boundary_interpolation: Option<&str>,
) -> PyResult<Bound<'py, PyAnalysisRequest>> {
    let kwargs = PyDict::new(py);
    if let Some(s) = sweep {
        kwargs.set_item("sweep", s)?;
    }
    if let Some(m) = integration_method {
        kwargs.set_item("integration_method", m)?;
    }
    if let Some(b) = boundary_interpolation {
        kwargs.set_item("boundary_interpolation", b)?;
    }
    let cls = py.get_type::<PyAnalysisRequest>();
    let obj = cls.call((analysis_type,), Some(&kwargs))?;
    obj.cast_into::<PyAnalysisRequest>().map_err(PyErr::from)
}

// -- happy-path construction ------------------------------------------------

#[test]
fn construct_dc_op_request_records_default_fields() {
    Python::attach(|py| {
        let req = fresh_request(py, "dc-operating-point", None, None, None)
            .expect("constructing a DC-OP AnalysisRequest must not fail");

        let analysis_type: String = req.getattr("analysis_type").unwrap().extract().unwrap();
        assert_eq!(analysis_type, "dc-operating-point");

        let sweep = req.getattr("sweep").unwrap();
        assert!(
            sweep.is_none(),
            "DC operating point is non-sweeping; sweep field must be None"
        );

        let integration: String = req
            .getattr("integration_method")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            integration, "trapezoidal",
            "design.md §Trapezoidal ringing: TR is the v1 default"
        );

        let boundary: String = req
            .getattr("boundary_interpolation")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            boundary, "zero_order_hold",
            "ADR-0007 mandates zero-order hold as the charge-conserving default"
        );
    });
}

/// Gherkin scenario `python-frontend#analysis-request-and-result-retrieval`,
/// *When `CircuitDesigner` creates an `AnalysisRequest` for DC operating
/// point*. The rest of the scenario (submit + `Result`) is out of scope
/// for tasks.md #56 — this test is the witness that the construction
/// step is implementable.
#[test]
fn gherkin_step_create_dc_op_analysis_request() {
    Python::attach(|py| {
        let req = fresh_request(py, "dc-operating-point", None, None, None)
            .expect("Gherkin 'creates an AnalysisRequest for DC operating point' must succeed");

        let analysis_type: String = req.getattr("analysis_type").unwrap().extract().unwrap();
        assert_eq!(analysis_type, "dc-operating-point");
    });
}

#[test]
fn short_alias_dc_op_canonicalises_to_full_slug() {
    Python::attach(|py| {
        for alias in ["dc_op", "dc"] {
            let req = fresh_request(py, alias, None, None, None)
                .unwrap_or_else(|e| panic!("short alias {alias:?} must be accepted: {e}"));
            let canonical: String = req.getattr("analysis_type").unwrap().extract().unwrap();
            assert_eq!(
                canonical, "dc-operating-point",
                "short alias must canonicalise to the AnalysisType::slug() form"
            );
        }
    });
}

#[test]
fn short_alias_transient_canonicalises_to_full_slug() {
    Python::attach(|py| {
        for alias in ["transient", "tran"] {
            let req = fresh_request(py, alias, None, None, None).unwrap();
            let canonical: String = req.getattr("analysis_type").unwrap().extract().unwrap();
            assert_eq!(canonical, "transient-time-domain");
        }
    });
}

#[test]
fn short_alias_ac_canonicalises_to_full_slug_with_sweep() {
    Python::attach(|py| {
        let sweep = make_sweep(py, 1.0, 1.0e6, 50, "log").unwrap();
        let req = fresh_request(py, "ac", Some(sweep.into_any()), None, None).unwrap();
        let canonical: String = req.getattr("analysis_type").unwrap().extract().unwrap();
        assert_eq!(canonical, "ac-small-signal");
    });
}

#[test]
fn all_six_canonical_slugs_round_trip() {
    Python::attach(|py| {
        // Non-sweeping kinds.
        for slug in [
            "dc-operating-point",
            "transient-time-domain",
            "mixed-signal-cosim",
        ] {
            let req = fresh_request(py, slug, None, None, None).unwrap();
            let canonical: String = req.getattr("analysis_type").unwrap().extract().unwrap();
            assert_eq!(canonical, slug);
        }
        // Sweeping kinds: dc-sweep, ac, noise.
        let sweep_lin = make_sweep(py, 0.0, 5.0, 11, "linear").unwrap();
        let req_dc_sweep =
            fresh_request(py, "dc-sweep", Some(sweep_lin.into_any()), None, None).unwrap();
        let canonical: String = req_dc_sweep
            .getattr("analysis_type")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(canonical, "dc-sweep");

        for slug in ["ac-small-signal", "noise-spectral-density"] {
            let sweep_log = make_sweep(py, 1.0, 1.0e9, 100, "log").unwrap();
            let req = fresh_request(py, slug, Some(sweep_log.into_any()), None, None).unwrap();
            let canonical: String = req.getattr("analysis_type").unwrap().extract().unwrap();
            assert_eq!(canonical, slug);
        }
    });
}

// -- sweep parameter shape -------------------------------------------------

#[test]
fn sweep_tuple_round_trips_through_getter() {
    Python::attach(|py| {
        let sweep_in = make_sweep(py, 1.0, 1.0e6, 50, "log").unwrap();
        let req =
            fresh_request(py, "ac-small-signal", Some(sweep_in.into_any()), None, None).unwrap();
        let sweep_out = req.getattr("sweep").unwrap();
        assert!(!sweep_out.is_none());
        let (start, stop, points, scale): (f64, f64, usize, String) = sweep_out.extract().unwrap();
        assert!((start - 1.0).abs() < 1e-12);
        assert!((stop - 1.0e6).abs() < 1.0);
        assert_eq!(points, 50);
        assert_eq!(scale, "log");
    });
}

#[test]
fn sweep_scale_linear_alias_canonicalises_to_linear() {
    Python::attach(|py| {
        let sweep = make_sweep(py, 0.0, 5.0, 11, "lin").unwrap();
        let req = fresh_request(py, "dc-sweep", Some(sweep.into_any()), None, None).unwrap();
        let sweep_out = req.getattr("sweep").unwrap();
        let (_start, _stop, _points, scale): (f64, f64, usize, String) =
            sweep_out.extract().unwrap();
        assert_eq!(scale, "linear");
    });
}

#[test]
fn sweep_scale_logarithmic_alias_canonicalises_to_log() {
    Python::attach(|py| {
        let sweep = make_sweep(py, 1.0, 1.0e3, 30, "logarithmic").unwrap();
        let req = fresh_request(py, "ac-small-signal", Some(sweep.into_any()), None, None).unwrap();
        let sweep_out = req.getattr("sweep").unwrap();
        let (_start, _stop, _points, scale): (f64, f64, usize, String) =
            sweep_out.extract().unwrap();
        assert_eq!(scale, "log");
    });
}

// -- integration method ----------------------------------------------------

#[test]
fn integration_method_accepts_three_canonical_tags() {
    Python::attach(|py| {
        for tag in ["backward_euler", "trapezoidal", "gear2"] {
            let req = fresh_request(py, "transient", None, Some(tag), None).unwrap();
            let out: String = req
                .getattr("integration_method")
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(
                out, tag,
                "design.md §Trapezoidal ringing lists BE / TR / Gear-2 as the three offered methods"
            );
        }
    });
}

#[test]
fn integration_method_short_aliases_canonicalise() {
    Python::attach(|py| {
        for (alias, canonical) in [
            ("be", "backward_euler"),
            ("tr", "trapezoidal"),
            ("gear-2", "gear2"),
            ("bdf2", "gear2"),
        ] {
            let req = fresh_request(py, "transient", None, Some(alias), None).unwrap();
            let out: String = req
                .getattr("integration_method")
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(out, canonical, "alias {alias:?} must canonicalise");
        }
    });
}

// -- boundary interpolation (ADR-0007) -------------------------------------

#[test]
fn boundary_interpolation_accepts_zoh_and_linear() {
    Python::attach(|py| {
        for tag in ["zero_order_hold", "linear"] {
            let req = fresh_request(py, "mixed-signal-cosim", None, None, Some(tag)).unwrap();
            let out: String = req
                .getattr("boundary_interpolation")
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(
                out, tag,
                "ADR-0007 specifies 'zero_order_hold' (default) and 'linear' as the only two valid tags"
            );
        }
    });
}

#[test]
fn boundary_interpolation_short_alias_zoh_canonicalises() {
    Python::attach(|py| {
        let req = fresh_request(py, "mixed-signal-cosim", None, None, Some("zoh")).unwrap();
        let out: String = req
            .getattr("boundary_interpolation")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(out, "zero_order_hold");
    });
}

// -- rejection paths -------------------------------------------------------

#[test]
fn unknown_analysis_type_raises_type_error() {
    Python::attach(|py| {
        let err = fresh_request(py, "bogus-analysis", None, None, None).unwrap_err();
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyTypeError>(py),
            "unknown analysis_type must raise TypeError, got {err}"
        );
        let msg = err.value(py).to_string();
        assert!(
            msg.contains("bogus-analysis"),
            "error message must echo the offending tag: {msg}"
        );
    });
}

#[test]
fn unknown_integration_method_raises_type_error() {
    Python::attach(|py| {
        let err = fresh_request(py, "transient", None, Some("rk4"), None).unwrap_err();
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyTypeError>(py),
            "unknown integration_method must raise TypeError"
        );
        let msg = err.value(py).to_string();
        assert!(msg.contains("rk4"));
    });
}

#[test]
fn unknown_boundary_interpolation_raises_type_error() {
    Python::attach(|py| {
        let err = fresh_request(py, "mixed-signal-cosim", None, None, Some("cubic")).unwrap_err();
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyTypeError>(py),
            "unknown boundary_interpolation must raise TypeError"
        );
        let msg = err.value(py).to_string();
        assert!(msg.contains("ADR-0007") || msg.contains("zero_order_hold"));
    });
}

#[test]
fn sweep_missing_on_sweeping_analysis_raises_value_error() {
    Python::attach(|py| {
        for slug in ["dc-sweep", "ac-small-signal", "noise-spectral-density"] {
            let err = fresh_request(py, slug, None, None, None).unwrap_err();
            assert!(
                err.is_instance_of::<pyo3::exceptions::PyValueError>(py),
                "sweeping analysis {slug:?} without sweep must raise ValueError"
            );
        }
    });
}

#[test]
fn sweep_present_on_non_sweeping_analysis_raises_value_error() {
    Python::attach(|py| {
        let sweep = make_sweep(py, 0.0, 1.0, 5, "linear").unwrap();
        let err = fresh_request(py, "dc-operating-point", Some(sweep.into_any()), None, None)
            .unwrap_err();
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyValueError>(py),
            "non-sweeping DC operating point with sweep must raise ValueError"
        );
    });
}

#[test]
fn sweep_with_zero_points_raises_value_error() {
    Python::attach(|py| {
        let sweep = make_sweep(py, 0.0, 1.0, 0, "linear").unwrap();
        let err = fresh_request(py, "dc-sweep", Some(sweep.into_any()), None, None).unwrap_err();
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyValueError>(py),
            "sweep.points = 0 must raise ValueError"
        );
    });
}

#[test]
fn sweep_with_non_finite_endpoint_raises_value_error() {
    Python::attach(|py| {
        let sweep_nan = make_sweep(py, f64::NAN, 1.0, 5, "linear").unwrap();
        let err =
            fresh_request(py, "dc-sweep", Some(sweep_nan.into_any()), None, None).unwrap_err();
        assert!(err.is_instance_of::<pyo3::exceptions::PyValueError>(py));

        let sweep_inf = make_sweep(py, 0.0, f64::INFINITY, 5, "linear").unwrap();
        let err =
            fresh_request(py, "dc-sweep", Some(sweep_inf.into_any()), None, None).unwrap_err();
        assert!(err.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    });
}

#[test]
fn sweep_with_unknown_scale_raises_value_error() {
    Python::attach(|py| {
        let sweep = make_sweep(py, 0.0, 1.0, 5, "quadratic").unwrap();
        let err = fresh_request(py, "dc-sweep", Some(sweep.into_any()), None, None).unwrap_err();
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyValueError>(py),
            "unknown sweep.scale must raise ValueError"
        );
    });
}

#[test]
fn sweep_with_wrong_arity_raises_type_error() {
    Python::attach(|py| {
        let sweep = make_short_sweep(py, 0.0, 1.0, 5).unwrap();
        let err = fresh_request(py, "dc-sweep", Some(sweep.into_any()), None, None).unwrap_err();
        assert!(
            err.is_instance_of::<pyo3::exceptions::PyTypeError>(py),
            "wrong-arity sweep tuple must raise TypeError"
        );
    });
}

// -- immutability ----------------------------------------------------------

/// The class is marked `#[pyclass(frozen)]`, so Python-side attribute
/// assignment must be rejected. This is the structural enforcement of
/// "value object" semantics — downstream code reading an
/// `AnalysisRequest` can rely on its fields not mutating mid-flight.
#[test]
fn analysis_request_is_frozen_against_python_mutation() {
    Python::attach(|py| {
        let req = fresh_request(py, "dc-operating-point", None, None, None).unwrap();
        let any = req.into_any();
        let mutate = any.setattr("analysis_type", "transient-time-domain");
        assert!(
            mutate.is_err(),
            "frozen pyclass must reject Python-side attribute assignment"
        );
    });
}

// -- list-vs-tuple acceptance for sweep ------------------------------------

/// Accepting either a Python `tuple` or `list` is a small ergonomic
/// concession: NumPy-using code often builds sweep specs as lists.
/// The internal parsing path uses `try_iter` so both shapes flow
/// through.
#[test]
fn sweep_accepts_python_list_form() {
    Python::attach(|py| {
        let items = [
            0.0_f64.into_pyobject(py).unwrap().into_any().unbind(),
            5.0_f64.into_pyobject(py).unwrap().into_any().unbind(),
            11_i64.into_pyobject(py).unwrap().into_any().unbind(),
            "linear".into_pyobject(py).unwrap().into_any().unbind(),
        ];
        let list = PyList::new(py, items).unwrap();
        let req = fresh_request(py, "dc-sweep", Some(list.into_any()), None, None)
            .expect("sweep as a python list of length 4 must be accepted");
        let sweep_out = req.getattr("sweep").unwrap();
        let (_start, _stop, points, scale): (f64, f64, usize, String) =
            sweep_out.extract().unwrap();
        assert_eq!(points, 11);
        assert_eq!(scale, "linear");
    });
}

// -- repr smoke test -------------------------------------------------------

#[test]
fn repr_includes_all_four_fields() {
    Python::attach(|py| {
        let req = fresh_request(py, "dc-operating-point", None, None, None).unwrap();
        let repr: String = req.call_method0("__repr__").unwrap().extract().unwrap();
        assert!(repr.contains("dc-operating-point"));
        assert!(repr.contains("sweep="));
        assert!(repr.contains("trapezoidal"));
        assert!(repr.contains("zero_order_hold"));
    });
}
