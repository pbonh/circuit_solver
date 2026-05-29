---
title: 'The Rust Programming Language — Chapter 4: Understanding Ownership'
type: source
id: summaries/rust-book-05-chapter-4-understanding-ownership
kind: publication
tags:
- rust
- ownership
- foundational
- memory-safety
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt
---

## Key Points

- Ownership is Rust's most distinctive feature: a compile-time discipline that manages memory without a garbage collector and without manual `free`.
- Three ownership rules: each value has a single owner; only one owner at a time; when the owner goes out of scope, the value is dropped (and any heap resources released).
- The stack/heap distinction is foundational: stack values have known size and fast push/pop access; heap values require an allocator and indirection.
- `String` (a heap-allocated, growable, UTF-8 string) contrasts with `&str` string literals (stored in the binary) and illustrates ownership of heap resources.
- Assignment of a `String` is a *move*: the source variable is invalidated to prevent double-free. Types implementing `Copy` (scalars, fixed-size compounds of Copy types) are duplicated bit-for-bit instead.
- `Clone` is an explicit, potentially expensive deep copy; `Copy` is implicit and only allowed when no resource ownership is involved.
- Passing a value to a function transfers ownership unless you pass a reference (`&T` or `&mut T`).
- References let you borrow without moving. Borrow-checker rules: at any given time, either many `&T` references OR exactly one `&mut T`, never both. References must always be valid (no dangling).
- Mutable references are exclusive to prevent data races — this is the foundation of Rust's data-race-free guarantee.
- Slices are a borrowed view into a contiguous sequence: `&str` for strings, `&[T]` for arrays/vectors. Slice borrow rules prevent invalidation through the underlying owner.
- Idiomatic functions take `&str` rather than `&String` so they accept both `String` and `&str` callers.

## Relevant Concepts

- [[concepts/ownership]] — the central topic of the chapter.
- [[concepts/borrowing]] — borrowing without taking ownership.
- [[concepts/references]] — `&T` and `&mut T`, including the aliasing rule.
- [[concepts/move-semantics]] — assignment invalidates the source for non-Copy types.
- [[concepts/copy-trait]] — opt-in bitwise duplication.
- [[concepts/clone-trait]] — explicit deep-copy.
- [[concepts/drop-trait]] — automatic cleanup at end of scope.
- [[concepts/string-type]] — heap-allocated growable UTF-8 string.
- [[concepts/slice-type]] — borrowed view (`&str`, `&[T]`).
- [[concepts/stack-and-heap]] — memory-model background.
- [[concepts/memory-safety]] — what ownership buys you.
- [[concepts/lifetimes]] — referenced; develops later.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 4 — Understanding Ownership
- File path: `raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt`
- Authors: Steve Klabnik and Carol Nichols
