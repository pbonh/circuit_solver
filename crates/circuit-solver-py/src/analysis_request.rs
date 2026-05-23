//! `PyO3` `AnalysisRequest` class — Python-facing analysis-submission
//! value object.
//!
//! This module implements **tasks.md item #56** for the
//! `2026-05-21-v1-spec` change. It introduces the `AnalysisRequest`
//! Python class with four user-facing fields:
//!
//! 1. **`analysis_type`** — the analysis discriminator (DC operating
//!    point, DC sweep, AC, transient, noise, mixed-signal).
//! 2. **`sweep`** — optional sweep parameters (`start`, `stop`,
//!    `points`, `scale`) carried as a `(f64, f64, usize, str)` tuple.
//!    Required for sweep-shaped analyses (`dc_sweep`, `ac`, `noise`);
//!    `None` for single-point analyses (`dc_op`, `transient`,
//!    `mixed_signal`).
//! 3. **`integration_method`** — choice of time-domain integration
//!    method (`backward_euler`, `trapezoidal`, `gear2`). Defaults to
//!    `trapezoidal` per design.md §"Trapezoidal ringing" (the v1
//!    default with documented LTE auto-shrink mitigation).
//! 4. **`boundary_interpolation`** — analog–digital boundary
//!    interpolation choice per ADR-0007: `zero_order_hold` (default,
//!    charge-conserving) or `linear` (smoother but non-conserving).
//!
//! # Scope (what this task does and does *not* deliver)
//!
//! Task #56 delivers the **value object** only. The Gherkin scenario
//! `python-frontend#analysis-request-and-result-retrieval` requires
//! *constructing* an `AnalysisRequest`, *submitting* it to a
//! `Simulator`, and inspecting a `Result`. Construction is fully
//! implemented here; the `Result` Python class is **tasks.md #57**
//! (depends on #56) and the actual submission entry point (`Simulator`
//! method that takes an `AnalysisRequest` + `CircuitGraph` and returns
//! a `Result`) is a downstream task — see `tasks.md` capability
//! `python-frontend` items #57–#61.
//!
//! Until the submission entry point lands, an `AnalysisRequest` is a
//! pure data carrier — well-formed, validated at construction, and
//! ready to be consumed by the analysis-orchestration layer once the
//! Python-facing `Simulator.run(...)` method is wired up.
//!
//! # Surface decisions (recorded for ADR-0010 callers)
//!
//! - **String-tag enums.** `analysis_type`, `integration_method`, and
//!   `boundary_interpolation` are all carried as short Python strings
//!   rather than dedicated `#[pyclass]` enum wrappers. This mirrors
//!   the `ElementKind` SPICE-letter-tag convention from
//!   [`crate::builder`] (see "Kind encoded as a SPICE-letter string"
//!   in its module docs) and keeps the v1 surface narrow per ADR-0010.
//!   The accepted analysis-type slugs match
//!   [`circuit_solver_types::AnalysisType::slug`] verbatim
//!   (`"dc-operating-point"`, `"dc-sweep"`, `"ac-small-signal"`,
//!   `"transient-time-domain"`, `"noise-spectral-density"`,
//!   `"mixed-signal-cosim"`); friendlier short-form aliases (`"dc_op"`,
//!   `"ac"`, `"transient"`, etc.) are also accepted so Python users
//!   can write idiomatic code.
//! - **`Sweep` as a tuple, not a class.** The sweep parameters are a
//!   `(start, stop, points, scale)` tuple of `(f64, f64, usize, str)`.
//!   `scale` is `"linear"` or `"log"`. Exposing a dedicated `Sweep`
//!   Python class would prematurely lock in a surface that ADR-0010
//!   keeps unstable; downstream tasks (#28's `Sweep` Rust type still
//!   in flight at #56-impl time, plus #29/#30/#31 frequency-sweep
//!   detail tasks) will refine the Rust-side representation. The
//!   tuple at the Python boundary stays stable while the inner Rust
//!   types churn.
//! - **`#[pyclass(frozen)]`.** Like [`crate::graph::PyCircuitGraph`],
//!   the request is immutable once constructed — all four fields are
//!   set in `__new__` and then read-only. This rules out the entire
//!   "mutated mid-flight" failure class downstream tasks must not
//!   guard against.
//! - **Construction validation.** Unknown enum strings, non-finite
//!   sweep endpoints, zero `points`, and unrecognised `scale` are all
//!   rejected at `__new__` with `TypeError` / `ValueError` —
//!   submission-time error paths are not in scope for #56.
//!
//! # ADR alignment
//!
//! - **ADR-0007 (Zero-Order Hold Default)** — `boundary_interpolation`
//!   accepts exactly the two values that ADR-0007's Consequences
//!   block specifies (`"zero_order_hold"` | `"linear"`), with ZOH as
//!   the default. The ADR's follow-up explicitly mandates this on
//!   the `AnalysisRequest` Python API.
//! - **ADR-0010 (Unstable Public Rust API for v1)** — no Rust-side
//!   `AnalysisRequest` type is added in `application-frontend` here;
//!   the Python class owns its fields directly. When the public
//!   Rust `AnalysisRequest` lands (a follow-up task), this Python
//!   class will gain a `to_inner()` adapter; until then the surface
//!   is pure Python-PyO3.

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use circuit_solver_types::AnalysisType;

