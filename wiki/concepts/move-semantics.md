---
title: "Move Semantics"
type: concept
tags: [rust, ownership, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt"]
confidence: high
---

## Definition

Move semantics is Rust's default behavior when transferring a non-`Copy` value: assignment, function argument passing, or returning a value moves ownership from the source to the destination, invalidating the source binding. Move prevents double-free and use-after-free without any runtime overhead.

## How It Works

When a non-`Copy` value is assigned (`let b = a;`), the bits are copied to the destination but the compiler marks the source `a` as invalid; subsequent uses of `a` are compile errors. For values containing heap pointers (e.g., `String`, `Vec`), this avoids two owners sharing the same heap allocation. Functions consume their arguments by default; to keep ownership in the caller, pass a reference instead.

## Key Parameters

- Triggered on assignment, function call, return, pattern binding
- Only applies when the type does not implement `Copy`
- Move is a shallow bit-copy plus source invalidation
- Tracked statically; no runtime cost

## When To Use

- Whenever ownership transfer is the natural semantic (constructors, builders)
- Hand off a resource (file handle, lock) to another function
- Avoid cloning when the source is no longer needed

## Risks & Pitfalls

- Use-after-move compile errors when the programmer forgot the source was consumed
- Patterns that bind by value inside a `match` can accidentally move out of a borrowed value
- Combining moves and references inside the same expression may require restructuring

## Related Concepts

- [[concepts/ownership]]
- [[concepts/copy-trait]]
- [[concepts/clone-trait]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
- [[summaries/rust-book-14-chapter-13-functional-language-features-iterators-and-closures]]
