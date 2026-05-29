---
title: Debug Trait
type: claim
id: claim-debug-trait
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/06-chapter-5-using-structs-to-structure-related-data.txt
confidence:
  base: 0.85
---

## Definition

`std::fmt::Debug` is the standard trait for programmer-facing string representations. Types that implement `Debug` can be printed with the `{:?}` and `{:#?}` format specifiers. The trait is most often derived via `#[derive(Debug)]`.

## How It Works

`#[derive(Debug)]` recursively requires every field to implement `Debug`. A custom impl writes to a `fmt::Formatter` using helper methods such as `debug_struct`, `debug_tuple`, `debug_list`, `debug_map`. The `{:?}` specifier produces a single-line representation; `{:#?}` pretty-prints with newlines and indentation.

## Key Parameters

- Required helpers on `Formatter`: `debug_struct`, `debug_tuple`, `debug_list`, `debug_map`
- Single-line `{:?}` vs pretty `{:#?}`
- Distinct from `Display`, which is the user-facing trait

## When To Use

- Any type that may appear in `dbg!`, `println!("{:?}")`, panic messages, or test failures
- Library types — implementing `Debug` is essentially mandatory

## Risks & Pitfalls

- Derived `Debug` reveals private fields and can leak secrets in logs
- Cyclic data structures cause infinite recursion in `Debug` impls without explicit limits
- Large data dumps clutter logs — implement custom `Debug` for summary forms

## Related Concepts

- [[concepts/derive-macros]]
- [[concepts/traits]]
- [[concepts/display-trait]]

## Sources

- [[summaries/rust-book-06-chapter-5-using-structs-to-structure-related-data]]
- [[summaries/rust-book-24-appendix-c-derivable-traits]]
