---
title: Test Organization (Rust)
type: claim
id: concepts/test-organization
tags:
- rust
- foundational
- testing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/12-chapter-11-writing-automated-tests.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Rust distinguishes two test categories: **unit tests** live alongside production code in a `#[cfg(test)] mod tests { ... }` block and can access private items; **integration tests** live in the `tests/` directory and exercise only the public API as an external user would. Doc tests, embedded in `///` comments, form a third class.

## How It Works

Unit tests are conditionally compiled with `#[cfg(test)]`, so they do not exist in the release binary. Each file under `tests/` becomes its own crate, importing the library by name. Shared helpers go in `tests/common/mod.rs` (the `/mod.rs` form keeps them out of the auto-detected test list). Documentation tests run with `cargo test`, evaluating the code in doc comment fences as executable examples.

## Key Parameters

- Unit tests: same file, `#[cfg(test)]`, private access
- Integration tests: `tests/<name>.rs`, public API only
- Doc tests: in `///` comments, evaluate as examples
- Binary crates cannot host integration tests directly

## When To Use

- Unit tests for internal logic and private-function verification
- Integration tests for end-to-end public API contracts
- Doc tests for compile-checked examples in documentation

## Risks & Pitfalls

- Putting helpers in `tests/common.rs` (not `common/mod.rs`) makes them appear as tests themselves
- Doc tests run more slowly than unit tests; some teams gate them behind CI
- Binary-only crates lock you out of integration tests — extract logic into a library

## Related Concepts

- [[concepts/automated-tests]]
- [[concepts/cfg-attribute]]
- [[concepts/cargo]]
- [[concepts/modules]]

## Sources

- [[summaries/rust-book-12-chapter-11-writing-automated-tests]]
