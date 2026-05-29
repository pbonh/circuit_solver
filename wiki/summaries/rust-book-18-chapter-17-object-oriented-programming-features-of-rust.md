---
title: 'The Rust Programming Language — Chapter 17: Object-Oriented Programming Features
  of Rust'
type: source
id: summaries/rust-book-18-chapter-17-object-oriented-programming-features-of-rust
kind: publication
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/18-chapter-17-object-oriented-programming-features-of-rust.txt
---

## Key Points

- Rust does not match every definition of an "object-oriented" language, but it offers the major capabilities OO programmers care about: encapsulation, polymorphism, and dynamic dispatch — implemented through traits rather than classes.
- "Object contains data and behavior": structs hold data, `impl` blocks define methods. Encapsulation is provided by module-level visibility (`pub` opt-in).
- Inheritance is *not* supported. Rust replaces inheritance with two mechanisms: trait default methods (code sharing) and trait objects / generics (polymorphism). The text argues this is by design — inheritance encourages over-coupling.
- Trait objects (`Box<dyn Trait>`, `&dyn Trait`) enable runtime polymorphism. Each trait object carries a pointer to data and a pointer to the trait's vtable.
- Generics + trait bounds enable *static* polymorphism; trait objects enable *dynamic* polymorphism. The choice trades flexibility (dynamic) against optimization opportunity (static).
- Object safety: a trait can become a trait object only if its methods do not use `Self` by value and have no generic methods (with some other constraints). Object-safety violations are compile errors when attempting to make `dyn Trait`.
- A `gui` example (`Vec<Box<dyn Draw>>`) shows heterogeneous storage of widgets implementing a common `Draw` trait, plus dynamic dispatch on `screen.run()`.
- The chapter closes with the **typestate pattern**: encoding state transitions as moves between distinct types so invalid operations become compile errors. Example: `Post::new()` returns `DraftPost`, `.request_review()` returns `PendingReviewPost`, `.approve()` returns `Post` — only the final state has a `.content()` method.
- Comparison with classical OO state-machine implementations: typestate replaces runtime checks with compile-time guarantees and exchanges flexibility for safety.

## Relevant Concepts

- [[concepts/trait-objects]] — `Box<dyn Trait>` for dynamic dispatch.
- [[concepts/dynamic-dispatch]] — vtable-based polymorphism.
- [[concepts/static-dispatch]] — monomorphized generic-bound dispatch.
- [[concepts/object-safety]] — what traits can become `dyn Trait`.
- [[concepts/typestate-pattern]] — state machines encoded as types.
- [[concepts/rust-polymorphism]] — Rust's flavor of polymorphism (traits, not inheritance).
- [[concepts/rust-encapsulation]] — module-level visibility in place of access modifiers.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 17 — Object-Oriented Programming Features of Rust
- File path: `raw/rust_book/_txt/18-chapter-17-object-oriented-programming-features-of-rust.txt`
- Authors: Steve Klabnik and Carol Nichols
