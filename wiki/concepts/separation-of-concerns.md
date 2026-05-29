---
title: Separation of Concerns
type: claim
id: claim-separation-of-concerns
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/13-chapter-12-an-i-o-project-building-a-command-line-program.txt
confidence:
  base: 0.65
---

## Definition

Separation of concerns is the design principle of partitioning a program so that each unit handles one well-defined responsibility. In the Rust book's minigrep example, `src/main.rs` handles only CLI parsing and error reporting; `src/lib.rs` holds the pure logic — so the library can be tested in isolation.

## How It Works

The pattern moves all non-I/O logic out of `main` into a library crate. `main` becomes a thin shim that builds a `Config`, calls a `run(config)` function, and prints any error. Doing so makes the logic accessible to unit tests, integration tests, doc tests, and benchmarks; binary crates by themselves cannot host integration tests.

## Key Parameters

- Boundary between I/O and pure logic
- Library crate vs binary crate split
- `Config` type carries parsed arguments
- `run` function returns `Result` so errors bubble out

## When To Use

- Any non-trivial CLI tool
- Programs that need integration tests against their core behavior
- Code that may be reused as a library by other crates
- Reducing the surface that depends on side effects

## Risks & Pitfalls

- Over-extraction creates ceremony for tiny programs
- Hiding side effects behind extra abstraction can obscure call sites
- Test-only abstractions can leak into production via traits and feature flags

## Related Concepts

- [[concepts/crates]]
- [[concepts/test-organization]]
- [[concepts/cli-argument-parsing]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-13-chapter-12-an-i-o-project-building-a-command-line-program]]
