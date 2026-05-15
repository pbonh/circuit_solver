---
title: "Rustfmt"
type: concept
tags: [rust, tooling, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/01-introduction.txt"]
confidence: medium
---

## Definition

Rustfmt is the official Rust source-code formatter. It rewrites Rust source files into a canonical style, ensuring a consistent look across developers and projects.

## How It Works

Rustfmt parses Rust source code and re-emits it according to a style configuration (`rustfmt.toml`). It is typically invoked via `cargo fmt`, which formats every file in the current package or workspace. CI pipelines often run `cargo fmt --check` to fail on unformatted code.

## Key Parameters

- Configurable via `rustfmt.toml` (line width, indent style, edition)
- Integrated with `cargo fmt`
- Editor plug-ins for on-save formatting

## When To Use

- Standard hygiene for any Rust codebase
- CI gates to keep style noise out of diffs
- Onboarding mixed-skill teams onto a shared style

## Risks & Pitfalls

- Some manual formatting (e.g., aligned tables in comments) gets rewritten
- Custom macros can render in surprising shapes
- Older `rustfmt.toml` options have changed across editions

## Related Concepts

- [[concepts/rust-language]]
- [[concepts/cargo]]

## Sources

- [[summaries/rust-book-01-introduction]]
- [[summaries/rust-book-25-appendix-d-useful-development-tools]]
