---
title: Methods
type: claim
id: concepts/methods
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/06-chapter-5-using-structs-to-structure-related-data.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Methods are functions associated with a type whose first parameter is some form of `self` (`self`, `&self`, `&mut self`). They are defined inside `impl` blocks and called with dot syntax (`value.method()`). Methods choose between consuming, borrowing, and mutably borrowing the receiver.

## How It Works

`&self` borrows the receiver shared; `&mut self` borrows it exclusively; `self` consumes (moves) it. The method-resolution algorithm auto-derefs the receiver: calling `s.len()` works for `&String` and `String`. Methods can return `Self` (the type) to enable builder-style chaining. Generic methods allow bounded polymorphism on additional type parameters.

## Key Parameters

- Receiver kind: `self`, `&self`, `&mut self`
- Return type — frequently `Self` for builder chains
- Generic parameters in addition to the type's parameters
- Visibility (`pub fn`)

## When To Use

- Operations naturally tied to a value (`vec.push(x)`, `node.degree()`)
- Builder APIs returning `Self` for chaining
- Encapsulating invariants by limiting how external code mutates internal state

## Risks & Pitfalls

- Choosing `self` when `&self` would suffice forces unnecessary moves
- Long method chains can produce confusing borrow-checker errors
- Method-resolution surprises with deref coercion and trait disambiguation
- Mutable methods that return `&mut Self` can run afoul of borrow rules

## Related Concepts

- [[concepts/impl-block]]
- [[concepts/associated-function]]
- [[concepts/traits]]
- [[concepts/ownership]]

## Sources

- [[summaries/rust-book-06-chapter-5-using-structs-to-structure-related-data]]
