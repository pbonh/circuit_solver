---
title: "FFI (Foreign Function Interface)"
type: concept
tags: [rust, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/20-chapter-19-advanced-features.txt"]
confidence: medium
---

## Definition

FFI — the Foreign Function Interface — is the mechanism by which Rust calls (or is called by) code written in other languages, most commonly C. It is exposed through `extern "ABI"` blocks for imports and `#[no_mangle] pub extern "C" fn ...` for exports. All FFI calls are `unsafe` because the foreign code lies outside Rust's safety guarantees.

## How It Works

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe { abs(-3); }
```

The compiler emits the standard C calling convention for `extern "C"`. Other ABIs (`"system"`, `"stdcall"`, `"win64"`, etc.) target specific platforms. Tools like `bindgen` autogenerate Rust bindings from C headers; `cbindgen` generates C headers from Rust crates. `cxx` provides safer bridge primitives for C++ interop.

## Key Parameters

- ABI string (`"C"`, `"system"`, etc.)
- Calling convention and layout (`#[repr(C)]`)
- `#[no_mangle]` to preserve the symbol name
- Marshaling rules: pointers, `CString`/`CStr`, `*const c_char`
- Header generation (`cbindgen`) and binding generation (`bindgen`)

## When To Use

- Wrapping existing C/C++ libraries (BLAS, LAPACK, SPICE engines, SuiteSparse)
- Exposing Rust libraries to other-language ecosystems
- Embedding Rust in larger systems (Python via `pyo3`, JavaScript via `wasm-bindgen`)

## Risks & Pitfalls

- Memory-safety guarantees evaporate at the boundary
- Mismatched ABIs cause silent miscompilation
- Lifetime management of `Box<T>`-allocated pointers handed to foreign code
- Foreign panics / longjmps across the boundary are undefined behavior

## Related Concepts

- [[concepts/unsafe-rust]]
- [[concepts/raw-pointers]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
