---
title: "Function Pointers"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/20-chapter-19-advanced-features.txt"]
confidence: high
---

## Definition

A function pointer in Rust is the type `fn(Args) -> Ret` — a first-class value pointing to a named function. Unlike closures, function pointers do not capture any environment. They implement all three closure traits (`Fn`, `FnMut`, `FnOnce`), so any API expecting one of those traits accepts a bare function.

## How It Works

```rust
fn add_one(x: i32) -> i32 { x + 1 }

let ptr: fn(i32) -> i32 = add_one;
let result = ptr(5); // 6
```

Function pointers are passed by value (one machine word), called via indirect branch like closures, and coerce in both directions with closures-without-captures. They can cross FFI boundaries as long as the callee uses the same ABI.

## Key Parameters

- Type form: `fn(Args) -> Ret`
- No captured environment
- ABI variants: `extern "C" fn(...)`
- Coercion from non-capturing closures
- Implements `Fn`, `FnMut`, `FnOnce`

## When To Use

- Callback parameters when capturing context is unnecessary
- FFI export of Rust functions to be called from C
- Generic APIs that should accept both closures and free functions
- Tables of handlers indexed by enum tags

## Risks & Pitfalls

- Cannot capture environment — promote to closure if needed
- Calling convention mismatches with FFI cause UB
- Mixing function pointers and `dyn Fn` in tables requires care

## Related Concepts

- [[concepts/closures]]
- [[concepts/fn-traits]]
- [[concepts/ffi]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
