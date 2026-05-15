---
title: "The Rust Programming Language — Chapter 16: Fearless Concurrency"
type: summary
tags: [rust, concurrency, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/17-chapter-16-fearless-concurrency.txt"]
confidence: high
---

## Key Points

- "Fearless concurrency" is Rust's slogan: the ownership/borrow rules that prevent memory-safety bugs in single-threaded code also prevent data races in multithreaded code, all at compile time.
- `std::thread::spawn(closure)` starts a new OS thread; it returns a `JoinHandle` whose `.join()` waits for the thread to finish (and yields its return value or panic).
- A spawned thread can outlive the function that started it, so closures passed to `spawn` typically need `move` to capture environment values by ownership rather than reference.
- Two principal concurrency models are presented: message passing (channels) and shared-state (mutexes + atomic reference counting).
- `std::sync::mpsc` provides multi-producer, single-consumer channels. `tx.send(value)` moves the value (transferring ownership across threads); `rx.recv()` blocks for the next value; iterating a `Receiver` consumes values until the channel closes.
- Senders implement `Clone` to enable multiple producers.
- Mutexes: `std::sync::Mutex<T>` encapsulates data with a lock. `mutex.lock()` returns a `MutexGuard<T>` that dereferences to `&mut T` and releases the lock when dropped (RAII).
- `Rc<T>` is single-threaded and forbidden across threads (no `Send`). The thread-safe analog is `Arc<T>` — atomic reference counting — paired with `Mutex<T>` or `RwLock<T>` for shared mutable state (`Arc<Mutex<T>>`).
- Two marker traits underpin the safety story: `Send` means "ownership can be transferred to another thread"; `Sync` means "the type can be referenced from multiple threads". Most types are automatically `Send + Sync` by virtue of their fields.
- The Rust standard library is intentionally minimal on concurrency primitives. Higher-level patterns (work stealing, async runtimes, lock-free data structures) come from external crates like `rayon`, `tokio`, `crossbeam`.

## Relevant Concepts

- [[concepts/fearless-concurrency]] — the chapter's framing.
- [[concepts/threads]] — `std::thread::spawn`, `JoinHandle`.
- [[concepts/channels]] — message-passing concurrency.
- [[concepts/mutex]] — exclusive access to shared state.
- [[concepts/arc-type]] — atomic reference counting for cross-thread sharing.
- [[concepts/send-sync]] — marker traits for thread safety.
- [[concepts/data-race]] — what Rust eliminates at compile time.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 16 — Fearless Concurrency
- File path: `raw/rust_book/_txt/17-chapter-16-fearless-concurrency.txt`
- Authors: Steve Klabnik and Carol Nichols
