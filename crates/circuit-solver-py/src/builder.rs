//! `PyO3` `CircuitBuilder` class — Python-facing wrapper around
//! `netlist_graph::CircuitBuilder`.
//!
//! This module implements **tasks.md items #52 and #53** for the
//! `2026-05-21-v1-spec` change. It exposes the four declaration
//! methods required by #52 — `add_element`, `add_wire`, `add_model`,
//! `add_subcircuit` — plus the terminal `build()` method owned by
//! #53, all as `#[pymethods]` that delegate to the upstream Rust
//! builder produced in task #5. `build()` returns the immutable
//! [`PyCircuitGraph`] handle that lights up the full Gherkin scenario
//! `python-frontend#incremental-circuit-construction-via-builder-api`.
//!
//! The builder-isolation invariant (tasks.md #55) is proven at this
//! layer via `PyCircuitGraph` snapshot semantics: each `build()`
//! returns a fresh immutable handle, and the underlying builder
//! remains reusable so further mutations only affect graphs returned
//! by *subsequent* `build()` calls.
//!
//! A legacy inspection helper [`PyCircuitBuilder::element_decl_count`]
//! is retained from the #52-only era — it exposes the count of
//! top-level element *declarations* (pre-subcircuit-expansion) for the
//! #52-era delegation tests. New code that wants post-expansion counts
//! should prefer `builder.build().element_count()`; the two counts
//! diverge whenever subcircuit instantiation multiplies declarations.
//!
//! # Surface decisions (recorded for ADR-0010 callers)
//!
//! - **Kind encoded as a SPICE-letter string.** `add_element` and the
//!   `body` dicts of `add_subcircuit` carry the element kind as the
//!   short tag string returned by [`netlist_graph::ElementKind::tag`]
//!   (`"R"`, `"C"`, `"L"`, `"V"`, `"I"`, `"DEV"`, `"X"`). This keeps the
//!   Python surface narrow for #52 — no Python-side `ElementKind` enum
//!   wrapper, no per-variant factory functions. A richer kind API
//!   (e.g. `circuit_solver.Resistor(1e3)`) is a later UX task once the
//!   Gherkin scenario exercises more variants.
//! - **Subcircuit bodies as `list[dict]`.** `add_subcircuit` accepts a
//!   list of dicts (`{"name", "kind", "terminals", "value"?, "model"?}`)
//!   rather than exposing a Python `SubcircuitDefinition` /
//!   `ElementDecl` class. This minimises surface that ADR-0010 must
//!   keep unstable.
//! - **Chaining returns `None`.** The Rust builder returns
//!   `&mut Self` for fluent chaining; Python idiom is `None`, so the
//!   bindings discard the chain result. Method chaining in Python is
//!   reconstructed by re-assigning the same builder reference.
//! - **Mutability and the GIL.** A `#[pyclass]` defaults to a `PyCell`
//!   that grants `&mut self` borrows under the GIL — the methods take
//!   `&mut self` directly and the Rust builder mutates in place. No
//!   `RefCell` / `Mutex` is needed.

use netlist_graph::{CircuitBuilder, ElementDecl, ElementKind, SubcircuitDefinition};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::errors::to_py_err;
use crate::graph::PyCircuitGraph;

/// Python class: `circuit_solver.CircuitBuilder`.
///
/// Thin wrapper around `netlist_graph::CircuitBuilder`. See the
/// module-level documentation for the per-method Python contract.
#[pyclass(name = "CircuitBuilder", module = "circuit_solver")]
pub struct PyCircuitBuilder {
    inner: CircuitBuilder,
}

