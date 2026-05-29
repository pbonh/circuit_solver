---
title: Russell
type: entity
id: entities/russell
tags:
- rust
- sparse-matrix
- lu-decomposition
- numeric-solver
- well-established
created: 2026-05-18
updated: 2026-05-18
sources:
- wiki/specs/circuit-solver
- wiki/decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer
---

## Overview

[Russell](https://github.com/cpmech/russell) is an open-source Rust scientific-computing toolkit (`russell_lab`, `russell_sparse`, `russell_ode`, etc.) providing dense and sparse linear algebra, sparse direct solvers, and ODE integrators. It is the v1 real-valued sparse-direct-LU backend for circuit-solver's DC and transient analyses, selected by [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]].

## Characteristics

- Pure-Rust API; some sparse backends link to system BLAS/LAPACK and to SuiteSparse-family solvers via feature flags, but the abstraction layer hides the FFI from circuit-solver.
- Tuned for general real-valued sparse systems — well suited to the [[concepts/modified-nodal-analysis]] matrices that DC and transient analyses produce.
- Exposes fill-reducing orderings (AMD, METIS) at the configuration layer; circuit-solver picks defaults at the [[contexts/numeric-solver]] boundary.
- Maintained by Dorival Pedroso (`cpmech`); active development cadence.

## Common Strategies

- Backs the real-valued `LinearSolver` in the [[contexts/numeric-solver]] context.
- Re-factors at every Newton iterate during DC iteration; re-factors per accepted timestep during transient analysis.

## Related Entities

- [[entities/faer]] — The complementary complex-valued sparse-LU backend used for AC analysis.
- [[entities/numpy]] — The Python-side array type that circuit-solver returns Russell-computed results into.
