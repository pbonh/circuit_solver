//! `PyO3` `CircuitBuilder` class — Python-facing wrapper around
//! `netlist_graph::CircuitBuilder`.
//!
//! This module implements **tasks.md item #52** for the
//! `2026-05-21-v1-spec` change. It exposes the four methods required by
//! that task — `add_element`, `add_wire`, `add_model`, `add_subcircuit`
//! — as `#[pymethods]` that delegate to the upstream Rust builder
//! produced in task #5.
//!
//! `build()` (the bridge into an immutable `CircuitGraph` Python
//! handle) is intentionally **out of scope** for this task: per
//! `tasks.md`, item #53 owns the `build()` method and the immutable
//! `CircuitGraph` `PyO3` handle. Until #53 lands, this class is only
//! useful for accumulating declarations and for unit-testing the
//! delegation paths; two private inspection helpers —
//! [`PyCircuitBuilder::element_decl_count`] and
//! [`PyCircuitBuilder::build_snapshot_element_count`] — expose the
//! post-add and post-build element counts respectively so the test
//! suite can verify the delegation side-effects (#52) and the
//! builder-isolation invariant (#55) without depending on #53's
//! `build()` returning a full `CircuitGraph` handle.
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
    ///   the dedicated `add_subcircuit_instance` entry-point (task
    ///   #53's scope; not exposed yet).
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
    /// [`netlist_graph::CircuitBuilder::element_decl_count`]. It exists
    /// so unit tests can verify the delegation side-effects of
    /// `add_element` without depending on `build()` (which lands in
    /// tasks.md #53). It is **not** part of the public Python contract
    /// the Gherkin scenarios exercise; it may be removed once #53
    /// provides a proper `CircuitGraph` handle to inspect.
    #[must_use]
    pub fn element_decl_count(&self) -> usize {
        self.inner.element_decl_count()
    }

    /// Build a `CircuitGraph` snapshot and return its post-expansion
    /// element count as an inspection helper.
    ///
    /// This method exists to verify the
    /// `python-frontend#builder-isolation-across-multiple-builds`
    /// Gherkin scenario at the Python-frontend layer **without**
    /// committing the full `CircuitGraph` `PyO3` handle, which is owned
    /// by tasks.md item #53. It drives
    /// [`netlist_graph::CircuitBuilder::build`] and returns
    /// `netlist_graph::CircuitGraph::element_count` for the resulting
    /// snapshot.
    ///
    /// The isolation invariant the spec scenario asserts is:
    ///
    /// > After `graph_a = builder.build()` and a subsequent
    /// > `add_element("R2", …)`, calling `builder.build()` again
    /// > produces a `graph_b` that contains `R2`, while `graph_a`
    /// > remains a frozen snapshot of the state at *its* `build()`
    /// > call site.
    ///
    /// The Rust core proves this invariant in
    /// `netlist_graph::builder::tests::builder_isolation_across_multiple_builds`;
    /// this Python-frontend helper lifts the invariant across the
    /// `PyO3` boundary by returning *owned* `usize` snapshots — every
    /// returned value is an independent Python int, captured from a
    /// distinct call to the underlying Rust builder. The Python-level
    /// regression test then verifies that the first snapshot's count
    /// does not move when later builds happen.
    ///
    /// # Stability
    ///
    /// Per [ADR-0010] the Rust API surface is unstable at v1.0.0; this
    /// helper is explicitly marked for replacement by #53's full
    /// `build() -> PyCircuitGraph` once that lands. The Python contract
    /// the Gherkin scenarios exercise will then move to
    /// `len(graph_a.elements())` (or equivalent), not this helper.
    ///
    /// # Errors
    ///
    /// Raises `CircuitBuilderError` if the underlying Rust builder's
    /// subcircuit expansion fails (unknown subcircuit reference,
    /// expansion cycle).
    ///
    /// [ADR-0010]: ../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md
    pub fn build_snapshot_element_count(&mut self) -> PyResult<usize> {
        let graph = self.inner.build().map_err(|e| to_py_err(&e))?;
        Ok(graph.element_count())
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
