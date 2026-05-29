---
title: Question Mark Operator
type: claim
id: claim-question-mark-operator
tags:
- rust
- foundational
- error-handling
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/10-chapter-9-error-handling.txt
confidence:
  base: 0.85
---

## Definition

The `?` operator is Rust's syntactic sugar for early-returning from a function on the error case of `Result` (or `None` case of `Option`). Placed after an expression of type `Result<T, E>` or `Option<T>`, it unwraps the success value and propagates the failure value up the call stack with one character.

## How It Works

`let x = foo()?;` desugars roughly to:

```rust
let x = match foo() {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
};
```

For `Option<T>`, the `Err` branch becomes `return None`. The `From::from` call enables ergonomic error conversion when the enclosing function returns a different (but `From`-compatible) error type. `?` only works inside functions whose return type is itself `Result` or `Option`.

## Key Parameters

- Operand type: `Result<T, E>` or `Option<T>`
- Enclosing function return type must match (after `From` conversion)
- Combines with combinators (`foo()?.bar()?.baz()`)
- `Try` trait underpins extensibility

## When To Use

- Propagating errors up the call stack in the common case
- Chaining many fallible operations cleanly
- Composing with custom error enums that implement `From<InnerErr>`

## Risks & Pitfalls

- Forgetting the function's return type must be `Result`/`Option` (compile error otherwise)
- Heavy reliance on `Box<dyn Error>` loses specificity
- `From` conversions can be surprising — when an unrelated `From` impl is in scope
- Deep `?` chains can hide *which* call failed; add `.context(...)` (anyhow) or `.map_err(...)`

## Related Concepts

- [[concepts/error-handling]]
- [[concepts/result-type]]
- [[concepts/option-type]]
- [[concepts/error-trait]]

## Sources

- [[summaries/rust-book-10-chapter-9-error-handling]]
- [[summaries/rust-book-23-appendix-b-operators-and-symbols]]
