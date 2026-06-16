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
