---
title: "Shadowing"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt", "raw/rust_book/_txt/04-chapter-3-common-programming-concepts.txt"]
confidence: high
---

## Definition

Shadowing is the Rust pattern of declaring a new variable that reuses the name of a prior binding. The new binding shadows (hides) the old one in the rest of the scope, optionally with a different type. Unlike mutation, shadowing creates a brand-new binding and so does not require `mut`.

## How It Works

Each `let name = ...;` introduces a fresh binding. If a binding with the same name already exists in scope, the new one shadows it. Because it is a new binding, the type can change. Shadowing is useful for stepwise transformations (`let guess: u32 = guess.trim().parse().expect("...");`) and for hygienic name reuse without making the final binding mutable.

## Key Parameters

- Scope-bound: shadowing is limited to the innermost block
- Allows type change between old and new bindings
- Distinct from `let mut`, which keeps a single mutable binding

## When To Use

- Type-changing transformations of a single conceptual value
- Avoiding contrived alternate names (`raw_guess` vs `guess`)
- Narrowing a value through parse/validate/convert steps

## Risks & Pitfalls

- Easy to confuse with mutation when reading code
- Aggressive shadowing can hide an earlier value the programmer thought was still in scope
- Lifetime of the shadowed value continues until end of scope, which can be surprising

## Related Concepts

- [[concepts/variables-and-mutability]]
- [[concepts/ownership]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-04-chapter-3-common-programming-concepts]]
