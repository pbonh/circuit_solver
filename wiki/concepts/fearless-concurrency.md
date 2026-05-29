---
title: Fearless Concurrency
type: claim
id: concepts/fearless-concurrency
tags:
- rust
- foundational
- concurrency
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/00-foreword.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Fearless concurrency is the Rust slogan for using the type system and ownership model to make concurrent programming statically safe: data races become compile-time errors rather than runtime hazards.

## How It Works

Rust models thread safety with marker traits (`Send`, `Sync`) and enforces the aliasing-XOR-mutability rule across threads as well as within a single thread. Standard library primitives — channels (`std::sync::mpsc`), mutexes (`Mutex<T>`), atomic reference counts (`Arc<T>`) — compose with the borrow checker to make sharing data between threads explicit and audited at compile time. The compiler refuses programs that would allow two threads to mutate the same data without synchronization.

## Key Parameters

- `Send` — a type can be moved between threads
- `Sync` — a type can be referenced from multiple threads
- `Arc<T>` for shared ownership across threads
- `Mutex<T>` / `RwLock<T>` for shared mutation
- Channel-based message passing as a first-class alternative

## When To Use

- Multithreaded servers, simulators, and pipelines
- Parallel numerical kernels (e.g., MNA matrix assembly across components)
- Anywhere shared mutable state would otherwise risk data races

## Risks & Pitfalls

- Compile-time prevention of data races does not prevent deadlock
- Channel-heavy designs can hide bottlenecks
- `Send`/`Sync` impls in `unsafe` code can lie and break guarantees
- Performance pitfalls from over-locking are still possible

## Related Concepts

- [[concepts/ownership]]
- [[concepts/borrowing]]
- [[concepts/rust-language]]
- [[concepts/memory-safety]]

## Sources

- [[summaries/rust-book-00-foreword]]
- [[summaries/rust-book-17-chapter-16-fearless-concurrency]]
