//! `PyO3` `Result` class — Python-facing unified analysis-output value
//! object.
//!
//! This module implements **tasks.md item #57** for the
//! `2026-05-21-v1-spec` change. It introduces the `Result` Python class
//! that carries the four output channels named by the spec's Glossary:
//!
//! 1. **Node voltages** — scalar DC voltage per node, keyed by node
//!    name (e.g. `"n1"`).
//! 2. **Branch currents** — scalar DC current per current-carrying
//!    element, keyed by element name (e.g. `"V1"`, `"I1"`).
//! 3. **Waveforms** — time-domain `(times, values)` pairs per node,
//!    keyed by node name. Produced by transient analyses.
//! 4. **Transfer functions** — frequency-domain `(frequencies_hz,
//!    magnitude_db, phase_degrees)` triples per node, keyed by node
//!    name. Produced by AC analyses.
//!
//! All four channels are accessible **by name**, satisfying the
//! task #57 contract: *"`Result` Python class: node voltages, branch
//! currents, `Waveform`s, `TransferFunction`s accessible by name."*
//!
//! # Scope (what this task does and does *not* deliver)
//!
//! Task #57 delivers the **value object** only — the Python type with
//! its by-name accessors, validated construction, and immutable
//! semantics. The Gherkin scenario
//! `python-frontend#analysis-request-and-result-retrieval` requires
//! `Simulator` *submission* to *produce* a `Result`; that submission
//! entry point is downstream (the broader python-frontend capability
//! still has items #58 and beyond in flight). Until it lands the
//! `Result` is constructed directly from Python in tests and from a
//! crate-private adapter once the Rust-side analysis runners are
//! wired through.
//!
//! Specifically out of scope for #57:
//!
//! - **Zero-copy `NumPy` arrays** — tasks.md #58 (`Implement zero-copy
//!   NumPy result arrays: PyO3 numpy feature, Rust-owned memory viewed
//!   as ndarray dtype float64`). For #57 the waveform/transfer-function
//!   accessors return plain Python tuples of `list[float]`. The
//!   `pyo3-numpy` dependency is intentionally **not** added here; #58
//!   will refactor the array-returning getters to hand back
//!   `numpy.ndarray` views without changing the by-name lookup
//!   surface.
//! - **GIL release** — tasks.md #59. Construction is pure-Python work;
//!   `Result` does no native solver work, so `Python::allow_threads`
//!   has nothing to wrap here. Future `Simulator.run` entry points
//!   are the load-bearing surface for that.
//! - **Simulator submission** — the entry point that consumes an
//!   `AnalysisRequest` + `CircuitGraph` and emits a `Result` is a
//!   later task.
//!
//! # Surface decisions (recorded for ADR-0010 callers)
//!
//! - **`#[pyclass(frozen)]`.** Like
//!   [`crate::analysis_request::PyAnalysisRequest`] and
//!   [`crate::graph::PyCircuitGraph`], the `Result` is immutable once
//!   constructed. All four channels are set in `__new__` and then
//!   read-only. This rules out the entire "mutated mid-flight"
//!   failure class and matches the
//!   `python-frontend#immutable-circuit-graph-prevents-post-build-mutation`
//!   scenario's hygiene posture extended to result objects.
//!
//! - **Names, not opaque `NodeId(u32)`.** The four channels are keyed
//!   by `String` rather than the internal `NodeId(u32)` /
//!   `BranchId(u32)` identifiers. Python users address nodes the same
//!   way they wrote them into the builder — by the netlist name. The
//!   crate-private adapter that wraps a Rust `OperatingPoint` /
//!   `Waveform` / `TransferFunction` (downstream task) is responsible
//!   for the name-resolution side: it pulls the node-name table off
//!   the originating `CircuitGraph` and projects voltages/currents
//!   into a name-keyed map before constructing the Python `Result`.
//!
//! - **Plain Python tuples of `list[float]` for arrays (for now).**
//!   The waveform getter returns `(times, values)` as a 2-tuple of
//!   `list[float]`; the transfer-function getter returns
//!   `(frequencies_hz, magnitude_db, phase_degrees)` as a 3-tuple of
//!   `list[float]`. Tasks.md #58 will refactor these to
//!   `numpy.ndarray` views over Rust-owned memory without changing
//!   the by-name lookup signature. The intermediate representation is
//!   `Vec<f64>` so the #58 swap is a single-site refactor.
//!
//! - **`PyKeyError` on misses, optional accessors for soft lookups.**
//!   The headline accessors (`node_voltage`, `branch_current`,
//!   `waveform`, `transfer_function`) raise `KeyError` on a missing
//!   name, mirroring Python's dict semantics — a `Result` is the
//!   user's primary handle and "voltage at the node I asked about
//!   isn't here" is a bug worth surfacing. Soft "is this name
//!   present?" probes use the `node_names()` /
//!   `transfer_function_names()` listing getters or Python's `in`
//!   operator (implemented via `__contains__` on the listing tuples).
//!
//! - **Construction validation.** `__new__` rejects non-finite values,
//!   length mismatches in waveform/transfer-function vectors, and
//!   empty names. The error vocabulary is `TypeError` (wrong shape)
//!   and `ValueError` (out-of-range / non-finite / mismatched
//!   lengths). Construction-time validation matches the
//!   `analysis_request` precedent: catch malformed data at the
//!   boundary so the downstream surface is consistently well-typed.
//!
//! - **No public Rust `AnalysisResult` type added in this task.**
//!   ADR-0010 keeps the Rust API surface unstable for v1; the Python
//!   `Result` owns its fields directly via the `Channels` struct
//!   defined below. When the public Rust `AnalysisResult` lands (a
//!   follow-up task), this Python class will gain a `from_inner`
//!   adapter that pulls from it; until then the surface is pure
//!   Python-PyO3.
//!
//! # ADR alignment
//!
//! - **ADR-0001 (`PyO3` In-Process Binding with Immutable Circuit
//!   Graph)** — `Result` is the read-only counterpart to the
//!   `CircuitGraph` write boundary: data flows *out* of the native
//!   solver through this object, just as construction flows *in*
//!   through `CircuitBuilder`. The frozen pyclass mirrors the ADR's
//!   "no shared mutable state crosses the language boundary"
//!   posture.
//! - **ADR-0010 (Unstable Public Rust API for v1)** — no public Rust
//!   `AnalysisResult` is exported here. The Python class owns its
//!   data via crate-private structs. Downstream tasks that produce a
//!   `Result` from a Rust analysis output will go through a
//!   crate-private `from_inner` adapter that this module will gain
//!   when the producer lands.

