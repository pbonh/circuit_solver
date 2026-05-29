---
title: Dynamic Dispatch (Rust)
type: claim
id: concepts/dynamic-dispatch
tags:
- rust
- traits
- advanced
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

Dynamic dispatch is the runtime selection of which method implementation to call based on the concrete type behind a trait object. In Rust, it is opt-in via `dyn Trait`: calls through a `&dyn Trait` or `Box<dyn Trait>` look up the method address in a vtable at run time.

## How It Works

When a value is coerced to `dyn Trait`, the compiler constructs a fat pointer: (data pointer, vtable pointer). Each trait method call indexes the vtable to fetch the function pointer and jumps to it. The compiler does not inline these calls. The cost is one indirect branch and a missed inlining opportunity per call, traded for the ability to mix concrete types behind one interface.

## Key Parameters

- Pointer forms: `&dyn Trait`, `Box<dyn Trait>`, `Arc<dyn Trait>`
- vtable per (Trait, ConcreteType) pair
- Object-safety constraints on the trait
- Lifetime bounds on the trait object

## When To Use

- Heterogeneous storage of values implementing a common trait
- Plugin architectures where types are unknown until run time
- APIs that want a non-generic signature
- Reducing binary bloat by avoiding monomorphization

## Risks & Pitfalls

- Per-call indirect branch can hurt micro-benchmarks
- Lost inlining means worse downstream optimization
- Object-safety constraints disqualify some traits
- Mixing dynamic and static dispatch in a hot path may not give expected speedups

## Related Concepts

- [[concepts/trait-objects]]
- [[concepts/static-dispatch]]
- [[concepts/object-safety]]
- [[concepts/traits]]
- [[concepts/monomorphization]]

## Sources

- [[summaries/rust-book-18-chapter-17-object-oriented-programming-features-of-rust]]
