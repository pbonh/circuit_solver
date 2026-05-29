---
title: Never Type
type: claim
id: concepts/never-type
tags:
- rust
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/20-chapter-19-advanced-features.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

The never type `!` is the type that has no values. It is the type assigned to expressions that never produce a value: `panic!(...)`, `loop { ... }` (without break-with-value), `continue`, `return`, and `process::exit`. `!` can coerce to any other type, which is what makes `match` arms that diverge typecheck alongside arms that produce a real value.

## How It Works

```rust
let value = match opt {
    Some(v) => v,
    None    => panic!("missing"),
};
```

The `panic!` arm has type `!`, which is silently coerced to whatever type `v` produces, making the whole match well-typed. `!` is also the return type of functions like `std::process::exit` that never return. It currently appears via stable usage but is not yet fully exposed as a first-class type (e.g., you cannot write `fn foo() -> !` in some positions stably-as-stable as of older editions; newer Rust treats `!` more uniformly).

## Key Parameters

- Expressions that produce `!`: `panic!`, `loop {}`, `return`, `continue`, `break`, `process::exit`
- Coercion: `!` to any `T`
- Function signature: `fn never() -> !`
- Stabilization: full first-class `!` is partial in stable, complete in nightly

## When To Use

- Functions that always panic, abort, or loop forever
- `match` arms that diverge (use `panic!`, `unreachable!`)
- Generic code where a default impl needs a divergent branch

## Risks & Pitfalls

- Confusing error messages when `!` interacts with type inference
- Not yet usable everywhere on stable Rust
- Easy to write functions that panic instead of returning a proper `Result`

## Related Concepts

- [[concepts/panic]]
- [[concepts/error-handling]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
