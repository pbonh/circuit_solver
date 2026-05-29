---
title: Integer Overflow
type: claim
id: concepts/integer-overflow
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

Integer overflow is what happens when an arithmetic operation produces a value outside the representable range of its integer type. Rust's behavior is deliberately split between debug builds (panic) and release builds (two's-complement wrap), with explicit methods to choose other policies.

## How It Works

In debug mode (the default for `cargo build` / `cargo run`), the compiler inserts overflow checks and panics with a message. In release mode (`cargo build --release`), overflow silently wraps. Programmers signal explicit intent with the methods `checked_*` (returns `Option`), `wrapping_*` (always wraps), `saturating_*` (clamps to MIN/MAX), and `overflowing_*` (returns `(value, did_overflow)`).

## Key Parameters

- Build profile: debug vs release
- Methods: `checked_add`, `wrapping_add`, `saturating_add`, `overflowing_add` (and similar for other ops)
- `Wrapping<T>` and `Saturating<T>` newtype wrappers for operator-level intent

## When To Use

- Use `checked_*` when overflow is a recoverable bug
- Use `wrapping_*` for hash functions, CRCs, modular arithmetic
- Use `saturating_*` for pixel arithmetic and audio
- Use `overflowing_*` when both the wrapped result and the carry flag are needed

## Risks & Pitfalls

- Relying on debug panics masks bugs in release
- `usize` underflow (e.g., subtracting from zero) is a common source of silent wrap-around
- Mixing signed and unsigned arithmetic can hide overflow

## Related Concepts

- [[concepts/scalar-types]]
- [[concepts/rust-language]]
- [[concepts/memory-safety]]

## Sources

- [[summaries/rust-book-04-chapter-3-common-programming-concepts]]
