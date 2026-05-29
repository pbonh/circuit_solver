---
title: Procedural Macros
type: claim
id: claim-procedural-macros
tags:
- rust
- macros
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/20-chapter-19-advanced-features.txt
confidence:
  base: 0.85
---

## Definition

Procedural macros are compile-time Rust programs that transform `TokenStream`s into `TokenStream`s. They come in three flavors: custom `#[derive(...)]` macros, attribute-like macros (`#[my_attr]`), and function-like macros (`my_macro!(...)`). They live in dedicated proc-macro crates marked with `proc-macro = true` in `Cargo.toml`.

## How It Works

A proc-macro crate exports functions annotated with `#[proc_macro]`, `#[proc_macro_derive(Name)]`, or `#[proc_macro_attribute]`. The compiler compiles the proc-macro crate first, then loads it as a dynamic library and invokes it during expansion of the dependent crate. Inputs and outputs are `TokenStream` values; the ecosystem standardizes on `syn` for parsing and `quote!` for emission.

## Key Parameters

- Three kinds: derive, attribute, function-like
- `proc-macro = true` in `Cargo.toml`
- Tools: `syn`, `quote`, `proc-macro2`
- Hygiene: tokens carry span metadata for error reporting

## When To Use

- Reducing boilerplate via `#[derive(Serialize, Debug, Builder)]`
- Domain-specific attribute macros (`#[route("/api/...")]`)
- DSLs implemented as `my_dsl! { ... }`
- Code generation that the type system alone cannot express

## Risks & Pitfalls

- Compile-time cost: proc macros run an extra build of the proc-macro crate
- Error messages can be confusing inside generated code
- Stability of `proc-macro` interfaces vs `proc-macro2`
- Hygiene is partial; collisions with surrounding identifiers possible

## Related Concepts

- [[concepts/macros]]
- [[concepts/declarative-macros]]
- [[concepts/derive-macros]]
- [[concepts/crates]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
