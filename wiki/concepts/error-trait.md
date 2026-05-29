---
title: Error Trait
type: claim
id: concepts/error-trait
tags:
- rust
- foundational
- error-handling
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/10-chapter-9-error-handling.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

`std::error::Error` is the standard trait every error type should implement. It requires `Debug` and `Display` super-traits and offers an optional `source()` method returning the underlying cause, enabling error chains.

## How It Works

A custom error enum derives `Debug`, manually implements `Display`, and implements `Error` (often with `source()` returning the wrapped underlying error). `Box<dyn Error>` is a common signature for application code that wants to accept any error type. Libraries typically define their own concrete error enum and implement `From<InnerErr>` so `?` does the conversion automatically.

## Key Parameters

- Super-traits: `Debug + Display`
- Optional method: `source(&self) -> Option<&(dyn Error + 'static)>`
- Crate helpers: `thiserror` (derive), `anyhow` (application boxed errors)
- `Send + Sync + 'static` bounds common in async contexts

## When To Use

- Every public error type from a library
- Composing layered failures via `source` chains
- Cross-module error propagation with `?`

## Risks & Pitfalls

- `Box<dyn Error>` loses the concrete type; downcasting requires `Any`
- Forgetting `Send + Sync` blocks async use
- Re-using `String` as an error type drops structure and machine-readability

## Related Concepts

- [[concepts/error-handling]]
- [[concepts/result-type]]
- [[concepts/question-mark-operator]]
- [[concepts/traits]]

## Sources

- [[summaries/rust-book-10-chapter-9-error-handling]]
