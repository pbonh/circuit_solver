---
title: Trait Objects
type: claim
id: concepts/trait-objects
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

A trait object is a type-erased pointer to a value of any type that implements a given trait, written `dyn Trait` and typically held behind a pointer (`&dyn Trait`, `Box<dyn Trait>`, `Arc<dyn Trait>`). Trait objects enable dynamic dispatch and heterogeneous collections.

## How It Works

A trait object is a fat pointer: one pointer to the data, one to a vtable containing function pointers for the trait's methods. Calls go through the vtable, so they are not inlined. Trait objects require their trait to be *object-safe*: no `Self`-by-value receiver, no generic methods, no associated constants (some constraints have been relaxed in newer Rust). Sized vs unsized: `dyn Trait` is unsized; it must always live behind a pointer.

## Key Parameters

- Pointer forms: `&dyn Trait`, `Box<dyn Trait>`, `Arc<dyn Trait>`
- Vtable layout (one pointer per method plus type metadata)
- Object-safety constraints
- Lifetime annotation: `Box<dyn Trait + 'a>`

## When To Use

- Heterogeneous collections (`Vec<Box<dyn Draw>>`)
- Plugin / extensibility APIs where types are unknown at compile time
- Reducing monomorphization-driven binary bloat
- Public APIs that should accept any conforming type without exposing generics

## Risks & Pitfalls

- Indirect call through vtable — slower than static dispatch in tight loops
- Object-safety errors can be obscure
- Storing `dyn Trait` requires deciding between `Send`/`Sync` annotations
- Coercion from concrete type to trait object can be surprising

## Related Concepts

- [[concepts/traits]]
- [[concepts/dynamic-dispatch]]
- [[concepts/static-dispatch]]
- [[concepts/object-safety]]

## Sources

- [[summaries/rust-book-18-chapter-17-object-oriented-programming-features-of-rust]]
