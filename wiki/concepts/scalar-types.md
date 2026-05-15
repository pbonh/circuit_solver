---
title: "Scalar Types"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/04-chapter-3-common-programming-concepts.txt"]
confidence: high
---

## Definition

Scalar types are Rust's single-value primitive types: signed and unsigned integers, floating-point numbers, booleans, and characters. They are stored by value, are `Copy`, and form the building blocks of all compound and user-defined types.

## How It Works

Integers range from `i8`/`u8` up to `i128`/`u128`, plus `isize`/`usize` whose width matches the target pointer size. Floats are `f32` and `f64`, both IEEE-754. `bool` is one byte holding `true` or `false`. `char` is four bytes and represents a Unicode scalar value (not a byte or grapheme cluster). All scalars implement `Copy`, so assignment duplicates the value rather than moving it.

## Key Parameters

- Width: 8/16/32/64/128/size bits for integers
- Signedness: signed (`i*`) vs unsigned (`u*`)
- Numeric literals: type suffix (`42u32`), separators (`1_000_000`), bases (`0b`, `0o`, `0x`)
- Float precision: `f32` vs `f64`
- `char` is a Unicode scalar, not a byte

## When To Use

- Numerical kernels — pick width based on memory budget and precision
- `usize`/`isize` for indices and pointer arithmetic
- `f64` for most simulation math, `f32` for memory-bound or SIMD throughput

## Risks & Pitfalls

- Integer overflow: panics in debug, wraps in release — use `checked_*`/`saturating_*` to make intent explicit
- Float comparisons: never use `==` on `f32`/`f64`; respect NaN, infinity, and denormals
- `char` vs `u8`: a byte is not a Unicode character
- `usize` arithmetic underflow can be silent

## Related Concepts

- [[concepts/compound-types]]
- [[concepts/integer-overflow]]
- [[concepts/variables-and-mutability]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-04-chapter-3-common-programming-concepts]]
