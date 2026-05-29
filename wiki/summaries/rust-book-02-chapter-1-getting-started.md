---
title: 'The Rust Programming Language — Chapter 1: Getting Started'
type: source
id: summaries/rust-book-02-chapter-1-getting-started
kind: publication
tags:
- rust
- foundational
- cargo
- tooling
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/02-chapter-1-getting-started.txt
---

## Key Points

- Rust is installed via `rustup`, a CLI tool that manages Rust toolchains and associated tools across Linux, macOS, and Windows.
- A linker is required; users may need to install a C compiler (GCC/Clang on Linux, Xcode CLT on macOS, MSVC build tools on Windows).
- Rust is an ahead-of-time compiled language: `rustc main.rs` produces a standalone executable that does not require Rust to be installed on the target machine.
- `println!` is a macro (note the `!`), not a function; macros do not follow all the same rules as functions.
- Cargo is Rust's build system and package manager; `cargo new` scaffolds a project with `Cargo.toml`, `src/main.rs`, and a Git repo.
- `Cargo.toml` uses TOML format and declares `[package]` (name, version, edition) and `[dependencies]`; external libraries are called "crates".
- Core Cargo commands: `cargo new`, `cargo build`, `cargo run`, `cargo check` (fast type-check without producing a binary), and `cargo build --release` for optimized production builds.
- `Cargo.lock` records exact dependency versions for reproducible builds; it is managed by Cargo, not edited by hand.
- Debug builds land in `target/debug/`, optimized release builds in `target/release/`.
- `rust-analyzer` is the IDE-integration component the Rust team now focuses on.

## Relevant Concepts

- [[concepts/cargo]] — introduced as the standard build/package manager.
- [[concepts/rustup]] — the toolchain installer used to acquire Rust.
- [[concepts/crates]] — Cargo's term for Rust packages of code.
- [[concepts/macros]] — `println!` is the chapter's first macro example.
- [[concepts/rust-language]] — the language being installed and demonstrated.
- [[concepts/rust-language-server]] — `rust-analyzer` is mentioned as the IDE integration.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 1 — Getting Started
- File path: `raw/rust_book/_txt/02-chapter-1-getting-started.txt`
- Authors: Steve Klabnik and Carol Nichols