/// Default integration method for transient analyses.
///
/// Per `design.md` §"Trapezoidal ringing": *"Three integration methods
/// offered (BE, TR, Gear-2); default TR with documented ringing risk;
/// LTE auto-shrink damps artifact."* Trapezoidal is the v1 default.
const DEFAULT_INTEGRATION_METHOD: &str = "trapezoidal";

/// Default analog–digital boundary interpolation per ADR-0007:
/// zero-order hold is the charge-conserving default; linear is
/// available as a per-request opt-in.
const DEFAULT_BOUNDARY_INTERPOLATION: &str = "zero_order_hold";

/// Python class: `circuit_solver.AnalysisRequest`.
///
/// Immutable value object describing a requested analysis. See the
/// module-level documentation for the field semantics and the
/// task-#56-specific scope (construction only; submission and
/// `Result` are downstream tasks).
#[pyclass(name = "AnalysisRequest", module = "circuit_solver", frozen)]
pub struct PyAnalysisRequest {
    /// Canonical analysis-type slug (matches
    /// [`AnalysisType::slug`]). Stored as a `String` so the
    /// Python-facing `analysis_type` getter can hand back exactly what
    /// the user supplied (after canonicalisation from any short-form
    /// aliases).
    analysis_type_slug: String,
    /// Strongly-typed analysis discriminator (parsed once at
    /// construction). Kept alongside the slug so downstream Rust code
    /// can dispatch without re-parsing.
    ///
    /// Currently consumed only by the crate-private
    /// [`PyAnalysisRequest::kind`] accessor; the `Simulator.run`
    /// entry point that consumes this field is tasks.md #57+. The
    /// `dead_code` allow is scoped to this field rather than the
    /// whole struct so future additions don't accidentally bypass
    /// the dead-code check.
    #[allow(dead_code)]
    analysis_type_kind: AnalysisType,
    /// Sweep parameters: `(start, stop, points, scale)`. `None` for
    /// non-sweeping analyses.
    sweep: Option<SweepParams>,
    /// Integration-method tag (validated). Always populated; defaults
    /// to `"trapezoidal"`.
    integration_method: String,
    /// Analog–digital boundary interpolation tag (validated). Always
    /// populated; defaults to `"zero_order_hold"` per ADR-0007.
    boundary_interpolation: String,
}

/// Strongly-typed view of the `(start, stop, points, scale)` sweep
/// tuple, with `scale` parsed to a closed enum at construction time.
#[derive(Debug, Clone, Copy, PartialEq)]
struct SweepParams {
    /// Sweep start value. Units depend on `AnalysisType`: volts for
    /// `DcSweep`, Hertz for `AcSmallSignal` and `Noise`. Must be
    /// finite.
    start: f64,
    /// Sweep stop value. Must be finite. `stop > start` is *not*
    /// enforced — reversed sweeps are a legitimate use case the
    /// downstream sweep-generator may consume.
    stop: f64,
    /// Number of sweep points, ≥ 1.
    points: usize,
    /// Linear or logarithmic spacing.
    scale: SweepScale,
}

