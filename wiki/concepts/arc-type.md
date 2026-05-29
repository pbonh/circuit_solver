---
title: Arc Type
type: claim
id: claim-arc-type
tags:
- rust
- concurrency
- smart-pointers
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/17-chapter-16-fearless-concurrency.txt
confidence:
  base: 0.85
---

## Definition

`std::sync::Arc<T>` is the thread-safe equivalent of `Rc<T>`: atomically-counted shared ownership of an immutable value across multiple threads. Combined with `Mutex<T>` or `RwLock<T>`, it provides shared mutable state. The "atomic" in the name refers to the lock-free increment/decrement of the reference count, not the protected data.

## How It Works

`Arc::new(value)` allocates a header (atomic strong/weak counts) plus the value on the heap. `Arc::clone(&arc)` bumps the count via an atomic instruction. Unlike `Rc`, `Arc<T>` implements `Send` and `Sync` when `T: Send + Sync`, so it can cross thread boundaries. Dropping an `Arc` atomically decrements the count and frees the data when the count hits zero. Weak references work via `Arc::downgrade`.

## Key Parameters

- Atomic strong/weak counts (cache-line traffic per clone/drop)
- `Send + Sync` propagation from `T`
- `Arc::clone(&arc)` idiom for cloning
- Combine with `Mutex`/`RwLock`/`atomic*` for mutation

## When To Use

- Multi-threaded read-only sharing of large/immutable data (config, lookup tables)
- Shared mutable state via `Arc<Mutex<T>>` / `Arc<RwLock<T>>`
- Building concurrent caches and registries
- Reference counts spanning async tasks across threads

## Risks & Pitfalls

- Cache-line bouncing under heavy contention on the count
- Cycles still leak — break with `Weak<T>`
- Arc clone is not free; do not abuse in tight loops
- `Arc<Mutex<T>>` invites lock-ordering deadlocks

## Related Concepts

- [[concepts/rc-type]]
- [[concepts/mutex]]
- [[concepts/weak-references]]
- [[concepts/send-sync]]
- [[concepts/smart-pointers]]

## Sources

- [[summaries/rust-book-17-chapter-16-fearless-concurrency]]
- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
