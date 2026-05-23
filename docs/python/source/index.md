# circuit_solver — Python API reference

`circuit_solver` is the Python frontend to the
[circuit-solver][upstream] mixed-signal SPICE engine. The module is
provided by the `circuit-solver-py` Rust crate via [PyO3] and is
loaded into `CPython` as a native extension module named
`circuit_solver` (PEP-8 lowercase, distinct from the Rust crate
name).

The Python surface for **v1.0.0** is intentionally narrow: an
incremental builder, an immutable graph handle returned by the
builder, an immutable value object describing a requested analysis,
and a SPICE-file parser. The submission entry point that consumes an
`AnalysisRequest` plus a `CircuitGraph` and returns a `Result` is a
later task (`tasks.md` #57 onwards) and is **not** documented here.

## Stability

Per [ADR-0010][adr-0010] the public Rust API is **unstable** at
v1.0.0; per [ADR-0001][adr-0001] the Python-facing surface is the
in-process PyO3 binding with an immutable `CircuitGraph` handle. The
contract documented on these pages is the stability surface — see
[Stability policy](stability.md) for what that buys you.

## Surface as of `tasks.md` items #52–#56, #60, #61

| Symbol                         | Kind                | Source                  |
|--------------------------------|---------------------|-------------------------|
| `circuit_solver.CircuitBuilder`| class               | `builder.rs`            |
| `circuit_solver.CircuitGraph`  | class (frozen)      | `graph.rs`              |
| `circuit_solver.AnalysisRequest`| class (frozen)     | `analysis_request.rs`   |
| `circuit_solver.parse_netlist` | function            | `lib.rs` / `parser.rs`  |
| `circuit_solver.CircuitBuilderError`| exception      | `errors.rs`             |
| `circuit_solver.ImmutableHandleError`| exception     | `errors.rs`             |
| `circuit_solver.NetlistParseError`| exception         | `errors.rs`             |

The rest of `tasks.md`'s Python items (`Result`, `Simulator`, NumPy
result arrays, GIL release, error-taxonomy mapping) are scheduled
under #57–#59 and #62. They are **not** part of this reference.

## Where to go next

- [Getting started](getting-started.md) — five-minute walkthrough.
- [Reference](reference/index.md) — class-by-class, method-by-method.
- [Stability policy](stability.md) — what callers may depend on.
- [Building this site](building.md) — `mkdocs serve` instructions.

[upstream]: https://github.com/pbonh/circuit_solver
[PyO3]: https://pyo3.rs
[adr-0010]: ../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md
[adr-0001]: ../../../wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph.md
