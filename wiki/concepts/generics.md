---
title: Generics
type: claim
id: claim-generics
tags:
- rust
- foundational
- generics
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/11-chapter-10-generic-types-traits-and-lifetimes.txt
confidence:
  base: 0.85
---

## Definition

Generics in Rust let definitions take type parameters so they work uniformly across many concrete types. Functions, structs, enums, methods, and traits may all be generic. The compiler monomorphizes each usage into a concrete copy, so generics impose no runtime cost.

## How It Works

`fn largest<T: PartialOrd>(list: &[T]) -> &T` introduces a type parameter `T` with a trait bound. At each call site, the compiler substitutes a concrete `T` and generates dedicated code for that instantiation. Structs (`struct Point<T> { x: T, y: T }`) and enums (`Option<T>`, `Result<T, E>`) follow the same model. Method `impl` blocks can carry their own additional generic parameters or constrain existing ones.

## Key Parameters

- Type parameter declaration: `<T>`, `<T, U>`, `<'a, T>`
- Trait bounds: `T: Trait`, `T: A + B`, `where` clauses
- Default type parameters: `Vec<T, A = Global>`
- Const generics: `[T; N]` with `N: usize`
- Conditional impls: `impl<T: Display> Foo<T>`

## When To Use

- Writing reusable data structures and algorithms
- Modeling polymorphism without runtime dispatch
- Numerical kernels parameterized over scalar type (`f32` vs `f64`)
- Trait-bounded helper functions

## Risks & Pitfalls

- Monomorphization can balloon binary size
- Long compile times for heavily generic code
- Type inference can fail in surprising ways with many parameters
- Generic error messages can be hard to read

## Related Concepts

- [[concepts/traits]]
- [[concepts/trait-bounds]]
- [[concepts/monomorphization]]
- [[concepts/lifetimes]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-11-chapter-10-generic-types-traits-and-lifetimes]]
