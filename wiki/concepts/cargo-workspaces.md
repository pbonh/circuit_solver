---
title: Cargo Workspaces
type: claim
id: claim-cargo-workspaces
tags:
- rust
- cargo
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/15-chapter-14-more-about-cargo-and-crates-io.txt
confidence:
  base: 0.85
---

## Definition

A Cargo workspace is a set of crates that share a single `Cargo.lock`, `target/` directory, and dependency-version graph. Workspaces let multi-crate projects coordinate builds, tests, and releases atomically.

## How It Works

A top-level directory holds a `Cargo.toml` containing `[workspace]` with a `members = ["crate-a", "crate-b"]` list (and optional `default-members`, `exclude`, `resolver`). Each member is itself a package with its own `Cargo.toml`. Internal cross-references use `path = "../crate-a"` dependencies. `cargo build` from the workspace root builds all members; `cargo build -p crate-a` targets one. All members share a unified resolved dependency set, which can be either positive (consistency) or negative (feature unification leaks).

## Key Parameters

- `[workspace]` table at root
- `members`, `default-members`, `exclude`
- `resolver = "2"` for Cargo's modern feature resolver
- Shared `target/` and `Cargo.lock`
- Path dependencies between members

## When To Use

- Splitting a large crate into multiple smaller crates for compile-time parallelism
- Cleanly separating CLI, library, and test/bench crates
- Coordinating releases of related packages
- Sharing common build settings across many crates

## Risks & Pitfalls

- Feature unification: a feature enabled by one member is unioned into the whole workspace
- Resolver v1 surprises with feature unification — prefer v2 for new workspaces
- Heavy dependency duplication across members slows builds if not factored carefully
- Cross-member `path` deps require careful version management when publishing

## Related Concepts

- [[concepts/cargo]]
- [[concepts/packages]]
- [[concepts/crates]]
- [[concepts/cargo-profiles]]

## Sources

- [[summaries/rust-book-15-chapter-14-more-about-cargo-and-crates-io]]