use std::collections::BTreeMap;

use pyo3::exceptions::{PyKeyError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyMapping, PyTuple};

/// Internal storage for one waveform: parallel time/value vectors.
///
/// Invariant maintained at `__new__` time: `times.len() ==
/// values.len()` and both are finite. The vectors are `Vec<f64>` so
/// the tasks.md #58 `NumPy` refactor can construct a zero-copy ndarray
/// view directly without intermediate copies.
#[derive(Debug, Clone, PartialEq)]
struct WaveformChannel {
    times: Vec<f64>,
    values: Vec<f64>,
}

/// Internal storage for one transfer function: parallel frequency,
/// magnitude (dB), and phase (degrees) vectors.
///
/// Invariant maintained at `__new__` time: all three vectors share a
/// common length and are finite at every index. Layout matches
/// `analysis_orchestration::TransferFunction` verbatim so the
/// downstream `from_inner` adapter is a no-op move.
#[derive(Debug, Clone, PartialEq)]
struct TransferFunctionChannel {
    frequencies_hz: Vec<f64>,
    magnitude_db: Vec<f64>,
    phase_degrees: Vec<f64>,
}

/// Python class: `circuit_solver.Result`.
///
/// Immutable value object holding the four output channels of an
/// analysis run — node voltages, branch currents, waveforms, and
/// transfer functions — each addressable by name. See the
/// module-level documentation for the field semantics and the
/// task-#57-specific scope (construction only; submission is
/// downstream).
#[pyclass(name = "Result", module = "circuit_solver", frozen)]
pub struct PyAnalysisResult {
    /// DC scalar voltages, keyed by node name. `BTreeMap` so the
    /// `node_names()` listing is deterministic (Python users iterating
    /// over a result must see a stable order across runs).
    node_voltages: BTreeMap<String, f64>,
    /// DC scalar currents, keyed by element name (e.g. `"V1"`,
    /// `"I1"`). Branch identity follows the convention from
    /// `analysis_orchestration::BranchCurrentSample`: positive =
    /// flowing from the element's `+` terminal to its `−` terminal.
    branch_currents: BTreeMap<String, f64>,
    /// Time-domain waveforms keyed by node name. Empty when the
    /// underlying analysis was non-transient.
    waveforms: BTreeMap<String, WaveformChannel>,
    /// Frequency-domain transfer functions keyed by output-node name.
    /// Empty when the underlying analysis was non-AC.
    transfer_functions: BTreeMap<String, TransferFunctionChannel>,
}

