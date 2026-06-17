# numeric-solver — agent notes

## MNA formulation verification tests (US-010)

### Pattern: place tests in a dedicated module under `src/`
Test modules that span assemble + flatten + sub_view + topology live in
`src/mna_verification.rs` (gated by `#[cfg(test)]`) and included from `lib.rs`
via:
```rust
#[cfg(test)]
mod mna_verification;
```
This keeps the lib test namespace clean while still satisfying `cargo test --lib`.

### Pattern: dense LU via faer `SpSolver`
To solve a ground-suppressed sub-view with `faer` dense partial-pivot LU:
```rust
use faer::prelude::SpSolver;  // provides .solve() on PartialPivLu
use faer::Mat;

let a_mat = Mat::<f64>::from_fn(dim, dim, |r, c| sv.matrix_entry(r as u32, c as u32).unwrap_or(0.0));
let rhs_mat = Mat::<f64>::from_fn(dim, 1, |r, _| sv.rhs_entry(r as u32).unwrap_or(0.0));
let sol = a_mat.partial_piv_lu().solve(&rhs_mat);
let v_node = sol.read(node_idx, 0);
```
Note: `faer::linalg::solvers::Solver` is NOT the right import — `faer::prelude::SpSolver`
provides `.solve()` on dense decompositions like `PartialPivLu<f64>`.

### Gotcha: `SubViewBuilder` defaults to suppress_ground = false
Always call `.suppress_ground(true)` before `.build()` when constructing a
ground-suppressed system for solving. Omitting it leaves the ground row as a
redundant KCL equation — the matrix will be singular and the LU will fail or
produce wrong results.

### Gotcha: node index ordering in `CircuitBuilder`
`CircuitBuilder` assigns `NodeId`s in encounter order: gnd = 0, then each new
net name in the order `add_element` first encounters it. For a divider wired
as `V1(n1→gnd), R1(n1→n2), R2(n2→gnd)`, the indices are:
  - `0` = gnd
  - `1` = n1
  - `2` = n2
Branch-current variables follow at `node_count` onward.

### Gotcha: `circuit-solver-py` linker failure is pre-existing
Use `--exclude circuit-solver-py` for workspace-wide checks.

## SparseLU with Markowitz pivot selection (US-018)

### Pattern: SparseLU lives in `src/sparse_lu.rs`, re-exported from `lib.rs`
`CsrMatrix`, `SparseLU`, and `SingularMatrix` are defined in `crates/numeric-solver/src/sparse_lu.rs`
and re-exported via `pub use sparse_lu::{CsrMatrix, SingularMatrix, SparseLU};` in `lib.rs`.
New public types also require `pub mod sparse_lu;` in `lib.rs`.

### Pattern: Markowitz cost is (row_nnz - 1) * (col_nnz - 1) in the active submatrix
Only count entries in the `step..n` submatrix (the "active" part), not the whole matrix.
This counts entries that will participate in future elimination steps.

### Pattern: threshold partial-pivot = reject candidates with |a_rk| < threshold * col_max
Default threshold is 0.1 (SPICE-conventional). Only rows passing this test are eligible
as pivot candidates; Markowitz cost breaks ties within that eligible set. Secondary
tiebreaker: largest absolute value (most numerically stable).

### Pattern: dense working copy for small circuits
`SparseLU` uses a dense n×n working copy internally. This is acceptable for circuit-
matrix sizes (up to a few thousand nodes). For production use, `RussellRealSolver`
(UMFPACK) is preferred. The dense approach avoids tracking dynamic sparsity patterns
during elimination, which is complex to implement correctly.

### Gotcha: pre-existing `clippy::let_and_return` warning in `adaptive.rs`
`cargo clippy -p numeric-solver -- -D warnings` fails with a pre-existing warning in
`crates/numeric-solver/src/integration/adaptive.rs:652`. This existed before US-018.
`cargo check -p numeric-solver` passes with 0 warnings (no new issues introduced).
This was fixed in US-020 by inlining the `raw` binding directly.

## Gmin insertion diagonal shunting (US-020)

### Pattern: GminInserter is a pure configuration value
`GminInserter` holds `gmin_siemens: f64` (default 1e-12). Its `apply` method takes
`(&MnaSystem, &FlattenedStructure, &CircuitGraph)` and returns a new `MnaSystem` —
the original is **not modified**. This makes it safe to call multiple times per NR
iteration without accumulating shunt.

### Pattern: shunt target is ElementKind::Semiconductor only
Only elements whose `ElementKind` is `Semiconductor` get the diagonal shunt. Linear
passives already contribute finite diagonal conductance; only nonlinear devices produce
near-zero diagonal entries that cause singular Jacobians.

### Pattern: MnaSystem needs `clone_with_matrix` helper
`GminInserter::apply` needs to produce a modified copy of the MnaSystem. Add a
`pub(crate) fn clone_with_matrix(&self, a: Vec<f64>, b: Vec<f64>) -> Self` method to
`assemble.rs` — debug_assert enforces `a.len() == dim*dim` and `b.len() == dim`.

### Gotcha: usize→u32 casts trigger clippy::cast_possible_truncation
Use `u32::try_from(x).expect("...")` or `u32::try_from(x).unwrap_or(u32::MAX)` instead
of `x as u32`. The assembler uses `u32` ids (NodeId, ElementId), so conversion is
needed when iterating over usize-indexed slices.

### Gotcha: `# Panics` section required for functions that call `.expect()`
Clippy pedantic (`missing_panics_doc`) requires a `# Panics` doc section for any
`pub fn` that contains `.expect()`, `.unwrap()`, or `debug_assert!`. Add the section
even for `expect` calls protected by a prior validation (clippy doesn't see the guard).
