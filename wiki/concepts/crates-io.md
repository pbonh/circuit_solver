---
title: "crates.io"
type: concept
tags: [rust, cargo, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/15-chapter-14-more-about-cargo-and-crates-io.txt"]
confidence: high
---

## Definition

crates.io is the official public registry of Rust crates. It is the default source from which Cargo resolves dependencies. Authors publish versioned packages; consumers pull them as transitive dependencies; releases are immutable once published (errors are handled by yanking, not deleting).

## How It Works

A package owner runs `cargo login <token>` once to authenticate, then `cargo publish` packages the crate, uploads it, and triggers a build of generated docs on docs.rs. Crate names are first-come-first-served and permanent. Required metadata: `name`, `version`, `license` (SPDX), `description`. `cargo yank --version X.Y.Z` prevents new resolutions from picking that version without breaking existing `Cargo.lock` users.

## Key Parameters

- Mandatory `Cargo.toml` metadata: `name`, `version`, `license` or `license-file`, `description`
- Optional but expected: `repository`, `homepage`, `documentation`, `keywords`, `categories`, `readme`
- Versioning follows SemVer
- Yanking is reversible (`--undo`)

## When To Use

- Publishing reusable libraries
- Distributing tools that other developers should `cargo install`
- Documenting work for the broader Rust community

## Risks & Pitfalls

- Crate names are permanent — choose carefully
- Versioning mistakes cannot be repaired by overwriting; only by publishing again
- Heavy dependencies inflate compile times for every downstream user
- License-mismatch causes downstream consumers to be unable to use a release legally

## Related Concepts

- [[concepts/cargo]]
- [[concepts/semver]]
- [[concepts/crates]]
- [[concepts/packages]]

## Sources

- [[summaries/rust-book-15-chapter-14-more-about-cargo-and-crates-io]]
