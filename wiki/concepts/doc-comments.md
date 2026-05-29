---
title: Doc Comments
type: claim
id: claim-doc-comments
tags:
- rust
- foundational
- documentation
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/15-chapter-14-more-about-cargo-and-crates-io.txt
confidence:
  base: 0.85
---

## Definition

Documentation comments are special Rust comments parsed by `rustdoc` to generate HTML API documentation. They use `///` for item-level docs (placed *before* the item) and `//!` for inner-attribute docs (placed *inside* the item, typically at the top of a crate or module). They support Markdown and standardized sections.

## How It Works

`cargo doc --open` invokes `rustdoc`, which reads doc comments, code blocks, and `#[doc(...)]` attributes to produce a searchable site. Convention sections include `# Examples`, `# Panics`, `# Errors`, `# Safety`. Fenced code blocks in doc comments compile and run as doc tests during `cargo test`, ensuring example code stays correct. The `?` operator works inside doc tests that return `Result`.

## Key Parameters

- `///` outer doc, `//!` inner doc
- Markdown formatting (headings, code, links)
- Doc tests: ` ```rust ... ``` `, `no_run`, `should_panic`, `ignore` annotations
- `#[doc = "..."]` attribute form
- `intra_doc_links` — `[Type::method]` style references

## When To Use

- Every public item should have at least one line of `///`
- Module/crate-level overviews via `//!`
- Examples that should never go stale
- Documenting safety invariants on `unsafe` functions

## Risks & Pitfalls

- Doc tests slow `cargo test`; consider `#[doc(no_run)]` for examples requiring external state
- Markdown links to private items become broken when visibility tightens
- Forgetting `# Safety` on `unsafe fn` makes the contract invisible to callers
- Long example chains hide the focal item

## Related Concepts

- [[concepts/cargo]]
- [[concepts/automated-tests]]
- [[concepts/crates]]

## Sources

- [[summaries/rust-book-15-chapter-14-more-about-cargo-and-crates-io]]
