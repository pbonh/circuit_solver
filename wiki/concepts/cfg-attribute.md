---
title: "cfg Attribute"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/12-chapter-11-writing-automated-tests.txt"]
confidence: high
---

## Definition

`#[cfg(...)]` is Rust's conditional-compilation attribute. It tells the compiler to include the annotated item only when the configuration predicate is true. The most common uses are `#[cfg(test)]` (compile only when running tests) and `#[cfg(feature = "...")]` (compile only when a Cargo feature is enabled).

## How It Works

The compiler evaluates the predicate against a set of `cfg` flags determined by the build profile, target, and `[features]`. Predicates compose with `not(...)`, `all(...)`, and `any(...)`: `#[cfg(all(unix, not(feature = "minimal")))]`. The `cfg!` macro evaluates the same predicate at compile-time inside an expression, returning a `bool`. `#[cfg_attr(predicate, attr)]` conditionally applies another attribute.

## Key Parameters

- Built-in predicates: `test`, `target_os`, `target_arch`, `unix`, `windows`, `debug_assertions`
- Feature gates: `feature = "name"` defined in `Cargo.toml`
- Logical combinators: `all`, `any`, `not`
- `cfg!` macro for expression-level branching

## When To Use

- Test-only modules and helpers
- Feature-gated optional functionality
- Platform-specific code paths
- Debug-only logging or instrumentation

## Risks & Pitfalls

- Forgetting a `cfg` flag silently drops the item from compilation
- Feature combinations are unioned across the workspace; one enabling crate affects all
- Heavy use of `cfg` complicates dependency graphs and CI matrices
- Inconsistent `cfg` between modules can produce confusing "item not found" errors

## Related Concepts

- [[concepts/cargo]]
- [[concepts/automated-tests]]
- [[concepts/crates]]

## Sources

- [[summaries/rust-book-12-chapter-11-writing-automated-tests]]
