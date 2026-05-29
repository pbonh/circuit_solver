---
title: Automated Tests (Rust)
type: claim
id: concepts/automated-tests
tags:
- rust
- foundational
- testing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/12-chapter-11-writing-automated-tests.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Automated tests in Rust are functions annotated with `#[test]` that `cargo test` compiles and runs as part of a dedicated test binary. The built-in framework supports assertions, panic expectations, parallel execution, filtering, and a `Result`-returning style for ergonomic propagation.

## How It Works

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_adds() {
        assert_eq!(2 + 2, 4);
    }
}
```

`cargo test` builds and links the test harness, runs every `#[test]` function in parallel by default, captures stdout/stderr, and reports passes/failures. `#[should_panic(expected = "...")]` inverts the success condition. Tests can return `Result<(), E>`; a returning-`Ok` is a pass.

## Key Parameters

- Attributes: `#[test]`, `#[should_panic]`, `#[ignore]`, `#[cfg(test)]`
- Threads: `--test-threads=N`
- Output: `--nocapture`
- Filtering: positional arg matches name substring
- Result-returning tests use `?` propagation

## When To Use

- Always — automated tests are table stakes in any non-trivial Rust project
- Cover algorithmic correctness, edge cases, regression scenarios
- Use `should_panic` for invariant-violation testing
- Integration tests for end-to-end public-API behavior

## Risks & Pitfalls

- Parallel tests sharing state can race — keep state local or guard with locks
- `--nocapture` is required to see prints from passing tests
- Long-running tests benefit from `#[ignore]` and a separate CI job
- Forgetting `#[cfg(test)]` on the `tests` module compiles tests into the production binary

## Related Concepts

- [[concepts/test-organization]]
- [[concepts/assertions]]
- [[concepts/cfg-attribute]]
- [[concepts/cargo]]

## Sources

- [[summaries/rust-book-12-chapter-11-writing-automated-tests]]
- [[summaries/rust-book-13-chapter-12-an-i-o-project-building-a-command-line-program]]