#[pymethods]
impl PyAnalysisResult {
    /// Construct a new `Result`.
    ///
    /// # Arguments
    ///
    /// All four arguments default to empty so a partial result (e.g.
    /// DC-only, transient-only, AC-only) is well-formed. Channels
    /// that the underlying analysis did not populate simply stay
    /// empty.
    ///
    /// - `node_voltages` — `dict[str, float]` mapping node name →
    ///   DC voltage in volts.
    /// - `branch_currents` — `dict[str, float]` mapping element name
    ///   → DC current in amperes.
    /// - `waveforms` — `dict[str, tuple[Sequence[float],
    ///   Sequence[float]]]` mapping node name → `(times, values)`
    ///   pair. Both inner sequences must share a length.
    /// - `transfer_functions` — `dict[str, tuple[Sequence[float],
    ///   Sequence[float], Sequence[float]]]` mapping node name →
    ///   `(frequencies_hz, magnitude_db, phase_degrees)` triple. All
    ///   three inner sequences must share a length.
    ///
    /// # Errors
    ///
    /// - `TypeError` if any of the four arguments is not a mapping,
    ///   or if a waveform/transfer-function value is not a tuple of
    ///   the expected arity, or an inner element cannot be coerced
    ///   to `float` / `str`.
    /// - `ValueError` if any voltage/current/array value is non-finite
    ///   (NaN or ±∞), if a name is empty, or if a
    ///   waveform/transfer-function's parallel arrays have mismatched
    ///   lengths.
    #[new]
    #[pyo3(signature = (
        node_voltages=None,
        branch_currents=None,
        waveforms=None,
        transfer_functions=None,
    ))]
    pub fn new(
        node_voltages: Option<&Bound<'_, PyAny>>,
        branch_currents: Option<&Bound<'_, PyAny>>,
        waveforms: Option<&Bound<'_, PyAny>>,
        transfer_functions: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let node_voltages = match node_voltages {
            None => BTreeMap::new(),
            Some(obj) if obj.is_none() => BTreeMap::new(),
            Some(obj) => parse_scalar_map(obj, "node_voltages")?,
        };
        let branch_currents = match branch_currents {
            None => BTreeMap::new(),
            Some(obj) if obj.is_none() => BTreeMap::new(),
            Some(obj) => parse_scalar_map(obj, "branch_currents")?,
        };
        let waveforms = match waveforms {
            None => BTreeMap::new(),
            Some(obj) if obj.is_none() => BTreeMap::new(),
            Some(obj) => parse_waveform_map(obj)?,
        };
        let transfer_functions = match transfer_functions {
            None => BTreeMap::new(),
            Some(obj) if obj.is_none() => BTreeMap::new(),
            Some(obj) => parse_transfer_function_map(obj)?,
        };

        Ok(Self {
            node_voltages,
            branch_currents,
            waveforms,
            transfer_functions,
        })
    }

    /// DC voltage at `name`, in volts.
    ///
    /// # Errors
    ///
    /// Raises `KeyError` if `name` is not a recorded node in this
    /// `Result`. Use `name in result.node_names()` for a soft check.
    pub fn node_voltage(&self, name: &str) -> PyResult<f64> {
        self.node_voltages
            .get(name)
            .copied()
            .ok_or_else(|| PyKeyError::new_err(format!("no node voltage recorded for {name:?}")))
    }

    /// DC current through the element named `name`, in amperes.
    ///
    /// Sign convention: positive current flows from the element's
    /// `+` terminal (terminal slot 0) to the element's `−` terminal
    /// (terminal slot 1), per
    /// `analysis_orchestration::BranchCurrentSample`.
    ///
    /// # Errors
    ///
    /// Raises `KeyError` if no branch current is recorded for `name`.
    pub fn branch_current(&self, name: &str) -> PyResult<f64> {
        self.branch_currents
            .get(name)
            .copied()
            .ok_or_else(|| PyKeyError::new_err(format!("no branch current recorded for {name:?}")))
    }

    /// Time-domain waveform at node `name`, as a `(times, values)`
    /// tuple. Both elements are `list[float]` of the same length;
    /// `times` is monotonically non-decreasing seconds, `values` is
    /// volts.
    ///
    /// # Errors
    ///
    /// - `KeyError` if no waveform is recorded for `name`.
    /// - `PyErr` if allocating the result tuple/lists fails (only
    ///   under host-Python OOM conditions).
    ///
    /// # Forward compatibility
    ///
    /// Tasks.md #58 will swap the inner `list[float]` for a
    /// `numpy.ndarray` view over Rust-owned memory. The outer
    /// 2-tuple shape and the `(times, values)` ordering are stable
    /// across that refactor.
    pub fn waveform<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyTuple>> {
        let wf = self
            .waveforms
            .get(name)
            .ok_or_else(|| PyKeyError::new_err(format!("no waveform recorded for {name:?}")))?;
        PyTuple::new(
            py,
            [
                wf.times.clone().into_pyobject(py)?.into_any().unbind(),
                wf.values.clone().into_pyobject(py)?.into_any().unbind(),
            ],
        )
    }

    /// Frequency-domain transfer function at node `name`, as a
    /// `(frequencies_hz, magnitude_db, phase_degrees)` 3-tuple of
    /// `list[float]` of the same length. Convention matches
    /// `analysis_orchestration::TransferFunction`: magnitude is
    /// `20·log10|H|` in dB; phase is `arg(H)·180/π` in degrees,
    /// principal value `(-180, 180]`.
    ///
    /// # Errors
    ///
    /// - `KeyError` if no transfer function is recorded for `name`.
    /// - `PyErr` if allocating the result tuple/lists fails.
    ///
    /// # Forward compatibility
    ///
    /// Tasks.md #58 will swap the inner `list[float]` for
    /// `numpy.ndarray` views without changing the outer 3-tuple shape
    /// or element ordering.
    pub fn transfer_function<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyTuple>> {
        let tf = self.transfer_functions.get(name).ok_or_else(|| {
            PyKeyError::new_err(format!("no transfer function recorded for {name:?}"))
        })?;
        PyTuple::new(
            py,
            [
                tf.frequencies_hz
                    .clone()
                    .into_pyobject(py)?
                    .into_any()
                    .unbind(),
                tf.magnitude_db
                    .clone()
                    .into_pyobject(py)?
                    .into_any()
                    .unbind(),
                tf.phase_degrees
                    .clone()
                    .into_pyobject(py)?
                    .into_any()
                    .unbind(),
            ],
        )
    }

    /// Tuple of every node name that has a recorded DC voltage, in
    /// stable sorted order.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` only on host-Python allocation failure.
    pub fn node_names<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.node_voltages.keys().map(String::as_str))
    }

    /// Tuple of every branch (element) name that has a recorded DC
    /// current, in stable sorted order.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` only on host-Python allocation failure.
    pub fn branch_names<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.branch_currents.keys().map(String::as_str))
    }

    /// Tuple of every node name that has a recorded waveform, in
    /// stable sorted order.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` only on host-Python allocation failure.
    pub fn waveform_names<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.waveforms.keys().map(String::as_str))
    }

    /// Tuple of every node name that has a recorded transfer
    /// function, in stable sorted order.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` only on host-Python allocation failure.
    pub fn transfer_function_names<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.transfer_functions.keys().map(String::as_str))
    }

    /// True iff no channel carries any data.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.node_voltages.is_empty()
            && self.branch_currents.is_empty()
            && self.waveforms.is_empty()
            && self.transfer_functions.is_empty()
    }

    /// Short diagnostic representation suitable for log scraping.
    ///
    /// Shape: `Result(nodes=2, branches=1, waveforms=0, transfer_functions=0)`.
    /// Stable enough for debugging but not part of the public
    /// contract; ADR-0010 keeps the `__repr__` surface unstable.
    fn __repr__(&self) -> String {
        format!(
            "Result(nodes={}, branches={}, waveforms={}, transfer_functions={})",
            self.node_voltages.len(),
            self.branch_currents.len(),
            self.waveforms.len(),
            self.transfer_functions.len(),
        )
    }
}

