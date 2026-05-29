---
title: Environment Variables (Rust)
type: claim
id: claim-environment-variables
tags:
- rust
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/13-chapter-12-an-i-o-project-building-a-command-line-program.txt
confidence:
  base: 0.65
---

## Definition

Environment variables provide named string configuration inherited from the operating system. Rust accesses them through `std::env::var(name) -> Result<String, VarError>` (returns `Err` if missing or non-UTF-8) and `std::env::var_os(name) -> Option<OsString>` (preserves arbitrary bytes).

## How It Works

A process inherits an environment from its parent. `std::env::vars()` iterates all variables. `std::env::set_var(name, value)` mutates the current process's environment; `std::env::remove_var(name)` deletes one. Build-time variables from `Cargo.toml` are exposed as `env!("CARGO_PKG_VERSION")` etc. via the `env!` macro.

## Key Parameters

- `std::env::var` / `var_os`
- `std::env::set_var` / `remove_var`
- `env!` / `option_env!` macros for compile-time access
- `Result` vs `Option` return shape

## When To Use

- Optional configuration toggles (`IGNORE_CASE=1`)
- Secrets in deployments (never hard-code keys)
- Build-time metadata embedded in binaries
- Conditional logging or instrumentation

## Risks & Pitfalls

- `set_var` / `remove_var` in multi-threaded programs is unsafe on some platforms
- Non-UTF-8 environment values can be missed with `var()` — use `var_os()`
- Tests that rely on env vars may race with parallel test execution

## Related Concepts

- [[concepts/cli-argument-parsing]]
- [[concepts/standard-streams]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-13-chapter-12-an-i-o-project-building-a-command-line-program]]
