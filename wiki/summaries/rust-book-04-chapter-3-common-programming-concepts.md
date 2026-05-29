---
title: 'The Rust Programming Language — Chapter 3: Common Programming Concepts'
type: source
id: source-rust-book-04-chapter-3-common-programming-concepts
kind: derived-summary
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/04-chapter-3-common-programming-concepts.txt
---

## Key Points

- Variables introduced with `let` are immutable by default; `let mut` opts in to mutation; `const` declares always-immutable, type-annotated, compile-time-known constants valid for an entire program.
- Shadowing differs from `mut`: it creates a new binding (possibly with a different type) and the previous value is dropped at the end of its scope. Shadowing avoids inventing new names like `guess_str` and `guess_num`.
- Rust is statically typed and infers most types; ambiguous cases (e.g., parsing strings) require explicit annotations.
- Scalar types: integers (`i8..i128`, `u8..u128`, `isize`, `usize`), floats (`f32`, `f64` — IEEE-754), booleans, and characters (`char`, 4-byte Unicode scalar value).
- Compound types: tuples (fixed-size heterogeneous) and arrays (fixed-size homogeneous, stack-allocated, bounds-checked at runtime).
- Integer overflow panics in debug builds and wraps (two's complement) in release builds; explicit `wrapping_*`, `checked_*`, `overflowing_*`, `saturating_*` methods make intent explicit.
- Functions use `snake_case`; parameters must be type-annotated; the function body is a series of statements optionally ending in an expression that is the return value.
- Statements (`let x = 1;`) do not return values; expressions (`x + 1`, blocks, `if`) do. Returning early uses `return`; the final expression is returned implicitly.
- Comments use `//` for line comments and `///` for doc comments (covered later).
- `if/else if/else` is an expression — branches must yield the same type — and the condition must be a `bool` (no implicit truthiness).
- Three loop constructs: `loop` (infinite, `break value;` returns from a loop), `while`, and `for ... in iter`. Loop labels (`'outer: loop`) disambiguate `break`/`continue` in nested loops.

## Relevant Concepts

- [[concepts/variables-and-mutability]] — `let`, `mut`, `const`, scope.
- [[concepts/shadowing]] — rebinding a name with a new type.
- [[concepts/scalar-types]] — integers, floats, bool, char.
- [[concepts/compound-types]] — tuples and arrays.
- [[concepts/integer-overflow]] — debug-panic vs release-wrap semantics.
- [[concepts/functions]] — fn syntax, parameters, return values.
- [[concepts/statements-and-expressions]] — expression-oriented language.
- [[concepts/rust-control-flow]] — if expressions, loops, labels.
- [[concepts/rust-language]] — overall context.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 3 — Common Programming Concepts
- File path: `raw/rust_book/_txt/04-chapter-3-common-programming-concepts.txt`
- Authors: Steve Klabnik and Carol Nichols