/// Parse a Python mapping into a `BTreeMap<String, f64>`. The `field`
/// argument is woven into error messages so the caller knows which of
/// the four channels failed validation.
fn parse_scalar_map(obj: &Bound<'_, PyAny>, field: &str) -> PyResult<BTreeMap<String, f64>> {
    let mapping = downcast_mapping(obj, field)?;
    let mut out = BTreeMap::new();
    let items = mapping.items()?;
    for item in items.try_iter()? {
        let pair = item?;
        let (key, value): (String, f64) = pair.extract().map_err(|e| {
            PyTypeError::new_err(format!(
                "{field}: expected each entry to be (str, float); {e}"
            ))
        })?;
        validate_name(&key, field)?;
        if !value.is_finite() {
            return Err(PyValueError::new_err(format!(
                "{field}[{key:?}]: value must be finite; got {value}"
            )));
        }
        out.insert(key, value);
    }
    Ok(out)
}

/// Parse the `waveforms` mapping into a `BTreeMap<String,
/// WaveformChannel>`.
fn parse_waveform_map(obj: &Bound<'_, PyAny>) -> PyResult<BTreeMap<String, WaveformChannel>> {
    let mapping = downcast_mapping(obj, "waveforms")?;
    let mut out = BTreeMap::new();
    let items = mapping.items()?;
    for item in items.try_iter()? {
        let pair = item?;
        let (key, value): (String, Bound<'_, PyAny>) = pair.extract().map_err(|e| {
            PyTypeError::new_err(format!(
                "waveforms: expected each entry to be (str, (times, values)); {e}"
            ))
        })?;
        validate_name(&key, "waveforms")?;
        let (times, values) = extract_pair(&value, "waveforms", &key)?;
        let times = extract_finite_vec(&times, "waveforms.times", &key)?;
        let values = extract_finite_vec(&values, "waveforms.values", &key)?;
        if times.len() != values.len() {
            return Err(PyValueError::new_err(format!(
                "waveforms[{key:?}]: times length ({}) must equal values length ({})",
                times.len(),
                values.len(),
            )));
        }
        out.insert(key, WaveformChannel { times, values });
    }
    Ok(out)
}

