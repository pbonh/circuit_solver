---
title: "Rust as the implementation language for the circuit simulator core"
status: proposed
date: 2026-06-16
decision-makers:
  - circuit-solver-team
consulted: []
informed: []
---

# Rust as the implementation language for the circuit simulator core


## Context and Problem Statement

The Circuit Solver Delta project requires selection of a systems programming language for the core simulator kernel, which must handle sparse matrix operations, Newton-Raphson iteration for DC analysis, and timestep-controlled transient simulation.

The language choice directly affects:
- Memory safety guarantees in the inner NR loop processing non-linear device equations
- Absence of garbage collection pauses during fixed timestep integration
- Concurrency model for parallel element stamping during matrix assembly
- Long-term maintainability and refactoring safety without runtime crashes

## Decision Drivers

1. **No GC pauses**: Transient simulation requires predictable performance; garbage collection pauses introduce jitter incompatible with fixed-timestep integration.
2. **Memory safety in sparse kernels**: The sparse matrix solver and Newton-Raphson loop manipulate pointers and dynamic data structures; use-after-free and buffer overruns must be impossible.
3. **Data-race-free parallelism**: Element stamping (population of the MNA matrix) is embarrassingly parallel; the language must guarantee no data races without global locks.
4. **Zero-cost abstractions**: Performance must match hand-optimized C; the language cannot impose runtime overhead.
5. **Ecosystem maturity**: The language must have production-grade libraries for sparse linear algebra (`faer`, `sprs`) and parallel iteration (`rayon`).
6. **Long-term maintainability**: Code written today must be refactorable in 5 years without architectural regressions.

## Considered Options

### Option 1: Rust
- **Pros**:
  - Ownership model eliminates entire classes of bugs (use-after-free, double-free, data races at compile time).
  - No GC; deterministic resource cleanup via RAII.
  - `rayon` crate provides fearless parallelism with zero-cost abstraction over data parallelism.
  - `faer` and `sprs` provide production-grade sparse linear algebra with BLAS integration.
  - Strict borrowing rules force safe refactoring; breaking code is a compile error, not a runtime crash.
  - Growing ecosystem for scientific computing (ndarray, polars, tch-rs for ML inference).
- **Cons**:
  - Steep learning curve for ownership/borrowing; new team members require 3–6 months to achieve productivity.
  - Borrow checker can force awkward code patterns; requires discipline in API design.
  - No existing SPICE Fortran interop libraries; FFI calls to legacy simulators require unsafe blocks.
  - Longer initial development velocity due to compile-time checks, though refactoring is faster later.

### Option 2: C++
- **Pros**:
  - Mature ecosystem (Eigen, Armadillo, Intel MKL all have C++ bindings).
  - Zero runtime overhead; full control over memory layout.
  - OpenMP and TBB provide parallelism; SPICE simulators already use C++ (Spectre is C++).
  - Faster initial development; team familiar with C++.
- **Cons**:
  - No memory safety guarantees; use-after-free and buffer overruns are runtime errors or silent memory corruption.
  - Manual memory management error-prone in large refactors; reference counting and smart pointers add overhead or complexity.
  - Data races in multi-threaded code are difficult to detect; requires careful discipline and static analysis tooling.
  - Undefined behavior (signed overflow, out-of-bounds access) makes optimization unpredictable.

### Option 3: Julia
- **Pros**:
  - Built for numerical computing; multiple dispatch idiomatically handles non-linear equations.
  - Rich linear algebra ecosystem (LinearAlgebra.jl, SparseArrays.jl).
  - JIT compilation can match C performance on hot paths.
  - Rapid prototyping and interactive development.
- **Cons**:
  - GC pauses are non-negligible; cannot guarantee fixed-timestep execution.
  - Startup time significant for small scripts; simulators are usually long-running, mitigating this.
  - Sparse matrix operations less optimized than Rust ecosystem; matrix assembly still slower than C/C++.
  - Runtime errors dominate; type instability causes performance cliffs.

