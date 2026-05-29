---
title: Unsafe Rust
type: claim
id: concepts/unsafe-rust
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

Unsafe Rust is the subset of the language that lets the programmer perform operations the borrow checker cannot statically verify. It is unlocked by the `unsafe` keyword inside blocks, functions, traits, and impls. The five "superpowers" unsafe enables: dereferencing raw pointers, calling unsafe functions, accessing/modifying mutable statics, implementing unsafe traits, and accessing union fields.

## How It Works

`unsafe { ... }` marks a region where the programmer accepts responsibility for upholding invariants the compiler cannot check. The borrow checker still applies; unsafe does not turn off safety blanketly. Idiomatic Rust wraps unsafe code in safe abstractions: the unsafe block contains the minimum needed to satisfy invariants, and the public function above is safe to call.

## Key Parameters

- `unsafe fn` — caller must use `unsafe { ... }` to invoke
- `unsafe trait` / `unsafe impl` — implementor guarantees additional invariants
- Raw pointers `*const T`, `*mut T`
- `static mut` globals
- `union` types
- `extern "C"` blocks

## When To Use

- Calling C libraries (FFI)
- Implementing low-level data structures (lock-free queues, allocators)
- Performance-critical paths where the compiler cannot prove safety
- Constructing safe abstractions over hardware or kernel APIs

## Risks & Pitfalls

- Undefined behavior if invariants are violated — UB is contagious
- Memory safety, data races, and aliasing rules all become the programmer's burden
- `unsafe` blocks should be small, audited, and well-documented
- Using `unsafe` is *not* a workaround for borrow-checker friction — it relocates the problem

## Related Concepts

- [[concepts/memory-safety]]
- [[concepts/ffi]]
- [[concepts/raw-pointers]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
- [[summaries/rust-book-22-appendix-a-keywords]]
