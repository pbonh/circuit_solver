---
title: String Type
type: claim
id: claim-string-type
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/05-chapter-4-understanding-ownership.txt
- raw/rust_book/_txt/09-chapter-8-common-collections.txt
confidence:
  base: 0.85
---

## Definition

Rust has two principal string types: `String`, an owned, heap-allocated, growable UTF-8 buffer, and `&str` (string slice), a borrowed view of UTF-8 bytes. Both guarantee well-formed UTF-8; indexing produces byte offsets, not character offsets.

## How It Works

`String` internally is a `Vec<u8>` with the UTF-8 invariant. It grows via `push`, `push_str`, `+` (via `Add`), or `format!`. `&str` references a contiguous run of bytes inside a `String`, a `'static` literal, or any UTF-8 byte slice. Conversion is cheap: `&my_string` coerces to `&str` via `Deref`. Functions usually take `&str` to accept both kinds of input.

## Key Parameters

- UTF-8 invariant — invalid sequences cannot exist in safe code
- Byte-indexed slicing only on character boundaries (panics otherwise)
- Iteration via `.chars()`, `.bytes()`, `.char_indices()`, `.graphemes()` (external crate)
- Growth strategy of underlying `Vec<u8>`

## When To Use

- `String` when ownership and mutation are needed
- `&str` for function parameters and read-only views
- `String::from(...)` or `.to_string()` for explicit conversion from `&str`
- `format!("{}-{}", a, b)` for templated construction

## Risks & Pitfalls

- Indexing by byte vs character is a common bug
- `.len()` returns byte length, not character count
- Slicing across a multi-byte UTF-8 sequence panics
- Premature `String` allocations in hot paths

## Related Concepts

- [[concepts/slice-type]]
- [[concepts/vec-type]]
- [[concepts/ownership]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
- [[summaries/rust-book-09-chapter-8-common-collections]]
