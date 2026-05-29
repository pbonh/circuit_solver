---
title: Crates
type: claim
id: claim-crates
tags:
- rust
- cargo
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/02-chapter-1-getting-started.txt
- raw/rust_book/_txt/08-chapter-7-managing-growing-projects-with-packages-crates-and-modules.txt
confidence:
  base: 0.85
---

## Definition

A crate is Rust's unit of compilation and distribution. Every Rust program or library is a crate. Crates are either binary crates (produce an executable) or library crates (produce a reusable library) and are organized inside packages managed by Cargo.

## How It Works

A package contains a `Cargo.toml` and one or more crates: at most one library crate (root `src/lib.rs`), and any number of binary crates (root `src/main.rs` plus files under `src/bin/`). The compiler treats the crate root as the entry point and walks the module tree from there. Crates published to crates.io are versioned per SemVer.

## Key Parameters

- Crate type: `bin` vs `lib`
- Crate root files: `src/main.rs`, `src/lib.rs`, `src/bin/<name>.rs`
- Edition: `2015 | 2018 | 2021 | 2024`
- SemVer version in `Cargo.toml`

## When To Use

- Library crates for reusable functionality (numerical solvers, parsers)
- Binary crates for end-user programs (CLIs, simulators)
- Splitting a large project into multiple crates to improve compile times

## Risks & Pitfalls

- Crate-level cyclic dependencies are not allowed
- Public API surface in a library crate must be carefully managed
- Large monolithic crates compile slowly

## Related Concepts

- [[concepts/cargo]]
- [[concepts/cargo-workspaces]]
- [[concepts/modules]]
- [[concepts/packages]]

## Sources

- [[summaries/rust-book-02-chapter-1-getting-started]]
- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-08-chapter-7-managing-growing-projects-with-packages-crates-and-modules]]
- [[summaries/rust-book-26-appendix-e-editions]]
