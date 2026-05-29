---
title: Compound Types
type: claim
id: concepts/compound-types
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/04-chapter-3-common-programming-concepts.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Compound types group multiple values into one type. Rust's two primitive compound types are tuples (fixed-size, heterogeneous) and arrays (fixed-size, homogeneous, stack-allocated). Both have known size at compile time and are `Copy` when all their elements are `Copy`.

## How It Works

A tuple `(i32, f64, char)` packs values of possibly different types and is destructured by position: `let (x, y, z) = tup;` or `tup.0`. An array `[T; N]` packs `N` values of the same type contiguously on the stack and is indexed `arr[i]`. Out-of-bounds indexing panics at runtime; for dynamic, heap-allocated, growable storage use `Vec<T>` (introduced in Chapter 8).

## Key Parameters

- Tuple arity (number of fields)
- Array element type `T` and length `N` (constant, compile-time)
- Bounds checking at index time
- The unit type `()` is the zero-arity tuple

## When To Use

- Tuples: ad-hoc grouping of related values, multiple return values, destructuring
- Arrays: small fixed-size buffers known at compile time (e.g., months of the year, matrix block tiles)
- Use `Vec<T>` instead when length is dynamic

## Risks & Pitfalls

- Tuples with many fields are unreadable — prefer a struct
- Arrays cannot grow; passing them by value copies the whole array
- Out-of-bounds index panics — use `.get(i)` to recover

## Related Concepts

- [[concepts/scalar-types]]
- [[concepts/vec-type]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-04-chapter-3-common-programming-concepts]]
