---
title: "The Rust Programming Language — Chapter 14: More about Cargo and Crates.io"
type: summary
tags: [rust, foundational, cargo, tooling]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/15-chapter-14-more-about-cargo-and-crates-io.txt"]
confidence: high
---

## Key Points

- Cargo has two main build profiles: `dev` (used by `cargo build`, `cargo run`, `cargo test`) and `release` (used by `cargo build --release`). Both can be tuned in `[profile.dev]` / `[profile.release]` sections of `Cargo.toml`.
- Optimization level (`opt-level`) defaults to 0 in dev and 3 in release; many other knobs (`debug`, `lto`, `codegen-units`, `panic`) tailor binary size, compile time, and runtime speed.
- Documentation comments use `///` (item docs) and `//!` (crate/module docs). They support Markdown and conventional sections (`# Examples`, `# Panics`, `# Errors`, `# Safety`).
- Example code in doc comments runs as tests when `cargo test` is invoked — preventing documentation from drifting from the implementation.
- `pub use` re-exports curate a friendly public API independent of the internal module layout — particularly important when publishing to crates.io.
- Publishing to crates.io requires a verified account, an API token (`cargo login`), and metadata in `Cargo.toml`: `description`, `license` (SPDX expression), `repository`, etc. Crate names are globally unique and permanent.
- `cargo publish` uploads a new version. Versions are immutable; mistakes are addressed with new versions or `cargo yank`, which prevents new projects from selecting a version while leaving existing `Cargo.lock` references intact.
- Cargo workspaces (`[workspace]` in a top-level `Cargo.toml`, then `members = ["..."]`) coordinate multiple crates sharing one `Cargo.lock` and `target/` directory. Internal `path` dependencies link member crates.
- `cargo install` builds and installs binary crates into `$CARGO_HOME/bin`, providing a uniform way to distribute developer tools.
- Cargo can be extended: any binary named `cargo-<command>` on the PATH is invokable as `cargo <command>`, enabling third-party subcommands.

## Relevant Concepts

- [[concepts/cargo]] — the build/package manager.
- [[concepts/cargo-workspaces]] — multi-crate coordination.
- [[concepts/cargo-profiles]] — dev/release build configuration.
- [[concepts/doc-comments]] — `///` and `//!` with Markdown and doc tests.
- [[concepts/crates-io]] — the central crate registry.
- [[concepts/semver]] — versioning conventions for published crates.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 14 — More about Cargo and Crates.io
- File path: `raw/rust_book/_txt/15-chapter-14-more-about-cargo-and-crates-io.txt`
- Authors: Steve Klabnik and Carol Nichols
