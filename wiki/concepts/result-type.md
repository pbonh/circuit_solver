---
title: Result Type
type: claim
id: claim-result-type
tags:
- rust
- foundational
- error-handling
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt
- raw/rust_book/_txt/10-chapter-9-error-handling.txt
confidence:
  base: 0.85
---

## Definition

`Result<T, E>` is Rust's standard-library enum for recoverable error handling. It has two variants: `Ok(T)` carrying a successful value and `Err(E)` carrying an error value. Functions that can fail return a `Result`, forcing the caller to address the error path.

## How It Works

`Result` is an ordinary enum, so it is consumed via pattern matching or via combinator methods. Common methods: `unwrap`, `expect`, `is_ok`, `is_err`, `map`, `map_err`, `and_then`, `or_else`, and the `?` operator that early-returns the `Err`. The compiler's `#[must_use]` attribute on `Result` triggers a warning if a returned `Result` is ignored.

## Key Parameters

- Generic parameters `T` (success) and `E` (error)
- Common conversion: `From<E>` impls combined with `?`
- Pairing with custom error enums or `Box<dyn Error>`

## When To Use

- Any operation that can fail in a recoverable way (I/O, parsing, network)
- Library boundaries where callers must decide how to react
- In place of exceptions found in other languages

## Risks & Pitfalls

- Reaching for `unwrap`/`expect` in production code can panic
- Mixing many error types complicates `?`; needs `From` impls or `Box<dyn Error>`
- Forgetting `#[must_use]` warning leads to silently ignored failures

## Related Concepts

- [[concepts/error-handling]]
- [[concepts/option-type]]
- [[concepts/pattern-matching]]
- [[concepts/enum-type]]
- [[concepts/question-mark-operator]]

## Sources

- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-10-chapter-9-error-handling]]
- [[summaries/rust-book-12-chapter-11-writing-automated-tests]]
