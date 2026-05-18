---
title: "Hybrid Sparse Direct Solver Backend (russell + faer)"
type: decision
tags: [decision, circuit-solver, sparse-matrix, lu-decomposition, russell, faer, rust, memory-safety, numeric-solver]
created: 2026-05-17
updated: 2026-05-18
sources:
  - "architecture/circuit-solver"
  - "contexts/numeric-solver"
  - "concepts/sparse-matrix"
  - "concepts/lu-decomposition"
  - "concepts/ffi"
  - "concepts/memory-safety"
confidence: high
---

## Status

accepted

## Context

The numeric solver must perform sparse direct LU factorization for all in-scope analysis types without crossing an FFI boundary to C/C++ legacy libraries, because [[concepts/memory-safety|memory safety]] is a core differentiator, and the analysis portfolio inherently requires both real-valued (DC/transient) and complex-valued (AC) solves. This is an [[concepts/architecturally-significant-requirement|architecturally significant requirement]] (ASR) because it constrains the dependency tree to pure-Rust crates and dictates that a single solver abstraction must serve both real and complex arithmetic domains.

The [[vision/circuit-solver|Circuit Solver vision]] bounds scope to DC, AC small-signal, transient, and noise analyses. DC and transient produce real-valued MNA matrices; AC small-signal produces complex-valued matrices (G + jωC). The [[grills/circuit-solver|grill Q&A]] explored solver backend alternatives (single pure-Rust crate, FFI to KLU/UMFPACK, mixed real/complex crate, and the hybrid two-crate approach) and converged on splitting by arithmetic type.

The [[contexts/numeric-solver|numeric-solver]] context owns `MNAMatrix`, `LinearSolver`, and `NewtonRaphsonSolver`. Its boundary with [[contexts/device-modeling|device-modeling]] accepts `LinearizedModel` stamps; its boundary with [[contexts/analysis-orchestration|analysis-orchestration]] delivers `SolutionVector` and `ConvergenceStatus`.

## Decision

We commit to a hybrid sparse direct solver backend inside the Numeric Solver Engine container:
- **[[concepts/sparse-matrix|russell]]** (`russell_lab` / `russell_sparse`) for real-valued sparse direct LU — used by DC operating-point and transient timestep solves.
- **faer** (`faer-rs`) for complex-valued sparse direct LU — used by AC small-signal analysis.

Both crates are pure Rust; neither requires an FFI boundary to C/C++ BLAS, LAPACK, or SuiteSparse. The Numeric Solver Engine abstracts the two backends behind a single `LinearSolver` trait or dispatch layer so that the Analysis Orchestrator requests a solve without naming the concrete backend. The dispatch selects `russell` when the matrix element type is `f64` and `faer` when the element type is `Complex<f64>`.

This decision accepts that the codebase maintains two distinct sparse-direct-LU dependency trees rather than one unified crate. It also accepts that transient analysis (real) and AC analysis (complex) cannot share a single factorization cache or symbolic analysis object across the type boundary, because `russell` and `faer` use incompatible matrix formats and symbolic structures.

## Consequences

**Positive:**
- No FFI to C/C++: the entire solver stack stays within Rust's memory-safety guarantees, eliminating `unsafe` blocks that would be required for SuiteSparse/KLU/UMFPACK interop.
- Type-appropriate crates: `russell` is optimized for real sparse systems; `faer` is optimized for real and complex dense/sparse systems with a unified generic API. Each analysis type gets a backend that natively supports its arithmetic.
- Pure-Rust dependency tree simplifies cross-compilation and static-binary deployment; no platform-specific BLAS/LAPACK linking or symbol-versioning issues.
- Newton-Raphson iteration in DC/transient benefits from `russell`'s real-only sparse structures without paying the abstraction cost of a complex-capable generic solver.

**Negative:**
- Two solver ecosystems to learn, profile, and debug; operational expertise must cover both `russell` and `faer` APIs, error handling styles, and fill-reducing ordering options.
- No shared symbolic analysis or factorization cache across real/complex boundaries; switching from DC to AC analysis rebuilds the sparse pattern and ordering from scratch.
- Potential API drift: if `russell` or `faer` evolve incompatibly, the abstraction layer must absorb the breakage. Two upstream crates means twice the semver surface area.
- `russell` may request a system BLAS/LAPACK for some dense subproblems; although the sparse path is pure Rust, the real backend is not *guaranteed* FFI-free unless compiled with the right feature flags.

**Neutral:**
- Noise analysis (which mixes real and complex small-signal transfer functions) will need explicit scoping in a later spec to determine whether it sits on the `russell` or `faer` path, or requires both in sequence.
- The decision does not preclude adding a third backend later (e.g., a GPU sparse solver) behind the same `LinearSolver` abstraction, provided the trait is designed with extension in mind.

## Related Decisions

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph|ADR-0001]] — Preceding ADR on the PyO3 binding; both ADRs preserve the pure-Rust, memory-safe core.
- [[architecture/circuit-solver]] — The container diagram that surfaces this decision under `## Decisions Surfaced`.
- [[grills/circuit-solver]] — Q&A log where solver backend alternatives were interrogated.
- [[vision/circuit-solver]] — Scope declaration that mandates DC, AC, and transient analyses.
- [[contexts/numeric-solver]] — Bounded context that owns the solver abstraction and backend selection.
