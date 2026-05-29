---
title: Closures
type: claim
id: concepts/closures
tags:
- rust
- foundational
- functional-programming
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/14-chapter-13-functional-language-features-iterators-and-closures.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A closure in Rust is an anonymous function value that can capture variables from its enclosing scope. Closures have types unique to each definition; they implement one or more of the `Fn`, `FnMut`, `FnOnce` traits based on how they use their captured environment.

## How It Works

`|x, y| x + y + offset` declares a closure with two parameters that captures `offset` from the surrounding scope. The compiler infers parameter and return types from usage. Capture mode is chosen automatically: read-only borrow (`&T`) for `Fn`, mutable borrow (`&mut T`) for `FnMut`, or move (`T`) for `FnOnce`. `move ||` forces the move mode — required when sending the closure to another thread or returning it from a function.

## Key Parameters

- Parameter list `|args|` (types optional)
- Capture mode: `&`, `&mut`, or move (forced by `move`)
- Trait family: `Fn` ⊂ `FnMut` ⊂ `FnOnce`
- Return-position types: `impl Fn`/`Box<dyn Fn>`

## When To Use

- Passing short pieces of logic to higher-order functions (`map`, `filter`, `sort_by`)
- Event handlers and callbacks
- Encapsulating per-task state inside a function-like value
- Builders that defer computation

## Risks & Pitfalls

- Borrowed captures keep the source alive — surprising lifetime errors
- Returning a closure capturing a local reference requires moving captures or boxing
- Closure types are unnameable; storing in a struct usually requires generic parameters
- Iterator adapter chains with many closures can be hard to type-check mentally

## Related Concepts

- [[concepts/fn-traits]]
- [[concepts/iterators]]
- [[concepts/move-semantics]]
- [[concepts/impl-trait]]

## Sources

- [[summaries/rust-book-14-chapter-13-functional-language-features-iterators-and-closures]]
- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
