---
title: Memory Safety
type: claim
id: concepts/memory-safety
tags:
- rust
- foundational
- memory-safety
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/00-foreword.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Memory safety is the property of a program that it never accesses memory in an undefined way — no use-after-free, double-free, buffer overflows, or reads of uninitialized memory. Rust aims to guarantee memory safety at compile time without a garbage collector.

## How It Works

Rust achieves memory safety through ownership, borrowing, and lifetimes enforced by the compiler. Each value has a single owner; references are either shared (immutable) or unique (mutable) but not both at once; the borrow checker proves that no reference outlives the data it points to. Unsafe code blocks allow targeted escape hatches but localize the audit surface.

## Key Parameters

- Single-owner rule
- Aliasing XOR mutability for references
- Lifetime parameters proving reference validity
- Move semantics on assignment

## When To Use

- Whenever correctness against memory-corruption classes of bugs matters
- Security-sensitive code (parsers, network stacks)
- Long-running services where leaks compound
- Numerical software where overruns silently corrupt results

## Risks & Pitfalls

- `unsafe` blocks can reintroduce memory unsafety if invariants are not upheld
- FFI to C/C++ requires careful adaptation
- Pure safety does not prevent logic errors or resource exhaustion

## Related Concepts

- [[concepts/ownership]]
- [[concepts/borrowing]]
- [[concepts/lifetimes]]
- [[concepts/rust-language]]
- [[concepts/fearless-concurrency]]

## Related Decisions

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph]] — Chooses PyO3 in-process binding to preserve Rust memory-safety guarantees at the Python boundary.
- [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]] — Preserves memory safety in the numeric solver by eliminating FFI to C/C++ sparse direct solver libraries.

## Sources

- [[summaries/rust-book-00-foreword]]
- [[summaries/rust-book-05-chapter-4-understanding-ownership]]