/// Parse the `transfer_functions` mapping into a `BTreeMap<String,
/// TransferFunctionChannel>`.
fn parse_transfer_function_map(
    obj: &Bound<'_, PyAny>,
) -> PyResult<BTreeMap<String, TransferFunctionChannel>> {
    let mapping = downcast_mapping(obj, "transfer_functions")?;
    let mut out = BTreeMap::new();
    let items = mapping.items()?;
    for item in items.try_iter()? {
        let pair = item?;
        let (key, value): (String, Bound<'_, PyAny>) = pair.extract().map_err(|e| {
            PyTypeError::new_err(format!(
                "transfer_functions: expected each entry to be \
                 (str, (frequencies_hz, magnitude_db, phase_degrees)); {e}"
            ))
        })?;
        validate_name(&key, "transfer_functions")?;
        let (frequencies_hz, magnitude_db, phase_degrees) =
            extract_triple(&value, "transfer_functions", &key)?;
        let frequencies_hz =
            extract_finite_vec(&frequencies_hz, "transfer_functions.frequencies_hz", &key)?;
        let magnitude_db =
            extract_finite_vec(&magnitude_db, "transfer_functions.magnitude_db", &key)?;
        let phase_degrees =
            extract_finite_vec(&phase_degrees, "transfer_functions.phase_degrees", &key)?;
        let n = frequencies_hz.len();
        if magnitude_db.len() != n || phase_degrees.len() != n {
            return Err(PyValueError::new_err(format!(
                "transfer_functions[{key:?}]: parallel arrays must share a length; \
                 got frequencies_hz={}, magnitude_db={}, phase_degrees={}",
                n,
                magnitude_db.len(),
                phase_degrees.len(),
            )));
        }
        out.insert(
            key,
            TransferFunctionChannel {
                frequencies_hz,
                magnitude_db,
                phase_degrees,
            },
        );
    }
    Ok(out)
}

