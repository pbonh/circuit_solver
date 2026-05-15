---
title: "The Rust Programming Language — Appendix B: Operators and Symbols"
type: summary
tags: [rust, reference, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/23-appendix-b-operators-and-symbols.txt"]
confidence: high
---

## Key Points

- Appendix B is a glossary of every Rust operator and syntactic symbol, with example usage, explanation, and the trait that overloads it (when overloading is possible).
- Arithmetic operators (`+`, `-`, `*`, `/`, `%`) and their compound-assignment variants (`+=`, `-=`, etc.) are overloadable via `std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, ...}`.
- Comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`) are overloadable via `PartialEq` / `PartialOrd`.
- Bitwise and shift operators (`&`, `|`, `^`, `<<`, `>>`) are overloadable via `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`.
- Logical operators `&&` and `||` are *not* overloadable; they short-circuit and are only defined on `bool`.
- Reference operators: `&expr`, `&mut expr`, `*expr` (dereference); the `&` and `*` symbols also appear in type syntax (`&T`, `&mut T`, `*const T`, `*mut T`).
- Pattern-related symbols: `..` (range/rest), `..=` (inclusive range), `|` (alternation in patterns), `@` (binding while matching), `_` (wildcard), `?` (the question-mark error-propagation operator outside patterns).
- Macro-related symbols: `!` after an identifier (macro invocation), `#[...]` outer attribute, `#![...]` inner attribute.
- Path and generics symbols: `::` (path separator and turbofish `::<T>`), `<>` (generic parameter brackets), `->` (function return type), `=>` (match arm separator).
- Lifetime syntax: `'a`, `'static`.

## Relevant Concepts

- [[concepts/operator-overloading]] — overloadable operators map to `std::ops` traits.
- [[concepts/question-mark-operator]] — `?` for `Result`/`Option` propagation.
- [[concepts/references]] — `&` / `&mut` / `*`.
- [[concepts/pattern-matching]] — `_`, `|`, `..`, `..=`, `@`.
- [[concepts/lifetimes]] — `'a` syntax.
- [[concepts/macros]] — `!` invocation suffix, `#[...]` attributes.

## Source Metadata

- Source type: book chapter (appendix)
- Book title: The Rust Programming Language
- Chapter: Appendix B — Operators and Symbols
- File path: `raw/rust_book/_txt/23-appendix-b-operators-and-symbols.txt`
- Authors: Steve Klabnik and Carol Nichols
