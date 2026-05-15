---
title: "Enum Type"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt", "raw/rust_book/_txt/07-chapter-6-enums-and-pattern-matching.txt"]
confidence: high
---

## Definition

A Rust enum is a sum type — a type that can be one of several named variants, each optionally carrying payload data of different shapes. Enums plus pattern matching give Rust algebraic-data-type semantics similar to ML or Haskell.

## How It Works

`enum Shape { Circle(f64), Rect { w: f64, h: f64 }, Point }` declares three variants. Values are constructed with `Shape::Circle(r)` etc. and consumed with `match` or `if let`. Internally, the compiler chooses a discriminant layout and pads variants so the enum has the size of the largest variant plus the discriminant; niche optimizations (e.g., `Option<&T>`) reuse invalid bit patterns to avoid the discriminant.

## Key Parameters

- Variants with named fields, tuple fields, or none
- Explicit discriminants for C-like enums (`#[repr(u32)]`)
- Generic parameters: `enum Result<T, E>`
- Recursive variants require indirection (`Box`)

## When To Use

- Modelling exclusive alternatives (parser tokens, AST nodes, state machines)
- Replacing class hierarchies from OO languages
- Encoding optional / fallible values via `Option<T>` / `Result<T, E>`

## Risks & Pitfalls

- Large variants cause memory bloat; consider `Box`-wrapping a few large ones
- Adding a variant is a breaking change for downstream `match` arms unless they used `_`
- Recursive types without indirection fail to compile

## Related Concepts

- [[concepts/pattern-matching]]
- [[concepts/option-type]]
- [[concepts/result-type]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-07-chapter-6-enums-and-pattern-matching]]
- [[summaries/rust-book-19-chapter-18-patterns-and-matching]]
