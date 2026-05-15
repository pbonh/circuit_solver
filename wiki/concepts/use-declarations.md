---
title: "Use Declarations"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/08-chapter-7-managing-growing-projects-with-packages-crates-and-modules.txt"]
confidence: high
---

## Definition

A `use` declaration brings an item from another module into the current scope, allowing shorter references. `use std::collections::HashMap;` makes `HashMap` usable without its full path inside the surrounding module.

## How It Works

`use path;` creates an alias from a short name to a full path. `use path as alias;` lets the programmer rename. Nested forms `use std::{io, cmp::Ordering};` and glob `use prelude::*;` reduce repetition. `pub use` *re-exports* the imported item under the current module's name, useful for shaping public APIs. Idiomatic Rust imports the parent module for functions but the item itself for types, traits, and enums.

## Key Parameters

- Path forms: absolute (`crate::...`), relative (`name::...`), prelude (no anchor)
- Renaming with `as`
- Nested paths via `{ }`
- Glob `*` (often discouraged outside preludes)
- `pub use` re-exports

## When To Use

- Any time the same path is used more than once
- Re-exporting deep types as part of a stable public API
- Renaming to disambiguate two items with the same name

## Risks & Pitfalls

- Glob imports can introduce name collisions that surface as compile errors after upstream updates
- Importing a function directly can make the call site ambiguous about which module it lives in
- Forgetting `pub` before `use` means re-exports won't actually be visible externally

## Related Concepts

- [[concepts/modules]]
- [[concepts/visibility]]
- [[concepts/crates]]

## Sources

- [[summaries/rust-book-08-chapter-7-managing-growing-projects-with-packages-crates-and-modules]]
