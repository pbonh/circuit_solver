---
title: "The Rust Programming Language — Chapter 19: Advanced Features"
type: summary
tags: [rust, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/20-chapter-19-advanced-features.txt"]
confidence: high
---

## Key Points

- **Unsafe Rust** unlocks five "superpowers" that the borrow checker normally forbids: dereferencing raw pointers, calling unsafe functions/methods, accessing or modifying mutable static variables, implementing unsafe traits, and accessing union fields. Wrapping unsafe code in safe abstractions is the idiomatic discipline.
- Raw pointers (`*const T`, `*mut T`) bypass the aliasing/lifetime rules and can be null. Dereferencing them is unsafe.
- The `extern "ABI" { ... }` block declares foreign functions for FFI; calls into them are always unsafe. `extern "C" fn` exports a Rust function with the C ABI for external callers.
- **Advanced traits** topics: associated types (one impl per type, simpler signatures than generic parameters), default generic type parameters and operator overloading (`impl Add for Point`), fully-qualified `<Type as Trait>::method` disambiguation, supertraits (`trait Sub: Super`), and the newtype pattern as an orphan-rule workaround.
- **Advanced types**: type aliases (`type Result<T> = std::result::Result<T, io::Error>`) for ergonomic shorthand without changing the type itself, the never type (`!`) inhabiting expressions like `panic!` and infinite loops, and Dynamically Sized Types (`str`, `dyn Trait`) which are always used behind a pointer because their size is unknown at compile time.
- The `Sized` trait is implicitly required on every generic parameter; opt out with `?Sized` to accept DSTs.
- **Function pointers** (`fn(i32) -> i32`) are first-class values implementing all three `Fn`-family traits. They are coercible to (and from) closures with no captures.
- Returning closures from functions requires `Box<dyn Fn(...) -> ...>` (heterogeneous) or `impl Fn(...) -> ...` (homogeneous).
- **Macros** divide into declarative macros (`macro_rules!`) and procedural macros (custom derive, attribute-like, function-like). Procedural macros are crates with `proc-macro = true` in `Cargo.toml`, exporting functions that take a `TokenStream` and produce a `TokenStream`.
- Macros differ from functions: they expand at compile time, can take variable numbers of arguments, are scoped by `#[macro_use]` / `use`, and can shadow callers' variable names if not hygienic.

## Relevant Concepts

- [[concepts/unsafe-rust]] — the unsafe escape hatch.
- [[concepts/ffi]] — foreign function interface.
- [[concepts/associated-types]] — placeholders inside trait definitions.
- [[concepts/operator-overloading]] — implementing `std::ops::*` traits.
- [[concepts/never-type]] — `!`, the type with no values.
- [[concepts/dynamically-sized-types]] — `str`, `dyn Trait`, etc.
- [[concepts/sized-trait]] — implicit bound on generics.
- [[concepts/function-pointers]] — first-class function values.
- [[concepts/procedural-macros]] — derive/attribute/function-like.
- [[concepts/declarative-macros]] — `macro_rules!`.
- [[concepts/newtype-pattern]] — wrapping foreign types for foreign traits.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 19 — Advanced Features
- File path: `raw/rust_book/_txt/20-chapter-19-advanced-features.txt`
- Authors: Steve Klabnik and Carol Nichols
