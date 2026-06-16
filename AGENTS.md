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

## US-002 patterns — SPICE netlist tokenizer

- Tokenizer lives in `src/netlist.rs`; public API re-exported from `lib.rs`
  as `tokenize`, `NetlistToken`, `ParseWarning`.
- **SPICE title line rule**: line 1 of every netlist is the title and MUST be
  skipped unconditionally.  Failure to skip it is the most common source of
  spurious `ParseWarning`s.
- Line continuation (SPICE `+` prefix) is handled by `join_continuation_lines`
  before token dispatch; each logical line may span multiple raw lines.
- Unknown element types (e.g. `Q` for BJT) produce a `ParseWarning` (with
  `line`, `text`, `reason`) — not a hard `Err`.  This allows partial parses
  of real-world netlists that use unsupported elements.
- Controlled sources use different arities:
  - `E`/`G` (VCVS/VCCS): four node fields + one value (6 total after name)
  - `H`/`F` (CCVS/CCCS): two node fields + sensing voltage source name + value (5 total)
- `parse_line` dispatches on the **first character** of the element name (case
  insensitive); the rest of the token (after the type char) is the element
  name suffix stored in `.name`.
- `let-chains` (`if cond && let Some(x) = expr { ... }`) are a nightly
  feature available because this crate targets nightly.  Clippy prefers them
  over nested `if`/`if let` blocks (`collapsible_if`).

