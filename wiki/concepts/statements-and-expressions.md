---
title: Statements and Expressions
type: claim
id: concepts/statements-and-expressions
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/04-chapter-3-common-programming-concepts.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Rust distinguishes statements, which perform an action and do not yield a value, from expressions, which evaluate to a value. Function bodies are a sequence of statements optionally ending in an expression whose value is the function's return value. This expression orientation makes Rust closer to ML/Haskell than to C.

## How It Works

A `let` binding is a statement: `let x = 5;` does not produce a value, so `let y = (let x = 5);` is rejected. A block `{ a; b; final_expr }` is itself an expression evaluating to `final_expr`. `if`, `match`, and `loop` (with a labelled `break value;`) are expressions. Adding a trailing semicolon to a function's final expression turns it into a statement, changing the return type to `()`.

## Key Parameters

- Trailing-semicolon convention
- Block expressions
- `if`/`match`/`loop` as expressions
- Final-expression return rule

## When To Use

- Idiomatic Rust uses block expressions to compute values inline
- Match expressions to assign one of several values based on shape
- Avoid `return` for the common final-expression case

## Risks & Pitfalls

- Accidental semicolon at the end of a body makes the return type `()`
- New developers from C-family languages often miss the expression nature of `if`/`match`

## Related Concepts

- [[concepts/functions]]
- [[concepts/control-flow]]
- [[concepts/pattern-matching]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-04-chapter-3-common-programming-concepts]]
