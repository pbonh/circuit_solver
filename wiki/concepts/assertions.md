---
title: Assertions (Rust)
type: claim
id: concepts/assertions
tags:
- rust
- foundational
- testing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/12-chapter-11-writing-automated-tests.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Assertions in Rust are macros that panic when a condition fails: `assert!(condition)`, `assert_eq!(left, right)`, `assert_ne!(left, right)`, and the debug-only variants `debug_assert!`, `debug_assert_eq!`, `debug_assert_ne!`. They are used in both production code (to enforce invariants) and tests (to verify behavior).

## How It Works

`assert!(cond)` panics with a default message if `cond` is false. `assert_eq!(a, b)` panics if `a != b`, printing both values for diagnostic purposes; the types must implement `PartialEq` and `Debug`. `debug_assert*` macros compile to no-ops in release mode, so expensive invariant checks remain inside the development build. All assertion macros accept additional format arguments for custom failure messages: `assert!(x > 0, "expected positive, got {}", x)`.

## Key Parameters

- Required trait bounds for `assert_eq!`: `PartialEq + Debug`
- Custom failure message format
- Debug vs release variants
- Combined with `#[should_panic]` to test failure paths

## When To Use

- Test bodies — primary mechanism for verifying expected results
- Production code — guard invariants that would corrupt program state if violated
- Use `debug_assert*` for expensive checks in hot paths

## Risks & Pitfalls

- Assertions in production code that should be `Result` returns
- Forgetting that `debug_assert*` is removed in release builds
- Floating-point equality with `assert_eq!` is brittle — use `(a - b).abs() < ε`
- Lossy `Debug` impls produce uninformative failure messages

## Related Concepts

- [[concepts/automated-tests]]
- [[concepts/panic]]
- [[concepts/macros]]

## Sources

- [[summaries/rust-book-12-chapter-11-writing-automated-tests]]
