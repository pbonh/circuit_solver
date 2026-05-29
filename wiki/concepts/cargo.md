---
title: Cargo
type: claim
id: concepts/cargo
tags:
- rust
- cargo
- foundational
- tooling
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/01-introduction.txt
- raw/rust_book/_txt/02-chapter-1-getting-started.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

Cargo is Rust's official build system and package manager. It manages dependencies, compiles crates, runs tests, builds documentation, and publishes packages to crates.io.

## How It Works

Cargo reads a `Cargo.toml` manifest file declaring the package, its dependencies, and build profiles. It resolves transitive dependencies, downloads them, and invokes `rustc` with consistent flags. `cargo build`, `cargo run`, `cargo test`, `cargo check`, and `cargo doc` cover the day-to-day workflow. A `Cargo.lock` file records exact resolved versions to make builds reproducible.

## Key Parameters

- `Cargo.toml` manifest — `[package]`, `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`, `[features]`, profile sections
- `Cargo.lock` — exact resolved dependency versions
- `--release` profile for optimized builds
- Workspaces — multi-crate projects sharing a single `Cargo.lock`
- Features — conditional compilation flags

## When To Use

- Any Rust project beyond a one-file experiment
- Sharing libraries through crates.io
- Coordinating multi-crate workspaces in a single repository
- Reproducible CI builds via `Cargo.lock`

## Risks & Pitfalls

- Transitive dependencies can explode build times and compile sizes
- Network access required by default for first build
- Long compile times for large dependency graphs
- Misuse of features can cause subtle conditional-compilation bugs

## Related Concepts

- [[concepts/rust-language]]
- [[concepts/crates]]
- [[concepts/cargo-workspaces]]

## Sources

- [[summaries/rust-book-01-introduction]]
- [[summaries/rust-book-02-chapter-1-getting-started]]
- [[summaries/rust-book-03-chapter-2-programming-a-guessing-game]]
- [[summaries/rust-book-08-chapter-7-managing-growing-projects-with-packages-crates-and-modules]]
- [[summaries/rust-book-12-chapter-11-writing-automated-tests]]
- [[summaries/rust-book-15-chapter-14-more-about-cargo-and-crates-io]]
- [[summaries/rust-book-25-appendix-d-useful-development-tools]]
- [[summaries/rust-book-26-appendix-e-editions]]