### Option 4: Python + Fortran (SPICE model)
- **Pros**:
  - Rapid prototyping in Python; performance-critical paths written in Fortran.
  - Existing SPICE device models (BSIM, MOS) are Fortran; direct code reuse.
  - Familiar to circuit designers without deep systems knowledge.
- **Cons**:
  - Two-language problem; boundary between Python and Fortran is impedance mismatch.
  - Python GC pauses; Fortran FFI calls require serialization of Python objects.
  - Sparse matrix operations via SciPy slower than compiled alternatives.
  - Debugging across language boundary is error-prone.

## Decision Outcome

**Decision**: Adopt **Rust** as the primary implementation language for Circuit Solver Delta.

**Rationale**:
- Rust's ownership model makes entire classes of sparse-matrix bugs impossible at compile time, eliminating memory-safety regressions during refactoring.
- No GC pauses guarantee predictable performance for transient integration.
- `rayon` and `faer` provide the parallel and sparse-algebra primitives needed without runtime overhead or global locks.
- The learning curve is steep, but the payoff in long-term safety and refactoring velocity outweighs initial development cost.
- Rust is sufficiently mature; projects like Polars (dataframes) and the Uutils coreutils prove production viability.

## Consequences

1. **Team ramp-up**: New developers require 3–6 months to achieve productivity with Rust idioms. Mitigation: pair programming on initial PRs; comprehensive Rust-for-systems-programmers training.
2. **Development velocity**: Initial feature delivery slower due to borrow-checker constraints and longer compile times. Mitigation: incremental compilation, `mold` linker, and cache-friendly architecture.
3. **Device model FFI**: Existing Fortran SPICE models (BSIM6) cannot be directly linked; must be reimplemented or wrapped via `bindgen`. Mitigation: prioritize core compact models (resistor, capacitor, diode); defer full BSIM adoption.
4. **Ecosystem gaps**: Some specialized solver algorithms may lack Rust bindings; may require unsafe wrappers or reimplementation. Mitigation: target well-established algorithms (Radau IIA, MNA) with strong academic literature.

## Confirmation

1. **CI gates**: Every PR must pass:
   - `cargo clippy --deny warnings` (no compiler warnings, no clippy lints)
   - `cargo test --all` (comprehensive unit and integration test suite)
   - `cargo check --all-targets` (no hidden type errors)
2. **Benchmarks**: Sparse matrix assembly and NR solve time must match or exceed equivalent C code on the same hardware.
3. **Code review**: All unsafe blocks require explicit justification and review; default is safe Rust.

## Pros and Cons of the Options

| Criterion | Rust | C++ | Julia | Python+Fortran |
|-----------|------|-----|-------|-----------------|
| Memory safety | ✓ compile-time | ✗ runtime | ~ GC pauses | ✗ Python/Fortran boundary |
| GC pauses | ✓ none | ✓ none | ✗ yes | ✗ yes |
| Parallelism | ✓ fearless | ~ requires discipline | ~ GC-friendly | ✗ FFI overhead |
| Sparse LA ecosystem | ✓ faer, sprs | ✓ Eigen, MKL | ~ SciPy | ✗ Fortran 77/90 |
| Initial velocity | ✗ slow | ✓ fast | ✓ fast | ✓ fast |
| Long-term refactoring | ✓ safe | ✗ risky | ~ medium | ✗ complex |
| SPICE interop | ✗ FFI only | ✓ direct | ✗ Python boundary | ✓ direct |
| Team learning curve | ✗ steep | ~ moderate | ~ moderate | ✓ shallow |

## Evidence

This decision is grounded in the following wiki evidence:
- [[rust-systems-programming]] — Rust's ownership model, borrowing semantics, and memory safety guarantees for systems programming
- [[rust-programming-language]] — Rust ecosystem maturity, production use cases, and library landscape (rayon, faer, sprs)
