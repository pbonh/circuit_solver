---
title: Sized Trait
type: claim
id: claim-sized-trait
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/20-chapter-19-advanced-features.txt
confidence:
  base: 0.85
---

## Definition

`Sized` is a marker trait the compiler auto-implements for every type whose size is known at compile time. By default, every generic type parameter `T` is implicitly bound by `T: Sized`. Opting out is done with `T: ?Sized`, which lets the parameter accept dynamically sized types as well.

## How It Works

`Sized` is implemented automatically; users do not write `impl Sized`. Most types — primitives, structs with sized fields, references, `Box<T>`, etc. — are `Sized`. Unsized types are `str`, `[T]` (the slice type, not `&[T]`), `dyn Trait`, and any struct whose last field is unsized. Generic functions with `?Sized` bounds must use the parameter only behind a pointer (`&T`, `Box<T>`, etc.).

## Key Parameters

- Implicit `T: Sized` on every generic parameter
- Explicit `T: ?Sized` to opt out
- Last-field-unsized struct layout
- Interaction with `&T` / `Box<T>` / `Rc<T>`

## When To Use

- The implicit bound is correct for nearly all generic code
- `?Sized` for generic library functions that should accept slices, strings, or trait objects
- Container types that should hold unsized values behind pointers

## Risks & Pitfalls

- Forgetting `?Sized` shuts out important inputs from generic functions
- Cannot move an unsized value by value
- Struct layout with a trailing unsized field is constrained

## Related Concepts

- [[concepts/dynamically-sized-types]]
- [[concepts/trait-objects]]
- [[concepts/generics]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
