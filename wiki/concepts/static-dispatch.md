---
title: Static Dispatch (Rust)
type: claim
id: concepts/static-dispatch
tags:
- rust
- traits
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/18-chapter-17-object-oriented-programming-features-of-rust.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Static dispatch is the compile-time selection of which method implementation to call, achieved via Rust generics with trait bounds. The compiler monomorphizes the generic function for each concrete type, producing a direct call that can be inlined.

## How It Works

`fn render<T: Draw>(items: &[T])` instantiates one copy of `render` per concrete `T`. Each instantiation contains direct calls to the appropriate `T::draw` implementation. The compiler can inline across the abstraction boundary, often producing code identical to what a hand-written non-generic version would emit. Static dispatch is Rust's default whenever trait bounds are used without `dyn`.

## Key Parameters

- Generic parameter with trait bound: `<T: Trait>` or `where T: Trait`
- `impl Trait` argument-position sugar
- Monomorphization per call site
- Binary size cost vs runtime speed trade-off

## When To Use

- Hot-path code where inlining and direct calls matter
- Generic library code where ergonomic monomorphization is the goal
- When the set of types is known at compile time

## Risks & Pitfalls

- Binary bloat for many concrete instantiations
- Longer compile times
- Cannot store mixed concrete types in a homogeneous container (use `dyn Trait` instead)

## Related Concepts

- [[concepts/traits]]
- [[concepts/dynamic-dispatch]]
- [[concepts/monomorphization]]
- [[concepts/generics]]
- [[concepts/trait-bounds]]

## Sources

- [[summaries/rust-book-18-chapter-17-object-oriented-programming-features-of-rust]]
