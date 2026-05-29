---
title: PyO3
type: entity
id: entities/pyo3
tags:
- rust
- python
- binding
- ffi
- well-established
created: 2026-05-18
updated: 2026-05-18
sources:
- wiki/specs/circuit-solver
- wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph
---

## Overview

[PyO3](https://pyo3.rs/) is the canonical Rust crate for building native CPython extension modules in safe Rust. It exposes a `Python<'py>` lifetime token that statically enforces correct interaction with the [[concepts/global-interpreter-lock]], a `PyAny`-rooted object hierarchy, and `#[pyclass]` / `#[pymethods]` proc-macros that generate the Python ABI glue.

## Characteristics

- In-process CPython binding — no IPC, no subprocess; the Rust extension runs in the same process as the Python interpreter.
- Supports zero-copy NumPy interop via the companion `numpy` crate (`PyArray<T, D>`), exposing Rust-owned buffers as `numpy.ndarray` with controllable read/write flags.
- The `Python::allow_threads` API explicitly releases the GIL around long-running Rust work, restoring parallelism for concurrent Python threads.
- Rust panics inside `#[pyfunction]` are converted to Python `RuntimeError` exceptions rather than aborting the interpreter.
- Selected by [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph|ADR-0001]] as the sole Rust↔Python binding mechanism for circuit-solver.

## Common Strategies

- Builder pattern exposed to Python that returns an immutable [[concepts/graph]] handle.
- Zero-copy NumPy result arrays with `writeable=False` to preserve Rust ownership.
- `allow_threads` blocks around the [[entities/russell]] and [[entities/faer]] solve phases.

## Related Entities

- [[entities/numpy]] — The Python-side counterpart for result arrays.
- [[entities/russell]] — Rust solver crate whose long solves run under `allow_threads`.
- [[entities/faer]] — Same, for AC complex-valued solves.
