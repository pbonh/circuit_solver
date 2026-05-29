---
title: Declarative Macros
type: claim
id: concepts/declarative-macros
tags:
- rust
- macros
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/20-chapter-19-advanced-features.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Declarative macros — defined with `macro_rules!` — are pattern-driven macros that match against the literal syntax of their arguments and emit substituted token streams. They are Rust's most accessible metaprogramming tool, used to write things like `vec![...]`, `println!(...)`, and `assert_eq!(...)`.

## How It Works

A `macro_rules!` definition consists of one or more match arms: `(matcher) => (transcriber)`. Matchers bind fragments of the input via "fragment specifiers" — `$name:expr`, `$name:tt`, `$name:ident`, `$name:ty`, etc. Repetition (`$( ... )*`, `$( ... ),*`) handles variadic shapes. Transcribers expand to Rust tokens that get parsed in the macro invocation's context. Declarative macros are partially hygienic — identifiers in the transcriber are scoped to the macro's definition, avoiding most variable-capture surprises.

## Key Parameters

- Fragment specifiers: `expr`, `tt`, `ident`, `ty`, `path`, `pat`, `stmt`, `block`, `item`, `meta`, `literal`
- Repetition `*`, `+`, `?`
- Separators (commas, semicolons) in repetition
- Hygiene scope
- `#[macro_export]` to expose across crates

## When To Use

- Variadic APIs (`vec![1, 2, 3]`, `println!(...)`)
- DSL fragments that don't need full proc-macro power
- Pattern-driven test helpers
- Boilerplate reduction inside a single crate

## Risks & Pitfalls

- Cryptic error messages from inside expansions
- Limited hygiene — outer identifiers can be captured
- Hard to debug; consider `cargo expand` to inspect output
- Logic in macros is harder to maintain than in functions

## Related Concepts

- [[concepts/macros]]
- [[concepts/procedural-macros]]
- [[concepts/derive-macros]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
