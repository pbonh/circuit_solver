---
title: Hash Map
type: claim
id: claim-hash-map
tags:
- rust
- foundational
- collections
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/09-chapter-8-common-collections.txt
confidence:
  base: 0.85
---

## Definition

`std::collections::HashMap<K, V>` is Rust's associative container backed by a hash table. It stores key-value pairs and offers expected O(1) lookup, insertion, and removal. Keys must implement `Eq + Hash`. The default hasher (SipHash) is DoS-resistant; faster non-cryptographic hashers are available via the `BuildHasher` trait.

## How It Works

`HashMap::new()` creates an empty map. `.insert(k, v)` adds or overwrites and returns the old value as `Option<V>`. `.get(&k)` returns `Option<&V>`. The `.entry(k)` API enables the canonical patterns `or_insert(default)`, `or_insert_with(|| ...)`, and read-modify-write via `*counter += 1`. Inserting an owned `String` key moves it; inserting `&str` requires a `'static` (or matching) lifetime.

## Key Parameters

- Key/value types `K`, `V` with `K: Eq + Hash`
- Hash builder: default `RandomState`, customizable for performance
- Capacity and load factor (managed automatically)
- Entry API for upsert semantics

## When To Use

- Lookup tables keyed by name, ID, or hashable struct
- Counting / histogram patterns
- Caches of computed values
- Sparse data (vs dense arrays indexed by integer)

## Risks & Pitfalls

- Iteration order is unspecified and changes across runs (intentional)
- Non-cryptographic hashers (e.g., `FxHashMap`) speed up access but lose DoS resistance
- Mutating a key in a way that changes its hash breaks the map
- Heavy `Clone` of `String` keys in inner loops is a common performance hazard

## Related Concepts

- [[concepts/collections]]
- [[concepts/vec-type]]
- [[concepts/traits]]
- [[concepts/ownership]]

## Sources

- [[summaries/rust-book-09-chapter-8-common-collections]]
