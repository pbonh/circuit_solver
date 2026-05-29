---
title: Data Race
type: claim
id: claim-data-race
tags:
- rust
- concurrency
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/17-chapter-16-fearless-concurrency.txt
confidence:
  base: 0.85
---

## Definition

A data race occurs when two or more threads access the same memory location concurrently, at least one writes, and no synchronization orders the accesses. The result is undefined behavior in C/C++ and most low-level languages. Rust statically prevents data races in safe code via ownership and the `Send`/`Sync` traits.

## How It Works

In Rust, mutable references are exclusive — only one thread can hold a `&mut T` at a time. Crossing thread boundaries with `&T` requires `T: Sync`, ensuring shared references are safe to use concurrently. Synchronization primitives (`Mutex<T>`, `RwLock<T>`, atomics) gate access so that mutation never overlaps. This static guarantee is the substance of "fearless concurrency": data races are caught at compile time rather than via stress tests.

## Key Parameters

- Two participating threads
- Same memory location
- At least one write
- No happens-before edge between the accesses

## When To Use

- A concept that *defines* the failure Rust prevents
- Discussion of synchronization primitives in design reviews
- Audit checklists for `unsafe` code, where data races become possible again

## Risks & Pitfalls

- `unsafe` code can reintroduce data races silently
- FFI to C/C++ libraries can cause races outside Rust's view
- Race conditions (interleaving bugs that are not data races) are still possible
- Confusing "data race" with the broader "race condition"

## Related Concepts

- [[concepts/fearless-concurrency]]
- [[concepts/send-sync]]
- [[concepts/mutex]]
- [[concepts/memory-safety]]

## Sources

- [[summaries/rust-book-17-chapter-16-fearless-concurrency]]
