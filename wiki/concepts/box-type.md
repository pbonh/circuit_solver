---
title: "Box Type"
type: concept
tags: [rust, smart-pointers, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/16-chapter-15-smart-pointers.txt"]
confidence: high
---

## Definition

`Box<T>` is the simplest Rust smart pointer: a unique owner of a heap-allocated value of type `T`. It has the same single-owner semantics as a stack value but stores its data on the heap, making it valuable for large data, recursive types, and trait objects.

## How It Works

`Box::new(value)` allocates space on the heap, moves `value` into it, and returns a pointer. The `Box` is a thin wrapper over a raw pointer with a `Drop` impl that frees the heap allocation. `Deref` makes `*b` yield the inner value, so methods auto-deref through it. Trait objects like `Box<dyn Trait>` enable dynamic dispatch and erase the concrete type.

## Key Parameters

- Heap allocation per construction
- Single ownership transferred on assignment
- Trait-object support via `Box<dyn Trait>`
- Recursive-type enabler (`enum List { Cons(i32, Box<List>), Nil }`)

## When To Use

- Recursive types whose size cannot be known at compile time
- Large values that would otherwise blow the stack
- Trait objects when type erasure is needed
- Transferring ownership of heap data across API boundaries

## Risks & Pitfalls

- Heap allocation has non-trivial cost — avoid in hot loops
- `Box<dyn Trait>` foregoes inlining and static dispatch
- Cannot deref-mut while shared via `&`
- Moving a `Box<T>` is cheap (just a pointer copy), but cloning typically requires `T: Clone`

## Related Concepts

- [[concepts/smart-pointers]]
- [[concepts/deref-trait]]
- [[concepts/drop-trait]]
- [[concepts/trait-objects]]
- [[concepts/ownership]]

## Sources

- [[summaries/rust-book-16-chapter-15-smart-pointers]]
