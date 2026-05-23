# Exceptions

::: source `crates/circuit-solver-py/src/errors.rs`

Three exception classes are exported by `circuit_solver`. All three
derive from CPython's built-in `Exception` (`PyException`). The
inheritance tree is intentionally flat — there is no common base
class today, because the three errors arise from disjoint failure
modes and downstream `except` clauses should be specific. Further
task-driven refinement of the taxonomy is tracked under `tasks.md`
#58 (Python error mapping).

| Class                   | Source of raise                                                  | Scenario(s)                                                                |
|-------------------------|------------------------------------------------------------------|----------------------------------------------------------------------------|
| `CircuitBuilderError`   | `CircuitBuilder` methods (mutation rejected by netlist-graph)    | `incremental-circuit-construction-via-builder-api`                         |
| `ImmutableHandleError`  | `CircuitGraph` trap methods (mutation on a built handle)         | `immutable-circuit-graph-prevents-post-build-mutation`                     |
| `NetlistParseError`     | `parse_netlist` (unrecognised SPICE device letter)               | `spice-netlist-file-parsing`, `error-on-malformed-netlist`                 |

## `CircuitBuilderError`

```python
class CircuitBuilderError(Exception): ...
```

Raised by `CircuitBuilder` Python methods when the underlying
`netlist_graph::CircuitBuilder` rejects an operation. Carries the
`Display` impl of the originating `NetlistGraphError` as its
message — a stable contract owned by the `netlist-graph` crate.

Conditions that raise this exception:

- Duplicate element name on
  [`add_element`](circuit-builder.md#add_element).
- Terminal-arity mismatch (e.g. a resistor with three terminals).
- Duplicate subcircuit name on
  [`add_subcircuit`](circuit-builder.md#add_subcircuit).
- Unknown subcircuit reference, port-arity mismatch, or expansion
  cycle on [`build()`](circuit-builder.md#build).

Example:

```python
from circuit_solver import CircuitBuilder, CircuitBuilderError

b = CircuitBuilder()
b.add_element("R1", "R", ["a", "b"], value=1e3)
try:
    b.add_element("R1", "R", ["a", "b"], value=2e3)
except CircuitBuilderError as e:
    print(e)  # "duplicate element name 'R1'"
```

## `ImmutableHandleError`

```python
class ImmutableHandleError(Exception): ...
```

Raised when Python code attempts to invoke a builder-mutation method
(`add_element`, `add_wire`, `add_model`, `add_subcircuit`) on an
already-built, immutable `CircuitGraph` handle. The handle returned
by `CircuitBuilder.build()` is frozen per [ADR-0001]; mutation must
be performed on a fresh `CircuitBuilder` instance and a new graph
produced via `build()`.

The message names the attempted method and the invariant violated
so the user can locate the misuse in their Python source without
consulting the docs.

This exception is the typed, actionable alternative to the bare
`AttributeError` Python would otherwise raise if the trap methods
were not present. See
[`CircuitGraph` § Immutability and trap methods](circuit-graph.md#immutability-and-trap-methods)
for the structural setup.

Example:

```python
from circuit_solver import CircuitBuilder, ImmutableHandleError

b = CircuitBuilder()
b.add_element("R1", "R", ["a", "b"], value=1e3)
g = b.build()
try:
    g.add_element("R2", "R", ["a", "b"], value=2e3)  # type: ignore[call-arg]
except ImmutableHandleError as e:
    print(e)
    # "`CircuitGraph.add_element` is not callable: a `CircuitGraph`
    # returned by `CircuitBuilder.build()` is immutable (ADR-0001).
    # To add elements, construct a fresh `CircuitBuilder` and call
    # `build()` again."
```

## `NetlistParseError`

```python
class NetlistParseError(Exception): ...
```

Raised by [`circuit_solver.parse_netlist`](parse-netlist.md) when
the input SPICE deck contains an unrecognised device letter (i.e.
the leading character of an element card is not one of `R`, `C`,
`L`, `V`, `I`, `D`, `Q`, `M`, `X`). The exception message
identifies the 1-indexed source line number and the unrecognised
token (the element-name token whose first character was the
unknown letter).

Lights up the `python-frontend#error-on-malformed-netlist` Gherkin
scenario (`tasks.md` #61):

```gherkin
Given CircuitDesigner has a SPICE netlist with an unsupported device letter
When CircuitDesigner calls circuit_solver.parse_netlist(path)
Then a NetlistParseError is raised
And the error message identifies the line number and the unrecognized token
```

Example:

```python
from circuit_solver import parse_netlist, NetlistParseError

# A netlist with a 'Z' card on line 3:
#   R1 a b 1k
#
#   Z1 c d
try:
    parse_netlist("malformed.cir")
except NetlistParseError as e:
    print(e)
    # "line 3: unrecognised SPICE device letter 'Z' on token 'Z1';
    # expected one of R, C, L, V, I, D, Q, M, X"
```

The `line N:` prefix is added by a parser-side `annotate_with_line`
wrapper that preserves the exception type while enriching the
message — see the source for details. Callers catching
`NetlistParseError` can rely on the line-number prefix being
present.

## Why three classes (not one, not many)

ADR-0001 and the v1 Gherkin scenarios consciously chose a minimal
taxonomy:

- **Three concrete classes** so user code can `except` on the
  failure shape it actually wants to handle (a malformed builder
  call, a mutation on a frozen handle, a parse-time deck error).
- **Flat hierarchy** — no abstract base class — because the
  failure modes are disjoint and a shared base would invite
  catch-everything error handling.
- **Stable Display-string messages** — the message text comes from
  the originating Rust error's `Display` impl (a stable contract of
  the underlying crate) so Gherkin scenarios can assert on message
  substrings without coupling to internal field layout.

Further taxonomy refinement (`tasks.md` #58) may add more exception
classes; the existing three are forward-compatible.

[ADR-0001]: ../../../wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph.md