#[pymethods]
impl PyCircuitBuilder {
    /// Construct an empty `CircuitBuilder`.
    ///
    /// Python equivalent:
    ///
    /// ```python
    /// from circuit_solver import CircuitBuilder
    /// b = CircuitBuilder()
    /// ```
    #[new]
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: CircuitBuilder::new(),
        }
    }

    /// Register an element instance.
    ///
    /// # Arguments
    ///
    /// - `name` — user-facing element name (e.g. `"R1"`). Must be unique
    ///   within this builder; duplicates raise `CircuitBuilderError`.
    /// - `kind` — SPICE-letter tag identifying the element kind. The
    ///   currently-accepted tags are `"R"` (resistor), `"C"`
    ///   (capacitor), `"L"` (inductor), `"V"` (independent voltage
    ///   source), `"I"` (independent current source), and `"DEV"`
    ///   (model-resolved semiconductor; pair with `model=...`). The
    ///   `"X"` subcircuit-instance tag is *not* accepted here — use
    ///   the dedicated `add_subcircuit_instance` entry-point (a
    ///   later task; not exposed yet).
    /// - `terminals` — ordered list of net names this element connects
    ///   to. Two-terminal kinds (`R`, `C`, `L`, `V`, `I`) require
    ///   exactly two terminals; `DEV` accepts any count (the
    ///   device-modeling crate later validates the model-specific
    ///   arity).
    /// - `value` — numeric value parameter the kind expects (resistance
    ///   in Ω for `R`, capacitance in F for `C`, voltage in V for `V`,
    ///   etc.). Required for `R`, `C`, `L`, `V`, `I`. Ignored for
    ///   `DEV`. Defaults to `0.0` if omitted for two-terminal kinds —
    ///   downstream stamp generation will error out on the zero value,
    ///   but the builder does not enforce non-zero values at this
    ///   layer.
    /// - `model` — optional model-name reference resolved by the
    ///   device-modeling context. Pass for `DEV` elements; ignored for
    ///   the linear kinds.
    ///
    /// # Errors
    ///
    /// Raises `CircuitBuilderError` if the underlying Rust builder
    /// rejects the element (duplicate name, terminal arity mismatch);
    /// raises `TypeError` if `kind` is not one of the recognised
    /// SPICE-letter tags.
    #[pyo3(signature = (name, kind, terminals, value=None, model=None))]
    pub fn add_element(
        &mut self,
        name: &str,
        kind: &str,
        terminals: Vec<String>,
        value: Option<f64>,
        model: Option<String>,
    ) -> PyResult<()> {
        let kind = parse_kind(kind, value, model.as_deref())?;
        let model_name = model.map(circuit_solver_types::ModelName::new);
        self.inner
            .add_element(name, kind, terminals, model_name)
            .map_err(|e| to_py_err(&e))?;
        Ok(())
    }

    /// Declare that two net names refer to the same electrical node.
    ///
    /// # Arguments
    ///
    /// - `a`, `b` — net names. Order is irrelevant; subsequent
    ///   `add_wire`s reuse the same disjoint-set union-find that
    ///   `build()` (#53) consults to assign `NodeId`s.
    pub fn add_wire(&mut self, a: &str, b: &str) {
        self.inner.add_wire(a, b);
    }

    /// Register a device-model name.
    ///
    /// Registering the same name twice is a no-op (the Rust builder
    /// dedupes silently). The string is later resolved by the
    /// `device-modeling` crate against its `ModelName → DeviceModel`
    /// registry.
    pub fn add_model(&mut self, name: &str) {
        self.inner
            .add_model(circuit_solver_types::ModelName::new(name));
    }

    /// Register a subcircuit definition.
    ///
    /// # Arguments
    ///
    /// - `name` — subcircuit definition name (e.g. `"INV"`). Must be
    ///   unique within this builder.
    /// - `ports` — ordered list of external port net-names. Bind to
    ///   parent-scope nets at instantiation time (handled by task
    ///   #53's `add_subcircuit_instance`).
    /// - `body` — list of element-declaration dicts. Each dict
    ///   accepts the same keys as [`add_element`](#method.add_element)
    ///   (`name`, `kind`, `terminals`, optional `value`, optional
    ///   `model`). The body is stored verbatim and replayed against
    ///   the parent net namespace during expansion at `build()` time.
    ///
    /// # Errors
    ///
    /// - `CircuitBuilderError` if the subcircuit name collides with a
    ///   previously-registered definition.
    /// - `TypeError` if any body dict is malformed (missing required
    ///   key, wrong type, unrecognised `kind`).
    pub fn add_subcircuit(
        &mut self,
        name: &str,
        ports: Vec<String>,
        body: &Bound<'_, PyList>,
    ) -> PyResult<()> {
        let mut decls: Vec<ElementDecl> = Vec::with_capacity(body.len());
        for item in body.iter() {
            decls.push(parse_decl(&item)?);
        }
        let definition = SubcircuitDefinition::new(name.into(), ports, decls);
        self.inner
            .add_subcircuit(definition)
            .map_err(|e| to_py_err(&e))?;
        Ok(())
    }

    /// Number of top-level element declarations recorded so far.
    ///
    /// This is an inspection helper that mirrors
    /// [`netlist_graph::CircuitBuilder::element_decl_count`]. It was
    /// originally added in tasks.md item #52 so unit tests could
    /// verify the delegation side-effects of `add_element` without
    /// depending on `build()`. With `build()` now available
    /// (tasks.md item #53), prefer
    /// `builder.build().element_count()` for new code; this helper
    /// is kept for back-compat with #52-era tests and may be removed
    /// in a future change.
    #[must_use]
    pub fn element_decl_count(&self) -> usize {
        self.inner.element_decl_count()
    }

    /// Finalize the build: expand subcircuits, resolve wire
    /// equivalences, assign `NodeId`s, and return an immutable
    /// [`PyCircuitGraph`].
    ///
    /// Per ADR-0001 and scenario
    /// `python-frontend#builder-isolation-across-multiple-builds`,
    /// each call returns a fresh `CircuitGraph` that does not share
    /// storage with previously-built handles; the builder itself
    /// remains usable, and further mutations only affect graphs
    /// returned by *subsequent* `build()` calls.
    ///
    /// # Returns
    ///
    /// A `circuit_solver.CircuitGraph` — see [`PyCircuitGraph`] for
    /// the read-only accessors it exposes.
    ///
    /// # Errors
    ///
    /// Raises `CircuitBuilderError` if subcircuit expansion fails
    /// (unknown subcircuit reference, port-arity mismatch, expansion
    /// cycle). Linear-element validation (duplicate names, terminal
    /// arity) is performed eagerly by `add_element`, so `build()`
    /// itself never fails on those.
    ///
    /// # GIL release
    ///
    /// `build()` is one of the two principal native-work entry points
    /// on the Python surface (the other being the
    /// `circuit_solver.parse_netlist` free function — see
    /// `crate::lib::parse_netlist_py`). The bulk of the call —
    /// subcircuit expansion, wire-equivalence resolution, and
    /// `NodeId` assignment — is pure Rust over data that does not
    /// touch `CPython`. We release the GIL around that core via
    /// [`pyo3::Python::detach`] (the pyo3 0.28 successor to
    /// `allow_threads`) so concurrent Python threads can continue to
    /// execute while a build is in flight. This is the witness site
    /// for tasks.md #59 / spec scenario
    /// `python-frontend#gil-release-during-simulation`. The
    /// `PyCircuitGraph::from_inner` re-wrap on the success path does
    /// no `CPython` work either, but for clarity we keep that step
    /// outside the `detach` boundary — only the long-running native
    /// compute is held inside.
    pub fn build(&mut self, py: Python<'_>) -> PyResult<PyCircuitGraph> {
        let graph = py
            .detach(|| self.inner.build())
            .map_err(|e| to_py_err(&e))?;
        Ok(PyCircuitGraph::from_inner(graph))
    }
}

