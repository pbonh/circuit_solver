---
title: "Blanket Impls"
type: concept
tags: [rust, foundational, traits, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/11-chapter-10-generic-types-traits-and-lifetimes.txt"]
confidence: high
---

## Definition

A blanket impl is a generic trait implementation that applies to every type meeting a constraint — for example, `impl<T: Display> ToString for T`. Blanket impls are how the standard library makes wide-ranging conveniences (every `Display` type is `ToString`, every iterator gets adapter methods) available automatically.

## How It Works

A blanket impl in the standard library means downstream users do not have to write `impl ToString` for their own types; satisfying `Display` is enough. The compiler proves uniqueness via the coherence/orphan rules — at most one impl can apply for a given concrete type. Blanket impls also enable conditional methods through the `impl<T: Bound> Type<T>` pattern.

## Key Parameters

- Generic parameter and its bound
- Trait being implemented (subject to orphan rule)
- Interaction with specialization (unstable in current Rust)

## When To Use

- Library design where one trait should imply another for free
- Adding extension methods to many existing types in your crate
- Bridging two ecosystems where every type implementing trait A should also implement trait B

## Risks & Pitfalls

- Blanket impls can prevent downstream users from providing their own impls (coherence)
- Adding a blanket impl is a major breaking change for downstream consumers
- Specialization to override blanket impls is not yet stable

## Related Concepts

- [[concepts/traits]]
- [[concepts/trait-bounds]]
- [[concepts/orphan-rule]]
- [[concepts/generics]]

## Sources

- [[summaries/rust-book-11-chapter-10-generic-types-traits-and-lifetimes]]
