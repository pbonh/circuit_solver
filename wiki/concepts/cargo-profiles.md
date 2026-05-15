---
title: "Cargo Profiles"
type: concept
tags: [rust, cargo, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/15-chapter-14-more-about-cargo-and-crates-io.txt"]
confidence: high
---

## Definition

A Cargo profile is a named set of build settings (optimization level, debug info, LTO, codegen units, panic strategy) applied to a build. The two main profiles are `dev` (used by `cargo build` and `cargo test`) and `release` (used by `cargo build --release`). Additional named profiles can be defined.

## How It Works

`Cargo.toml` may include `[profile.dev]`, `[profile.release]`, `[profile.test]`, `[profile.bench]`, plus custom names. Settings include `opt-level` (0–3, `s`, `z`), `debug` (line-table / full), `lto` (false/true/thin/fat), `codegen-units`, `panic` ("unwind"/"abort"), `incremental`, and `overflow-checks`. Settings inherit from base profiles when not specified explicitly. `[profile.<name>.package.<crate>]` overrides settings for specific dependencies.

## Key Parameters

- `opt-level` — optimization aggressiveness
- `lto` — link-time optimization (slower link, faster code)
- `codegen-units` — parallelism in codegen (fewer = better optimization but slower)
- `debug` / `strip` — debug-symbol policy
- `panic` — `unwind` vs `abort`

## When To Use

- `release` for production binaries
- Custom profiles for benchmarking variants
- Per-dependency overrides to optimize hot crates inside otherwise-debug builds (`opt-level = 3` on a math kernel)

## Risks & Pitfalls

- Cranking optimization helps runtime but increases compile time significantly
- LTO+single codegen-unit can make linking very slow on large workspaces
- `panic = "abort"` saves binary size but eliminates `catch_unwind` recovery
- Debug-mode integer overflow panics may mask issues in release

## Related Concepts

- [[concepts/cargo]]
- [[concepts/cargo-workspaces]]
- [[concepts/integer-overflow]]
- [[concepts/panic]]

## Sources

- [[summaries/rust-book-15-chapter-14-more-about-cargo-and-crates-io]]
