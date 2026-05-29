---
title: faer
type: entity
id: entity-faer
tags:
- rust
- sparse-matrix
- lu-decomposition
- complex-arithmetic
- numeric-solver
- well-established
created: 2026-05-18
updated: 2026-05-18
sources:
- wiki/specs/circuit-solver
- wiki/decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer
---

## Overview

[faer](https://github.com/sarah-quinones/faer-rs) is an open-source pure-Rust linear-algebra crate providing dense and sparse factorisations with a unified generic API over real and complex scalar types. It is the v1 complex-valued sparse-direct-LU backend for circuit-solver's AC small-signal analysis, selected by [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]].

## Characteristics

- Pure-Rust, no mandatory FFI to BLAS/LAPACK or SuiteSparse — preserves the [[concepts/memory-safety]] differentiator.
- Generic over scalar type via the `RealField` / `ComplexField` traits — circuit-solver instantiates the complex variant for AC's `Complex<f64>` MNA matrices.
- Maintained by Sarah Quiñones; rapid release cadence with API stabilisation underway.
- Used in circuit-solver only for AC analysis; DC and transient use [[entities/russell]].

## Common Strategies

- Backs the complex-valued `LinearSolver` in the [[contexts/numeric-solver]] context.
- Symbolic factorisation is rebuilt on every AC sweep (no cache sharing with the real-valued backend, per the trade-off documented in ADR-0002).

## Related Entities

- [[entities/russell]] — The complementary real-valued sparse-LU backend used for DC and transient.
- [[entities/numpy]] — The Python-side array type that circuit-solver returns faer-computed AC results into.
