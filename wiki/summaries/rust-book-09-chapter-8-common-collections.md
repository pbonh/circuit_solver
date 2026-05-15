---
title: "The Rust Programming Language — Chapter 8: Common Collections"
type: summary
tags: [rust, foundational, collections, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/09-chapter-8-common-collections.txt"]
confidence: high
---

## Key Points

- The chapter covers three heap-allocated, growable collections: `Vec<T>`, `String`, and `HashMap<K, V>`. Unlike arrays and tuples, their size can change at runtime.
- `Vec<T>` is a contiguous, heap-allocated, growable list of `T`. `Vec::new()` or the `vec![1, 2, 3]` macro builds one; `.push()` appends; indexing with `v[i]` panics on out-of-bounds while `.get(i)` returns `Option<&T>`.
- Iterating a `Vec`: `for x in &v` borrows; `for x in &mut v` mutably borrows (use `*x +=` to write through); `for x in v` consumes the vector.
- To store mixed types, use a vector of an enum whose variants cover the cases: `Vec<SpreadsheetCell>` where `enum SpreadsheetCell { Int(i32), Text(String), Float(f64) }`.
- `String` is a `Vec<u8>` wrapper with the UTF-8 invariant. Methods: `push_str`, `push`, `+` (consumes the left side), `format!`. Byte/character indexing distinctions matter: `s.len()` is byte length; `s.chars()` iterates Unicode scalar values; `s.bytes()` iterates raw bytes; grapheme clusters need an external crate.
- Indexing a `String` directly (e.g., `s[0]`) is rejected by the compiler because UTF-8 means byte indices and character indices are different.
- `HashMap<K, V>` from `std::collections::HashMap` stores key-value pairs hashed for O(1) average-case access. The default hasher is DoS-resistant SipHash; faster alternatives (e.g., `ahash`, `fxhash`) trade safety for speed when input is trusted.
- Hash map ownership: inserting a `String` key moves it; inserting a borrowed key requires the borrow live as long as the map.
- Updating a hash map: `insert` overwrites; `entry(key).or_insert(default)` inserts only if absent; `entry(key).or_insert(0) += 1` is the canonical counter pattern.
- Choice guidance: `Vec<T>` for ordered/indexed access, `String` for text, `HashMap<K, V>` for keyed lookup.

## Relevant Concepts

- [[concepts/vec-type]] — growable heap-allocated list.
- [[concepts/string-type]] — heap-allocated UTF-8 string.
- [[concepts/hash-map]] — keyed associative container.
- [[concepts/collections]] — broader family of standard collections.
- [[concepts/iterators]] — iteration over collections.
- [[concepts/ownership]] — collections own their elements.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 8 — Common Collections
- File path: `raw/rust_book/_txt/09-chapter-8-common-collections.txt`
- Authors: Steve Klabnik and Carol Nichols
