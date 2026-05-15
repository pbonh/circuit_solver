---
title: "Tuple Struct"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/06-chapter-5-using-structs-to-structure-related-data.txt"]
confidence: high
---

## Definition

A tuple struct is a named type that holds a fixed sequence of positionally-accessed fields. It is declared like a tuple but is its own distinct type, e.g., `struct Color(u8, u8, u8);`. Tuple structs are typically used as lightweight newtypes or wrappers.

## How It Works

`struct Wrapper(InnerType);` creates a new type that the compiler considers distinct from `InnerType`. Field access uses positional indices: `w.0`. Unit-like variants (`struct Marker;`) are tuple structs with zero fields and are useful with traits. The newtype pattern uses a tuple struct to enforce stronger typing on otherwise-identical primitive types (e.g., `struct Volts(f64);` vs `struct Amperes(f64);`).

## Key Parameters

- Number of positional fields
- Public/private fields with `pub`
- Use case categories: newtype, unit-like, tag

## When To Use

- Newtype wrappers for type-level units (Volts, Amperes, MeshNodeId)
- Public marker types
- Lightweight pairs/triples that should not implicitly interconvert
- Zero-size types used for type-level dispatch

## Risks & Pitfalls

- Positional access is opaque — name fields once they exceed two-three
- Wrappers add ergonomic friction (constant `.0` access) unless `Deref`-impl'd
- Newtypes do not inherit traits from the inner type automatically

## Related Concepts

- [[concepts/struct-type]]
- [[concepts/newtype-pattern]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-06-chapter-5-using-structs-to-structure-related-data]]
