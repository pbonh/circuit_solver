---
title: Mutex (Rust)
type: claim
id: concepts/mutex
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
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

`std::sync::Mutex<T>` wraps a value `T` behind a mutual-exclusion lock. Threads call `.lock()` to obtain a `MutexGuard<T>`, which dereferences to `&mut T` and releases the lock when dropped (RAII). Combined with `Arc<T>`, `Mutex<T>` is the standard way to share mutable state across threads in safe Rust.

## How It Works

`Mutex::new(value)` constructs a fresh, unlocked mutex containing `value`. `mutex.lock()` blocks until the lock is acquired, returning `LockResult<MutexGuard<T>>`. The guard implements `Deref` and `DerefMut`, so the wrapped value is accessed as if it were a normal `&mut T`. The lock is released when the guard goes out of scope. If a thread panics while holding the mutex, the lock becomes *poisoned*; subsequent `lock()` calls return `Err(PoisonError<T>)`.

## Key Parameters

- `lock()` returning `LockResult<MutexGuard<T>>`
- `try_lock()` non-blocking variant
- Poisoning on panic
- Reentrancy: `Mutex` is not reentrant; recursive locking deadlocks
- `RwLock<T>` sibling for many-readers / one-writer

## When To Use

- Shared mutable state behind `Arc<Mutex<T>>`
- Coordinating writes to a logger, cache, registry
- When message passing would require excessive copying
- Lazy-initialized global state via `Mutex<Option<T>>` (or `OnceLock` for many cases)

## Risks & Pitfalls

- Deadlocks from non-deterministic lock ordering
- Holding the lock across `.await` or expensive work serializes threads
- Poisoning surprise after panics — many crates use `parking_lot::Mutex` which has no poisoning
- `Mutex<T>` requires `T: Send`, sometimes a surprise

## Related Concepts

- [[concepts/arc-type]]
- [[concepts/threads]]
- [[concepts/send-sync]]
- [[concepts/interior-mutability]]

## Sources

- [[summaries/rust-book-17-chapter-16-fearless-concurrency]]
- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
