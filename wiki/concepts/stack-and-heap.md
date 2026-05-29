---
title: Stack and Heap
type: claim
id: concepts/stack-and-heap
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The stack and the heap are the two principal memory regions a Rust program uses at runtime. The stack stores values of statically-known size in LIFO frames; the heap stores values whose size or lifetime cannot be tied to a frame. Rust's ownership model is built on top of this distinction.

## How It Works

Each thread has a stack on which function calls allocate frames. Values of `Sized` types are stored inline. Heap allocation goes through the global allocator (default: system `malloc`/`free` proxy) and returns a pointer. Heap-owning types like `String`, `Vec<T>`, and `Box<T>` keep a pointer plus metadata (length, capacity) on the stack and the buffer on the heap. Drop impls release the heap memory deterministically when the owner falls out of scope.

## Key Parameters

- Stack: fast, frame-bounded, no global synchronization
- Heap: slower, arbitrary lifetime, requires allocator
- `Box<T>`: simplest heap pointer
- `Sized` vs `?Sized` (DST) types
- Default allocator can be swapped via `#[global_allocator]`

## When To Use

- Stack for small, short-lived, known-size data
- Heap for large buffers, dynamic-size data, polymorphic objects (`Box<dyn Trait>`)
- Heap when ownership must outlive the producing function's frame

## Risks & Pitfalls

- Stack overflow from deep recursion or huge stack-allocated arrays
- Heap fragmentation in long-running services
- Per-allocation overhead in tight loops — pre-allocate or pool
- Hidden allocations from `format!`, `Vec::push` (on grow), etc.

## Related Concepts

- [[concepts/ownership]]
- [[concepts/box-type]]
- [[concepts/smart-pointers]]
- [[concepts/memory-safety]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
