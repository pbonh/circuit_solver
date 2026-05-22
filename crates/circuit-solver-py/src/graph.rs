//! `PyO3` `CircuitGraph` class — Python-facing wrapper around
//! `netlist_graph::CircuitGraph`.
//!
//! This module implements **tasks.md item #53** for the
//! `2026-05-21-v1-spec` change. It exposes the immutable
//! `CircuitGraph` handle returned by
//! [`PyCircuitBuilder::build`](crate::builder::PyCircuitBuilder::build),
//! plus the read-only inspection surface the Gherkin scenario
//! `python-frontend#incremental-circuit-construction-via-builder-api`
//! exercises (`element_count`, `node_count`).
//!
//! # Immutability
//!
//! The `#[pyclass(frozen)]` attribute marks the class as immutable at
//! the `PyO3` layer: Python code may not obtain a `&mut self` borrow,
//! so adding `#[pymethods]` that mutate is structurally impossible —
//! `&mut self` receivers would fail to compile. This is the strongest
//! enforcement of ADR-0001's immutable-handle requirement available
//! at the binding boundary. The companion scenario
//! `python-frontend#immutable-circuit-graph-prevents-post-build-mutation`
//! (tasks.md item #54) will add a dedicated `ImmutableHandleError`
//! Python exception class for the "attempted mutation" path; for now
//! such attempts surface as the standard `AttributeError` Python
//! raises when a class lacks the requested method, which is itself a
//! valid "mutation rejected" signal.
//!
//! # Surface decisions (recorded for ADR-0010 callers)
//!
//! - **Opaque storage.** The wrapper stores an owned
//!   `netlist_graph::CircuitGraph` directly. Cloning a `CircuitGraph`
//!   is cheap relative to building it, so `build()` clones the inner
//!   builder state once at finalization (the netlist-graph crate
//!   already does this internally; see ADR-0001 +
//!   `python-frontend#builder-isolation-across-multiple-builds`).
//! - **Lookups return strings, not handle objects.** `node_names` and
//!   `element_names` return `list[str]` rather than dedicated `PyNode`
//!   / `PyElement` wrapper classes. Exposing richer per-entity
//!   wrappers is a downstream UX task (post-#61) once the Gherkin
//!   scenarios drive a need for them — keeping the v1 surface narrow
//!   is required by ADR-0010.
//! - **`__repr__` for diagnostics.** A short `__repr__` is provided so
//!   `print(graph)` from a Python REPL produces something useful.

use netlist_graph::CircuitGraph;
use pyo3::prelude::*;

/// Python class: `circuit_solver.CircuitGraph`.
///
/// Immutable handle returned by
/// [`PyCircuitBuilder::build`](crate::builder::PyCircuitBuilder::build).
/// Read-only — no `#[pymethods]` accept `&mut self`.
#[pyclass(name = "CircuitGraph", module = "circuit_solver", frozen)]
pub struct PyCircuitGraph {
    inner: CircuitGraph,
}

impl PyCircuitGraph {
    /// Wrap an owned `netlist_graph::CircuitGraph`. Crate-private;
    /// Python user code obtains a `PyCircuitGraph` only via
    /// `CircuitBuilder.build()`.
    pub(crate) fn from_inner(inner: CircuitGraph) -> Self {
        Self { inner }
    }

    /// Borrow the underlying graph (for tests and for downstream
    /// `PyO3` layers that need to inspect the resolved graph; not
    /// exposed to Python). Not part of the stable Python surface.
    #[must_use]
    pub fn as_inner(&self) -> &CircuitGraph {
        &self.inner
    }
}

#[pymethods]
impl PyCircuitGraph {
    /// Number of electrical nodes in the graph, ground included.
    ///
    /// Mirrors [`netlist_graph::CircuitGraph::node_count`]. Used by
    /// the Gherkin assertion
    /// *"the `CircuitGraph` contains two elements and three nodes"*.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    /// Number of elements in the graph after subcircuit expansion.
    ///
    /// Mirrors [`netlist_graph::CircuitGraph::element_count`]. Used by
    /// the Gherkin assertion
    /// *"the `CircuitGraph` contains two elements and three nodes"*.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.inner.element_count()
    }

    /// Number of device-model definitions registered on the
    /// originating builder.
    ///
    /// Mirrors [`netlist_graph::CircuitGraph::model_count`].
    #[must_use]
    pub fn model_count(&self) -> usize {
        self.inner.model_count()
    }

    /// All node names in `NodeId` order. Ground appears first under
    /// its canonical net name (`"0"` by default).
    #[must_use]
    pub fn node_names(&self) -> Vec<String> {
        self.inner
            .nodes()
            .iter()
            .map(|n| n.name().to_string())
            .collect()
    }

    /// All element names in insertion / `ElementId` order.
    #[must_use]
    pub fn element_names(&self) -> Vec<String> {
        self.inner
            .elements()
            .iter()
            .map(|e| e.name().as_str().to_string())
            .collect()
    }

    /// True iff the graph contains zero elements. Useful for
    /// asserting in unit tests that an empty builder produces an
    /// empty (but well-formed) graph.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// True iff every element in the graph is a non-subcircuit kind.
    /// `CircuitBuilder.build()` always runs subcircuit expansion
    /// before returning, so this is `True` for every graph
    /// constructible from Python today. Exposed for symmetry with
    /// the Rust-side accessor and for downstream verification.
    #[must_use]
    pub fn is_fully_expanded(&self) -> bool {
        self.inner.is_fully_expanded()
    }

    /// Short diagnostic representation.
    ///
    /// Shape: `CircuitGraph(elements=2, nodes=3, models=0)`. Stable
    /// enough for log scraping but not part of the public contract;
    /// ADR-0010 keeps the `__repr__` surface unstable.
    fn __repr__(&self) -> String {
        format!(
            "CircuitGraph(elements={}, nodes={}, models={})",
            self.inner.element_count(),
            self.inner.node_count(),
            self.inner.model_count(),
        )
    }
}
