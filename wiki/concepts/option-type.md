---
title: "Option Type"
type: concept
tags: [rust, foundational, error-handling, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/07-chapter-6-enums-and-pattern-matching.txt"]
confidence: high
---

## Definition

`Option<T>` is the standard-library enum that represents an optionally present value. Its two variants are `Some(T)` and `None`. Rust uses `Option<T>` everywhere other languages would use a nullable pointer or reference, making absence explicit in the type system.

## How It Works

`Option<T>` is exported in the prelude. The compiler refuses to use `Option<T>` as if it were `T`; the caller must pattern-match (`match`, `if let`) or use one of the many combinators: `unwrap`, `expect`, `unwrap_or`, `unwrap_or_else`, `map`, `and_then`, `or`, `ok_or`, etc. Niche optimization makes `Option<Box<T>>` and `Option<&T>` zero-overhead — the null pointer pattern represents `None`.

## Key Parameters

- Variants: `Some(T)`, `None`
- Combinators: `map`, `and_then`, `or`, `unwrap_or`, `filter`, `flatten`
- Conversion to `Result`: `ok_or(err)`, `ok_or_else(|| err)`
- The `?` operator works on `Option` (early returns `None`)

## When To Use

- Optional fields in structs
- Function return values where "no value" is normal (e.g., `HashMap::get`)
- Replacing nullable pointers from C-style APIs
- Optional parameters via `Option<T>` (or builder pattern)

## Risks & Pitfalls

- Liberal use of `.unwrap()` panics on `None`
- Forgetting that `None` is also a value — comparing without matching may not do what you expect
- Wrapping in `Option` when the absent case never actually occurs adds friction
- Nesting `Option<Option<T>>` is usually a smell — flatten with `.flatten()`

## Related Concepts

- [[concepts/enum-type]]
- [[concepts/result-type]]
- [[concepts/pattern-matching]]
- [[concepts/null-safety]]

## Sources

- [[summaries/rust-book-07-chapter-6-enums-and-pattern-matching]]
- [[summaries/rust-book-10-chapter-9-error-handling]]
