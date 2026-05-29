---
title: Derive Macros
type: claim
id: claim-derive-macros
tags:
- rust
- macros
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/06-chapter-5-using-structs-to-structure-related-data.txt
- raw/rust_book/_txt/24-appendix-c-derivable-traits.txt
confidence:
  base: 0.85
---

## Definition

Derive macros are procedural macros invoked via the `#[derive(...)]` attribute. They synthesize trait implementations for the annotated type. The standard library provides derivable impls for `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, and `Default`; many crates provide custom derives.

## How It Works

Placing `#[derive(Debug, Clone)]` above a type causes the compiler to expand it into hand-written trait impls. Custom derives are defined in proc-macro crates declared with `#[proc_macro_derive(MyTrait)]`. They receive the parsed `TokenStream` of the type definition and emit an `impl` block as token output. The derived impl typically recurses on each field, requiring each field type to also implement the trait.

## Key Parameters

- Standard derivable traits (Appendix C lists them)
- Field-level constraints (every field must satisfy the derived trait)
- Custom derive crates (e.g., `serde::Serialize`, `serde::Deserialize`)
- Compile-time cost of expanding many derives

## When To Use

- `Debug` for any type used in tests or logs
- `Clone`/`Copy` when value semantics are needed
- `PartialEq`/`Eq` for comparison and hash map keys
- Serde derives for serialization
- Custom derives for project-specific traits (visitor patterns, type-level tags)

## Risks & Pitfalls

- Each derive expands inline — heavy use slows compile times
- Derived impls may not match what you want (e.g., `Debug` exposes private fields)
- `#[derive(Eq)]` requires `PartialEq` first; ordering of derives matters
- Custom derives can produce confusing error messages from inside the generated code

## Related Concepts

- [[concepts/macros]]
- [[concepts/procedural-macros]]
- [[concepts/traits]]
- [[concepts/debug-trait]]

## Sources

- [[summaries/rust-book-06-chapter-5-using-structs-to-structure-related-data]]
- [[summaries/rust-book-24-appendix-c-derivable-traits]]
