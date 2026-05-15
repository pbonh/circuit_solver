---
title: "The Rust Programming Language — Chapter 11: Writing Automated Tests"
type: summary
tags: [rust, foundational, testing, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/12-chapter-11-writing-automated-tests.txt"]
confidence: high
---

## Key Points

- A test in Rust is any function annotated with `#[test]`. `cargo test` builds a test binary and runs all such functions, reporting pass/fail and time.
- Test assertions: `assert!(condition)`, `assert_eq!(left, right)`, `assert_ne!(left, right)` — each can be followed by a custom format-string message.
- `assert_eq!`/`assert_ne!` print both values on failure (they require the types to implement `PartialEq` and `Debug`); arguments are conventionally called `left` and `right`.
- `#[should_panic]` marks tests that pass when the body panics. Adding `expected = "substring"` checks the panic message contains the substring.
- Returning `Result<(), E>` from a test allows using `?`; a test passes when it returns `Ok(())` and fails on `Err`.
- `cargo test` runs tests in parallel by default; `--test-threads=1` runs sequentially. Test output is captured and only shown on failure; `--nocapture` shows it always.
- Filtering: `cargo test foo` runs tests whose name contains "foo"; `#[ignore]` skips a test until `cargo test -- --ignored` runs only the ignored set.
- Test organization splits into **unit tests** (same file as the production code, inside a `#[cfg(test)] mod tests` block — private-item access) and **integration tests** (separate files under `tests/`, treating the crate as an external user would).
- Integration tests live in `tests/<file>.rs`; each file becomes its own crate, so they exercise only the public API of the library.
- Helper modules under `tests/common/mod.rs` (note: must be in a subdirectory) are shared across integration tests without being themselves treated as test entry points.
- Binary-only crates cannot have integration tests by default — that is one reason to extract logic into `src/lib.rs` and keep `src/main.rs` minimal.

## Relevant Concepts

- [[concepts/automated-tests]] — the `#[test]` attribute and `cargo test` workflow.
- [[concepts/test-organization]] — unit vs integration tests.
- [[concepts/assertions]] — `assert!`, `assert_eq!`, `assert_ne!`.
- [[concepts/cfg-attribute]] — `#[cfg(test)]` conditional compilation.
- [[concepts/cargo]] — `cargo test` orchestration.
- [[concepts/result-type]] — tests returning `Result`.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 11 — Writing Automated Tests
- File path: `raw/rust_book/_txt/12-chapter-11-writing-automated-tests.txt`
- Authors: Steve Klabnik and Carol Nichols
