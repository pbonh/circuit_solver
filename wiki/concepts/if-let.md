---
title: if let
type: claim
id: claim-if-let
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/07-chapter-6-enums-and-pattern-matching.txt
confidence:
  base: 0.85
---

## Definition

`if let pattern = expr { ... } else { ... }` is sugar for a `match` expression with one interesting arm and a default. It allows succinct handling of an `Option`, `Result`, or other enum when only one variant carries the work the programmer wants to do.

## How It Works

```rust
if let Some(value) = maybe_value {
    use(value);
} else {
    no_value();
}
```

`if let` may also appear in `while let pattern = expr { ... }` form, repeating the body while the pattern still matches. Bindings introduced in the pattern are scoped to the `if let`/`while let` body. Unlike `match`, `if let` is not exhaustive — only the matched pattern is handled, with everything else going to the optional `else` branch.

## Key Parameters

- Pattern may bind values
- Optional `else` branch
- `while let` variant
- Loses exhaustiveness checking compared to `match`

## When To Use

- Single-variant handling of `Option`/`Result`
- Loops that consume an iterator until exhausted
- Reducing noise when 90% of enum variants are irrelevant in this place

## Risks & Pitfalls

- Easy to ignore variants that the type system would otherwise force you to consider
- Combined with `else if let` chains, can become harder to read than `match`
- New variants added later may silently fall through the `else`

## Related Concepts

- [[concepts/pattern-matching]]
- [[concepts/option-type]]
- [[concepts/result-type]]
- [[concepts/enum-type]]

## Sources

- [[summaries/rust-book-07-chapter-6-enums-and-pattern-matching]]
- [[summaries/rust-book-19-chapter-18-patterns-and-matching]]