impl Default for PyCircuitBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a Python-side SPICE-letter kind tag into a Rust
/// [`netlist_graph::ElementKind`]. Numeric `value`s are taken from the
/// caller; missing values default to `0.0` for the linear kinds (the
/// builder itself does not enforce non-zero values — that is the
/// stamp generator's job).
fn parse_kind(tag: &str, value: Option<f64>, model: Option<&str>) -> PyResult<ElementKind> {
    let v = value.unwrap_or(0.0);
    match tag {
        "R" => Ok(ElementKind::Resistor { resistance_ohms: v }),
        "C" => Ok(ElementKind::Capacitor {
            capacitance_farads: v,
        }),
        "L" => Ok(ElementKind::Inductor {
            inductance_henries: v,
        }),
        "V" => Ok(ElementKind::VoltageSource { voltage_volts: v }),
        "I" => Ok(ElementKind::CurrentSource { current_amperes: v }),
        "DEV" => {
            if model.is_none() {
                return Err(PyTypeError::new_err(
                    "element kind 'DEV' requires a model=... argument",
                ));
            }
            Ok(ElementKind::Semiconductor)
        }
        other => Err(PyTypeError::new_err(format!(
            "unrecognised element kind tag: {other:?}; expected one of \
             'R', 'C', 'L', 'V', 'I', 'DEV'"
        ))),
    }
}

