---
title: "Fn Traits"
type: concept
tags: [rust, foundational, traits, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/14-chapter-13-functional-language-features-iterators-and-closures.txt"]
confidence: high
---

## Definition

`Fn`, `FnMut`, and `FnOnce` are the family of standard-library traits that describe callable values. Every closure implements at least `FnOnce`; closures that don't move captures also implement `FnMut`; closures that don't even mutate captures implement `Fn`. Function pointers implement all three.

## How It Works

The hierarchy is `Fn: FnMut: FnOnce`. A function expecting `F: Fn(T) -> U` accepts the most restricted (read-only-capture) closures, while one expecting `F: FnOnce(T) -> U` accepts any closure including those that consume captures. Trait choice in the bound determines what callers can pass. Calling a `Fn` is `f()`; calling an `FnOnce` consumes it (`(f)()` only once).

## Key Parameters

- `Fn(Args) -> Ret` — shared borrow of captures
- `FnMut(Args) -> Ret` — exclusive borrow of captures
- `FnOnce(Args) -> Ret` — consumes captures
- Function pointer type `fn(Args) -> Ret` implements all three

## When To Use

- Iterator adapters typically bound on `FnMut`
- Callbacks that may consume state once should bound on `FnOnce`
- Read-only callbacks bound on `Fn`
- Thread spawning requires `FnOnce + Send + 'static`

## Risks & Pitfalls

- Over-restricting to `Fn` excludes the common mutating-capture case
- Storing closures in a struct requires picking one trait at a time (or trait objects)
- `Send` and `Sync` are orthogonal — they must be added explicitly when threading

## Related Concepts

- [[concepts/closures]]
- [[concepts/traits]]
- [[concepts/iterators]]

## Sources

- [[summaries/rust-book-14-chapter-13-functional-language-features-iterators-and-closures]]
- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
