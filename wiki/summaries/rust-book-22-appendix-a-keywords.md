---
title: "The Rust Programming Language — Appendix A: Keywords"
type: summary
tags: [rust, reference, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/22-appendix-a-keywords.txt"]
confidence: high
---

## Key Points

- Appendix A enumerates the keywords reserved by the Rust language. Identifiers (function, variable, type, trait names) cannot collide with these keywords unless escaped via raw identifiers (`r#name`).
- **In-use keywords** include the obvious control-flow set (`if`, `else`, `match`, `loop`, `while`, `for`, `break`, `continue`, `return`), the ownership/binding set (`let`, `mut`, `move`, `ref`), the type-system set (`fn`, `struct`, `enum`, `trait`, `impl`, `type`, `union`, `Self`, `self`, `dyn`, `where`, `as`), the module/visibility set (`mod`, `pub`, `use`, `crate`, `super`, `extern`), and the special concurrency/safety set (`async`, `await`, `unsafe`, `static`, `const`).
- **Reserved-for-future-use keywords**: `abstract`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`. These cannot currently be used as identifiers either.
- **Raw identifiers** (`r#name`) provide an escape hatch when a keyword must be used as an identifier — most commonly when interoperating with crates from older editions where a now-keyword was a normal name.
- This list is the authoritative reference for the grammar of Rust source.

## Relevant Concepts

- [[concepts/rust-language]] — the keywords define the surface syntax.
- [[concepts/variables-and-mutability]] — `let`, `mut`, `const`.
- [[concepts/rust-control-flow]] — `if`, `match`, loop family.
- [[concepts/traits]] — `trait`, `impl`, `dyn`.
- [[concepts/modules]] — `mod`, `use`, `pub`, `super`, `crate`.
- [[concepts/unsafe-rust]] — `unsafe`.

## Source Metadata

- Source type: book chapter (appendix)
- Book title: The Rust Programming Language
- Chapter: Appendix A — Keywords
- File path: `raw/rust_book/_txt/22-appendix-a-keywords.txt`
- Authors: Steve Klabnik and Carol Nichols
