---
title: "Clone Trait"
type: concept
tags: [rust, ownership, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt"]
confidence: high
---

## Definition

`Clone` is the standard-library trait for explicit duplication. Calling `value.clone()` produces an independent copy of `value` whose semantics are defined by the implementation — typically a deep copy of any owned data. Unlike `Copy`, `Clone` is always explicit and may be expensive.

## How It Works

The trait requires implementing `fn clone(&self) -> Self`. Most types use `#[derive(Clone)]`, which recursively clones every field. `String::clone` allocates a new heap buffer and copies bytes; `Vec<T>::clone` requires `T: Clone` and clones each element. The `Copy` trait extends `Clone`, so `Copy` types are also `Clone`, but their `clone` is just a bit-copy.

## Key Parameters

- Method `clone(&self) -> Self`
- `#[derive(Clone)]` for the common deep-copy case
- Combined with `#[derive(Copy)]` for the trivial case

## When To Use

- When you need an independent copy of an owned value
- When the borrow checker requires keeping the source alive but you also need a moved version
- Implementing builder patterns where a partial state must branch

## Risks & Pitfalls

- Liberal use of `.clone()` to "silence" the borrow checker is a code smell and a performance trap
- Deep-clone of large structures (e.g., a Vec of Strings) is O(N) and may allocate many times
- Custom clones with side effects break naive assumptions

## Related Concepts

- [[concepts/copy-trait]]
- [[concepts/ownership]]
- [[concepts/move-semantics]]
- [[concepts/traits]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
- [[summaries/rust-book-24-appendix-c-derivable-traits]]
