---
title: 'The Rust Programming Language — Chapter 7: Managing Growing Projects with
  Packages, Crates, and Modules'
type: source
id: source-rust-book-08-chapter-7-managing-growing-projects-with-packages-crates-and-modules
kind: derived-summary
tags:
- rust
- foundational
- cargo
- modularity
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/08-chapter-7-managing-growing-projects-with-packages-crates-and-modules.txt
---

## Key Points

- Rust organizes code through a four-level hierarchy: **packages** (a Cargo unit) → **crates** (compilation units; one library + many binaries) → **modules** (a namespace inside a crate) → **items** (functions, types, constants, traits).
- A package is anything with a `Cargo.toml`. A package may contain at most one library crate (root `src/lib.rs`) and any number of binary crates (root `src/main.rs`, additional files in `src/bin/`).
- Modules are declared with `mod name;` (loads `name.rs` or `name/mod.rs`) or inline with `mod name { ... }`. The crate root forms an implicit anonymous module.
- Paths are absolute (`crate::module::item`), relative (`module::item`), or use `self`, `super`, and `crate` as anchors.
- All items default to private. `pub` makes them public; `pub(crate)`, `pub(super)`, `pub(in path)` constrain visibility. Private fields in a public struct require constructor functions.
- `use path;` brings an item into scope; idiomatic style imports the parent module for functions (`use crate::module::sub; sub::foo()`) and the item itself for structs/enums/traits (`use std::collections::HashMap; HashMap::new()`).
- `as` renames imports (`use std::fmt::Result as FmtResult`).
- Nested paths (`use std::{io, cmp::Ordering};`) and globs (`use std::collections::*;`) reduce repetition.
- `pub use` *re-exports* an item, exposing a deeper API at a shallower path — useful for crafting a stable public API independent of internal module layout.
- External crates appear in `Cargo.toml [dependencies]`; their items are reached via `use <crate>::...`.
- A common pattern: extract pure logic into `src/lib.rs`, keep `src/main.rs` as a thin shell wiring CLI args, I/O, and error reporting.

## Relevant Concepts

- [[concepts/packages]] — Cargo's outermost unit.
- [[concepts/crates]] — compilation units; library + binary distinction.
- [[concepts/modules]] — namespaces inside a crate.
- [[concepts/use-declarations]] — bringing names into scope.
- [[concepts/visibility]] — `pub`, `pub(crate)`, `pub(super)`, default-private.
- [[concepts/cargo]] — manages packages and dependencies.
- [[concepts/rust-language]]

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 7 — Managing Growing Projects with Packages, Crates, and Modules
- File path: `raw/rust_book/_txt/08-chapter-7-managing-growing-projects-with-packages-crates-and-modules.txt`
- Authors: Steve Klabnik and Carol Nichols
