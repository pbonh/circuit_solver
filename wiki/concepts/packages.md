---
title: Packages
type: claim
id: claim-packages
tags:
- rust
- cargo
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/08-chapter-7-managing-growing-projects-with-packages-crates-and-modules.txt
confidence:
  base: 0.85
---

## Definition

A package is Cargo's outermost unit: a directory containing a `Cargo.toml`. A package contains at most one library crate (`src/lib.rs`) and any number of binary crates (`src/main.rs` plus files in `src/bin/`). Packages are what get published to crates.io.

## How It Works

Cargo reads `Cargo.toml` for package metadata (`name`, `version`, `edition`, `authors`), `[dependencies]`, build profiles, and feature flags. It detects crates by convention (`src/lib.rs`, `src/main.rs`, `src/bin/*.rs`, `tests/*.rs`, `benches/*.rs`, `examples/*.rs`). A workspace package can list `[workspace]` to coordinate many crates with a shared `Cargo.lock`.

## Key Parameters

- `Cargo.toml` (package metadata, dependencies, features)
- One implicit library crate + many binaries
- `[workspace]` for multi-crate coordination
- Feature flags for conditional compilation

## When To Use

- Any Rust project — a package is the minimum unit of work
- Splitting a large project into multiple crates inside one workspace
- Publishing a reusable library to crates.io

## Risks & Pitfalls

- Package name vs crate name conventions: hyphens in package, underscores in crate
- Workspace `Cargo.lock` resolves the union of all members' dependency graphs
- Feature unification across workspace members can pull in unexpected code

## Related Concepts

- [[concepts/cargo]]
- [[concepts/crates]]
- [[concepts/modules]]
- [[concepts/cargo-workspaces]]

## Sources

- [[summaries/rust-book-08-chapter-7-managing-growing-projects-with-packages-crates-and-modules]]
