---
title: Borrowing
type: claim
id: concepts/borrowing
tags:
- rust
- ownership
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Borrowing in Rust is the act of creating a reference to a value without taking ownership. The borrow checker enforces a strict aliasing rule: at any given point in a program, a value may have either many shared (`&T`) borrows OR exactly one mutable (`&mut T`) borrow — never both. References must always be valid (no dangling).

## How It Works

A function parameter typed `&T` or `&mut T` borrows the argument for the duration of the call. The borrow checker tracks the scope of every reference and rejects programs that would alias mutably or use a reference after the owner is moved/dropped. Non-lexical lifetimes (NLL) shrink the lifetime of a reference to its last use, allowing more programs to typecheck.

## Key Parameters

- Shared borrow `&T` — multiple simultaneous borrows allowed
- Exclusive borrow `&mut T` — only one at a time
- Reborrow chains: `&mut *r` to pass through nested calls
- Lifetime parameters when stored in structs or returned

## When To Use

- Whenever a function needs to read or modify a value without consuming it
- Passing large structures (vectors, matrices) without copying
- Implementing iterators and view APIs

## Risks & Pitfalls

- Cannot mutate through a `&T`
- Cannot hold a `&mut T` and any other reference to the same data
- Reference invalidation when a `Vec`/`HashMap` reallocates
- Confusing borrow-checker errors when control flow allows both arms to escape a borrow

## Related Concepts

- [[concepts/ownership]]
- [[concepts/references]]
- [[concepts/lifetimes]]
- [[concepts/memory-safety]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
