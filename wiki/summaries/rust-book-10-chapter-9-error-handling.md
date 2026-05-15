---
title: "The Rust Programming Language — Chapter 9: Error Handling"
type: summary
tags: [rust, foundational, error-handling, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/10-chapter-9-error-handling.txt"]
confidence: high
---

## Key Points

- Rust separates *unrecoverable* errors (use `panic!`) from *recoverable* errors (return `Result<T, E>`). There are no exceptions.
- `panic!` unwinds the stack by default, running each frame's destructors, then aborts the thread; `panic = "abort"` in `Cargo.toml` skips unwinding to save binary size at the cost of cleanup.
- The `RUST_BACKTRACE=1` environment variable enables backtrace printing for panics — essential during debugging.
- `Result<T, E>` is consumed via `match`, `if let`, combinators (`map`, `map_err`, `and_then`), or the `?` operator.
- `unwrap` and `expect` panic on `Err`; prefer `expect("contextual message")` to give callers a useful failure message.
- Propagation: the `?` operator at the end of an expression returning `Result` early-returns the `Err` from the enclosing function; it calls `From::from` to convert error types when needed.
- `?` also works on `Option<T>`, propagating `None`.
- For functions returning multiple kinds of errors, options include: a custom enum that implements `Error`, `Box<dyn std::error::Error>`, or libraries like `anyhow` (for applications) and `thiserror` (for library error types).
- Guidelines: panic for invariant violations, prototype code, examples, and tests; return `Result` from library code where the caller has a reasonable response.
- Newtype validation pattern: a `struct Guess { value: i32 }` with a private field and a `Guess::new` that range-checks pushes validation into the type system so the rest of the program can rely on the invariant.

## Relevant Concepts

- [[concepts/error-handling]] — overall philosophy.
- [[concepts/result-type]] — recoverable errors.
- [[concepts/option-type]] — `?` also works here.
- [[concepts/question-mark-operator]] — error propagation sugar.
- [[concepts/panic]] — unrecoverable errors.
- [[concepts/newtype-pattern]] — for type-level validation.
- [[concepts/error-trait]] — `std::error::Error`.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 9 — Error Handling
- File path: `raw/rust_book/_txt/10-chapter-9-error-handling.txt`
- Authors: Steve Klabnik and Carol Nichols
