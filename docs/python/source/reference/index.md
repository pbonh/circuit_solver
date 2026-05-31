# Reference — overview

This section is the **authoritative Python API reference** for the
v1.0.0 surface. Each page mirrors the `///` docstring content in
`crates/circuit-solver-py/src/`; the source-of-truth is the Rust
docstring, but this site renders it without requiring a
`maturin develop`-built extension to be importable.

## Module layout

The `circuit_solver` extension module exposes exactly seven names:

| Name                  | Page                                            |
|-----------------------|-------------------------------------------------|
| `CircuitBuilder`      | [CircuitBuilder](circuit-builder.md)            |
| `CircuitGraph`        | [CircuitGraph](circuit-graph.md)                |
| `AnalysisRequest`     | [AnalysisRequest](analysis-request.md)          |
| `parse_netlist`       | [parse_netlist](parse-netlist.md)               |
| `CircuitBuilderError` | [Exceptions](exceptions.md#circuitbuildererror) |
| `ImmutableHandleError`| [Exceptions](exceptions.md#immutablehandleerror)|
| `NetlistParseError`   | [Exceptions](exceptions.md#netlistparseerror)   |

Anything not listed above is **not** part of the v1.0.0 contract.
Per [ADR-0010] the Rust API is unstable; per [ADR-0001] the Python
binding is the stability contract callers may depend on. See the
[Stability policy](../stability.md) for what that buys you.

## Mapping to spec scenarios

| Symbol                  | Scenario(s)                                                                            |
|-------------------------|----------------------------------------------------------------------------------------|
| `CircuitBuilder`        | `python-frontend#incremental-circuit-construction-via-builder-api` (`tasks.md` #52/#53)|
| `CircuitGraph` (frozen) | `python-frontend#immutable-circuit-graph-prevents-post-build-mutation` (#54)           |
| `CircuitBuilder.build`  | `python-frontend#builder-isolation-across-multiple-builds` (#55)                       |
| `AnalysisRequest`       | (value object only — submission scenario is `tasks.md` #57+)                           |
| `parse_netlist`         | `python-frontend#spice-netlist-file-parsing` (#60)                                     |
| `NetlistParseError`     | `python-frontend#error-on-malformed-netlist` (#61)                                     |

[ADR-0010]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md
[ADR-0001]: ../../../../wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph.md
