---
title: "Monomorphization"
type: concept
tags: [rust, foundational, performance, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/11-chapter-10-generic-types-traits-and-lifetimes.txt"]
confidence: high
---

## Definition

Monomorphization is the compile-time process by which Rust turns generic code into one specialized copy per concrete type substitution used in the program. The result is that generics impose no runtime overhead — each call site dispatches statically to a specialized function.

## How It Works

When the compiler encounters `Vec::push::<i32>` and `Vec::push::<f64>`, it generates a separate code path for each. Method dispatch on a generic parameter becomes a direct call to the appropriate specialization. This contrasts with type erasure (Java generics) and dynamic dispatch (`dyn Trait`). The downside is binary-size growth proportional to the number of instantiations, and longer compile times because each instantiation goes through codegen.

## Key Parameters

- Instantiation set: every (type-parameter, concrete-type) combination
- Inlining heuristics decide whether specializations are merged at link time
- Generic code in dependencies is monomorphized into the dependent crate
- LTO (link-time optimization) can deduplicate

## When To Use

- Default — generics in Rust use monomorphization unless `dyn Trait` is chosen
- Performance-critical code where static dispatch matters
- Wherever inlining across abstraction boundaries is important

## Risks & Pitfalls

- Binary bloat from many specializations of large generic functions
- Compile-time growth, especially in deeply generic libraries
- Hidden costs when generic functions get inlined repeatedly

## Related Concepts

- [[concepts/generics]]
- [[concepts/traits]]
- [[concepts/zero-cost-abstractions]]
- [[concepts/trait-objects]]

## Sources

- [[summaries/rust-book-11-chapter-10-generic-types-traits-and-lifetimes]]
