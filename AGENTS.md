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

## US-006 patterns — MNA stamping for linear elements

- All stamping functions live in `src/stamper.rs`; public API re-exported from
  `lib.rs` as `stamp_resistor`, `stamp_capacitor`, `stamp_inductor`,
  `stamp_voltage_source`, `stamp_current_source`.
- **Ground-node convention**: callers pass `Option<usize>` for each node.
  `None` means ground; the stamper silently skips any stamp that would touch a
  ground row/column.  This avoids redundant `if node != 0` guards at call sites.
- **Backward-Euler companion models**:
  - Capacitor: `G_eff = C / h`; history current `I_hist = G_eff * V_prev`
    added to RHS (+into n_pos, -into n_neg).
  - Inductor: extra branch-current row at `branch_row`; `-L/h` on diagonal;
    `+(L/h)` coupling at (n_pos, branch_row) and (branch_row, n_pos);
    `-L/h * I_prev` on RHS at `branch_row`.
- **Voltage source / inductor share the same branch-current pattern**: KCL
  coupling ±1 at (node, branch) and (branch, node); only the diagonal and RHS
  differ (voltage source: no diagonal term, `V_src` on RHS; inductor: `-L/h`
  on diagonal, history voltage on RHS).
- **Controlled sources** (`stamp_vccs`, `stamp_vcvs`, `stamp_cccs`, `stamp_ccvs`)
  live in `src/controlled_sources.rs` and use 1-based node indices (0 = ground,
  non-ground nodes start at 1).  The linear-element stamper uses `Option<usize>`
  (0-based) instead — be consistent within each file.
- The 2-node resistor-divider test (`resistor_divider_mna_2node`) is the
  canonical integration test: V1 (5 V) + R1 (1 kΩ) + R2 (1 kΩ), 3×3 MNA,
  hand-computes the full G matrix and RHS before asserting.

## US-007 patterns — Controlled-source MNA stamping

- Controlled-source stamp functions live in `src/controlled_sources.rs`;
  exported from `lib.rs` as `stamp_vccs`, `stamp_vcvs`, `stamp_cccs`, `stamp_ccvs`.
- **Node convention**: nodes are 1-based (ground = 0 is skipped silently).
  Pass raw SPICE node indices; the stamp functions subtract 1 internally.
  This differs from `stamper.rs` which uses `Option<usize>` (0-based indices,
  `None` for ground).  Choose consistently per module.
- **VCCS** needs no extra row/col — stamps four cross-entries directly on the
  conductance sub-matrix: `±gm` at `(n±, nc±)`.
- **VCVS/CCVS** each introduce one branch-current unknown `j_row`; the MNA
  matrix must be pre-allocated to `n_nodes + n_branch_currents`.
- **CCCS** borrows the sensing voltage-source's existing branch-current column
  (`j_sense`) — no new row needed for the CCCS itself.
- Sedra-Smith example (g_m = 0.04 S, nodes 1→4): `G[0,2]=+0.04`,
  `G[0,3]=-0.04`, `G[1,2]=-0.04`, `G[1,3]=+0.04` — use as a regression anchor.
- Child task (t_daa2b394) also added `stamper.rs` for passive/linear elements
  and extended `netlist.rs` with `Mosfet`, `Diode`, `Bjt` tokens plus
  `ModelCard` / `ModelRegistry` (`.model` directive).  The `tokenize` function
  now returns a 3-tuple `(tokens, warnings, models)`.  All 38 tests pass.


## US-009 patterns — VarMap

- `VarMap` lives in `src/var_map.rs`; re-exported from `lib.rs` as `VarMap`.
- Ground node is always index 0, stored as name `"0"`, pre-seeded by `VarMap::new()`.
- `add_node(name)` assigns the next available node index; idempotent on repeat calls.
- `add_branch(name)` appends a branch-current variable after all nodes; idempotent.
- `node_index(&str) -> Option<usize>` and `var_name(usize) -> Option<&str>` are the
  public read API for MNA assemblers and result extractors.
- **Ordering invariant**: all node indices are contiguous at `0..node_count`, then
  branch variables at `node_count..len()`.  If `add_node` is called after
  `add_branch`, existing branch indices shift up by 1 (to preserve the invariant).
  For stable indices, always add all nodes before any branches.
- `len()` returns total variables (nodes + branches); `node_count()` returns only
  the node count (including ground).
- Internal storage: `HashMap<String, usize>` for name→index; `Vec<String>` for
  index→name (dense; `Vec::insert` used when shifting branches).