/// Downcast a Python object to a [`PyMapping`], producing a clear
/// `TypeError` if it isn't one. Accepts both `dict` and any other
/// mapping (e.g. `collections.OrderedDict`) because the
/// `PyMapping` protocol is what we actually need.
fn downcast_mapping<'py>(obj: &Bound<'py, PyAny>, field: &str) -> PyResult<Bound<'py, PyMapping>> {
    obj.cast::<PyMapping>().cloned().map_err(|_| {
        PyTypeError::new_err(format!(
            "{field}: expected a mapping (dict-like); got {}",
            obj.get_type()
                .name()
                .map_or_else(|_| "<unknown type>".to_string(), |s| s.to_string()),
        ))
    })
}

/// Extract a 2-tuple from a value while producing a contextful error.
fn extract_pair<'py>(
    value: &Bound<'py, PyAny>,
    field: &str,
    key: &str,
) -> PyResult<(Bound<'py, PyAny>, Bound<'py, PyAny>)> {
    let seq: Vec<Bound<'py, PyAny>> = value
        .try_iter()
        .map_err(|_| {
            PyTypeError::new_err(format!(
                "{field}[{key:?}]: expected a 2-tuple (times, values)"
            ))
        })?
        .collect::<PyResult<Vec<_>>>()?;
    if seq.len() != 2 {
        return Err(PyTypeError::new_err(format!(
            "{field}[{key:?}]: expected a 2-tuple (times, values); got {} elements",
            seq.len()
        )));
    }
    let mut it = seq.into_iter();
    let times = it.next().unwrap();
    let values = it.next().unwrap();
    Ok((times, values))
}

