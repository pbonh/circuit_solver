---
title: Raw Pointers
type: claim
id: claim-raw-pointers
tags:
- rust
- unsafe
- pointers
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/20-chapter-19-advanced-features.txt
confidence:
  base: 0.65
---

## Definition

Per The Rust Programming Language Chapter 19 ("Advanced Features → Unsafe Rust → Dereferencing a Raw Pointer"): "Unsafe Rust has two new types called raw pointers that are similar to references. As with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`, respectively. The asterisk isn't the dereference operator; it's part of the type name. In the context of raw pointers, immutable means that the pointer can't be directly assigned to after being dereferenced."

## How It Works

Raw pointers differ from references and smart pointers in four ways enumerated by the Rust Book:

- "Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location."
- "Aren't guaranteed to point to valid memory."
- "Are allowed to be null."
- "Don't implement any automatic cleanup."

Construction is safe; dereferencing is unsafe. Listing 19-1 of the Rust Book:

```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

Reading or writing `*r1` / `*r2` requires an `unsafe` block.

## Key Parameters

- `*const T` vs `*mut T` — write-permission marker in the type.
- Provenance — converting from a reference yields a pointer guaranteed to be valid at conversion time, but not afterwards.
- Alignment, validity, aliasing — caller responsibility under unsafe.

## When To Use

- FFI boundaries where C structures or function pointers expect plain pointers.
- Performance-critical data-structure internals that need aliasing patterns the borrow checker rejects.
- Implementing primitives like `Cell`, `RefCell`, `UnsafeCell`, `Pin`, and most smart pointers.

## Risks & Pitfalls

- Null dereference, dangling-pointer dereference, data races — all are undefined behavior.
- `unsafe` does not turn off the borrow checker for safe references; it only enables the five unsafe superpowers.
- Provenance bugs are subtle: a raw pointer can outlive the reference it was derived from.

## Related Concepts

- [[concepts/unsafe-rust]]
- [[concepts/raii]]
- [[concepts/ownership]]
- [[concepts/smart-pointers]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
