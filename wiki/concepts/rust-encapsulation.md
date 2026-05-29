---
title: Rust Encapsulation
type: claim
id: claim-rust-encapsulation
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/18-chapter-17-object-oriented-programming-features-of-rust.txt
confidence:
  base: 0.85
---

## Definition

Encapsulation in Rust is enforced at the module boundary via the `pub` visibility modifier on items and fields. Private items default to invisible outside the defining module, so internal data structures can evolve freely without breaking external callers, mirroring the goal of OO access modifiers.

## How It Works

A `pub struct Foo` with non-`pub` fields exposes a name but no field access — callers must use methods. A `pub fn new(...) -> Foo` constructor can enforce invariants on creation. Module visibility (`pub`, `pub(crate)`, `pub(super)`) gives fine-grained control over what crosses each boundary. Encapsulation works equally well for enums (private variants are reachable only through the module's API).

## Key Parameters

- Default-private items
- `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`
- Module hierarchy as the visibility scope
- Private fields with public methods

## When To Use

- Library design: curate a stable public surface; hide implementation
- Enforcing invariants via constructors and methods
- Refactoring without breaking external users
- Separating crate-internal helpers from public API

## Risks & Pitfalls

- Forgetting `pub` results in confusing "function is private" errors
- Public-by-default fields are tempting but freeze the data layout
- `pub use` re-exports can leak internal names if the path looks public

## Related Concepts

- [[concepts/visibility]]
- [[concepts/modules]]
- [[concepts/struct-type]]
- [[concepts/newtype-pattern]]

## Sources

- [[summaries/rust-book-18-chapter-17-object-oriented-programming-features-of-rust]]
