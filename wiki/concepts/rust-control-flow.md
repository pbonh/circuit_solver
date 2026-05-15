---
title: "Rust Control Flow"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/04-chapter-3-common-programming-concepts.txt"]
confidence: high
---

## Definition

Rust control flow is provided by `if` expressions (and `if let` sugar), three loop forms (`loop`, `while`, `for`), and the `match` pattern-matching construct. All branching constructs are expressions: they evaluate to a value usable in `let` bindings or as a function's return value.

## How It Works

`if condition { ... } else { ... }` requires a `bool` condition (no implicit truthiness) and yields the value of the executed branch. `loop` runs forever until `break`; `break value;` returns that value to the loop position. `while condition { ... }` loops while the condition holds. `for x in iter { ... }` iterates any `IntoIterator`. Loop labels (`'name: loop`) let inner `break`/`continue` target outer loops.

## Key Parameters

- Strict boolean condition typing (no truthiness)
- `break` with value to exit `loop`
- Loop labels for disambiguation
- `for` driven by `IntoIterator`
- `if`, `match`, `loop` are expressions

## When To Use

- `if`/`else` for binary or simple multi-way branching
- `match` for structural branching (preferred over chained `else if`)
- `for` for collection iteration (idiomatic)
- `while` when the bound is data-dependent and unknown
- `loop` when you need a labelled break or an event loop

## Risks & Pitfalls

- Branches of `if` must have the same type, or Rust complains
- Nested `break` targets the innermost loop unless labelled
- `for` consumes the iterator; if you need to keep iterating, take `&iter` or `iter_mut`

## Related Concepts

- [[concepts/pattern-matching]]
- [[concepts/statements-and-expressions]]
- [[concepts/iterators]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-04-chapter-3-common-programming-concepts]]
- [[summaries/rust-book-22-appendix-a-keywords]]
