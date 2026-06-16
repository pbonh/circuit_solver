# AGENTS.md — circuit_solver_delta

## Rust toolchain

Cargo is installed via Homebrew rustup, not the default `rustup-init` path.
The binary lives at `/opt/homebrew/Cellar/rustup/1.29.0_2/bin/cargo`.
Export `PATH="/opt/homebrew/Cellar/rustup/1.29.0_2/bin:$PATH"` at the start
of every terminal session (or add it to `.bashrc`/`.zshrc`).

Active toolchain: nightly (set via `RUSTUP_TOOLCHAIN` env var).

## Worktree setup

Worktrees for kanban tasks are created under `.worktrees/<task_id>/`.
Creation command from repo root:

```
git worktree add .worktrees/<task_id> -b <branch>
```

For this project the branch name is `ralph/circuit-solver-delta`.

## Crate structure

- `src/lib.rs`   — re-exports public surface from submodules
- `src/graph.rs` — `NodeId` newtype + `CircuitGraph` struct (US-001)

## US-001 patterns

- `NodeId` is a newtype `NodeId(usize)` with `From<usize>` and `From<NodeId>
  for usize` impls (not `Into` — the stdlib blanket handles that).
- `CircuitGraph::new()` seeds the ground node, setting `node_count = 1`.
- `CircuitGraph::default()` is derived and leaves `node_count = 0`; callers
  that want a properly-seeded graph must use `::new()`.
- Ground node index is always 0; exposed via `CircuitGraph::ground() -> NodeId`.

## US-004 patterns — VamsParser

- `src/vams_parser.rs` is a hand-rolled lexer + recursive-descent parser.
  No external parser-combinator crates are used; all deps remain zero.
- The lexer runs in a single pass at `Parser::new()`, collecting all tokens
  into a `Vec<Token>`.  This trades a small up-front allocation for a much
  simpler borrow-checker story (no lifetime dance between Lexer and Parser).
- `expect()` uses `std::mem::discriminant` for structural token matching,
  which avoids having to match on contained values when checking for brackets,
  operators, and keywords.  Only call it for tokens with no payload you care
  about; for idents use `expect_ident()`.
- Operator precedence: `parse_add_sub` → `parse_mul_div` → `parse_unary` →
  `parse_primary`.  Standard Pratt/precedence-climbing pattern, two levels.
- AMS operator keywords (`idt`, `ddt`, `transition`, `slew`) are dispatched
  in `parse_primary`; they each consume `( arg, arg, … )` inline.
- If port declarations (`input`/`output`/`inout`) are absent but a port-name
  list is present in the module header, the parser fabricates `Inout`/`None`
  entries so `module.ports` is always populated.
- Public re-export in `lib.rs`: `pub use vams_parser::parse_module;`

## Gotchas

- Do not implement `Into` manually — Rust provides a blanket `impl<T, U: From<T>>
  Into<T> for U`. Implementing both causes a compile error.
- `#[derive(Default)]` on `CircuitGraph` is intentional: it lets the struct be
  used in aggregate `Default` derives for parent structs, but consumers should
  use `::new()` for correct initial state.

## US-005 patterns — MnaMatrix / CsrMatrix

- `MnaMatrix` is a COO accumulator: `stamp(row, col, val)` pushes a raw
  triplet; duplicate `(row, col)` entries are **summed** during `to_csr()`.
  This is intentional — elements stamp independently without needing to look
  up existing values.
- `to_csr()` uses a dense `n × n` scratch buffer to accumulate duplicates
  before compressing.  Correct and simple for small-to-medium circuits;
  a future optimisation could use sorted-triplet merging for large `n`.
- `reset()` calls `Vec::clear()` (not `Vec::new()`) to preserve capacity —
  no heap reallocation on subsequent stamps.
- Module doc comments use `//!` (inner doc), not `///` (outer doc), because
  clippy `empty_line_after_doc_comments` fires when an outer `///` block is
  separated from the next item by a blank line at the top of a file.
- `CsrMatrix::get()` is a linear scan within the row; sufficient for tests
  and small circuits.  For solver use, pass `row_ptr`/`col_idx`/`values`
  directly to LAPACK-style routines.
- Public re-exports live in `lib.rs`: `pub use mna_matrix::{CsrMatrix, MnaMatrix};`
