---
title: Copy Trait
type: claim
id: concepts/copy-trait
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

`Copy` is a marker trait indicating that a type's value can be duplicated by a simple bit-copy of its memory. Types implementing `Copy` are not moved on assignment; both the source and destination remain valid. Only types whose entire content is themselves `Copy` (and which do not own heap resources) may implement `Copy`.

## How It Works

`Copy` extends `Clone` and has no methods of its own. A type can derive `Copy` only if every field is `Copy` and the type does not implement `Drop`. The compiler uses `Copy` at assignment, argument-pass, return, and pattern-binding sites: instead of moving, it bit-copies. All scalars (integers, floats, bool, char), `&T` references, and fixed-size tuples/arrays of `Copy` types are `Copy`.

## Key Parameters

- Marker trait; no functions
- Requires `Clone` to also be implemented
- Forbids `Drop`
- Heap-owning types (`String`, `Vec<T>`, `Box<T>`) cannot be `Copy`

## When To Use

- Small "value-like" types (numeric wrappers, IDs, tags)
- Plain old data structs used as inputs/outputs in hot loops
- Avoid for any type holding a resource

## Risks & Pitfalls

- Adding `Copy` to a type is an API-breaking commitment to never own resources
- Large `Copy` types silently copy a lot of memory at every assignment
- Confusing for newcomers: scalars look like primitives but the rule is the trait

## Related Concepts

- [[concepts/clone-trait]]
- [[concepts/ownership]]
- [[concepts/move-semantics]]
- [[concepts/drop-trait]]
- [[concepts/traits]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
- [[summaries/rust-book-24-appendix-c-derivable-traits]]
