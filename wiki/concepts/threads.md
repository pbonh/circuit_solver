---
title: "Threads (Rust)"
type: concept
tags: [rust, concurrency, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/17-chapter-16-fearless-concurrency.txt"]
confidence: high
---

## Definition

A thread in Rust is an OS-level concurrent execution unit managed via `std::thread`. The 1:1 thread model maps one Rust thread to one OS thread. Threads are spawned with `thread::spawn` and joined via the returned `JoinHandle`.

## How It Works

`std::thread::spawn(|| { ... })` schedules a closure to run on a new OS thread. The closure must be `Send + 'static` because it is detached from the spawning frame. `JoinHandle::join()` blocks until the thread finishes and returns its result (or its panic). Thread-local storage uses the `thread_local!` macro for per-thread state. Higher-level abstractions (thread pools, work stealing) come from external crates such as `rayon` and `crossbeam`.

## Key Parameters

- `thread::spawn(closure) -> JoinHandle<T>`
- Closure bounds: `FnOnce() -> T + Send + 'static`
- `Builder::stack_size`, `name` for customization
- `thread_local!` for per-thread state

## When To Use

- CPU-bound workloads with independent units of work
- I/O-bound workloads where async overhead is unwarranted
- Heavy numerical kernels parallelized with `rayon` (which is built on threads)
- Long-lived background workers (loggers, telemetry flushers)

## Risks & Pitfalls

- Thread overhead: per-thread stack allocation and context switch costs
- Synchronization mistakes: deadlocks, contention, priority inversion
- Compile errors when captured variables fail `Send + 'static`
- Joining is mandatory or the thread is detached and its result lost

## Related Concepts

- [[concepts/fearless-concurrency]]
- [[concepts/channels]]
- [[concepts/mutex]]
- [[concepts/arc-type]]
- [[concepts/send-sync]]

## Sources

- [[summaries/rust-book-17-chapter-16-fearless-concurrency]]
- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
