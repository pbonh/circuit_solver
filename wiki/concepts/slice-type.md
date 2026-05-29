---
title: Slice Type
type: claim
id: claim-slice-type
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt
confidence:
  base: 0.85
---

## Definition

A slice is a dynamically-sized view into a contiguous sequence of elements: `&[T]` for arrays/vectors, `&str` for UTF-8 strings. A slice carries a pointer and a length and does not own the underlying buffer. Slices are the idiomatic way to write APIs that accept "some run of contiguous elements" without forcing the caller to give up ownership.

## How It Works

A slice is represented as a fat pointer: `(ptr, len)`. `&v[a..b]` constructs a slice over indices `[a, b)` and is bounds-checked at construction. Slices borrow from their owner; the borrow checker forbids modifying the owner (e.g., `Vec::push`) while a slice exists. `&str` is the slice form of `String`; the type is just `&[u8]` with the UTF-8 invariant.

## Key Parameters

- Fat-pointer representation (`ptr`, `len`)
- Half-open ranges `a..b`, `..b`, `a..`, `..`
- Shared (`&[T]`) vs mutable (`&mut [T]`) slices
- `[T; N]` (fixed array) vs `&[T]` (slice) vs `Vec<T>` (owned heap buffer)

## When To Use

- Function parameters that should accept both arrays and vectors
- Substring operations on `String`/`&str`
- Lending a region of a buffer to a sub-routine
- Iteration over collection windows

## Risks & Pitfalls

- Holding a slice prevents the owner from being mutated (good!) but can confuse callers
- Multi-byte UTF-8 boundary panics for `&str` slices
- Forgetting that slices have a runtime length adds bounds-check costs in inner loops (optimizer usually removes them)

## Related Concepts

- [[concepts/string-type]]
- [[concepts/vec-type]]
- [[concepts/references]]
- [[concepts/borrowing]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
