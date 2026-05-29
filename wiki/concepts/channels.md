---
title: Channels (Rust)
type: claim
id: claim-channels
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

Channels in Rust transmit values between threads via a producer–consumer pipeline. The standard library's `std::sync::mpsc` provides multi-producer, single-consumer channels. The `Sender<T>` and `Receiver<T>` halves communicate by moving `T` values; sending transfers ownership across threads.

## How It Works

`mpsc::channel()` returns a `(Sender<T>, Receiver<T>)` pair. `tx.send(value)` enqueues the value and returns `Result<(), SendError<T>>` (errors when the receiver is dropped). `rx.recv()` blocks until a value arrives or the last sender is dropped. Cloning `Sender` enables multiple producers. Receivers implement `IntoIterator`, so `for value in rx { ... }` consumes until the channel closes. Crates like `crossbeam-channel` extend this with multi-consumer, bounded, and select-style channels.

## Key Parameters

- Unbounded `mpsc::channel()` — grow without limit
- Bounded `mpsc::sync_channel(capacity)` — back-pressure
- Sender clone for multiple producers
- Drop semantics: closes the channel half

## When To Use

- Producer–consumer pipelines (workers, batchers, loggers)
- Decoupling threads with explicit messages instead of shared state
- Avoiding shared mutability altogether for cleaner reasoning
- Implementing actor-style designs

## Risks & Pitfalls

- Unbounded channels can grow without limit, masking back-pressure
- `recv()` blocks forever if no sender exists and senders are not dropped
- Many copies of `Sender` complicate shutdown ordering
- Standard `mpsc` lacks select; reach for `crossbeam-channel` when needed

## Related Concepts

- [[concepts/threads]]
- [[concepts/mutex]]
- [[concepts/arc-type]]
- [[concepts/fearless-concurrency]]

## Sources

- [[summaries/rust-book-17-chapter-16-fearless-concurrency]]
- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
