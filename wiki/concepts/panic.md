---
title: Panic
type: claim
id: claim-panic
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

A panic in Rust is an unrecoverable error that aborts (the default after unwinding) the current thread. Panics are produced by the `panic!` macro, by `unwrap`/`expect` on `None`/`Err`, by out-of-bounds indexing, by arithmetic overflow in debug builds, and by other invariant violations.

## How It Works

`panic!("message")` prints the message and a backtrace (if `RUST_BACKTRACE=1`), then unwinds the stack, running each frame's `Drop` impls, until the thread terminates. Setting `panic = "abort"` in `Cargo.toml`'s `[profile]` swaps unwinding for immediate process abort — smaller binaries, faster panics, but skipped cleanup. `std::panic::catch_unwind` allows recovering from a panic at a designated boundary (useful when crossing FFI).

## Key Parameters

- Strategy: `unwind` (default) vs `abort`
- `RUST_BACKTRACE` env var for backtraces
- `std::panic::catch_unwind` for trusted boundaries
- `#[panic_handler]` for `no_std` environments

## When To Use

- Invariant violations (impossible state)
- Tests where the failure message is the diagnostic
- Prototypes and examples
- Initialization code where any failure is fatal anyway

## Risks & Pitfalls

- Panicking across FFI is undefined behavior unless caught
- `panic = "abort"` skips destructors, leaking external resources
- Panics inside `Drop::drop` during unwinding cause process abort
- Over-reliance on `unwrap` in production code

## Related Concepts

- [[concepts/error-handling]]
- [[concepts/result-type]]
- [[concepts/drop-trait]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-10-chapter-9-error-handling]]
