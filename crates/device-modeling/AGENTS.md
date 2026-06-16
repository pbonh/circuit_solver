# device-modeling — AGENTS.md

Patterns and gotchas for AI coding assistants working in `crates/device-modeling`.

## Two dispatch styles coexist

This crate has **two** `DeviceModel` concepts:

- `device_modeling::model::DeviceModel` — the **closed enum** (ADR-0005).
  Zero-cost, monomorphized, hot-path dispatch via `match`.  Adding a variant
  is a deliberate breaking change; every `match` arm must be updated.
- `device_modeling::traits::DeviceModel` — the **open trait** (US-011).
  `dyn`-safe, used when the NR engine holds a `Vec<Box<dyn DeviceModel>>`.
  Implement this trait on new device types that don't belong in the closed
  enum.

Do **not** re-export both at the crate root under the same name.  The closed
enum is exported as `pub use model::{DeviceFamily, DeviceModel}` from
`lib.rs`; the trait lives under `device_modeling::traits::DeviceModel` (not
the crate root) to avoid the name collision.

## MnaMatrix / VarMap — the stamping interface

`mna_matrix::MnaMatrix<'_>` is a **borrow** of two external `Vec<f64>`
slices (the matrix `a` and the RHS `b`), plus the stride `dim`.  Stamp
methods call:

- `matrix.add_element(row, col, value)` — accumulate into `a[row*dim+col]`.
- `matrix.add_rhs(row, value)` — accumulate into `b[row]`.

`var_map::VarMap` maps `NodeId` → `usize` and `BranchId` → `usize`.  Build
it with `VarMap::from_nodes(&[NodeId; N])` + optional `.with_branches(...)`.

Both types live in `device-modeling` (not `circuit-solver-types`) to avoid a
`numeric-solver ↔ device-modeling` circular dependency.

## Trait-object safety rules

`traits::DeviceModel` is `dyn`-safe.  Keep it that way:

- No generic method parameters (`fn foo<T>(&self, …)` breaks object safety).
- No `Sized` constraint on the trait.
- No associated types.
- All methods take `&self` or `&mut self`.

## `#[deny(missing_docs)]` is active

All `pub` items must have doc comments.  The crate-level lint is
`#![deny(missing_docs)]`; a missing doc on any new public struct, trait,
method, or constant is a compile error.

## Linear vs. nonlinear stamp split

- `stamp_linear` — called **once** per analysis (or once per timestep for
  companion models).  For purely nonlinear devices, this can be a no-op.
- `stamp_nonlinear` — called **every NR iteration**.  For purely linear
  devices (resistors, fixed current sources), this is a no-op.
  For nonlinear devices: read terminal voltages from `x` via `var_map`,
  evaluate the tangent conductance `g = dI/dV`, call `add_element` for
  Jacobian entries, call `add_rhs` for the companion current
  `I_eq = I(V) - g * V`.

## Tests

Run `cargo test -p device-modeling` to exercise all 171+ tests.  All tests
are `#[cfg(test)]` inline unit tests; no integration tests in this crate.
New stamp methods must include:
1. A test that verifies KCL closure (diagonal and off-diagonal sums).
2. A test that the Jacobian matches a numerical finite-difference for at
   least one operating point.

## BjtEbersMoll — traits::DeviceModel concrete implementor (US-015)

`bjt_ebers_moll::BjtEbersMoll` is the first concrete implementor of the open
`traits::DeviceModel` trait for a nonlinear device.  It bridges
`stamp::linearize_bjt` (tasks.md #10) to the `dyn`-dispatch path.

Key conventions to follow for future nonlinear trait implementors:

- `stamp_linear` is a **no-op** for purely nonlinear devices.
- `stamp_nonlinear` reads terminal voltages from `x[var_map.node_index(t)]`,
  calls the matching `linearize_*` helper, then:
  - accumulates `lin.jacobian[i][j]` into `matrix.add_element(row_i, row_j, …)`.
  - subtracts `lin.companion_current[i]` via `matrix.add_rhs(row_i, -…)`.
- Use `rows.iter().enumerate()` (not `for i in 0..N`) to avoid the
  `clippy::needless_range_loop` lint that fires on `rows[i]` patterns.
- `is_smooth()` returns `true` for exponential-family (smooth) equations;
  `false` for piecewise-linear / switch-level models.

## Linear elements as DeviceModel trait implementors (US-012)

`linear_elements.rs` provides five concrete `traits::DeviceModel` structs:
`Resistor`, `Capacitor`, `Inductor`, `VoltageSource`, `CurrentSource`.

Key conventions:

- **stamp_nonlinear delegates to stamp_linear** for all five — linear devices
  have no operating-point-dependent contribution.
- **Capacitor** uses `G_eq = C / timestep_s` (default `timestep_s = 1.0`) so
  it produces a non-trivial stamp.  For real transient accuracy use
  `CapacitorCompanion` instead.
- **Inductor and VoltageSource** require a `BranchId` field.  Callers must
  register the branch in `VarMap` via `VarMap::from_nodes(...).with_branches(&[bid])`.
  The branch row is at `var_map.branch_index(bid)` which equals `node_count + branch_idx`.
- **CurrentSource** stamps RHS only (`add_rhs`) — the A matrix is untouched.
- All five are re-exported at the crate root (`pub use linear_elements::{...}`).

