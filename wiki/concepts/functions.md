---
title: Functions
type: claim
id: concepts/functions
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

Functions in Rust are first-class units of computation declared with `fn`. They take typed parameters and return a single value (possibly the unit type `()`). Functions are statically typed and may be generic, may capture lifetimes, and integrate with traits to support polymorphism.

## How It Works

`fn name(p1: T1, p2: T2) -> R { body }` declares a function. Each parameter requires an explicit type. The body is a block whose final expression (without trailing semicolon) becomes the return value; `return expr;` returns early. Naming convention is `snake_case`. Functions can be associated with types (`impl Type { fn ... }`) as inherent methods or associated functions.

## Key Parameters

- Parameter type annotations (mandatory)
- Return type (defaults to `()` if omitted)
- Generic parameters `<T>` and lifetime parameters `<'a>`
- Trait bounds for polymorphic functions

## When To Use

- Always — functions are the principal abstraction in Rust
- Pull out reusable logic, hide complexity
- Pair with traits for static or dynamic dispatch

## Risks & Pitfalls

- Forgetting the missing semicolon turns the final expression into a statement returning `()`
- Lifetime elision rules can confuse newcomers
- Generic explosion via monomorphization can slow compile times

## Related Concepts

- [[concepts/statements-and-expressions]]
- [[concepts/traits]]
- [[concepts/generics]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-04-chapter-3-common-programming-concepts]]
