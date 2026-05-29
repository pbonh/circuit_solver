---
title: Operator Overloading
type: claim
id: claim-operator-overloading
tags:
- rust
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/20-chapter-19-advanced-features.txt
confidence:
  base: 0.85
---

## Definition

Rust supports operator overloading by allowing types to implement specific traits from `std::ops`. Implementing `Add` enables `+`, `Sub` enables `-`, `Mul` enables `*`, `Index` enables `[]`, etc. The set of overloadable operators is fixed; users cannot define wholly new operators.

## How It Works

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point { x: f64, y: f64 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
```

`Add` takes a default generic parameter `Rhs = Self` so binary operators work both with same-type and cross-type operands. The trait's `Output` associated type lets the result differ from the operand types (useful for unit-conversion arithmetic).

## Key Parameters

- `std::ops::{Add, Sub, Mul, Div, Rem, Neg, Not, Index, IndexMut, Deref, DerefMut, ...}`
- Default `Rhs` parameter on binary operators
- `Output` associated type for result types
- `AddAssign`, `SubAssign`, etc., for compound assignment

## When To Use

- Mathematical types (vectors, matrices, complex numbers, units)
- Custom collections needing `[]` indexing
- DSLs where operator overloading aids readability
- Newtype wrappers that should behave like their inner numeric type

## Risks & Pitfalls

- Surprising behavior when operator semantics differ from the obvious
- Multiplying a unit-typed value can hide allocations
- Excessive overloading makes API documentation harder to scan
- Auto-deref combined with overloaded operators can produce confusing diagnostics

## Related Concepts

- [[concepts/traits]]
- [[concepts/associated-types]]
- [[concepts/newtype-pattern]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
- [[summaries/rust-book-23-appendix-b-operators-and-symbols]]
