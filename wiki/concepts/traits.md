---
title: "Traits"
type: concept
tags: [rust, foundational, traits, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/11-chapter-10-generic-types-traits-and-lifetimes.txt"]
confidence: high
---

## Definition

A trait is a contract: a set of methods (and optionally associated types and constants) that any implementing type must provide. Traits are Rust's mechanism for shared behavior, replacing interfaces and abstract classes from OO languages. They power generic constraints, dynamic dispatch via trait objects, and operator overloading.

## How It Works

`trait Summary { fn summarize(&self) -> String; }` declares the contract. `impl Summary for Article { fn summarize(&self) -> String { ... } }` provides the implementation. Methods may have default bodies that subclasses can override. Trait methods can be called as `value.method()` after `use` brings the trait into scope or as `Trait::method(&value)`. Traits also enable dynamic dispatch via `Box<dyn Trait>` / `&dyn Trait`.

## Key Parameters

- Required methods (no body)
- Default methods (overridable)
- Associated types and constants
- Super-traits: `trait Sub: Super + Send`
- Object safety constraints (no `Self`-by-value, no generic methods, etc.)

## When To Use

- Sharing behavior across unrelated types
- Defining abstraction boundaries (`Iterator`, `Display`, `Read`)
- Operator overloading via `std::ops` traits
- Plug-in architectures with `Box<dyn Trait>`

## Risks & Pitfalls

- Orphan rule restricts where impls can live
- Object safety prevents some traits from becoming `dyn Trait`
- Default methods may rely on overridden methods in subtle ways
- Multiple traits with the same method name require disambiguation

## Related Concepts

- [[concepts/trait-bounds]]
- [[concepts/generics]]
- [[concepts/trait-objects]]
- [[concepts/impl-block]]
- [[concepts/orphan-rule]]

## Sources

- [[summaries/rust-book-11-chapter-10-generic-types-traits-and-lifetimes]]
- [[summaries/rust-book-22-appendix-a-keywords]]
- [[summaries/rust-book-24-appendix-c-derivable-traits]]
