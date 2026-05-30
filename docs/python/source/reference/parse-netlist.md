# `circuit_solver.parse_netlist`

::: source `crates/circuit-solver-py/src/lib.rs`, `crates/circuit-solver-py/src/parser.rs`

```python
circuit_solver.parse_netlist(path) -> CircuitGraph
```

Parse a SPICE-format netlist file from disk and return an immutable
[`CircuitGraph`](circuit-graph.md). Implements `tasks.md` #60 — the
Python-facing entry point for the
`python-frontend#spice-netlist-file-parsing` Gherkin scenario.

### Arguments

- `path` (`str | os.PathLike`) — path to a SPICE netlist file. The
  conversion to a Rust `PathBuf` is handled by PyO3's `FromPyObject`
  derivation; both `str` and `pathlib.Path` work.

### Returns

A [`CircuitGraph`](circuit-graph.md) constructed the same way
[`CircuitBuilder`](circuit-builder.md) would build it incrementally.
The Gherkin scenario explicitly asserts this equivalence:

```gherkin
Given CircuitDesigner has a SPICE netlist file on disk
When CircuitDesigner calls circuit_solver.parse_netlist(path)
Then the returned object is a CircuitGraph
And the CircuitGraph contains all elements, models, and
    subcircuits declared in the netlist
And the CircuitGraph is identical to one built incrementally
    with the same topology
```

### Raises

- `IOError` — if the file cannot be read.
- [`NetlistParseError`](exceptions.md#netlistparseerror) — if a
  card's leading character is not one of the recognised SPICE
  device letters (`R`, `C`, `L`, `V`, `I`, `D`, `Q`, `M`, `X`).
  The message identifies the 1-indexed line number and the
  unrecognised token, per `tasks.md` #61 and the
  `python-frontend#error-on-malformed-netlist` Gherkin scenario.
- `ValueError` — if a line is malformed in a way other than the
  unrecognised-device-letter case (wrong arity, missing model
  name, malformed numeric value, unterminated `.SUBCKT`, etc.).
  The broader Python-error-mapping refactor that may migrate
  these onto the structured taxonomy is `tasks.md` #58.
- [`CircuitBuilderError`](exceptions.md#circuitbuildererror) — if
  the resulting builder-replay sequence is rejected by the
  underlying `netlist-graph` builder (duplicate element names,
  unknown subcircuit references, port-arity mismatches, expansion
  cycles).

## Supported SPICE subset (v1)

The parser implements an intentionally minimal subset of SPICE —
enough to light up the v1 Gherkin scenarios and the conformance
harness. The accepted device letters are:

| Letter | Element kind                          | Builder tag |
|--------|---------------------------------------|-------------|
| `R`    | resistor                              | `"R"`       |
| `C`    | capacitor                             | `"C"`       |
| `L`    | inductor                              | `"L"`       |
| `V`    | independent voltage source            | `"V"`       |
| `I`    | independent current source            | `"I"`       |
| `D`    | diode (model-resolved semiconductor)  | `"DEV"`     |
| `Q`    | BJT (model-resolved semiconductor)    | `"DEV"`     |
| `M`    | MOSFET (model-resolved semiconductor) | `"DEV"`     |
| `X`    | subcircuit instance                   | (internal)  |

Anything else on a non-blank, non-comment line raises
[`NetlistParseError`](exceptions.md#netlistparseerror).

`.SUBCKT` / `.ENDS` blocks and `.MODEL` cards are recognised. Numeric
values accept the standard SPICE engineering suffixes (`k`, `meg`,
`m`, `u`, `n`, `p`, `f`).

## Equivalence with `CircuitBuilder`

Behind the scenes, `parse_netlist` constructs a fresh
`CircuitBuilder`, replays the parsed declarations through its
`add_element` / `add_wire` / `add_model` / `add_subcircuit` calls,
then invokes `build()`. The "parsed graph equals incrementally-built
graph" assertion in the Gherkin scenario is therefore a structural
invariant, not a fragile test.

The architectural rationale: any divergence between "parsed graph"
and "incrementally-built graph" can only arise from the parser
sending different `add_*` calls, not from a parallel construction
path. The per-test equivalence harness inside `parser.rs` asserts
this directly.

## Example

Given a file `rc.cir`:

```text
* simple RC lowpass
R1 in out 1k
C1 out 0 1u
V1 in 0 5
.END
```

```python
from circuit_solver import parse_netlist

graph = parse_netlist("rc.cir")
graph.element_count()  # → 3
graph.node_names()     # → ["0", "in", "out"]  (order: ground first)
graph.element_names()  # → ["R1", "C1", "V1"]  (order: insertion)
```
