---
title: PyO3 In-Process Binding with Immutable Circuit Graph
type: claim
id: decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph
tags:
- decision
- circuit-solver
- pyo3
- rust
- python
- binding
- memory-safety
created: 2026-05-17
updated: 2026-05-18
sources:
- architecture/circuit-solver
- grills/circuit-solver
- contexts/application-frontend
- vision/circuit-solver
confidence:
  base: 0.95
  source_count: 4
  contradicted: false
  effective: 1.045
  inputs_hash: 06f4fee59367837e
---

"In the context of providing a Python API for the circuit-solver Rust core, facing the need for ergonomic interactive circuit construction without sacrificing Rust's ownership guarantees, we decided for an in-process PyO3 extension module with an immutable `CircuitGraph` built via a Rust-backed builder API exposed to Python, and per-request mutable analysis state passed from Python, to achieve zero-copy NumPy-compatible results with full Rust ownership discipline, accepting that the Python runtime must be compiled with a compatible ABI and that debugging across the language boundary is harder than pure Rust or pure Python."

## Status

accepted

## Context

The simulator must expose an ergonomic, zero-copy Python API for interactive circuit construction and analysis while preserving Rust's ownership and memory-safety guarantees. This is an [[concepts/architecturally-significant-requirement|architecturally significant requirement]] (ASR) because it affects both the user-facing API design and the internal ownership model of the [[contexts/application-frontend|application-frontend]] context.

The [[vision/circuit-solver|Circuit Solver vision]] explicitly bounds scope to a Python frontend (`python -m circuit_solver`) and a programmatic API, making the Rust↔Python binding mechanism a structural commitment. The [[grills/circuit-solver|grill Q&A]] explored four binding alternatives (in-process PyO3, in-process C FFI + cffi, out-of-process gRPC/IPC, hybrid) and three state-management patterns (stateful session, stateless functional, read-only graph + mutable analysis state, builder pattern), settling on in-process PyO3 with read-only graph + mutable analysis state and a builder API for graph construction.

## Decision

We commit to an in-process PyO3 extension module as the sole Rust-to-Python binding mechanism. The binding exposes a builder API from the Netlist Graph Builder context to Python, allowing incremental construction of an immutable `CircuitGraph`. Once built, the graph is owned by Rust and exposed to Python as an opaque, immutable handle. Per-request mutable analysis state (analysis type, sweep parameters, options) flows from Python to the Analysis Orchestrator as value types. Simulation results return to Python as NumPy-compatible arrays without copying underlying Rust data.

This decision preserves [[concepts/ownership|Rust ownership discipline]]: the immutable graph has a single owner in Rust, Python holds only a PyO3-wrapped reference that cannot mutate it. Mutable state is transient and scoped to individual analysis requests, avoiding shared mutable state across the language boundary.

## Consequences

**Positive:**
- Zero-copy result arrays via PyO3's NumPy interop (`numpy` crate feature), eliminating serialization overhead.
- Ergonomic incremental circuit construction in Python without exposing mutable shared state to Python.
- Full [[concepts/memory-safety|memory safety]]: PyO3 enforces Rust ownership rules at the boundary; Python cannot create dangling references or data races against the immutable graph.
- Single-process deployment; no IPC or networking configuration for local interactive use.

**Negative:**
- Python runtime ABI compatibility requirement: the PyO3 extension must be compiled against the target Python version (CPython 3.9+).
- Debugging complexity: panics in Rust propagate across the PyO3 boundary and may crash the Python interpreter if not caught.
- GIL contention: long-running Rust computations hold the Python GIL unless explicitly released with `allow_threads`, which must be audited per analysis path.

**Neutral:**
- The builder API adds a layer of indirection over raw netlist strings; users who prefer deck-based input still pass strings to the builder.
- Device model parameters remain sourced from `.model` cards in the netlist; runtime parameter override from Python is out of scope unless a later ADR expands it.

## Related Decisions

- [[architecture/circuit-solver]] — The container diagram that surfaces this decision under `## Decisions Surfaced`.
- [[grills/circuit-solver]] — Q&A log where binding mechanism and state-management alternatives were interrogated.
- [[vision/circuit-solver]] — Scope declaration that mandates a Python frontend.
