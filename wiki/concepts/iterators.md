---
title: Iterators
type: claim
id: concepts/iterators
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/14-chapter-13-functional-language-features-iterators-and-closures.txt
- raw/rust_book/_txt/09-chapter-8-common-collections.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

An iterator in Rust is any type that implements the `Iterator` trait — a single required method `next(&mut self) -> Option<Self::Item>` plus many provided adapter methods. Iterators are lazy: adapters like `map` and `filter` produce new iterators that do no work until consumed by a terminal operation like `collect`, `sum`, or `for`.

## How It Works

`for x in iter` desugars to a `loop { match iter.next() { Some(x) => ..., None => break } }`. Collections expose `.iter()`, `.iter_mut()`, and `.into_iter()` to produce iterators that borrow, mutably borrow, or consume the collection. Adapter chains (`v.iter().map(|x| x*2).filter(|x| x > &10).collect()`) typically inline to the same code a hand-written loop would generate — iterators are a zero-cost abstraction.

## Key Parameters

- Required method: `next` returning `Option<Item>`
- Common adapters: `map`, `filter`, `take`, `skip`, `enumerate`, `zip`, `chain`, `flat_map`
- Terminal operations: `collect`, `sum`, `product`, `fold`, `count`, `any`, `all`, `for_each`
- Sub-traits: `ExactSizeIterator`, `DoubleEndedIterator`, `FusedIterator`

## When To Use

- Transforming collections without intermediate allocation
- Streaming over lines, bytes, network packets
- Composing computation pipelines
- Replacing imperative loops with declarative chains

## Risks & Pitfalls

- Iterators are lazy: forgetting a terminal call means no work happens
- `collect::<Vec<_>>()` allocates; consider streaming instead in hot loops
- Borrow-checker friction when iterating and mutating the same collection
- Iterator chains can become unreadable if too long — extract intermediate `let` bindings

## Related Concepts

- [[concepts/closures]]
- [[concepts/traits]]
- [[concepts/vec-type]]
- [[concepts/zero-cost-abstractions]]

## Sources

- [[summaries/rust-book-09-chapter-8-common-collections]]
- [[summaries/rust-book-14-chapter-13-functional-language-features-iterators-and-closures]]
