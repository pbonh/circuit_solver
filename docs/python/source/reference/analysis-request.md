# `circuit_solver.AnalysisRequest`

::: source `crates/circuit-solver-py/src/analysis_request.rs`

Immutable value object describing a requested analysis. Constructed
ahead of submission; held by the future `Simulator.run` entry point
(`tasks.md` #57+, not yet exposed) to dispatch the appropriate
analysis kernel.

`AnalysisRequest` is `#[pyclass(frozen)]` — fields are read-only,
exposed via `@getter`s, and the class has no setters. There is no
`__eq__` / `__hash__` today; equality and hashing on `AnalysisRequest`
are **not** part of the v1 contract.

## Fields

| Field                    | Type                       | Meaning                                                                                       |
|--------------------------|----------------------------|-----------------------------------------------------------------------------------------------|
| `analysis_type`          | `str`                      | Canonical analysis-type slug.                                                                  |
| `sweep`                  | `tuple | None`             | `(start, stop, points, scale)` or `None`.                                                     |
| `integration_method`     | `str`                      | `"backward_euler"`, `"trapezoidal"`, or `"gear2"`.                                             |
| `boundary_interpolation` | `str`                      | `"zero_order_hold"` or `"linear"` per [ADR-0007].                                              |

## `__init__(analysis_type, sweep=None, integration_method=None, boundary_interpolation=None)`

Construct a new `AnalysisRequest`.

### Arguments

- `analysis_type` (`str`) — analysis-type slug. Canonical values:
    - `"dc-operating-point"`
    - `"dc-sweep"`
    - `"ac-small-signal"`
    - `"transient-time-domain"`
    - `"noise-spectral-density"`
    - `"mixed-signal-cosim"`

  Friendlier short-form aliases are also accepted:
  `"dc_op"` / `"dc"`, `"dc_sweep"`, `"ac"`, `"transient"` / `"tran"`,
  `"noise"`, `"mixed_signal"` / `"mixed"`. Whichever you pass, the
  `analysis_type` getter returns the canonical slug.
- `sweep` (`tuple | list | None`, optional) — `(start, stop, points,
  scale)` 4-tuple of `(float, float, int, str)`. `scale` is
  `"linear"` or `"log"`. Required for sweeping analyses
  (`dc-sweep`, `ac-small-signal`, `noise-spectral-density`); must
  be `None` for single-point kinds (`dc-operating-point`,
  `transient-time-domain`, `mixed-signal-cosim`). Construction
  raises `ValueError` if this rule is violated.
- `integration_method` (`str`, optional) — one of
  `"backward_euler"`, `"trapezoidal"`, or `"gear2"`. Defaults to
  `"trapezoidal"` per the design's "Trapezoidal ringing" section.
  Only meaningful for time-domain analyses (transient,
  mixed-signal); accepted but inert for other kinds so
  default-constructed requests have a sensible value.
- `boundary_interpolation` (`str`, optional) —
  `"zero_order_hold"` (default) or `"linear"` per [ADR-0007]. Only
  meaningful for mixed-signal; accepted but inert for other kinds
  for the same reason.

### Raises

- `TypeError` — if `analysis_type`, `integration_method`, or
  `boundary_interpolation` is not one of the recognised tags, or
  if `sweep` is the wrong shape (not a 4-element iterable, wrong
  element types).
- `ValueError` — if `sweep.start` or `sweep.stop` is not finite,
  `sweep.points == 0`, `sweep.scale` is not `"linear"` or
  `"log"`, or the `sweep` presence does not match the analysis
  type's sweep requirement.

### Examples

```python
from circuit_solver import AnalysisRequest

# Single-point DC operating point (no sweep, defaults everything).
op = AnalysisRequest("dc-operating-point")
op.analysis_type            # → "dc-operating-point"
op.sweep                    # → None
op.integration_method       # → "trapezoidal"
op.boundary_interpolation   # → "zero_order_hold"

# AC small-signal sweep, log-spaced 1 Hz to 1 MHz, 401 points.
ac = AnalysisRequest("ac", sweep=(1.0, 1e6, 401, "log"))
ac.analysis_type            # → "ac-small-signal" (canonical slug)
ac.sweep                    # → (1.0, 1e6, 401, "log")

# Mixed-signal with linear (non-conserving) boundary interpolation.
ms = AnalysisRequest(
    "mixed-signal-cosim",
    integration_method="gear2",
    boundary_interpolation="linear",
)
```

## Getters

- `analysis_type → str` — the canonical analysis-type slug, even
  when the constructor received a short-form alias.
- `sweep → tuple[float, float, int, str] | None` — the sweep
  parameters reconstructed as a fresh tuple, or `None`. The fourth
  element is always the canonical lowercase tag (`"linear"` or
  `"log"`).
- `integration_method → str` — `"backward_euler"`,
  `"trapezoidal"`, or `"gear2"`. Always populated.
- `boundary_interpolation → str` — `"zero_order_hold"` or
  `"linear"`. Always populated.

## `__repr__`

Returns a short diagnostic of the form

```
AnalysisRequest(type=ac-small-signal, sweep=(1.0, 1000000.0, 401, "log"),
                integration=trapezoidal, boundary=zero_order_hold)
```

Stable enough for log scraping but **not** part of the public
contract — [ADR-0010] keeps the `__repr__` surface unstable.

[ADR-0007]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0007-zero-order-hold-analog-digital-boundary.md
[ADR-0010]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md
