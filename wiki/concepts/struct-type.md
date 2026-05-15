---
title: "Struct Type"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/06-chapter-5-using-structs-to-structure-related-data.txt"]
confidence: high
---

## Definition

A struct in Rust is a user-defined product type composed of named fields. Each field has its own type. Structs are the principal way to model "things" in a Rust program — netlist nodes, simulation matrices, parser states, configuration records.

## How It Works

`struct User { username: String, email: String, active: bool }` defines a type. Instances are created with `User { username, email, active }`. Field-init shorthand omits the `: name` part when local variables match field names. Struct update syntax `..other` fills the remaining fields from another instance (potentially moving non-`Copy` fields). Accessing fields uses dot notation: `user.email`. Field visibility defaults to private; `pub` makes a field part of the public API.

## Key Parameters

- Named fields with explicit types
- Field privacy (`pub` opt-in)
- Field-init shorthand
- Struct update syntax (`..other`)
- Visibility of the struct itself (`pub struct ...`)

## When To Use

- Domain models (devices, nets, simulation results)
- Configuration containers
- Function-call records and builder patterns
- Anywhere a tuple's positional access becomes confusing

## Risks & Pitfalls

- Update syntax may move fields out of `other`, invalidating it
- Forgetting to derive `Debug`/`Clone` causes friction in tests
- Large structs passed by value copy a lot — pass by reference
- Visibility surprises when mixing public struct with private fields

## Related Concepts

- [[concepts/tuple-struct]]
- [[concepts/impl-block]]
- [[concepts/methods]]
- [[concepts/derive-macros]]
- [[concepts/ownership]]

## Sources

- [[summaries/rust-book-06-chapter-5-using-structs-to-structure-related-data]]
