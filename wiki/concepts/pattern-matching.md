---
title: Pattern Matching
type: claim
id: concepts/pattern-matching
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt
- raw/rust_book/_txt/07-chapter-6-enums-and-pattern-matching.txt
- raw/rust_book/_txt/19-chapter-18-patterns-and-matching.txt
confidence:
  base: 0.95
  source_count: 3
  contradicted: false
  effective: 1.026
  inputs_hash: 5a90c05bd2bbff0d
---

## Definition

Pattern matching is Rust's mechanism for destructuring values and dispatching on shape. Implemented primarily via the `match` expression, it is exhaustive (the compiler ensures every variant is handled) and integrates with `if let`, `while let`, function parameters, and `let` bindings.

## How It Works

A `match` expression takes a scrutinee and a series of arms `pattern => expression`. Patterns can match literals, ranges (`1..=5`), variant constructors with destructured payloads (`Some(x)`, `Point { x, y }`), wildcards (`_`), bindings with guards (`x if x > 0`), and references. The compiler computes a decision tree, verifies exhaustiveness, and rejects unreachable arms.

## Key Parameters

- Exhaustiveness checking
- Guards (`if condition` in an arm)
- Bindings with `@` (`n @ 1..=10`)
- Nested destructuring of structs, enums, tuples, arrays
- `if let` / `while let` sugar for single-pattern cases

## When To Use

- Branching on enum variants (`Option`, `Result`, custom enums)
- Destructuring complex data into named bindings
- Replacing chains of `if/else if` on value shape
- Implementing state machines

## Risks & Pitfalls

- Wildcards (`_`) can mask future variants when the enum gains members
- Heavy pattern matching can obscure control flow
- Guards make exhaustiveness reasoning harder

## Related Concepts

- [[concepts/enum-type]]
- [[concepts/option-type]]
- [[concepts/result-type]]
- [[concepts/if-let]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-07-chapter-6-enums-and-pattern-matching]]
- [[summaries/rust-book-19-chapter-18-patterns-and-matching]]
- [[summaries/rust-book-23-appendix-b-operators-and-symbols]]
