---
title: 'The Rust Programming Language — Chapter 13: Functional Language Features:
  Iterators and Closures'
type: source
id: summaries/rust-book-14-chapter-13-functional-language-features-iterators-and-closures
kind: publication
tags:
- rust
- foundational
- functional-programming
- iterators
- closures
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/14-chapter-13-functional-language-features-iterators-and-closures.txt
---

## Key Points

- Closures are anonymous functions that capture variables from their environment: `|x| x + offset`. Type annotations on parameters and return values are usually inferred from usage.
- Closures capture from the environment in one of three ways — by reference (`&T`), by mutable reference (`&mut T`), or by move (`T`). The compiler picks the least restrictive mode that allows the body to compile.
- `move ||` forces a closure to take ownership of its captures — required when moving the closure to another thread or returning it from a function.
- Closures implement one or more of three traits in the `Fn` family: `FnOnce` (consumes captures), `FnMut` (mutates captures), `Fn` (read-only captures). Each is a super-trait of the next.
- Returning closures from functions uses `Box<dyn Fn(...) -> ...>` or `impl Fn(...) -> ...`. `impl Fn` is preferred when one concrete closure type is returned; `Box<dyn Fn>` allows different closure types at runtime.
- The `Iterator` trait requires one method: `fn next(&mut self) -> Option<Self::Item>`. Many provided adapter methods are built on top of `next`.
- Adapter methods divide into *consuming* (`collect`, `sum`, `fold`, `count`) and *producing* (`map`, `filter`, `take`, `enumerate`, `zip`). Adapters are lazy: nothing happens until a consuming call.
- Iterators integrate with closures: `v.iter().map(|x| x * 2).filter(|x| x > &10).collect()` is the canonical pipeline.
- Implementing a custom iterator requires only the `next` method; default impls of `map`, `filter`, `collect`, etc., automatically work.
- Benchmarks at the end of the chapter show iterator-based code can match or beat hand-written index loops — Rust's compiler aggressively inlines and unrolls them.

## Relevant Concepts

- [[concepts/closures]] — anonymous functions capturing environment.
- [[concepts/fn-traits]] — `Fn`, `FnMut`, `FnOnce` family.
- [[concepts/iterators]] — `Iterator` trait and adapter chains.
- [[concepts/zero-cost-abstractions]] — iterator chains compile to tight loops.
- [[concepts/move-semantics]] — `move ||` captures.
- [[concepts/impl-trait]] — return-position closure types.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 13 — Functional Language Features: Iterators and Closures
- File path: `raw/rust_book/_txt/14-chapter-13-functional-language-features-iterators-and-closures.txt`
- Authors: Steve Klabnik and Carol Nichols
