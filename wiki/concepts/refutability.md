---
title: Refutability
type: claim
id: claim-refutability
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/19-chapter-18-patterns-and-matching.txt
confidence:
  base: 0.85
---

## Definition

Refutability classifies Rust patterns into two groups: *irrefutable* patterns always match (e.g., `let x = 5`); *refutable* patterns may fail to match (e.g., `Some(x)` against an `Option`, which is `None` half the time). The Rust compiler picks the appropriate category for each context — `let` and function parameters require irrefutable; `if let`, `while let`, and `match` arms accept refutable.

## How It Works

When a pattern context requires irrefutability, the compiler verifies the pattern matches every possible value of the scrutinee's type. `let Some(x) = maybe;` is rejected because `None` does not match. The fix is to use `if let Some(x) = maybe { ... } else { ... }`, the new `let ... else { ... return ... }` form (Rust 1.65+), or `match`. Conversely, an irrefutable pattern in an `if let` produces a warning because the `else` branch is dead.

## Key Parameters

- Irrefutable contexts: `let`, function/closure parameters, `for` loops
- Refutable contexts: `match` arms, `if let`, `while let`
- New: `let ... else { diverging-block }`
- Compiler warnings vs errors on mismatch

## When To Use

- Knowing this distinction explains many compile errors
- Choosing between `let-else` and `if let` for ergonomic early returns
- Designing API ergonomics around `Option`/`Result`

## Risks & Pitfalls

- "Refutable pattern in local binding" error is a common newcomer stumble
- Adding a new enum variant can turn a previously-irrefutable match arm refutable
- `let-else` blocks must diverge (panic, return, break, continue, loop)

## Related Concepts

- [[concepts/pattern-matching]]
- [[concepts/if-let]]
- [[concepts/enum-type]]

## Sources

- [[summaries/rust-book-19-chapter-18-patterns-and-matching]]
