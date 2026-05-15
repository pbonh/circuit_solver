---
title: "Rust Release Process"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/28-appendix-g-how-rust-is-made-and-nightly-rust.txt"]
confidence: high
---

## Definition

The Rust release process is the train-model schedule used by the Rust project: every six weeks, code on `master` graduates to `beta`, and the previous `beta` graduates to `stable`. The three channels — `nightly`, `beta`, `stable` — allow users to choose between bleeding-edge features and rock-solid guarantees while keeping the language stable for the long term.

## How It Works

A new feature lands behind a feature gate on `master`, where nightly users can opt in by enabling the gate in their source. Every six weeks, the current state of master snapshots into a new beta. Six weeks later, that beta becomes the new stable. Beta and stable cannot enable feature gates — only fully stabilized features. Patches found in beta land back on master and are backported to the beta branch. The cadence is mechanical: knowing one release date predicts the next.

## Key Parameters

- Channels: nightly, beta, stable
- Cadence: six weeks per release
- Feature gates: nightly-only opt-in for unstable features
- Editions: every 2–3 years, packaged language updates
- RFC process for feature design

## When To Use

- Stable for production
- Beta in CI to catch regressions before stable lands
- Nightly for unstable features (compiler internals, async generators, GAT-extensions in some periods)
- Per-project override via `rustup override` or `rust-toolchain.toml`

## Risks & Pitfalls

- Building on nightly features means migration work when they stabilize (or get removed)
- Channel skew across team members
- Forgetting `rust-toolchain.toml` pins gives non-reproducible builds
- Some ecosystem crates require nightly for proc-macro features

## Related Concepts

- [[concepts/rustup]]
- [[concepts/rust-language]]
- [[concepts/cargo]]

## Sources

- [[summaries/rust-book-28-appendix-g-how-rust-is-made-and-nightly-rust]]
