---
title: "Rustup"
type: concept
tags: [rust, tooling, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/02-chapter-1-getting-started.txt"]
confidence: high
---

## Definition

Rustup is the official Rust toolchain installer and version manager. It installs `rustc`, `cargo`, and supporting components, and lets a developer switch between stable, beta, and nightly toolchains or pin per-project toolchains.

## How It Works

`rustup` downloads pre-built toolchains from `static.rust-lang.org`, places them under `~/.rustup`, and installs proxy binaries (`rustc`, `cargo`, etc.) in `~/.cargo/bin`. Project-level toolchain pinning is done via a `rust-toolchain.toml` (or legacy `rust-toolchain`) file. `rustup update` upgrades installed toolchains; `rustup component add` installs extras like `rustfmt`, `clippy`, `rust-src`.

## Key Parameters

- Channels: `stable`, `beta`, `nightly`
- Components: `rustfmt`, `clippy`, `rust-src`, `rust-analyzer`
- Targets: cross-compilation triples (e.g., `wasm32-unknown-unknown`)
- Per-project pinning via `rust-toolchain.toml`

## When To Use

- Standard way to install Rust on any developer machine
- Managing multiple toolchains for testing against nightly features
- CI setups that need reproducible toolchain versions

## Risks & Pitfalls

- PATH ordering issues if other Rust installs exist
- Nightly toolchains can break without notice
- Component availability varies by date on the nightly channel

## Related Concepts

- [[concepts/cargo]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-02-chapter-1-getting-started]]
- [[summaries/rust-book-25-appendix-d-useful-development-tools]]
- [[summaries/rust-book-28-appendix-g-how-rust-is-made-and-nightly-rust]]