/// Closed enum for the `scale` tag in the sweep tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SweepScale {
    Linear,
    Logarithmic,
}

impl SweepScale {
    /// Canonical lowercase tag returned to Python.
    const fn tag(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Logarithmic => "log",
        }
    }
}

#[pymethods]
impl PyAnalysisRequest {
    /// Construct a new `AnalysisRequest`.
    ///
    /// # Arguments
    ///
    /// - `analysis_type` — analysis-type slug. Canonical values match
    ///   [`circuit_solver_types::AnalysisType::slug`]:
    ///   `"dc-operating-point"`, `"dc-sweep"`, `"ac-small-signal"`,
    ///   `"transient-time-domain"`, `"noise-spectral-density"`,
    ///   `"mixed-signal-cosim"`. Friendlier short-form aliases are
    ///   also accepted: `"dc_op"` / `"dc"`, `"dc_sweep"`, `"ac"`,
    ///   `"transient"` / `"tran"`, `"noise"`, `"mixed_signal"` /
    ///   `"mixed"`.
    /// - `sweep` — optional `(start, stop, points, scale)` tuple of
    ///   `(float, float, int, str)`. `scale` is `"linear"` or `"log"`.
    ///   Required when `analysis_type` is a sweeping kind
    ///   (`dc-sweep`, `ac-small-signal`, `noise-spectral-density`);
    ///   must be `None` for the single-point kinds. Construction
    ///   raises `ValueError` if this rule is violated.
    /// - `integration_method` — `"backward_euler"`, `"trapezoidal"`,
    ///   or `"gear2"`. Defaults to `"trapezoidal"` per design.md.
    ///   Only meaningful for time-domain analyses (transient,
    ///   mixed-signal); accepted but inert for other kinds so
    ///   default-constructed requests have a sensible value.
    /// - `boundary_interpolation` — `"zero_order_hold"` (default) or
    ///   `"linear"` per ADR-0007. Only meaningful for mixed-signal;
    ///   accepted but inert for other kinds for the same reason.
    ///
    /// # Errors
    ///
    /// - `TypeError` if `analysis_type`, `integration_method`, or
    ///   `boundary_interpolation` is not one of the recognised tags,
    ///   or if `sweep` is the wrong shape (not a 4-tuple, wrong
    ///   element types).
    /// - `ValueError` if `sweep.start` or `sweep.stop` is not finite,
    ///   `sweep.points == 0`, `sweep.scale` is unrecognised, or the
    ///   `sweep` presence does not match the analysis type's sweep
    ///   requirement.
    #[new]
    #[pyo3(signature = (
        analysis_type,
        sweep=None,
        integration_method=None,
        boundary_interpolation=None,
    ))]
    pub fn new(
        analysis_type: &str,
        sweep: Option<&Bound<'_, PyAny>>,
        integration_method: Option<&str>,
        boundary_interpolation: Option<&str>,
    ) -> PyResult<Self> {
        let analysis_type_kind = parse_analysis_type(analysis_type)?;
        let analysis_type_slug = analysis_type_kind.slug().to_string();

        let sweep = match sweep {
            None => None,
            Some(obj) if obj.is_none() => None,
            Some(obj) => Some(parse_sweep(obj)?),
        };
        validate_sweep_presence(analysis_type_kind, sweep.as_ref())?;

        let integration_method = match integration_method {
            None => DEFAULT_INTEGRATION_METHOD.to_string(),
            Some(s) => parse_integration_method(s)?.to_string(),
        };
        let boundary_interpolation = match boundary_interpolation {
            None => DEFAULT_BOUNDARY_INTERPOLATION.to_string(),
            Some(s) => parse_boundary_interpolation(s)?.to_string(),
        };

        Ok(Self {
            analysis_type_slug,
            analysis_type_kind,
            sweep,
            integration_method,
            boundary_interpolation,
        })
    }

    /// The analysis-type slug. One of the six canonical strings from
    /// [`circuit_solver_types::AnalysisType::slug`], regardless of
    /// whether the user passed the canonical slug or a short-form
    /// alias to `__new__`.
    #[getter]
    #[must_use]
    pub fn analysis_type(&self) -> &str {
        &self.analysis_type_slug
    }

    /// The sweep parameters as a `(start, stop, points, scale)` tuple,
    /// or `None` for non-sweeping analyses.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` only if allocating the result tuple or
    /// converting one of its `f64` / `usize` / `&str` elements fails —
    /// both are infallible in practice on `CPython >= 3.9` with the
    /// `abi3-py39` feature, but `PyResult` is the conservative
    /// signature so future refactors that introduce a fallible
    /// conversion don't need to change the API.
    #[getter]
    pub fn sweep<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyTuple>>> {
        match self.sweep {
            None => Ok(None),
            Some(s) => {
                let tup = PyTuple::new(
                    py,
                    [
                        s.start.into_pyobject(py)?.into_any().unbind(),
                        s.stop.into_pyobject(py)?.into_any().unbind(),
                        s.points.into_pyobject(py)?.into_any().unbind(),
                        s.scale.tag().into_pyobject(py)?.into_any().unbind(),
                    ],
                )?;
                Ok(Some(tup))
            }
        }
    }

    /// The chosen integration method (`"backward_euler"`,
    /// `"trapezoidal"`, or `"gear2"`). Always populated; the default
    /// is `"trapezoidal"`.
    #[getter]
    #[must_use]
    pub fn integration_method(&self) -> &str {
        &self.integration_method
    }

    /// The chosen analog–digital boundary interpolation
    /// (`"zero_order_hold"` or `"linear"`) per ADR-0007. Always
    /// populated; the default is `"zero_order_hold"`.
    #[getter]
    #[must_use]
    pub fn boundary_interpolation(&self) -> &str {
        &self.boundary_interpolation
    }

    /// Short diagnostic representation suitable for log scraping.
    ///
    /// Shape: `AnalysisRequest(type=dc-operating-point, sweep=None,
    /// integration=trapezoidal, boundary=zero_order_hold)`. Stable
    /// enough for debugging but not part of the public contract;
    /// ADR-0010 keeps the `__repr__` surface unstable.
    fn __repr__(&self) -> String {
        let sweep_repr = match self.sweep {
            None => "None".to_string(),
            Some(s) => format!(
                "({:?}, {:?}, {}, {:?})",
                s.start,
                s.stop,
                s.points,
                s.scale.tag()
            ),
        };
        format!(
            "AnalysisRequest(type={}, sweep={sweep_repr}, integration={}, boundary={})",
            self.analysis_type_slug, self.integration_method, self.boundary_interpolation,
        )
    }
}

