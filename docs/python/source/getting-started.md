# Getting started

The Python frontend is a CPython extension module produced by the
[`circuit-solver-py`][crate] crate via [PyO3]. There are two ways to
import it today:

1. From a `maturin develop`-built workspace (the developer path).
2. From a `maturin build`-produced wheel (the distribution path,
   wired up under [tasks.md #72] — not the subject of this
   reference).

This page assumes you have an importable `circuit_solver` module on
your `PYTHONPATH`; see [Building this site](building.md) for the
maturin commands.

## Five-minute walkthrough

The four moving parts you need are `CircuitBuilder`, `CircuitGraph`,
`parse_netlist`, and `AnalysisRequest`. The first three light up the
"how do I describe a circuit?" question; the fourth describes "what
do I want to compute?". The "kick off the simulation and read
results" entry point is **not** part of v1.0.0 and is not documented
here.

### 1. Build a circuit incrementally

```python
from circuit_solver import CircuitBuilder

b = CircuitBuilder()

# A 1 kΩ resistor between nets "in" and "out".
b.add_element("R1", "R", ["in", "out"], value=1e3)

# A 1 µF capacitor between "out" and "gnd".
b.add_element("C1", "C", ["out", "gnd"], value=1e-6)

# A 5 V DC source between "in" and "gnd".
b.add_element("V1", "V", ["in", "gnd"], value=5.0)

graph = b.build()  # → frozen CircuitGraph
```

The `CircuitGraph` returned by `build()` is immutable. The builder
itself is reusable: calling `build()` a second time yields a fresh
graph; calling `add_element` between two `build()` calls only
affects the graph returned by the *second* call. This is the
"builder isolation" invariant from
`python-frontend#builder-isolation-across-multiple-builds`
(`tasks.md` #55).

### 2. Inspect the graph

```python
graph.element_count()       # → 3
graph.node_count()          # → 3 ("in", "out", "gnd")
graph.element_names()       # → ["R1", "C1", "V1"]
graph.node_names()          # → ["in", "out", "gnd"] (order unspecified)
graph.is_empty()            # → False
graph.is_fully_expanded()   # → True (no unexpanded subcircuits)
```

### 3. Parse a SPICE deck

If you already have a SPICE-format netlist on disk, parse it
directly into a `CircuitGraph`:

```python
from circuit_solver import parse_netlist

graph = parse_netlist("rc_filter.cir")
```

The result is identical to one built incrementally with the same
topology — this is the equivalence guarantee from
`python-frontend#spice-netlist-file-parsing` (`tasks.md` #60).

If the deck contains an unrecognised device letter (i.e. the leading
character of an element card is not one of `R`, `C`, `L`, `V`, `I`,
`D`, `Q`, `M`, `X`), the call raises
[`NetlistParseError`](reference/exceptions.md#netlistparseerror)
with the 1-indexed source line number and the offending token.

### 4. Describe what you want to compute

```python
from circuit_solver import AnalysisRequest

# A DC operating-point request — no sweep, default integration.
req = AnalysisRequest("dc-operating-point")

# An AC small-signal sweep, log-spaced 1 Hz to 1 MHz, 401 points.
ac = AnalysisRequest(
    "ac-small-signal",
    sweep=(1.0, 1e6, 401, "decade"),
)

# A transient run with explicit integration choice and ZOH
# analog/digital boundary (the default per ADR-0007).
tran = AnalysisRequest(
    "transient-time-domain",
    integration_method="trapezoidal",
    boundary_interpolation="zero_order_hold",
)
```

`AnalysisRequest` is `frozen` and stores normalised string slugs.
Submission and result retrieval (`Simulator.run` → `Result`) is
**not** part of v1.0.0 — that landing strip is `tasks.md` #57+.

## Common gotchas

- **Calling a builder-mutation method on a `CircuitGraph` raises
  [`ImmutableHandleError`](reference/exceptions.md#immutablehandleerror).**
  This is by design: the graph is the post-`build()` snapshot, and
  attempting to mutate it surfaces as a typed, actionable error
  rather than a bare `AttributeError`.
- **`add_element("X", ...)` is rejected.** The `"X"` SPICE-letter
  tag is reserved for subcircuit *instances*, which are constructed
  through a dedicated entry point that is not yet exposed. Use
  `add_subcircuit` to register a definition and instantiate it via
  a later task's API.
- **`DEV` requires `model=...`.** Other linear kinds (`R`, `C`, `L`,
  `V`, `I`) take a numeric `value`. Mixing them up raises
  `TypeError`.
- **`sweep` presence must match the analysis type.**
  `dc_sweep`, `ac`, and `noise` require a sweep; `dc_op`,
  `transient`, and `mixed_signal` reject one. Mismatches raise
  `ValueError`.

[crate]: https://github.com/pbonh/circuit_solver/tree/main/crates/circuit-solver-py
[PyO3]: https://pyo3.rs
[tasks.md #72]: ../../openspec/changes/circuit-solver-2026-05-21-v1-spec/tasks.md
