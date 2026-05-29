---
title: 'The Rust Programming Language — Chapter 18: Patterns and Matching'
type: source
id: summaries/rust-book-19-chapter-18-patterns-and-matching
kind: publication
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/19-chapter-18-patterns-and-matching.txt
---

## Key Points

- Patterns are a special Rust syntax for matching against the structure of types — they appear in many places: `match` arms, `if let`, `while let`, `for ... in ...`, `let` bindings, function parameters, and the new `let ... else { ... }` form.
- All places where patterns appear fall into two categories: those that accept *refutable* patterns (which may fail to match — `if let`, `while let`, `match` arms) and those that accept only *irrefutable* patterns (`let`, function parameters, `for` loops).
- Pattern syntax covers literals (`1 | 2 | 3`), named variables (`x`), wildcards (`_`), ranges (`1..=5`), structs (`Point { x, y }`), tuples, enums, references (`&v`), and rest (`..`).
- `@` bindings let you both test against a pattern and bind the matched value: `id @ 3..=7 => ...`.
- `match` is exhaustive; a missing case is a compile error unless `_` catches everything else.
- Destructuring exposes nested fields ergonomically: `let Message::Move { x, y } = msg;`.
- Ignoring values: `_` ignores a single value (no binding, no consumption); `..` skips multiple parts of a struct or tuple; `_var` binds but suppresses unused-variable warnings.
- Match guards (`if cond` on an arm) allow extra runtime conditions, but they bypass exhaustiveness checking.
- Variables in patterns *shadow*, so be careful: `match v { Some(y) => ... }` binds a fresh `y` even if an outer `y` is in scope.

## Relevant Concepts

- [[concepts/pattern-matching]] — `match`, `if let`, `while let`, destructuring.
- [[concepts/if-let]] — single-pattern sugar.
- [[concepts/enum-type]] — primary source of destructurable variants.
- [[concepts/refutability]] — the refutable vs irrefutable distinction.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 18 — Patterns and Matching
- File path: `raw/rust_book/_txt/19-chapter-18-patterns-and-matching.txt`
- Authors: Steve Klabnik and Carol Nichols
