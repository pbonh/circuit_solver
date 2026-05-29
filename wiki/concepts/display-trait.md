---
title: Display Trait
type: claim
id: claim-display-trait
tags:
- rust
- traits
- formatting
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/11-chapter-10-generic-types-traits-and-lifetimes.txt
confidence:
  base: 0.65
---

## Definition

`std::fmt::Display` is the standard-library Rust trait for user-facing formatting via the `{}` format specifier in macros like `println!`, `format!`, and `write!`. The Rust Book Chapter 10 introduces Display as the canonical example of an "external trait" that can be implemented on local types (subject to Rust's orphan rule for trait coherence): "we can implement standard library traits like Display on a custom type like Tweet as part of our aggregator crate functionality, because the type Tweet is local to our aggregator crate."

## How It Works

A type implements `Display` by writing `impl fmt::Display for MyType { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { ... } }`. The `Formatter` carries flags for width, precision, fill, alignment, etc. The Rust Book Chapter 10 also shows `Display` as a typical trait bound:

```
pub fn notify(item: &(impl Summary + Display)) {
```

## Key Parameters

- The single required method `fmt(&self, f: &mut Formatter) -> Result`.
- Distinct from [[concepts/debug-trait]] (`{:?}` / `{:#?}`), which is for developer-facing output and is usually `#[derive]`-able.

## When To Use

- Any type that should have a single canonical user-facing string form.
- Public API types whose `to_string()` should be meaningful (Display blanket-implements `ToString`).

## Risks & Pitfalls

- Display is NOT `#[derive]`-able — must be hand-implemented. (Derive Debug instead for diagnostic output.)
- Orphan rule: cannot implement Display on a foreign type from a foreign crate.
- A type's Display impl is its public face; changing it can break callers that parse the formatted string.

## Related Concepts

- [[concepts/debug-trait]] — developer-facing counterpart.
- [[concepts/traits]] — generic Rust trait mechanics.
- [[concepts/derive-macros]]

## Sources

- [[summaries/rust-book-11-chapter-10-generic-types-traits-and-lifetimes]]
