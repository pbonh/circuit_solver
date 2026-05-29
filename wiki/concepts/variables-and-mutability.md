---
title: Variables and Mutability
type: claim
id: claim-variables-and-mutability
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt
- raw/rust_book/_txt/04-chapter-3-common-programming-concepts.txt
confidence:
  base: 0.85
---

## Definition

In Rust, variables declared with `let` are immutable by default; once bound, their value cannot change. Adding `mut` (`let mut x = 5;`) opts the binding into mutability. Constants, declared with `const`, are always immutable, must be type-annotated, and are inlined at compile time.

## How It Works

`let` introduces a new immutable binding in the current scope. `let mut` makes the binding mutable. Constants use `const NAME: TYPE = expr;` and must be initializable from a constant expression. Static items (`static`) hold a fixed memory address and can be mutable only inside `unsafe` blocks. Immutability by default eliminates a class of bugs where state changes unexpectedly and improves the compiler's ability to reason about and optimize the program.

## Key Parameters

- `let` vs `let mut`
- `const NAME: T = ...`
- `static NAME: T = ...`
- Type inference vs explicit annotation
- Block scoping and shadowing

## When To Use

- Default to immutable bindings; opt into `mut` only where needed
- `const` for compile-time constants (PI, default tolerances)
- `static` for global mutable state behind synchronization (rarely)

## Risks & Pitfalls

- Over-use of `mut` defeats the safety value of the default
- Confusion between shadowing and mutation
- `const` and `static` have subtly different semantics; mixing them up causes surprising behavior

## Related Concepts

- [[concepts/shadowing]]
- [[concepts/ownership]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-04-chapter-3-common-programming-concepts]]
- [[summaries/rust-book-22-appendix-a-keywords]]