impl PyAnalysisRequest {
    /// Parsed analysis-type discriminator. Crate-private accessor for
    /// downstream tasks (`Simulator.run`, #57+) that need to dispatch
    /// without re-parsing the slug.
    ///
    /// Currently has no in-tree consumer; the `dead_code` allow is
    /// the explicit "this is forward-declared" signal so the
    /// workspace's `-D warnings` clippy preflight does not reject it
    /// before #57+ lands.
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn kind(&self) -> AnalysisType {
        self.analysis_type_kind
    }
}

/// Parse an analysis-type string into an [`AnalysisType`]. Accepts the
/// six canonical slugs and a handful of short-form aliases.
fn parse_analysis_type(tag: &str) -> PyResult<AnalysisType> {
    match tag {
        "dc-operating-point" | "dc_op" | "dc" => Ok(AnalysisType::DcOperatingPoint),
        "dc-sweep" | "dc_sweep" => Ok(AnalysisType::DcSweep),
        "ac-small-signal" | "ac" => Ok(AnalysisType::AcSmallSignal),
        "transient-time-domain" | "transient" | "tran" => Ok(AnalysisType::Transient),
        "noise-spectral-density" | "noise" => Ok(AnalysisType::Noise),
        "mixed-signal-cosim" | "mixed_signal" | "mixed" => Ok(AnalysisType::MixedSignal),
        other => Err(PyTypeError::new_err(format!(
            "unrecognised analysis_type {other:?}; expected one of \
             'dc-operating-point', 'dc-sweep', 'ac-small-signal', \
             'transient-time-domain', 'noise-spectral-density', \
             'mixed-signal-cosim' (short aliases: 'dc_op', 'dc_sweep', \
             'ac', 'transient', 'noise', 'mixed_signal')"
        ))),
    }
}

