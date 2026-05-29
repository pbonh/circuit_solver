---
title: 'The Rust Programming Language — Appendix G: How Rust is Made and "Nightly
  Rust"'
type: source
id: summaries/rust-book-28-appendix-g-how-rust-is-made-and-nightly-rust
kind: publication
tags:
- rust
- reference
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/28-appendix-g-how-rust-is-made-and-nightly-rust.txt
---

## Key Points

- Rust's guiding principle is "stability without stagnation": users should never fear upgrading; each release should bring new features, fewer bugs, and faster compiles.
- Rust uses a *train-model* release process with three channels: `nightly` (built every day from master), `beta` (snapshot of nightly six weeks before release), `stable` (snapshot of beta after another six weeks). New stable releases ship every six weeks.
- A new feature lands first behind a feature flag on `master` and is available only in nightly. Beta and stable cannot enable feature flags — only stabilized features compile there.
- The `rustup` tool installs and switches between channels: `rustup toolchain install nightly`, `rustup override set nightly` (per-directory), `rustup default stable` (global).
- Most Rust users live on stable, occasionally testing CI against beta to catch regressions before they reach stable.
- New language features go through the **RFC process**: anyone may write a proposal; the relevant Rust subteam (language, compiler, library, etc.) reads, comments, and eventually accepts or rejects. Implementation lands behind a feature gate, gets nightly soak time, and finally graduates to stable.
- The book deliberately documents only stable features — nightly features are too volatile to be guaranteed.

## Relevant Concepts

- [[concepts/rust-language]] — the language whose evolution this describes.
- [[concepts/rustup]] — channel and toolchain manager.
- [[concepts/rust-release-process]] — the train model.
- [[entities/rust-project]] — the organization running the process.

## Source Metadata

- Source type: book chapter (appendix)
- Book title: The Rust Programming Language
- Chapter: Appendix G — How Rust is Made and "Nightly Rust"
- File path: `raw/rust_book/_txt/28-appendix-g-how-rust-is-made-and-nightly-rust.txt`
- Authors: Steve Klabnik and Carol Nichols
