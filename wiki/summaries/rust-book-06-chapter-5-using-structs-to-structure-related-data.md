---
title: 'The Rust Programming Language — Chapter 5: Using Structs to Structure Related
  Data'
type: source
id: source-rust-book-06-chapter-5-using-structs-to-structure-related-data
kind: derived-summary
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/06-chapter-5-using-structs-to-structure-related-data.txt
---

## Key Points

- A `struct` groups related, named fields into a single custom type — Rust's record type.
- Instance creation lists field values; field-init shorthand (`User { email, username }`) avoids repetition when the field name and variable name match.
- Struct update syntax (`..base`) copies remaining fields from another instance, moving them where they are not `Copy`.
- Tuple structs (`struct Point(i32, i32);`) create distinct types from a tuple shape — useful for unit-of-measure or wrapper types.
- Unit-like structs (`struct UnitMarker;`) carry no data but participate in the type system, useful with traits.
- Ownership of struct fields: by default, fields are owned; storing references requires lifetime annotations (covered in Chapter 10).
- The `#[derive(Debug)]` attribute auto-implements the `Debug` trait so values can be printed with `{:?}` / `{:#?}` format specifiers.
- Methods are functions associated with a type, defined inside `impl Type { ... }`; they take `&self`, `&mut self`, or `self` to choose borrow vs ownership.
- Associated functions (no `self`) are used as constructors (e.g., `Rectangle::new(w, h)`); they are called with `::` rather than `.`.
- Multiple `impl` blocks per type are allowed — useful when grouping methods by trait or with conditional compilation.
- The chapter's running example refactors a rectangle area calculation from loose variables to a tuple to a struct with a `width × height` method, demonstrating the rising clarity of typed encapsulation.

## Relevant Concepts

- [[concepts/struct-type]] — record types with named fields.
- [[concepts/tuple-struct]] — distinct named types from tuple shape.
- [[concepts/impl-block]] — where methods and associated functions live.
- [[concepts/methods]] — `fn name(&self, ...)`.
- [[concepts/associated-function]] — `fn new(...)` constructors.
- [[concepts/derive-macros]] — `#[derive(Debug)]` etc.
- [[concepts/debug-trait]] — printable representations.
- [[concepts/ownership]] — fields are owned by default.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 5 — Using Structs to Structure Related Data
- File path: `raw/rust_book/_txt/06-chapter-5-using-structs-to-structure-related-data.txt`
- Authors: Steve Klabnik and Carol Nichols
