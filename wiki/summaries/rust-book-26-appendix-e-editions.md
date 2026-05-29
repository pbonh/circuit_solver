---
title: 'The Rust Programming Language — Appendix E: Editions'
type: source
id: summaries/rust-book-26-appendix-e-editions
kind: publication
tags:
- rust
- reference
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/26-appendix-e-editions.txt
---

## Key Points

- Rust ships on a six-week release cadence; every two-to-three years the team packages accumulated changes into a new *edition*: 2015, 2018, 2021 (current at the time of writing; 2024 has since landed). The book uses Rust 2021 idioms.
- The `edition = "..."` key in `Cargo.toml` selects the edition. Omitted, it defaults to 2015 for backward compatibility.
- Editions allow *incompatible* changes — most often new keywords — without breaking existing code. Each crate opts in to an edition independently; mixed-edition workspaces work because edition changes affect only the initial parsing of source.
- All compiler versions support every prior edition. Most language improvements (new APIs, performance) reach all editions; only changes that introduce new keywords or break parsing are edition-gated.
- `cargo fix --edition` migrates a project from one edition to the next, applying mechanical rewrites.
- Editions are a stability promise: opting in to a new edition is voluntary, and existing code continues to build with newer compilers indefinitely.

## Relevant Concepts

- [[concepts/cargo]] — `edition` key lives in `Cargo.toml`.
- [[concepts/rust-language]] — language evolution unit.
- [[concepts/crates]] — edition is per-crate.

## Source Metadata

- Source type: book chapter (appendix)
- Book title: The Rust Programming Language
- Chapter: Appendix E — Editions
- File path: `raw/rust_book/_txt/26-appendix-e-editions.txt`
- Authors: Steve Klabnik and Carol Nichols
