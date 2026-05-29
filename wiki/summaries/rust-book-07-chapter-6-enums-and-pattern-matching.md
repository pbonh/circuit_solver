---
title: 'The Rust Programming Language — Chapter 6: Enums and Pattern Matching'
type: source
id: source-rust-book-07-chapter-6-enums-and-pattern-matching
kind: derived-summary
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/07-chapter-6-enums-and-pattern-matching.txt
---

## Key Points

- Enums in Rust are sum types: each variant may carry differently-shaped payload data — `enum Message { Quit, Move { x:i32, y:i32 }, Write(String), ChangeColor(i32, i32, i32) }`.
- Enums replace many uses of class hierarchies from OO languages — different variants are alternative shapes of one "thing".
- The `Option<T>` enum (`Some(T)` or `None`) is Rust's null-replacement; it is exported in the prelude. The compiler refuses to use an `Option<T>` as if it were a `T`, forcing the programmer to handle the `None` case explicitly.
- `match` is the principal way to consume an enum: each variant gets an arm with optional binding of payload values, and the compiler enforces exhaustiveness.
- Pattern arms may destructure: `Coin::Quarter(state)` binds the inner state value to a variable named `state`.
- A catch-all named arm (`other => ...`) or the discard arm (`_ => ...`) covers the remaining cases. `_ => ()` does nothing.
- `if let pattern = value { ... } else { ... }` is sugar for a `match` with one interesting arm and a default, useful when only one variant matters.
- Exhaustiveness checking is load-bearing: when you add a new variant later, every `match` in the codebase that did not use `_` will refuse to compile until you handle the new variant — turning silent fall-through bugs into compile errors.
- Method syntax also works on enums: `impl Message { fn call(&self) { ... } }` defines methods that match internally.

## Relevant Concepts

- [[concepts/enum-type]] — sum types with payload variants.
- [[concepts/option-type]] — `Some` / `None` null replacement.
- [[concepts/pattern-matching]] — exhaustive `match`.
- [[concepts/if-let]] — sugar for one-variant match.
- [[concepts/null-safety]] — Rust avoids null pointers via `Option`.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 6 — Enums and Pattern Matching
- File path: `raw/rust_book/_txt/07-chapter-6-enums-and-pattern-matching.txt`
- Authors: Steve Klabnik and Carol Nichols
