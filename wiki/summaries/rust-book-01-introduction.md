---
title: "The Rust Programming Language — Introduction"
type: summary
tags: [rust, foundational, cargo]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/01-introduction.txt"]
confidence: high
---

## Key Points

- Rust aims to reconcile high-level ergonomics with low-level control — to give programmers both safety and performance without trade-off.
- The compiler plays a gatekeeper role: it refuses to compile code with subtle low-level bugs, including concurrency bugs.
- Rust ships with modern tooling: Cargo (build/dependency manager), Rustfmt (formatter), and the Rust Language Server (IDE integration).
- Target audiences include collaborative teams, students learning systems concepts, companies in production, open-source contributors, and developers who want both speed and stability.
- Rust uses "zero-cost abstractions" so safe code is also fast code — higher-level features compile down to the same machine code a hand-tuned low-level version would.
- The book mixes concept chapters with project chapters (Chapters 2, 12, 20) and concludes with appendices on keywords, operators, derivable traits, dev tools, editions, translations, and nightly Rust.
- Learning to read the Rust compiler's error messages is presented as an essential skill; the book deliberately shows examples that fail to compile.
- The mascot Ferris annotates code that does not compile, panics, or otherwise misbehaves.

## Relevant Concepts

- [[concepts/rust-language]] — the subject of the entire book.
- [[concepts/cargo]] — Rust's package manager and build tool, introduced as the standard workflow.
- [[concepts/zero-cost-abstractions]] — the core design promise that safety does not cost performance.
- [[concepts/rustfmt]] — formatting tool ensuring style consistency.
- [[concepts/rust-language-server]] — IDE integration component.
- [[concepts/ownership]] — Rust's signature feature, introduced in Chapter 4.
- [[concepts/cargo]] — Rust's package manager and build tool.
- [[entities/ferris]] — the unofficial Rust mascot used as an annotation in the book.

## Source Metadata

- Source type: book chapter (introduction)
- Book title: The Rust Programming Language
- Chapter: Introduction
- File path: `raw/rust_book/_txt/01-introduction.txt`
- Authors: Steve Klabnik and Carol Nichols, with contributions from the Rust Community
