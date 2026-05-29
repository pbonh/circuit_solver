---
title: Vec Type
type: claim
id: concepts/vec-type
tags:
- rust
- foundational
- collections
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/09-chapter-8-common-collections.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

`Vec<T>` is Rust's growable, heap-allocated, contiguous list type. It stores a pointer, length, and capacity on the stack and the elements themselves in a single heap buffer. `Vec<T>` is the workhorse collection in idiomatic Rust.

## How It Works

`Vec::new()`, `Vec::with_capacity(n)`, or the `vec![...]` macro constructs a vector. `push` appends and amortizes O(1); when the buffer is full the capacity doubles. `pop` removes the last element. Indexing with `[i]` panics on out-of-bounds; `.get(i)` returns `Option<&T>`. Iteration uses `&v`, `&mut v`, or `v` to borrow shared, borrow exclusive, or consume. Slicing `&v[a..b]` yields a `&[T]`.

## Key Parameters

- Element type `T`
- Length vs capacity (growth strategy)
- Reallocation on grow invalidates existing references into the buffer
- Iteration modes: `iter`, `iter_mut`, `into_iter`

## When To Use

- Ordered, dynamically-sized lists of homogeneous data
- Stacks (push/pop at end)
- Building results before passing as slices
- Numerical kernels working on dense arrays (sometimes prefer `ndarray` for multi-dim)

## Risks & Pitfalls

- Reallocation invalidates `&v[i]` references — keep borrows local
- O(N) `insert`/`remove` in the middle
- Capacity may significantly exceed length; `shrink_to_fit` to reclaim
- For huge vectors, predict capacity with `with_capacity` to avoid repeated allocations

## Related Concepts

- [[concepts/slice-type]]
- [[concepts/string-type]]
- [[concepts/collections]]
- [[concepts/iterators]]
- [[concepts/ownership]]

## Sources

- [[summaries/rust-book-09-chapter-8-common-collections]]
