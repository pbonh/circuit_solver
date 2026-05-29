---
title: Drop Trait
type: claim
id: concepts/drop-trait
tags:
- rust
- ownership
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt
- raw/rust_book/_txt/16-chapter-15-smart-pointers.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

`Drop` is the trait Rust uses to run custom cleanup code when a value goes out of scope. Implementations provide `fn drop(&mut self)`, which the compiler calls automatically (in reverse declaration order) just before the value's memory is reclaimed. This is Rust's deterministic, RAII-style resource cleanup.

## How It Works

When a binding goes out of scope, the compiler emits a call to `drop` for any owned values. For a struct, fields are dropped after the struct's `drop`. For `Box<T>`, `Vec<T>`, `File`, `MutexGuard<T>`, etc., the impl releases heap memory, OS handles, or locks. You cannot call `drop` directly on a value — use `std::mem::drop(v)` to force early cleanup. Types implementing `Drop` cannot implement `Copy`.

## Key Parameters

- Single method `drop(&mut self)`
- Drop order: reverse declaration order for fields; LIFO for locals
- Forbidden alongside `Copy`
- `std::mem::drop` for explicit early-drop

## When To Use

- Resource handles: file descriptors, sockets, GPU buffers, locks
- Smart pointers managing heap allocations
- Anything that must release a non-memory resource

## Risks & Pitfalls

- Panics inside `drop` during unwinding cause `abort`
- Cyclic `Rc<RefCell<T>>` graphs leak because no one drops
- Drop order surprises when restructuring fields
- `mem::forget` skips `drop` entirely (intentional leak)

## Related Concepts

- [[concepts/ownership]]
- [[concepts/smart-pointers]]
- [[concepts/raii]]
- [[concepts/traits]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
- [[summaries/rust-book-16-chapter-15-smart-pointers]]
- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
