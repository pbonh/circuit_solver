---
title: Lifetimes
type: claim
id: concepts/lifetimes
tags:
- rust
- lifetimes
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt
- raw/rust_book/_txt/11-chapter-10-generic-types-traits-and-lifetimes.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

A lifetime is a compile-time region during which a reference is guaranteed to be valid. Rust attaches a lifetime parameter (e.g., `'a`) to every reference, either explicitly in function signatures and struct definitions or implicitly via *lifetime elision*. The borrow checker uses these regions to prove that no reference outlives the data it points to.

## How It Works

A function signature like `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` says: the returned reference lives at least as long as the shorter of `x` and `y`. The compiler then verifies, at each call site, that the actual references satisfy this constraint. Three elision rules cover most signatures so explicit lifetimes are rarely needed. Structs holding references must declare a lifetime parameter (`struct Token<'src> { text: &'src str }`).

## Key Parameters

- Lifetime parameters `<'a, 'b>`
- Lifetime elision rules (one input ref → one output ref; `&self` → output ref; etc.)
- `'static` — the lifetime of the entire program (e.g., string literals)
- Higher-Ranked Trait Bounds (`for<'a> Fn(&'a T) -> ...`)
- Non-Lexical Lifetimes (NLL) — borrows end at last use, not end of scope

## When To Use

- Storing references in structs (mandatory)
- Returning references from functions where elision is ambiguous
- Trait objects involving references (`Box<dyn Trait + 'a>`)
- Modeling lifetime-sensitive APIs (parsers, iterators)

## Risks & Pitfalls

- Confusing or cryptic borrow-checker errors that mention lifetimes
- Over-constraining with explicit lifetimes that elision would have inferred
- Trying to return references to local data — a dangling-reference compile error
- `'static` is often misused as "I don't care about the lifetime"

## Related Concepts

- [[concepts/ownership]]
- [[concepts/borrowing]]
- [[concepts/references]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
- [[summaries/rust-book-11-chapter-10-generic-types-traits-and-lifetimes]]
- [[summaries/rust-book-13-chapter-12-an-i-o-project-building-a-command-line-program]]
- [[summaries/rust-book-23-appendix-b-operators-and-symbols]]