/// Parse a Python object into a [`SweepParams`]. Accepts a `tuple` or
/// `list` of length 4 with element types `(float, float, int, str)`.
fn parse_sweep(obj: &Bound<'_, PyAny>) -> PyResult<SweepParams> {
    let seq: Vec<Bound<'_, PyAny>> = obj
        .try_iter()
        .map_err(|_| {
            PyTypeError::new_err(
                "sweep must be a 4-tuple (start: float, stop: float, \
             points: int, scale: str)",
            )
        })?
        .collect::<PyResult<Vec<_>>>()?;
    if seq.len() != 4 {
        return Err(PyTypeError::new_err(format!(
            "sweep must be a 4-tuple (start, stop, points, scale); got {} elements",
            seq.len()
        )));
    }
    let start: f64 = seq[0]
        .extract()
        .map_err(|e| PyTypeError::new_err(format!("sweep.start: expected float, {e}")))?;
    let stop: f64 = seq[1]
        .extract()
        .map_err(|e| PyTypeError::new_err(format!("sweep.stop: expected float, {e}")))?;
    let points: usize = seq[2].extract().map_err(|e| {
        PyTypeError::new_err(format!("sweep.points: expected non-negative int, {e}"))
    })?;
    let scale_tag: String = seq[3]
        .extract()
        .map_err(|e| PyTypeError::new_err(format!("sweep.scale: expected str, {e}")))?;

    if !start.is_finite() {
        return Err(PyValueError::new_err(format!(
            "sweep.start must be finite; got {start}"
        )));
    }
    if !stop.is_finite() {
        return Err(PyValueError::new_err(format!(
            "sweep.stop must be finite; got {stop}"
        )));
    }
    if points == 0 {
        return Err(PyValueError::new_err("sweep.points must be >= 1"));
    }
    let scale = match scale_tag.as_str() {
        "linear" | "lin" => SweepScale::Linear,
        "log" | "logarithmic" => SweepScale::Logarithmic,
        other => {
            return Err(PyValueError::new_err(format!(
                "sweep.scale {other:?} unrecognised; expected 'linear' or 'log'"
            )))
        }
    };
    Ok(SweepParams {
        start,
        stop,
        points,
        scale,
    })
}

/// Enforce that the presence of `sweep` matches the analysis-type's
/// expectation. Sweeping kinds require a sweep; non-sweeping kinds
/// reject one.
fn validate_sweep_presence(kind: AnalysisType, sweep: Option<&SweepParams>) -> PyResult<()> {
    let sweeping = matches!(
        kind,
        AnalysisType::DcSweep | AnalysisType::AcSmallSignal | AnalysisType::Noise
    );
    match (sweeping, sweep) {
        (true, None) => Err(PyValueError::new_err(format!(
            "analysis_type {:?} requires a sweep argument",
            kind.slug()
        ))),
        (false, Some(_)) => Err(PyValueError::new_err(format!(
            "analysis_type {:?} does not accept a sweep argument",
            kind.slug()
        ))),
        _ => Ok(()),
    }
}

/// Validate an `integration_method` tag against the closed set defined
/// in `design.md` §"Trapezoidal ringing". Returns the canonical
/// lowercase tag to store.
fn parse_integration_method(tag: &str) -> PyResult<&'static str> {
    match tag {
        "backward_euler" | "be" => Ok("backward_euler"),
        "trapezoidal" | "tr" => Ok("trapezoidal"),
        "gear2" | "gear-2" | "bdf2" => Ok("gear2"),
        other => Err(PyTypeError::new_err(format!(
            "unrecognised integration_method {other:?}; expected one of \
             'backward_euler', 'trapezoidal', 'gear2'"
        ))),
    }
}

/// Validate a `boundary_interpolation` tag against ADR-0007's closed
/// set. Returns the canonical lowercase tag to store.
fn parse_boundary_interpolation(tag: &str) -> PyResult<&'static str> {
    match tag {
        "zero_order_hold" | "zoh" => Ok("zero_order_hold"),
        "linear" => Ok("linear"),
        other => Err(PyTypeError::new_err(format!(
            "unrecognised boundary_interpolation {other:?}; expected \
             'zero_order_hold' or 'linear' (per ADR-0007)"
        ))),
    }
}
