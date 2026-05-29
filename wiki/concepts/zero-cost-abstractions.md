---
title: Zero-Cost Abstractions
type: claim
id: concepts/zero-cost-abstractions
tags:
- rust
- foundational
- performance
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/01-introduction.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Zero-cost abstractions are higher-level language features that compile down to machine code no slower than the equivalent hand-written lower-level code. Rust's design ethos is that you should not have to pay (in runtime performance) for what you do not use, and what you do use should be as efficient as if you had written it by hand.

## How It Works

Rust achieves zero-cost abstractions via monomorphization of generics, aggressive inlining, and a type system that allows the compiler to prove away runtime checks. Iterator chains, traits, and `Option`/`Result` types typically optimize down to the same assembly as imperative loops with raw pointers.

## Key Parameters

- Monomorphization of generics into concrete types
- Inlining across crate boundaries (with `#[inline]` hints)
- Static dispatch via traits with generic parameters
- No implicit allocation or hidden indirection

## When To Use

- Performance-critical inner loops (numerical kernels, simulators)
- Library APIs where ergonomics must not cost speed
- Anywhere you would otherwise drop to C for raw performance

## Risks & Pitfalls

- Monomorphization can balloon binary size
- "Zero-cost" depends on the optimizer; debug builds are not zero-cost
- Trait objects (`dyn Trait`) use dynamic dispatch and are not zero-cost in the same way

## Related Concepts

- [[concepts/rust-language]]
- [[concepts/traits]]
- [[concepts/generics]]

## Sources

- [[summaries/rust-book-01-introduction]]
- [[summaries/rust-book-14-chapter-13-functional-language-features-iterators-and-closures]]
