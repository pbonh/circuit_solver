---
title: "Object Safety"
type: concept
tags: [rust, traits, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/18-chapter-17-object-oriented-programming-features-of-rust.txt"]
confidence: high
---

## Definition

Object safety is the property that a trait can be used to form a trait object (`dyn Trait`). A trait is object-safe if all its methods satisfy specific constraints — primarily, no `Self` by value, no generic-method parameters, and no associated constants. Object-unsafe traits cannot be coerced to `dyn`.

## How It Works

The compiler checks object-safety when a value is coerced to `dyn Trait` or when a trait object type appears. Common rules: methods take `&self`, `&mut self`, or `Box<Self>`; methods are not generic (otherwise the vtable would need infinitely many entries); the trait has no associated constants by default. The 2024 edition relaxes some of these via "dyn-compatible" framing. Methods that would violate object-safety can be guarded with `where Self: Sized` to make only those methods unavailable on the trait object.

## Key Parameters

- Receiver kinds allowed: `&self`, `&mut self`, `Box<Self>`, others depending on impl
- Generic methods prohibited (workaround: `where Self: Sized`)
- Associated constants and `Self: Sized` bounds
- Auto-traits added to trait objects (`dyn Trait + Send + Sync`)

## When To Use

- Decision point when designing a trait that may be used dynamically
- Refactoring: split a trait into object-safe core + Sized helper trait when needed
- Library APIs that want to accept `Box<dyn Trait>` from callers

## Risks & Pitfalls

- Error messages around object safety are dense and confusing
- Adding a new method that breaks object-safety is a semver hazard
- Workarounds (e.g., `where Self: Sized`) split the API surface

## Related Concepts

- [[concepts/trait-objects]]
- [[concepts/dynamic-dispatch]]
- [[concepts/traits]]
- [[concepts/generics]]

## Sources

- [[summaries/rust-book-18-chapter-17-object-oriented-programming-features-of-rust]]
