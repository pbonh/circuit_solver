---
title: Dynamically Sized Types
type: claim
id: concepts/dynamically-sized-types
tags:
- rust
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/20-chapter-19-advanced-features.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Dynamically sized types (DSTs, also "unsized types") are types whose size is not known at compile time. The most familiar are `str` (the underlying type of `&str`) and trait objects (`dyn Trait`). DSTs must always be used behind some pointer-like type (`&`, `Box`, `Rc`, `Arc`) that carries the runtime size or vtable.

## How It Works

The compiler does not allow you to declare a binding of type `str` or `dyn Trait` directly — those are unsized and cannot live on the stack. References to DSTs are *fat pointers*: `&str` is `(*const u8, len: usize)`, `&dyn Trait` is `(*const T, vtable: *const Vtable)`. Generics implicitly carry a `Sized` bound; opting out requires `T: ?Sized` ("possibly unsized").

## Key Parameters

- Sized vs `?Sized` bound on generics
- Fat-pointer representation: data pointer + size/vtable
- Common DSTs: `str`, `[T]`, `dyn Trait`
- Pointer wrappers: `&T`, `&mut T`, `Box<T>`, `Rc<T>`, `Arc<T>`

## When To Use

- Function parameters that should accept both arrays and vectors via `&[T]`
- String parameters via `&str`
- Plug-in / extensibility APIs via `&dyn Trait` or `Box<dyn Trait>`
- Generic library code with `T: ?Sized`

## Risks & Pitfalls

- Cannot put DSTs directly in collections or on the stack
- `?Sized` opt-out is easy to forget for generic library functions
- Fat-pointer methods can be slower than thin-pointer equivalents in inner loops

## Related Concepts

- [[concepts/sized-trait]]
- [[concepts/trait-objects]]
- [[concepts/slice-type]]
- [[concepts/string-type]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
