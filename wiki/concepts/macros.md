---
title: Macros
type: claim
id: claim-macros
tags:
- rust
- macros
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/02-chapter-1-getting-started.txt
- raw/rust_book/_txt/20-chapter-19-advanced-features.txt
confidence:
  base: 0.85
---

## Definition

Macros are a metaprogramming feature in Rust that generate code at compile time. They come in two flavors: declarative macros (`macro_rules!`) that pattern-match on token trees, and procedural macros that execute arbitrary Rust code at compile time to transform input tokens.

## How It Works

Declarative macros (`macro_rules!`) use a matcher/transcriber syntax to bind fragments of input syntax and emit replacement token streams. Procedural macros are special crates that expose functions taking `TokenStream` and returning `TokenStream`; they come in three sub-flavors: function-like, derive, and attribute macros. The `!` in a call (e.g., `println!`) indicates a macro invocation rather than a function call; macros can take variadic arguments and reference identifiers that do not yet exist.

## Key Parameters

- `macro_rules!` patterns: `$name:tt`, `$name:expr`, `$name:ident`, `$name:ty`, etc.
- Repetition: `$( ... )*`, `$( ... ),*`
- Procedural macro kinds: function-like, `#[derive(...)]`, attribute-like
- Hygiene: declarative macros are partially hygienic; procedural macros require explicit care

## When To Use

- DSLs (small embedded languages) inside Rust code
- Boilerplate elimination across many types (often via `#[derive(...)]`)
- Variadic-like APIs (`println!`, `vec!`)
- Code generation that the type system alone cannot express

## Risks & Pitfalls

- Macros are harder to read and debug than functions
- Procedural macros slow compile times because they run an extra crate
- Error messages from inside macro expansion can be confusing
- Hygiene gotchas with identifier capture in `macro_rules!`

## Related Concepts

- [[concepts/rust-language]]
- [[concepts/derive-macros]]
- [[concepts/procedural-macros]]
- [[concepts/declarative-macros]]

## Sources

- [[summaries/rust-book-02-chapter-1-getting-started]]
- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-23-appendix-b-operators-and-symbols]]
