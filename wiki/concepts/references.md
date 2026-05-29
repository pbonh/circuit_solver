---
title: References
type: claim
id: concepts/references
tags:
- rust
- foundational
- ownership
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt
- raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

A reference is a pointer-like value in Rust that lets code access data owned elsewhere without taking ownership. References come in two forms: shared (`&T`) and mutable (`&mut T`). The borrow checker enforces aliasing-XOR-mutability so that you can have many shared references or one mutable reference, but not both at once.

## How It Works

`&value` produces a shared reference; `&mut value` produces a mutable reference (requires the binding be `mut`). The compiler tracks each reference's lifetime and ensures the referenced data outlives every reference. Dereferencing uses `*r`; most method calls auto-deref. References are zero-cost: they compile to ordinary machine pointers but carry compile-time provenance.

## Key Parameters

- `&T` shared reference (read-only)
- `&mut T` exclusive reference (mutating)
- Lifetime `'a` ties reference validity to source data
- Auto-deref via the `Deref` trait

## When To Use

- Passing large data to a function without copying
- Borrowing collections for read or write operations
- Implementing iterators and view-like APIs

## Risks & Pitfalls

- Borrow-checker errors when shared and mutable borrows overlap
- Self-referential structures require workarounds (`Pin`, `Rc<RefCell<T>>`, unsafe)
- Reference invalidation when underlying collection grows (e.g., `Vec::push`)
- Confusing lifetime elision in function signatures

## Related Concepts

- [[concepts/ownership]]
- [[concepts/borrowing]]
- [[concepts/lifetimes]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
- [[summaries/rust-book-23-appendix-b-operators-and-symbols]]
