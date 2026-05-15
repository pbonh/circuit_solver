---
title: "Standard Streams (Rust)"
type: concept
tags: [rust, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/13-chapter-12-an-i-o-project-building-a-command-line-program.txt"]
confidence: medium
---

## Definition

Rust's standard streams — `stdin`, `stdout`, `stderr` — provide line- and byte-oriented I/O via the `std::io` module. The `println!` and `eprintln!` macros write to stdout and stderr respectively; both buffer line-by-line in interactive contexts.

## How It Works

`std::io::stdout()`, `std::io::stderr()`, and `std::io::stdin()` return handles with thread-safe locking via `.lock()`. `println!("...")` and `eprintln!("...")` are shortcuts for `writeln!(stdout(), "...")`. Locking the handle once and writing in batches (`let mut out = stdout().lock(); writeln!(out, ...)`) avoids per-call lock overhead in hot loops.

## Key Parameters

- `stdout` (line-buffered when interactive, block-buffered when piped)
- `stderr` (unbuffered for diagnostic visibility)
- `stdin` (blocking by default; can be configured non-blocking on Unix)
- `println!`, `eprintln!`, `write!`, `writeln!`, `dbg!`

## When To Use

- Diagnostic output → `stderr` (`eprintln!`)
- Program output / pipeline data → `stdout` (`println!`)
- Interactive prompts use `stdout()` then read `stdin`
- High-volume output benefits from explicit lock + buffered writer

## Risks & Pitfalls

- Mixing `println!` in hot loops causes per-call lock contention
- Piping a Rust program that uses `println!` may behave differently because of buffering
- Forgetting to `flush` before exit can drop final output
- Writing diagnostic info to stdout corrupts pipelines

## Related Concepts

- [[concepts/cli-argument-parsing]]
- [[concepts/environment-variables]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-13-chapter-12-an-i-o-project-building-a-command-line-program]]
