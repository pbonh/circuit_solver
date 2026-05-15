---
title: "Ownership"
type: concept
tags: [rust, ownership, foundational, memory-safety, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt"]
confidence: high
---

## Definition

Ownership is Rust's core memory-management discipline. Every value has a single owner; when the owner goes out of scope the value is dropped. Ownership can be moved or borrowed, but not duplicated by default. This set of rules is enforced at compile time and lets Rust guarantee memory safety without a garbage collector.

## How It Works

Three rules:

1. Each value in Rust has a variable that is its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

Assignment of non-`Copy` types moves the value; the source binding becomes invalid. Function calls move arguments unless they take references. Heap-allocated resources (e.g., `String`, `Vec<T>`) are dropped via their `Drop` impl when the owner falls out of scope, releasing memory deterministically.

## Key Parameters

- Move semantics for non-`Copy` types
- `Copy` trait for cheap, bitwise-copyable types
- `Clone` trait for explicit deep copies
- Scope-based drop order
- Interaction with borrowing rules (aliasing XOR mutability)

## When To Use

- Always — ownership is mandatory in safe Rust
- Strongly relevant when managing heap-allocated resources, file handles, sockets, locks
- Crucial for predictable resource cleanup (RAII) in long-running systems

## Risks & Pitfalls

- Fighting the borrow checker when porting GC-language patterns
- Over-use of `clone()` to silence errors at the cost of performance
- Confusion between move and copy for primitive vs. heap types
- Self-referential structures require alternative patterns (Rc/RefCell, Pin, unsafe)

## Related Concepts

- [[concepts/borrowing]]
- [[concepts/lifetimes]]
- [[concepts/memory-safety]]
- [[concepts/rust-language]]
- [[concepts/move-semantics]]
- [[concepts/drop-trait]]

## Sources

- [[summaries/rust-book-01-introduction]]
- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
- [[summaries/rust-book-06-chapter-5-using-structs-to-structure-related-data]]
- [[summaries/rust-book-09-chapter-8-common-collections]]
