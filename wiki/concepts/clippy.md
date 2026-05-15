---
title: "Clippy"
type: concept
tags: [rust, tooling, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/25-appendix-d-useful-development-tools.txt"]
confidence: high
---

## Definition

Clippy is Rust's official linter — a collection of hundreds of lints catching common mistakes, idiomatic improvements, and performance issues. It is distributed as a `rustup` component and invoked with `cargo clippy`.

## How It Works

Clippy hooks into the compiler's type-checked AST and HIR, walking the code to apply each enabled lint. Lints are categorized (`correctness`, `suspicious`, `style`, `complexity`, `perf`, `pedantic`, `cargo`, `nursery`) so projects can tune severity per category. Configuration goes in `clippy.toml` and `#[allow(...)]` / `#[deny(...)]` attributes can override individual lints. CI typically runs `cargo clippy --all-targets -- -D warnings` to fail builds on any lint.

## Key Parameters

- Installation: `rustup component add clippy`
- Invocation: `cargo clippy [-- flags]`
- Categories: correctness, perf, style, pedantic, etc.
- Configuration: `clippy.toml`, `#[allow]` / `#[deny]` attributes
- Auto-fix mode: `cargo clippy --fix`

## When To Use

- Standard CI gate in any Rust project
- Local development on every save
- Adopting idiomatic Rust patterns
- Surfacing performance pitfalls before benchmarking

## Risks & Pitfalls

- Some `pedantic` lints over-trigger and can drown signal in noise
- Clippy lints occasionally conflict with each other across versions
- Auto-fix can produce confusing diffs without review
- Heavy `#[allow]` lists in a codebase suggest a different lint configuration is needed

## Related Concepts

- [[concepts/cargo]]
- [[concepts/rustfmt]]
- [[concepts/rustup]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-25-appendix-d-useful-development-tools]]
