---
title: Error Handling
type: claim
id: claim-error-handling
tags:
- rust
- foundational
- error-handling
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/10-chapter-9-error-handling.txt
- raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt
confidence:
  base: 0.85
---

## Definition

Error handling in Rust separates two kinds of failures: *unrecoverable* errors signalled by `panic!`, which terminate (or unwind) the thread, and *recoverable* errors expressed as `Result<T, E>` values that callers must explicitly handle. There is no exception mechanism; control flow stays visible in the types.

## How It Works

Library APIs return `Result` whenever failure is a normal outcome. Callers use `match`, combinators (`map_err`, `and_then`), or the `?` operator to propagate failure up the call stack. The `?` operator early-returns on `Err`, calling `From::from` to convert error types when needed. Panics may either unwind the stack (default) or abort the process (configured via `panic = "abort"` in `Cargo.toml`).

## Key Parameters

- `panic!` macro and `unwind` vs `abort` strategies
- `Result<T, E>` and `Option<T>` for recoverable errors and missing values
- `?` operator for ergonomic propagation
- Custom error enums vs `Box<dyn std::error::Error>`
- The `From` trait for error conversion

## When To Use

- Library code should almost always return `Result`
- Panic for invariant violations and unreachable code paths
- Recoverable errors when the caller has a sensible response (retry, alternative path)

## Risks & Pitfalls

- Over-reliance on `unwrap`/`expect` in production code
- Boxing `dyn Error` loses type information needed for specific handling
- Mixing panic and Result strategies inconsistently across a codebase
- Forgetting to set `panic = "abort"` for binary size if you never catch panics

## Related Concepts

- [[concepts/result-type]]
- [[concepts/option-type]]
- [[concepts/question-mark-operator]]
- [[concepts/panic]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-10-chapter-9-error-handling]]
- [[summaries/rust-book-13-chapter-12-an-i-o-project-building-a-command-line-program]]
