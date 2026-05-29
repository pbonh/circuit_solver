---
title: 'The Rust Programming Language — Appendix C: Derivable Traits'
type: source
id: source-rust-book-24-appendix-c-derivable-traits
kind: derived-summary
tags:
- rust
- reference
- traits
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/24-appendix-c-derivable-traits.txt
---

## Key Points

- Appendix C catalogs the standard-library traits that can be derived with `#[derive(...)]` on structs and enums: `Debug`, `Clone`, `Copy`, `Hash`, `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`.
- **`Debug`**: enables `{:?}` / `{:#?}` formatting for diagnostics; required by `assert_eq!` and similar test macros.
- **`PartialEq`**: enables `==` / `!=`. Derived implementations require all fields to be `PartialEq` and compare field-by-field (struct) or variant-by-variant (enum).
- **`Eq`** is a marker trait extending `PartialEq` with the guarantee that *every* value equals itself — floats are *not* `Eq` because `NaN != NaN`. Hash map keys require `Eq` (plus `Hash`).
- **`PartialOrd`** enables `<`, `<=`, `>`, `>=` via `partial_cmp -> Option<Ordering>`. Returns `None` for incomparable values like NaN.
- **`Ord`** strengthens `PartialOrd` with a total order and the `cmp -> Ordering` method. Required for `BTreeMap`/`BTreeSet` keys.
- **`Clone`** provides an explicit `clone()` that recursively clones each field; can be expensive (deep copy of heap data).
- **`Copy`** marks types as duplicable by bit-copy; requires every field to be `Copy` and forbids `Drop` impls. Copy implies Clone.
- **`Hash`** combines per-field hashes; required for hash map keys.
- **`Default`** generates a `default()` constructor that calls `default()` on every field. Used by `unwrap_or_default()` and struct-update syntax `..Default::default()`.
- The appendix notes the list is *not* exhaustive: many ecosystem crates ship custom derives (Serde's `Serialize` / `Deserialize`, `thiserror::Error`, `clap::Parser`, etc.). `Display` is deliberately not derivable because the right user-facing format cannot be inferred automatically.

## Relevant Concepts

- [[concepts/derive-macros]] — `#[derive(...)]` mechanism.
- [[concepts/debug-trait]] — programmer-facing format.
- [[concepts/copy-trait]] — bitwise-copy semantics.
- [[concepts/clone-trait]] — explicit deep-copy.
- [[concepts/traits]] — trait system underpinning derives.

## Source Metadata

- Source type: book chapter (appendix)
- Book title: The Rust Programming Language
- Chapter: Appendix C — Derivable Traits
- File path: `raw/rust_book/_txt/24-appendix-c-derivable-traits.txt`
- Authors: Steve Klabnik and Carol Nichols
