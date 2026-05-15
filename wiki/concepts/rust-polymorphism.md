---
title: "Rust Polymorphism"
type: concept
tags: [rust, traits, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/18-chapter-17-object-oriented-programming-features-of-rust.txt"]
confidence: high
---

## Definition

Polymorphism in Rust is provided by traits, not by class hierarchies. Rust offers parametric polymorphism (generics + trait bounds, dispatched statically) and subtype-like polymorphism over a common trait (trait objects, dispatched dynamically). The book explicitly contrasts this with inheritance, which Rust intentionally omits.

## How It Works

Traits define behavior contracts; many types may implement the same trait. Functions can be polymorphic over `<T: Trait>` (static dispatch, monomorphized) or take `&dyn Trait` (dynamic dispatch, vtable). Composition replaces inheritance for code reuse: factor shared functionality into a smaller trait or struct field and combine, rather than extending.

## Key Parameters

- Static polymorphism: `fn foo<T: Trait>(x: T)`
- Dynamic polymorphism: `fn foo(x: &dyn Trait)`
- Default methods on traits provide one form of code sharing
- Auto traits (`Send`, `Sync`) compose orthogonally

## When To Use

- Static when performance and inlining matter and types are known at compile time
- Dynamic when heterogeneous collections or plugin loading are required
- Composition + traits in place of inheritance for code reuse

## Risks & Pitfalls

- Trying to recreate OO inheritance via deep trait hierarchies is awkward and often unidiomatic
- Misusing dynamic dispatch when static would do is a performance cost
- Object-safety constraints surprise developers expecting Java-style interfaces

## Related Concepts

- [[concepts/traits]]
- [[concepts/trait-objects]]
- [[concepts/dynamic-dispatch]]
- [[concepts/static-dispatch]]
- [[concepts/generics]]

## Sources

- [[summaries/rust-book-18-chapter-17-object-oriented-programming-features-of-rust]]
