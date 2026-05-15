---
title: "Visibility"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/08-chapter-7-managing-growing-projects-with-packages-crates-and-modules.txt"]
confidence: high
---

## Definition

Visibility in Rust controls which modules can name an item. Items default to private (visible only inside the defining module and its descendants); the `pub` keyword raises visibility upward. Granular forms `pub(crate)`, `pub(super)`, and `pub(in path)` scope visibility to the crate, parent module, or specified module path.

## How It Works

A public item in a module is visible to that module's siblings and ancestors and, if the chain of containing modules is also public, eventually to external crates. Private fields inside a public struct still require constructor functions for external instantiation. Trait methods inherit the visibility of the trait itself. The orphan rule for trait impls is independent of visibility.

## Key Parameters

- `pub` — visible everywhere the parent module is
- `pub(crate)` — limited to the current crate
- `pub(super)` — limited to the parent module
- `pub(in some::path)` — limited to a specific ancestor
- Default — private to the defining module and its descendants

## When To Use

- Public API curation: only expose what callers should depend on
- `pub(crate)` for internal-only helpers shared across modules
- Private fields with public constructors enforce invariants

## Risks & Pitfalls

- Forgetting `pub` results in obscure "function is private" errors
- Exposing fields directly makes future refactoring an API break
- Wrapping access in `pub(crate)` and forgetting to widen it later

## Related Concepts

- [[concepts/modules]]
- [[concepts/use-declarations]]
- [[concepts/struct-type]]
- [[concepts/traits]]

## Sources

- [[summaries/rust-book-08-chapter-7-managing-growing-projects-with-packages-crates-and-modules]]
