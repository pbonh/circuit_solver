---
title: 'The Rust Programming Language — Chapter 12: An I/O Project: Building a Command-Line
  Program'
type: source
id: source-rust-book-13-chapter-12-an-i-o-project-building-a-command-line-program
kind: derived-summary
tags:
- rust
- project
- cli
- error-handling
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/13-chapter-12-an-i-o-project-building-a-command-line-program.txt
---

## Key Points

- The chapter builds `minigrep`, a small `grep` clone that takes a query string and a filename, reads the file, and prints matching lines — exercising error handling, modularity, environment variables, I/O, and testing.
- CLI argument parsing uses `std::env::args()` which returns an iterator yielding `String` values. Calling `.collect::<Vec<String>>()` materializes them.
- Project structure follows the "thin binary" pattern: `src/main.rs` only parses CLI args and dispatches; all logic lives in `src/lib.rs` so it can be exercised by integration tests.
- A `Config` struct centralizes parsed CLI args (`query`, `file_path`, `ignore_case`). `Config::build(args)` returns `Result<Config, &'static str>` — using `Result` instead of `panic!` for user-facing errors.
- `Box<dyn Error>` is used as the return type of the runner function: `fn run(config: Config) -> Result<(), Box<dyn Error>>`, so different error sources can be propagated with `?`.
- File contents are read with `std::fs::read_to_string` returning `Result<String, io::Error>`.
- Test-driven development drives the design: a failing `test_one_result` test is written first, then the production `search` function is added to make it pass.
- Lifetime annotations on `search` (`fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>`) communicate that returned references point into `contents`, not `query`.
- An environment variable `IGNORE_CASE` toggles case-insensitive search via `std::env::var("IGNORE_CASE").is_ok()` — illustrating environment-driven configuration.
- Error messages are written to standard error using `eprintln!`, while results go to standard output via `println!` — making the program well-behaved in shell pipelines.

## Relevant Concepts

- [[concepts/cli-argument-parsing]] — reading and validating CLI args.
- [[concepts/error-handling]] — `Result`, `?`, `Box<dyn Error>`.
- [[concepts/automated-tests]] — TDD shapes the design of `search`.
- [[concepts/lifetimes]] — annotating which input a returned reference points to.
- [[concepts/environment-variables]] — `std::env::var`.
- [[concepts/standard-streams]] — `println!` vs `eprintln!`.
- [[concepts/separation-of-concerns]] — thin binary / fat library.

## Source Metadata

- Source type: book chapter (project chapter)
- Book title: The Rust Programming Language
- Chapter: 12 — An I/O Project: Building a Command-Line Program
- File path: `raw/rust_book/_txt/13-chapter-12-an-i-o-project-building-a-command-line-program.txt`
- Authors: Steve Klabnik and Carol Nichols