/// Parse one subcircuit-body element-declaration dict into an
/// [`ElementDecl`]. The accepted shape is:
///
/// ```text
/// {
///   "name": str,
///   "kind": str,                   # SPICE-letter tag
///   "terminals": list[str],
///   "value": float | None,         # optional
///   "model": str | None,           # optional
/// }
/// ```
fn parse_decl(item: &Bound<'_, PyAny>) -> PyResult<ElementDecl> {
    let dict = item.cast::<PyDict>().map_err(|_| {
        PyTypeError::new_err(
            "subcircuit body entries must be dicts with keys \
             {name, kind, terminals[, value, model]}",
        )
    })?;
    let name: String = required_str(dict, "name")?;
    let kind_tag: String = required_str(dict, "kind")?;
    let terminals: Vec<String> = required_str_list(dict, "terminals")?;
    let value: Option<f64> = optional_f64(dict, "value")?;
    let model: Option<String> = optional_str(dict, "model")?;
    let kind = parse_kind(&kind_tag, value, model.as_deref())?;
    Ok(ElementDecl {
        name: name.into(),
        kind,
        terminals,
        model: model.map(circuit_solver_types::ModelName::new),
    })
}

fn required_str(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<String> {
    let item = dict.get_item(key)?.ok_or_else(|| {
        PyTypeError::new_err(format!(
            "subcircuit body entry missing required key {key:?}"
        ))
    })?;
    item.extract::<String>().map_err(|e| {
        PyTypeError::new_err(format!(
            "subcircuit body entry key {key:?}: expected str, {e}"
        ))
    })
}

fn required_str_list(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<Vec<String>> {
    let item = dict.get_item(key)?.ok_or_else(|| {
        PyTypeError::new_err(format!(
            "subcircuit body entry missing required key {key:?}"
        ))
    })?;
    item.extract::<Vec<String>>().map_err(|e| {
        PyTypeError::new_err(format!(
            "subcircuit body entry key {key:?}: expected list[str], {e}"
        ))
    })
}

fn optional_str(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<Option<String>> {
    match dict.get_item(key)? {
        None => Ok(None),
        Some(item) if item.is_none() => Ok(None),
        Some(item) => item.extract::<String>().map(Some).map_err(|e| {
            PyTypeError::new_err(format!(
                "subcircuit body entry key {key:?}: expected str | None, {e}"
            ))
        }),
    }
}

fn optional_f64(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<Option<f64>> {
    match dict.get_item(key)? {
        None => Ok(None),
        Some(item) if item.is_none() => Ok(None),
        Some(item) => item.extract::<f64>().map(Some).map_err(|e| {
            PyTypeError::new_err(format!(
                "subcircuit body entry key {key:?}: expected float | None, {e}"
            ))
        }),
    }
}
