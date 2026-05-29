---
title: SemVer
type: claim
id: claim-semver
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/15-chapter-14-more-about-cargo-and-crates-io.txt
confidence:
  base: 0.65
---

## Definition

Semantic Versioning (SemVer) is the `MAJOR.MINOR.PATCH` versioning convention Cargo and crates.io use to communicate compatibility. A change in the major version signals a breaking API change, a minor version signals additive non-breaking changes, and a patch version signals backwards-compatible fixes. Cargo's default version selector picks the highest compatible release.

## How It Works

`cargo add foo` writes `foo = "1.2"` to `Cargo.toml`, which is shorthand for `^1.2` — "any version `>=1.2 < 2.0`". Pre-1.0 packages treat each minor bump as potentially breaking. `cargo update` re-resolves within these constraints. The Rust API evolution guide enumerates which changes count as breaking (e.g., removing a public type) versus additive (adding a new trait impl is normally minor, but can sometimes be breaking).

## Key Parameters

- Format: `MAJOR.MINOR.PATCH[-pre.release][+build]`
- Pre-1.0 special handling
- Caret operator `^` (default)
- Tilde operator `~` (only-patch updates)
- Exact pin (`=1.2.3`)

## When To Use

- Publishing any crate to crates.io
- Communicating compatibility expectations to downstream users
- Choosing constraints in `Cargo.toml`

## Risks & Pitfalls

- Accidental breaking changes in a minor release
- Pre-1.0 instability surprising new users
- Inconsistent SemVer interpretation across ecosystems
- "Yanking" a version is not a release rollback — old `Cargo.lock`s still resolve

## Related Concepts

- [[concepts/cargo]]
- [[concepts/crates-io]]
- [[concepts/crates]]
- [[concepts/packages]]

## Sources

- [[summaries/rust-book-15-chapter-14-more-about-cargo-and-crates-io]]
