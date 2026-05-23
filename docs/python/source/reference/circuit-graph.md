# `circuit_solver.CircuitGraph`

::: source `crates/circuit-solver-py/src/graph.rs`

The immutable handle returned by
[`CircuitBuilder.build()`](circuit-builder.md#build)
and [`circuit_solver.parse_netlist`](parse-netlist.md). The Python
class is declared `#[pyclass(frozen)]`, which makes adding any
mutating `#[pymethods]` structurally impossible — `&mut self`
receivers would fail to compile. This is the strongest enforcement
of [ADR-0001]'s immutable-handle requirement available at the
binding boundary.

## Construction

`CircuitGraph` is **not directly constructible** from Python; it can
only be obtained from:

- [`CircuitBuilder.build()`](circuit-builder.md#build)
- [`circuit_solver.parse_netlist(path)`](parse-netlist.md)

Attempting `CircuitGraph()` raises `TypeError` (no `__new__`
exposed).

## Read-only accessors

| Method                  | Returns        | Description                                                                 |
|-------------------------|----------------|-----------------------------------------------------------------------------|
| `node_count()`          | `int`          | Number of electrical nodes in the graph, ground included.                   |
| `element_count()`       | `int`          | Number of elements after subcircuit expansion.                              |
| `model_count()`         | `int`          | Number of device-model definitions registered on the originating builder.   |
| `node_names()`          | `list[str]`    | All node names in `NodeId` order. Ground appears first under its canonical net name (`"0"` by default). |
| `element_names()`       | `list[str]`    | All element names in insertion / `ElementId` order.                         |
| `is_empty()`            | `bool`         | True iff the graph contains zero elements.                                  |
| `is_fully_expanded()`   | `bool`         | True iff every element in the graph is a non-subcircuit kind. Always `True` for graphs constructible from Python today, since `build()` runs subcircuit expansion before returning. |

`__repr__` returns a short diagnostic of the form
`CircuitGraph(elements=2, nodes=3, models=0)`. Stable enough for log
scraping but **not** part of the public contract — [ADR-0010] keeps
the `__repr__` surface unstable.

## Immutability and trap methods

The four builder-mutation method *names* (`add_element`,
`add_wire`, `add_model`, `add_subcircuit`) are present on
`CircuitGraph` as **trap methods**: each exists so Python attribute
lookup succeeds, but the body unconditionally raises
[`ImmutableHandleError`](exceptions.md#immutablehandleerror).

The trap signature is `(*args, **kwargs)` so any call shape resolves
to the trap before argument-type checking can produce a `TypeError`
that would obscure the real diagnostic. The intent is that an
attempted mutation surfaces as a *typed, actionable* error rather
than the bare `AttributeError` the missing-method path would
otherwise produce.

This lights up the Gherkin scenario
`python-frontend#immutable-circuit-graph-prevents-post-build-mutation`
(`tasks.md` #54).

### Example

```python
from circuit_solver import CircuitBuilder, ImmutableHandleError

b = CircuitBuilder()
b.add_element("R1", "R", ["a", "b"], value=1e3)
g = b.build()

try:
    g.add_element("R2", "R", ["a", "b"], value=2e3)  # type: ignore[call-arg]
except ImmutableHandleError as e:
    print(e)  # "CircuitGraph is immutable: add_element is not callable on a built handle ..."
```

The error message names the method that was attempted and points
the reader at the builder pattern. See
[`ImmutableHandleError`](exceptions.md#immutablehandleerror) for the
exception class.

[ADR-0001]: ../../../../wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph.md
[ADR-0010]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md
