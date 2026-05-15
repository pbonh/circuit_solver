---
title: "Send and Sync"
type: concept
tags: [rust, concurrency, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/17-chapter-16-fearless-concurrency.txt"]
confidence: high
---

## Definition

`Send` and `Sync` are auto-derived marker traits that express thread-safety guarantees. `T: Send` means a value of type `T` can be safely moved to another thread. `T: Sync` means `&T` can be safely shared across threads — equivalently, `&T` is `Send`. The compiler infers these for ordinary types; `unsafe impl` is needed for types that escape the auto-derivation rules.

## How It Works

The compiler auto-implements `Send` for a type if every field is `Send`, and `Sync` if every field is `Sync`. Notable non-`Send` types: `Rc<T>` (non-atomic counts), `*const T`/`*mut T` raw pointers, `RefCell<T>`. Non-`Sync` types: `RefCell<T>`, `Cell<T>`. Crossing a thread boundary requires the compiler to verify the trait bounds, statically eliminating data races at compile time.

## Key Parameters

- Auto-derived for types built only from `Send`/`Sync` parts
- `unsafe impl Send for T` / `unsafe impl Sync for T` for custom types
- Negative impls (`!Send`, `!Sync`) deliberately opt out
- Affect APIs that take `Send + 'static` bounds (e.g., `thread::spawn`)

## When To Use

- The bounds appear in any API that crosses threads or async tasks
- Implementing custom synchronization primitives or raw-pointer types
- Designing data structures intended for multithreaded use

## Risks & Pitfalls

- `unsafe impl Send/Sync` can lie and cause undefined behavior
- Mixing single- and multi-threaded smart pointers leads to confusing compile errors
- A surprise `!Send` field deep in a struct can block thread spawning
- Some external types' `Send/Sync` status changes between versions

## Related Concepts

- [[concepts/threads]]
- [[concepts/mutex]]
- [[concepts/arc-type]]
- [[concepts/traits]]
- [[concepts/fearless-concurrency]]

## Sources

- [[summaries/rust-book-17-chapter-16-fearless-concurrency]]
