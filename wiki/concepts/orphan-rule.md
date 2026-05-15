---
title: "Orphan Rule"
type: concept
tags: [rust, foundational, traits, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/11-chapter-10-generic-types-traits-and-lifetimes.txt"]
confidence: high
---

## Definition

The orphan rule is Rust's coherence constraint: you may implement a trait for a type only if either the trait or the type is defined in your current crate. The rule prevents conflicting "global" impls of the same trait for the same type from different crates and is essential for Rust's compilation model.

## How It Works

If both the trait and the type belong to upstream crates, a downstream crate cannot add an impl — even if it would be useful — because two different downstream crates could each add a conflicting impl, and the compiler has no way to pick one. The standard workaround is the *newtype pattern*: define a thin wrapper struct in your crate and implement the foreign trait for that.

## Key Parameters

- "Local" definition test (trait or type in current crate)
- Newtype-pattern workaround
- Interaction with blanket impls and specialization

## When To Use

- Always — the orphan rule is structural
- Newtype wrapping foreign types for foreign-trait impls
- Defining a local trait when you need behavior across many foreign types

## Risks & Pitfalls

- Surprising rejections when working across crate boundaries
- Workarounds add boilerplate
- Confusion with how some impls "feel" downstream when they actually live in `std`

## Related Concepts

- [[concepts/traits]]
- [[concepts/newtype-pattern]]
- [[concepts/blanket-impls]]
- [[concepts/crates]]

## Sources

- [[summaries/rust-book-11-chapter-10-generic-types-traits-and-lifetimes]]