/// Extract a 3-tuple from a value while producing a contextful error.
fn extract_triple<'py>(
    value: &Bound<'py, PyAny>,
    field: &str,
    key: &str,
) -> PyResult<(Bound<'py, PyAny>, Bound<'py, PyAny>, Bound<'py, PyAny>)> {
    let seq: Vec<Bound<'py, PyAny>> = value
        .try_iter()
        .map_err(|_| {
            PyTypeError::new_err(format!(
                "{field}[{key:?}]: expected a 3-tuple \
                 (frequencies_hz, magnitude_db, phase_degrees)"
            ))
        })?
        .collect::<PyResult<Vec<_>>>()?;
    if seq.len() != 3 {
        return Err(PyTypeError::new_err(format!(
            "{field}[{key:?}]: expected a 3-tuple \
             (frequencies_hz, magnitude_db, phase_degrees); got {} elements",
            seq.len()
        )));
    }
    let mut it = seq.into_iter();
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    let c = it.next().unwrap();
    Ok((a, b, c))
}

/// Extract a sequence of `f64` while validating that every element is
/// finite. Produces a contextful `TypeError`/`ValueError` on failure.
fn extract_finite_vec(value: &Bound<'_, PyAny>, field: &str, key: &str) -> PyResult<Vec<f64>> {
    let xs: Vec<f64> = value.extract().map_err(|e| {
        PyTypeError::new_err(format!(
            "{field}[{key:?}]: expected a sequence of floats; {e}"
        ))
    })?;
    for (i, x) in xs.iter().enumerate() {
        if !x.is_finite() {
            return Err(PyValueError::new_err(format!(
                "{field}[{key:?}][{i}]: value must be finite; got {x}"
            )));
        }
    }
    Ok(xs)
}

/// Reject empty / whitespace-only names early so accessor lookups have
/// well-typed keys.
fn validate_name(name: &str, field: &str) -> PyResult<()> {
    if name.is_empty() {
        return Err(PyValueError::new_err(format!(
            "{field}: name must be non-empty"
        )));
    }
    Ok(())
}

impl PyAnalysisResult {
    /// Crate-private constructor for downstream tasks (`Simulator.run`
    /// wiring) that already hold validated `BTreeMap`s of channel
    /// data. The `PyO3` `#[new]` is the only public construction path
    /// today; this method is the seam where the analysis-orchestration
    /// adapters will plug in once they exist.
    ///
    /// Currently has no in-tree consumer; the `dead_code` allow is
    /// the explicit "this is forward-declared" signal so the
    /// workspace's `-D warnings` clippy preflight does not reject it
    /// before the downstream wiring lands.
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn from_channels(
        node_voltages: BTreeMap<String, f64>,
        branch_currents: BTreeMap<String, f64>,
        waveforms: WaveformChannels,
        transfer_functions: TransferFunctionChannels,
    ) -> Self {
        Self {
            node_voltages,
            branch_currents,
            waveforms: waveforms
                .into_iter()
                .map(|(k, (times, values))| (k, WaveformChannel { times, values }))
                .collect(),
            transfer_functions: transfer_functions
                .into_iter()
                .map(|(k, (frequencies_hz, magnitude_db, phase_degrees))| {
                    (
                        k,
                        TransferFunctionChannel {
                            frequencies_hz,
                            magnitude_db,
                            phase_degrees,
                        },
                    )
                })
                .collect(),
        }
    }
}

/// Crate-private convenience alias for the
/// [`PyAnalysisResult::from_channels`] waveform parameter — a map from
/// node name to `(times, values)` pair. Hoisted out of the function
/// signature so the `clippy::type_complexity` lint stays clean
/// without an inline `#[allow]`.
#[allow(dead_code)]
pub(crate) type WaveformChannels = BTreeMap<String, (Vec<f64>, Vec<f64>)>;

/// Crate-private convenience alias for the
/// [`PyAnalysisResult::from_channels`] transfer-function parameter —
/// a map from output-node name to `(frequencies_hz, magnitude_db,
/// phase_degrees)` triple.
#[allow(dead_code)]
pub(crate) type TransferFunctionChannels = BTreeMap<String, (Vec<f64>, Vec<f64>, Vec<f64>)>;
