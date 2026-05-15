---
title: "Interior Mutability"
type: concept
tags: [rust, ownership, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/16-chapter-15-smart-pointers.txt"]
confidence: high
---

## Definition

Interior mutability is the Rust pattern of mutating data through a shared (`&T`) reference, deferring the borrow check from compile time to runtime (or using atomics for thread-safe versions). The standard types are `Cell<T>`, `RefCell<T>` (single-thread), and `Mutex<T>`/`RwLock<T>`/`Atomic*` (multi-thread).

## How It Works

The outer type — `&Cell<T>`, `&RefCell<T>`, `&Mutex<T>` — looks immutable to the borrow checker but exposes APIs (`set`, `borrow_mut`, `lock`) that mutate the inside. `RefCell` keeps a runtime count of outstanding borrows and panics on conflict; `Mutex` uses an OS lock; `Cell` uses `Copy`-based get/set with no borrowing at all. The pattern enables APIs that take `&self` while still mutating internal state, useful for caching, lazy evaluation, and mock objects.

## Key Parameters

- `Cell<T>` — `T: Copy`, get/set semantics
- `RefCell<T>` — borrow/borrow_mut guards, runtime check, single-thread
- `Mutex<T>` — multi-thread variant, blocking
- `RwLock<T>` — many readers / one writer
- `OnceCell<T>` / `LazyCell<T>` — one-time initialization

## When To Use

- Caching results inside `&self` methods
- Implementing lazy-init patterns
- Mock objects in tests for trait methods taking `&self`
- Wrapping `Rc<T>` to allow mutation among multiple owners

## Risks & Pitfalls

- Runtime borrow panics are easy to trigger in cyclic data structures
- Hidden contention when many threads share an `Arc<Mutex<T>>`
- Over-reliance can mask poor data ownership design
- `Cell<T>` only takes `Copy` types — newcomers often expect more

## Related Concepts

- [[concepts/refcell-type]]
- [[concepts/rc-type]]
- [[concepts/mutex]]
- [[concepts/smart-pointers]]

## Sources

- [[summaries/rust-book-16-chapter-15-smart-pointers]]
