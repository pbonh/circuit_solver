# Stability policy

This page is the authoritative answer to "what may I depend on?"
for the v1.0.0 Python API.

## TL;DR

- **The Python surface** documented in [Reference](reference/index.md)
  is the contract.
- **The Rust API** of `circuit-solver-py` and the upstream
  `circuit-solver-*` crates is **unstable** per [ADR-0010] —
  treat it as a moving target.
- The Python surface evolves under SemVer-with-deprecation; the
  Rust API evolves freely.

## What is stable

The following are part of the v1.0.0 contract:

| Surface                                                             | Stability                                                                 |
|---------------------------------------------------------------------|---------------------------------------------------------------------------|
| `circuit_solver.CircuitBuilder` class and the five methods documented in [Reference](reference/circuit-builder.md) | Stable. Renames, signature changes, and additions to required arguments are breaking changes that require a major-version bump and a deprecation cycle. |
| `circuit_solver.CircuitGraph` class and the seven read-only accessors documented in [Reference](reference/circuit-graph.md) | Stable.                                                                   |
| `circuit_solver.CircuitGraph` trap-methods *raising* `ImmutableHandleError` | Stable (the *behaviour* is the contract; the trap-method *names* must match the builder).         |
| `circuit_solver.AnalysisRequest` class and its four getters         | Stable.                                                                   |
| `circuit_solver.parse_netlist(path)` signature and return type      | Stable.                                                                   |
| `circuit_solver.CircuitBuilderError`, `ImmutableHandleError`, `NetlistParseError` exception **types** | Stable. They derive from `Exception` today and may grow more-specific bases in a future minor version (additive). |
| Exception **messages** for variants the Gherkin scenarios assert on | Stable. Messages may grow longer; assertion substrings retained.          |

## What is *not* stable

| Surface                                       | Why it's not stable                                                                                                                                |
|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| Any `pub fn` on the Rust `PyCircuitBuilder` / `PyCircuitGraph` / `PyAnalysisRequest` types | Per [ADR-0010] the public Rust API is unstable at v1.0.0. Do not import these crates as Rust dependencies expecting source-compat. |
| `__repr__` strings on any class                | Useful for log scraping; not part of the contract.                                                                                                |
| Internal type stubs (none shipped yet)        | If a `.pyi` lands in a future task it will be additive.                                                                                            |
| Submission entry points (`Simulator.run`, `Result`, NumPy arrays, etc.) | Not yet implemented; landing strip is `tasks.md` #57 onwards. Anything anyone tells you about it today is speculative.                       |
| The set of supported SPICE device letters in `parse_netlist` | Currently `R, C, L, V, I, D, Q, M, X`. May grow additively (e.g. `K` mutual-inductance) in a minor version. Reductions require a major bump. |

## Versioning model

The project follows **SemVer** on the *Python* surface:

- **Major** — breaking change to a stable surface. Cycle requires a
  deprecation warning in a prior minor.
- **Minor** — additive change. New methods, new exception classes,
  new analysis types, new accepted strings.
- **Patch** — fixes that preserve documented behaviour, including
  performance improvements and clarified error messages.

## Deprecation policy

When a stable surface needs to change:

1. The change is announced in a release-note entry and the affected
   symbol gets a `DeprecationWarning` (or, for class-level changes,
   a `__init_subclass__`-style guard).
2. The deprecation persists for **at least one full minor version**.
3. The breaking change ships in the next major release.

There are no deprecations in v1.0.0 — this section describes the
policy that will apply once v1.x evolves.

## Discovery: how do I find what's stable?

- Anything imported via `from circuit_solver import X` is stable
  iff `X` appears in [Reference](reference/index.md).
- Anything reachable via `circuit_solver.<x>` but not documented is
  **not** stable. Notable example: there is no `_internal` module
  shipped today, but private-looking names that happen to be
  accessible should not be relied upon.

## Why this policy

- ADR-0001 chose an in-process PyO3 binding precisely to make the
  Python surface the canonical user interface. Stability lives
  here.
- ADR-0010 keeps the Rust API unstable so the v1 workspace can
  evolve without forcing a Rust-side semver dance during the
  scenario-driven build-out.
- Conformance harness consumers (`tasks.md` #62–#68) consume the
  Python surface, not the Rust surface, so Python stability is
  load-bearing.

[ADR-0010]: ../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md
