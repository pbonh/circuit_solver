---
title: "The Rust Programming Language — Chapter 2: Programming a Guessing Game"
type: summary
tags: [rust, foundational, error-handling, project]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt"]
confidence: high
---

## Key Points

- The guessing-game project introduces Rust concretely: read user input, parse it as a number, compare it against a randomly generated secret, and loop until the user guesses correctly.
- Variables in Rust are immutable by default; mutability is opted into with `let mut`.
- The standard library prelude provides commonly used items automatically; everything else must be brought into scope with `use`.
- `String::new` is an *associated function* on the `String` type, called with the `::` syntax — Rust distinguishes between associated functions and methods.
- References are passed with `&` and made mutable with `&mut`; like variables, references are immutable by default.
- `Result<T, E>` is an enum with `Ok` and `Err` variants; methods like `.expect("...")` panic on `Err` and unwrap the `Ok` value.
- The compiler emits a `#[warn(unused_must_use)]` warning when a `Result` is ignored — a deliberate nudge toward error handling.
- `println!` placeholders include named (`{guess}`) and positional (`{}`) forms.
- External crates (here `rand`) are added by listing them under `[dependencies]` in `Cargo.toml` and brought into scope with `use rand::Rng;`.
- `Cargo.lock` pins resolved versions; `cargo update` re-resolves dependencies within SemVer constraints.
- `match` expressions provide exhaustive pattern matching; the chapter uses `match` against `Ordering::Less | Greater | Equal` and against `Result` to recover gracefully from parse errors.
- Shadowing (`let guess: u32 = guess.trim().parse().expect(...)`) lets you reuse a name with a new type while keeping the binding immutable.
- The `loop` keyword creates an infinite loop; `break` exits, and the body can contain `continue` to skip an iteration.

## Relevant Concepts

- [[concepts/cargo]] — used for project scaffolding and dependency management.
- [[concepts/crates]] — `rand` introduced as an external crate.
- [[concepts/variables-and-mutability]] — let / let mut introduced.
- [[concepts/shadowing]] — reusing a variable name to change type.
- [[concepts/references]] — `&` and `&mut` introduced in the context of `read_line`.
- [[concepts/result-type]] — `Result<T, E>` for error handling.
- [[concepts/pattern-matching]] — `match` on `Ordering` and on `Result`.
- [[concepts/enum-type]] — `Result` and `Ordering` are enums.
- [[concepts/macros]] — `println!` is again called out as a macro.
- [[concepts/error-handling]] — chapter introduces error propagation via `.expect`.

## Source Metadata

- Source type: book chapter (project chapter)
- Book title: The Rust Programming Language
- Chapter: 2 — Programming a Guessing Game
- File path: `raw/rust_book/_txt/03-chapter-2-programming-a-guessing-game.txt`
- Authors: Steve Klabnik and Carol Nichols
