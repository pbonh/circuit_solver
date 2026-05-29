---
title: Modules
type: claim
id: concepts/modules
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/08-chapter-7-managing-growing-projects-with-packages-crates-and-modules.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A module is a namespace inside a crate that groups related items (functions, types, constants, traits, sub-modules) and scopes their visibility. Modules form a tree rooted at the crate root and govern paths used to refer to items.

## How It Works

`mod name;` in a file makes the compiler look for `name.rs` (or `name/mod.rs`) in the same directory; `mod name { ... }` declares an inline module. Items inside a module default to private; `pub` exposes them upward. Paths use `crate::`, `super::`, `self::` anchors or the module name itself for relative reference. The module tree maps to the file tree by convention but is not required to.

## Key Parameters

- File-based vs inline modules
- `mod name;` vs `pub mod name;`
- Anchors: `crate`, `super`, `self`
- Visibility modifiers
- Re-exports via `pub use`

## When To Use

- Organizing a growing crate by feature/subsystem
- Hiding implementation details behind a curated public surface
- Sharing helper code across multiple files in the same crate

## Risks & Pitfalls

- Forgetting `mod name;` in the parent file silently drops the file from compilation
- Deep nesting creates long paths; consider `pub use` re-exports for a flatter public API
- Circular module references require restructuring
- Mixing `name.rs` and `name/mod.rs` patterns in the same project is confusing

## Related Concepts

- [[concepts/use-declarations]]
- [[concepts/visibility]]
- [[concepts/crates]]
- [[concepts/packages]]

## Sources

- [[summaries/rust-book-08-chapter-7-managing-growing-projects-with-packages-crates-and-modules]]
- [[summaries/rust-book-22-appendix-a-keywords]]
