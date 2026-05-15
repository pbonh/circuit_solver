---
title: "Typestate Pattern"
type: concept
tags: [rust, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/18-chapter-17-object-oriented-programming-features-of-rust.txt"]
confidence: medium
---

## Definition

The typestate pattern encodes the states of a state machine as distinct Rust types, so that state-dependent operations are valid only for the appropriate type. Transitions consume one type and return the next, making invalid operations compile-time errors rather than runtime checks.

## How It Works

A `Post::new()` returns `DraftPost`. `DraftPost` has `.request_review(self) -> PendingReviewPost`. `PendingReviewPost` has `.approve(self) -> Post`. Only `Post` has `.content() -> &str`. Because each transition consumes the previous state by value, no leftover handle to an obsolete state remains. Adding the `#[must_use]` attribute and `Drop` impls can further tighten the contract.

## Key Parameters

- One type per state
- Move-by-value transition methods
- Optional generic parameters to share method bodies across states
- Use with `PhantomData<State>` markers in advanced forms

## When To Use

- State machines where every state has a distinct allowed operation set
- API designs that should refuse misuse at compile time (builder patterns)
- Protocols (connection states, parser stages, simulator phases)
- Resource lifecycle (open → ready → closed)

## Risks & Pitfalls

- Boilerplate per state — heavy for many transient states
- Difficulty when a state must store optional callbacks or polymorphic data
- Refactoring transitions is harder than adding an `enum` variant
- Tooling/IDE support may struggle with state-explosion

## Related Concepts

- [[concepts/struct-type]]
- [[concepts/enum-type]]
- [[concepts/newtype-pattern]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-18-chapter-17-object-oriented-programming-features-of-rust]]
