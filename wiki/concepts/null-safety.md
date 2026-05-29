---
title: Null Safety
type: claim
id: concepts/null-safety
tags:
- rust
- foundational
- memory-safety
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/07-chapter-6-enums-and-pattern-matching.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Null safety is the property of a language that prevents the "billion-dollar mistake" of null pointer dereferences. Rust achieves null safety by not having a null value for ordinary references: every `&T` is guaranteed non-null. Optionality is expressed explicitly as `Option<T>`, forcing programmers to handle the absent case.

## How It Works

References in Rust are non-nullable by construction. Where C/C++ would use a null pointer or sentinel value, Rust uses `Option<T>`. The compiler's type system rejects code that ignores the `None` case. For interop with C, raw pointers (`*const T`, `*mut T`) can be null, but dereferencing them requires `unsafe`.

## Key Parameters

- Non-null `&T`, `&mut T`, `Box<T>`
- `Option<T>` for optional values
- Niche optimization: `Option<&T>` is one pointer wide, with null representing `None`
- Raw pointers (`*const T`, `*mut T`) can be null but require `unsafe` to deref

## When To Use

- Always — null safety is the default
- Modeling "may be missing" data with `Option<T>` rather than sentinel values
- Avoiding the "valid-or-null" convention from C APIs

## Risks & Pitfalls

- Interop with C requires careful Option <-> nullable-pointer conversion
- Over-wrapping in `Option` adds friction when the absent case is impossible
- Forgetting that raw pointers bypass null safety in `unsafe` blocks

## Related Concepts

- [[concepts/option-type]]
- [[concepts/memory-safety]]
- [[concepts/references]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-07-chapter-6-enums-and-pattern-matching]]
