---
title: "Collections (Rust std)"
type: concept
tags: [rust, foundational, collections, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/09-chapter-8-common-collections.txt"]
confidence: high
---

## Definition

Rust's `std::collections` module provides the standard library's general-purpose collection types: sequence collections (`Vec<T>`, `VecDeque<T>`, `LinkedList<T>`), associative collections (`HashMap<K, V>`, `BTreeMap<K, V>`), sets (`HashSet<T>`, `BTreeSet<T>`), and the binary heap (`BinaryHeap<T>`). All are heap-allocated and growable.

## How It Works

Each collection owns its contents. They expose iteration via the `Iterator` trait and slicing/indexing where appropriate. Hash-based collections require `Eq + Hash`; tree-based collections require `Ord`. Most collections have an `entry` API or equivalent upsert pattern. Cargo's `std::collections` choices are conservative; the broader ecosystem provides `IndexMap`, `SmallVec`, `ArrayVec`, `Rope`, etc., for specialized needs.

## Key Parameters

- Trait requirements per collection (`Eq`, `Hash`, `Ord`, `Clone`)
- Growth and reallocation strategy
- Iteration order (insertion, sorted, undefined)
- Memory layout (contiguous vs node-based)

## When To Use

- `Vec` for ordered, indexed sequences (default choice)
- `HashMap` for keyed lookup with arbitrary hashable keys
- `BTreeMap` when sorted iteration order matters
- `HashSet`/`BTreeSet` for membership tests
- `VecDeque` for double-ended queue / ring buffer
- `BinaryHeap` for priority queues

## Risks & Pitfalls

- `LinkedList<T>` is almost always slower than `Vec` and is rarely the right choice
- Hash-collection iteration order changes across runs
- Re-hashing on grow invalidates iterators
- Choosing the wrong collection for the workload can dominate hot-path cost

## Related Concepts

- [[concepts/vec-type]]
- [[concepts/hash-map]]
- [[concepts/iterators]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-09-chapter-8-common-collections]]
