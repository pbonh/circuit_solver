# `circuit_solver.CircuitBuilder`

::: source `crates/circuit-solver-py/src/builder.rs`

Incremental construction entry point for building a circuit. Wraps
the upstream `netlist_graph::CircuitBuilder` with the four
declaration methods required by `tasks.md` #52 — `add_element`,
`add_wire`, `add_model`, `add_subcircuit` — plus the terminal
`build()` method owned by #53.

Each `build()` call returns a fresh immutable
[`CircuitGraph`](circuit-graph.md). The builder itself remains
reusable after `build()`; further mutations only affect graphs
returned by *subsequent* `build()` calls. This is the "builder
isolation" invariant from
`python-frontend#builder-isolation-across-multiple-builds`
(`tasks.md` #55).

## Surface decisions

The decisions below are recorded for ADR-0010 callers — they
constrain what the v1 Python API can change without breaking
downstream code.

- **Kind is a SPICE-letter string.** `add_element` and the body
  dicts of `add_subcircuit` carry the element kind as the short
  tag string `"R"`, `"C"`, `"L"`, `"V"`, `"I"`, or `"DEV"`. There
  is no Python-side `ElementKind` enum wrapper, no per-variant
  factory function. A richer kind API
  (e.g. `circuit_solver.Resistor(1e3)`) is a later UX task.
- **Subcircuit bodies are `list[dict]`.** `add_subcircuit` accepts
  a list of dicts (`{"name", "kind", "terminals", "value"?,
  "model"?}`) rather than exposing a Python
  `SubcircuitDefinition` / `ElementDecl` class. This minimises
  surface that ADR-0010 must keep unstable.
- **Chaining returns `None`.** The Rust builder returns `&mut Self`
  for fluent chaining; Python idiom is `None`, so the bindings
  discard the chain result. Method chaining in Python is
  reconstructed by re-assigning the same builder reference.
- **Mutability and the GIL.** `CircuitBuilder` is a mutable
  `#[pyclass]` whose methods take `&mut self` directly under the
  GIL. No `RefCell` / `Mutex` is needed at the binding boundary.

---

## `__init__()` { #init }

Construct an empty `CircuitBuilder`.

```python
from circuit_solver import CircuitBuilder
b = CircuitBuilder()
```

The builder is mutable until you call `build()`. There is no
configuration; per-element parameters are supplied at `add_element`
time.

---

## `add_element(name, kind, terminals, value=None, model=None) -> None` { #add_element }

Register an element instance.

**Arguments**

- `name` (`str`) — user-facing element name (e.g. `"R1"`). Must be
  unique within this builder; duplicates raise
  [`CircuitBuilderError`](exceptions.md#circuitbuildererror).
- `kind` (`str`) — SPICE-letter tag identifying the element kind.
  The currently-accepted tags are:
    - `"R"` — resistor
    - `"C"` — capacitor
    - `"L"` — inductor
    - `"V"` — independent voltage source
    - `"I"` — independent current source
    - `"DEV"` — model-resolved semiconductor; pair with
      `model=...`.

  The `"X"` subcircuit-instance tag is **not** accepted here — use
  the dedicated `add_subcircuit_instance` entry point (a later
  task; not exposed yet).
- `terminals` (`list[str]`) — ordered list of net names this
  element connects to. Two-terminal kinds (`R`, `C`, `L`, `V`, `I`)
  require exactly two terminals; `DEV` accepts any count (the
  device-modeling crate later validates model-specific arity).
- `value` (`float`, optional) — numeric value parameter the kind
  expects (resistance in Ω for `R`, capacitance in F for `C`,
  voltage in V for `V`, etc.). Required for `R`, `C`, `L`, `V`,
  `I`. Ignored for `DEV`. Defaults to `0.0` if omitted for
  two-terminal kinds — downstream stamp generation will error out
  on the zero value, but the builder does not enforce non-zero
  values at this layer.
- `model` (`str`, optional) — model-name reference resolved by the
  device-modeling context. Pass for `DEV` elements; ignored for the
  linear kinds.

**Raises**

- [`CircuitBuilderError`](exceptions.md#circuitbuildererror) — if
  the underlying Rust builder rejects the element (duplicate name,
  terminal arity mismatch).
- `TypeError` — if `kind` is not one of the recognised SPICE-letter
  tags, or if a `DEV` element is registered without `model=...`.

---

## `add_wire(a, b) -> None` { #add_wire }

Declare that two net names refer to the same electrical node.

**Arguments**

- `a` (`str`), `b` (`str`) — net names. Order is irrelevant;
  subsequent `add_wire`s reuse the same disjoint-set union-find
  that `build()` consults to assign `NodeId`s.

---

## `add_model(name) -> None` { #add_model }

Register a device-model name.

**Arguments**

- `name` (`str`) — model name to register. Registering the same
  name twice is a no-op (the Rust builder dedupes silently). The
  string is later resolved by the `device-modeling` crate against
  its `ModelName → DeviceModel` registry.

---

## `add_subcircuit(name, ports, body) -> None` { #add_subcircuit }

Register a subcircuit definition.

**Arguments**

- `name` (`str`) — subcircuit definition name (e.g. `"INV"`). Must
  be unique within this builder.
- `ports` (`list[str]`) — ordered list of external port net-names.
  Bind to parent-scope nets at instantiation time (handled by the
  later `add_subcircuit_instance` task).
- `body` (`list[dict]`) — list of element-declaration dicts. Each
  dict has the same keys as
  [`add_element`](#add_element)
  (`name`, `kind`, `terminals`, optional `value`, optional
  `model`). The body is stored verbatim and replayed against the
  parent net namespace during expansion at `build()` time.

**Raises**

- [`CircuitBuilderError`](exceptions.md#circuitbuildererror) — if
  the subcircuit name collides with a previously-registered
  definition.
- `TypeError` — if any body dict is malformed (missing required
  key, wrong type, unrecognised `kind`).

---

## `element_decl_count() -> int` { #element_decl_count }

Number of top-level element declarations recorded so far. Legacy
inspection helper retained from the `tasks.md` #52-only era —
exposes the count of top-level element *declarations*
(pre-subcircuit-expansion) for the #52-era delegation tests. New
code that wants post-expansion counts should prefer
`builder.build().element_count()`; the two counts diverge whenever
subcircuit instantiation multiplies declarations.

---

## `build() -> CircuitGraph` { #build }

Finalize the build: expand subcircuits, resolve wire equivalences,
assign `NodeId`s, and return an immutable
[`CircuitGraph`](circuit-graph.md).

Per [ADR-0001] and scenario
`python-frontend#builder-isolation-across-multiple-builds`, each
call returns a fresh `CircuitGraph` that does not share storage
with previously-built handles; the builder itself remains usable,
and further mutations only affect graphs returned by *subsequent*
`build()` calls.

**Raises**

- [`CircuitBuilderError`](exceptions.md#circuitbuildererror) — if
  subcircuit expansion fails (unknown subcircuit reference,
  port-arity mismatch, expansion cycle). Linear-element validation
  (duplicate names, terminal arity) is performed eagerly by
  `add_element`, so `build()` itself never fails on those.

[ADR-0001]: ../../../../wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph.md
