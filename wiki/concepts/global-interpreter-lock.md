---
title: "Global Interpreter Lock (GIL)"
type: concept
tags: [python, concurrency, foundational, well-established]
created: 2026-05-18
updated: 2026-05-18
sources: ["wiki/specs/circuit-solver"]
confidence: low
---

## Definition

The *Global Interpreter Lock* (GIL) is a mutex inside the CPython interpreter that allows only one native thread to execute Python bytecode at a time. It exists to make CPython's memory management thread-safe without per-object locks.

## How It Works

Every Python thread must hold the GIL to run bytecode. The interpreter periodically releases the GIL to allow other threads to make progress. C extensions (including [[entities/pyo3]] extensions) can explicitly release the GIL around long-running native work using mechanisms like `Py_BEGIN_ALLOW_THREADS` (C API) or `Python::allow_threads` (PyO3). While the GIL is released, the C extension's native code can run in parallel with other Python threads on different cores.

## Key Parameters

- **Hold vs release decision** — whether a native function pays the cost of releasing the GIL or holds it for simplicity.
- **Reacquisition** — the extension must reacquire the GIL before touching any Python object.
- **Granularity** — coarse release (one release for an entire solve) versus fine release (release inside hot loops).

## When To Use

- A long-running Rust solve inside a PyO3 extension should release the GIL so that concurrent Python threads (UI, I/O, other workers) are not starved.
- Short hot paths that touch Python objects frequently should NOT release the GIL — the reacquire cost exceeds the benefit.

## Risks & Pitfalls

- Holding the GIL during long Rust work makes the Python interpreter unresponsive and prevents parallel Python worker threads from making progress.
- Releasing the GIL while still holding raw `PyObject` pointers leads to crashes; PyO3 enforces this statically via the `Python<'py>` token.
- Tests for GIL release require an actual concurrent Python thread; single-threaded benchmarks cannot demonstrate the property.

## Related Concepts

- [[concepts/ownership]]

## Sources

- [[specs/circuit-solver]] — Story 5 requires the solver to release the GIL during long-running analyses so that a concurrent Python thread observes ≥ 80 % CPU utilisation.
