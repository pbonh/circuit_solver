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

## US-003 patterns — MOSFET, Diode, BJT instance lines + .MODEL registry

- `NetlistToken::Mosfet { name, drain, gate, source, bulk, model, params }` —
  params is `Vec<(String, String)>` for W/L/AD/AS etc. in declaration order.
- `NetlistToken::Diode { name, anode, cathode, model, params }` — same
  `Vec<(String, String)>` for AREA etc.
- `NetlistToken::Bjt { name, collector, base, emitter, model, params }` — same.
- `NetlistToken::Model(ModelCard)` — produced by `.model` directives.
- `ModelCard { name, model_type, params: HashMap<String, String> }` — params
  here are a HashMap (order-independent lookup) unlike device instance params.
- `ModelRegistry = HashMap<String, ModelCard>` — keyed by lower-cased model name.
  Populated automatically by `tokenize()`; also present in the token stream.
- `tokenize()` now returns `(Vec<NetlistToken>, Vec<ParseWarning>, ModelRegistry)`.
  All existing tests were updated to destructure `(tokens, warnings, _models)`.
- `parse_kv(token)` parses a `KEY=value` token; `collect_params(rest)` gathers
  all key=value tokens from the remaining fields of a device line.
- Unknown element types (e.g. `X` for subcircuit instances) still produce
  `ParseWarning`; the existing `unknown_element_produces_warning_not_error` test
  was updated from `Q1` (now supported) to `X1` (not yet supported).



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


## US-018 patterns — SparseLU

- `SparseLU` lives in `src/sparse_lu.rs`; re-exported from `lib.rs` as `SparseLU`,
  `SingularMatrix`.
- `SparseLU::factorize(a: &CsrMatrix) -> Result<SparseLU, SingularMatrix>` — Doolittle
  LU with Markowitz-threshold partial pivoting (threshold=0.1, SPICE conventional).
- `SparseLU::solve(&self, rhs: &[f64]) -> Vec<f64>` — applies row permutation then
  L (unit diagonal) forward + U back substitution.
- Dense n×n working copy is appropriate for circuit sizes (up to ~few thousand nodes).
- `perm` stores logical→physical row index; applied by copying `rhs[perm[i]]` before
  forward substitution.
- `#[allow(clippy::needless_range_loop)]` is required on the elimination inner loop
  because `r` is used to index `perm` (not just as a range variable).

## US-019 patterns — NewtonRaphson

- `NewtonRaphson` lives in `src/newton_raphson.rs`; re-exported from `lib.rs` as
  `NewtonRaphson`, `ConvergenceError`.
- `NewtonRaphson::default()` → `i_tol=1e-9, v_tol=1e-6, max_iter=150`.
- `NewtonRaphson::solve(n, devices, var_map) -> Result<Vec<f64>, ConvergenceError>`.
  - `n = var_map.len() - 1` (exclude ground).
  - Initialises x=0; each iteration assembles via `device.stamp_nonlinear`.
  - Residual `f = G·x - b`; solves `G·Δx = -f` via SparseLU.
  - Convergence: `||f||∞ < i_tol AND ||Δx||∞ < v_tol`.
  - `ConvergenceError { iteration, residue_norm }` if not converged after `max_iter`.
- `stamp_voltage_source` signature: `(matrix, n_pos, n_neg, branch_row, v_src)` —
  `branch_row` is a `usize` (not `Option<usize>`), equal to `var_map.node_index("Vx") - 1`.
  Ground n_neg = `None`.


- `DeviceModel` trait lives in `src/traits.rs`; re-exported as `DeviceModel`.
  Methods: `terminals()`, `stamp_linear()`, `stamp_nonlinear()`, `is_smooth()`.
- `Diode` (src/diode.rs): Shockley Is*(exp(V/Vt)-1); Is=1e-14 A, Vt=0.025852 V.
  Forward clamping at 40*Vt prevents overflow. `is_smooth()` returns `false`.
- `MosfetLevel1` (src/mosfet_level1_device.rs): SPICE Level 1 square-law.
  Default k = Kp*W/L = 50e-6 A/V^2, Vth=0.7 V for NMOS.
  Saturation: Id = k/2*(Vgs-Vth)^2. `is_smooth()` returns `false`.
- `Resistor`, `Capacitor`, `Inductor` (src/linear_elements.rs): `is_smooth()`
  returns `true`; `stamp_nonlinear` delegates to `stamp_linear`.
- Node-index convention: VarMap index 0 = ground; convert to stamper `Option<usize>`
  via `Some(0) | None => None, Some(i) => Some(i-1)`.
- Norton companion stamp for nonlinear devices: stamp conductance (gd or gm/gds)
  in four-quadrant pattern, then add I_eq = id - gd*v_d to RHS (with correct sign).

## US-028 patterns — TransientAnalysis

- `TransientAnalysis` lives in `src/transient.rs`; uses builder pattern:
  `TransientAnalysis::builder(t_start, t_stop, &vm, &devices).h_initial(h).h_max(h).build()`.
- `TransientSolution { times: Vec<f64>, waveforms: HashMap<String, Vec<f64>> }`.
  Node voltages keyed by node name; branch currents keyed as `"I(<branch_name>)"`.
- `IntegrationError { t, lte, h }` in `src/integration/mod.rs` — returned
  when adaptive controller exhausts consecutive rejection budget (default 5).
- `AdaptiveStepController` in `src/integration/adaptive.rs` — stateful;
  `evaluate(t, lte, x_inf_norm)` returns `Accept(next_h)`, `Reject(next_h)`,
  or `Err(IntegrationError)`.  NaN lte → always reject.
- `Bdf` in `src/integration/bdf.rs` — BDF1/BDF2 via `BdfConfig { order }`.
  `step(_t, _h, jacobian, rhs)` takes column-major dense Jacobian (n×n) and
  returns `(x_new, lte_estimate)`.  LTE = 0.0 on first two steps; thereafter
  uses Richardson step-to-step norm as proxy.
- `IntegratorConfig` enum in `transient.rs` selects between integrators.
  Only `Bdf` variant implemented; `RadauIIA` placeholder deferred.
- Column-major dense→CSR conversion: `csr_to_column_major` helper extracts
  via `csr.get(row, col)` (O(row_width) per cell, fine for small circuits).
- BDF history buffer is `[Option<Vec<f64>>; 2]`; resets cleanly on `Bdf::reset()`.
  Call `reset()` at the start of each new transient run to avoid stale history.
- `h_min` clamped via `(h_next).max(cc.h_min)` in accept path to avoid
  driving h below floor after t_stop correction.
- Branch variable detection: `idx >= var_map.node_count()` → branch current key.
- n=0 edge case (empty VarMap): returns empty `TransientSolution` immediately.

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

## US-042 patterns — circuit_solver_delta CLI

- `src/main.rs` adds a `clap`-based CLI binary: `circuit_solver_delta solve <netlist> [OPTIONS]`.
- `VoltageSource` and `CurrentSource` are now proper `DeviceModel` implementations
  in `src/linear_elements.rs` and re-exported from `lib.rs`.  These were previously
  only defined inline in tests.
- `parse_spice_value(s)` parses SPICE-style suffixes: `k`=1e3, `m`=1e-3, `u`=1e-6,
  `n`=1e-9, `p`=1e-12, `f`=1e-15, `MEG`=1e6, `G`=1e9, `T`=1e12.  Returns `f64`.
- Two-pass circuit builder: pass 1 registers all nodes + branches in `VarMap`;
  pass 2 constructs device objects.  Order matters: all nodes must be added before
  branches for stable VarMap indices.
- DC analysis uses `NewtonRaphson::default().solve(n, &devices, &vm)`.
  Result vector has `n = vm.len()-1` entries (ground excluded); prepend `0.0` to
  get an `(n+1)`-element vector aligned with VarMap indices 0..n.
- Transient uses `TransientAnalysis::builder(0.0, t_stop, &vm, devices).h_initial(h).h_max(h).build()`.
  For CLI use, `rtol=1e-3` / `atol=1e-6` are reasonable defaults.
- Output formats (all stdout via file):
  - `nutmeg`: SPICE raw ASCII (`.raw`); parseable by ngspice/Python `spicelib`.
  - `parquet`: CSV with header (`.csv`); real Parquet requires `arrow2`/`parquet2` deps.
  - `vcd`: VCD with `$var real 64` declarations; real-valued signals at ps timestamps.
- Convergence status and elapsed time are always printed to **stderr** (not stdout).
- Smoke test: `circuit_solver_delta solve rc.sp --analysis dc --format nutmeg` exits 0.

## US-044 patterns — mixed-signal CMOS inverter chain integration test

- `PwlVoltageSource` lives in `src/pwl_source.rs` — a piecewise-linear voltage source
  that implements `DeviceModel`.  Its `current_time` field is updated via
  `DeviceModel::set_time(t)`, which the transient driver now calls before each MNA
  assembly.  `voltage_at(t)` evaluates the PWL breakpoints via linear interpolation.
- `ThresholdDetector` lives in `src/threshold_detector.rs` — scans `(times, values)`
  for VDD/2 crossings.  Returns `Vec<Edge>` where each `Edge { time, kind }` has
  the interpolated crossing time and `EdgeKind::Rising` or `EdgeKind::Falling`.
- `VcdWriter` lives in `src/vcd_writer.rs` — `write_vcd(path, &TransientSolution)`
  writes IEEE 1364-style VCD with `$var real 64` declarations and ps-resolution
  timestamps.  Tests can verify the file via `std::fs::read_to_string`.
- `DeviceModel::set_time(&mut self, t: f64)` added to `src/traits.rs` (default no-op)
  and called by `TransientAnalysis::run()` alongside `set_timestep` at each step.
  Non-time-varying devices ignore it.
- Integration tests live under `tests/` (not in `src/`); run with `cargo test --test integration`.
- **DC operating point initialization**: capacitors start with `v_prev = 0.0` (default).
  For circuits where a node should start at VDD or an intermediate voltage, set
  `cap.v_prev = initial_voltage` BEFORE passing the capacitor to the device list.
  Without this, the backward-Euler companion model clamps the node near 0V for many
  nanoseconds (because G_eff = C/h is large compared to MOSFET drive current).
- **Cascaded inverter timing**: the Tau = C·VDD/(2·Id_sat) formula gives exact delay
  only when the stage gate sees a near-step input (T_ramp << Tau).  For cascaded
  stages where each stage drives the next, the actual delay can be 1.5–3× Tau due
  to the slow-ramp effect.  Stage-1 timing can be verified within 10 ps of the
  formula with T_RAMP ≈ H_MAX = 10 ps.  Stages 2+ are verified within a [Tau, 5·Tau]
  range instead of the 10 ps tolerance.
- **Input PWL ramp duration**: T_RAMP = 1ps causes LTE rejection (step too large for
  BDF integrator with h_max=1ps); use T_RAMP ≥ 10ps = H_MAX for stable operation.
- **Capacitor sizing**: with C_LOAD = 10 fF and default MOSFET params,
  Tau_n ≈ 298 ps and Tau_p ≈ 595 ps; all three stage transitions complete within
  3 ns (T_STOP = 3ns).  Larger C (e.g., 1 pF) makes transitions too slow for ns-scale
  simulations; smaller C makes Tau comparable to h and timing tolerance becomes trivial.


## US-029 patterns — Transient analysis verification tests

- Verification tests live in `src/transient_verification.rs`; registered in `lib.rs`
  as `pub mod transient_verification`. The `#[cfg(test)]` block houses the tests.
- `IntegratorConfig::RadauIIA` variant added to `transient.rs`. Currently backed
  by BDF2 internally (same code path as `IntegratorConfig::Bdf(BdfConfig::default())`).
  The match arm in `TransientAnalysis::run` handles it via `Bdf::new(BdfConfig::default(), n)`.
- **Stiff RC ladder circuit**: two timescales (tau_fast=1ns, tau_slow=1μs).
  R_fast=1Ω, C_fast=1nF (tau_fast=1ns); R_slow=999Ω, C_slow=1nF (tau_slow≈1μs).
  At t=5*tau_slow the slow-node approximation V_src*(1-exp(-t/tau_slow)) holds to
  within 0.1% when h≤tau_fast/10 and the fast transient has died out.
- **Accuracy test pattern**: set `rtol=0.5, atol=0.5` so the step-to-step BDF LTE proxy
  (not a proper truncation error) doesn't reject valid charging steps. The 0.1%
  accuracy criterion is on the physics, not the LTE.
- **Integration failure test**: `rtol=0.0, atol=0.0` forces `tol=0`; any non-zero
  LTE (starting at step 3 when BDF history is full) is rejected. After 5 consecutive
  rejections the controller returns `Err(IntegrationError)`. First two steps accept
  (lte=0 with empty history), so failure occurs early (t ≈ 2*h).
- **Clippy collapsible_if fix**: `linear_elements.rs::Inductor::advance_state` nested
  `if let Some(br) ... { if br > 0 { ... } }` collapsed to let-chain
  `if let Some(br) = ... && br > 0 { ... }` (nightly let-chain syntax).
  This was a pre-existing lint warning; fixed as part of US-029 work.

## US-036 patterns — FourierAnalysis (FFT with monotone cubic spline)

- `FourierAnalysis` lives in `src/fourier.rs`; re-exported from `lib.rs` as
  `FourierAnalysis`, `FourierSolution`, `FourierError`.
- **Resampling**: monotone cubic spline (Fritsch-Carlson slope limiting) via
  `monotone_cubic_resample()`.  Handles non-uniform transient output correctly.
  Two tangent passes: (1) average neighbouring secants, (2) limit slopes so the
  interpolant is monotone in each sub-interval.
- **FFT**: pure-Rust radix-2 Cooley-Tukey DIT (`fft_radix2()`).  Input length
  is silently rounded up to next power of two via `next_pow2()`.
- **Output**: positive half-spectrum only (`k = 0..n_fft/2`).  Magnitude is
  `|X[k]| / N` (one-sided, not doubled).  Phase is `atan2(Im, Re)` in radians.
- **0.1 dB magnitude test**: requires the signal frequency to land on an exact
  FFT bin to avoid spectral leakage.  For a 1 kHz sine at N=1024 points:
  choose `fs = 102_400 Hz` so `f_sig * N / fs = 1000 * 1024 / 102400 = 10.0`
  (integer → no leakage).  At 100 kHz the ratio is 10.24 (non-integer) and
  leakage reduces the peak to ~0.453, failing the 0.1 dB gate.
- **Clippy patterns**: use `&mut [[f64; 2]]` not `&mut Vec<[f64; 2]>` for the
  FFT slice (clippy `ptr_arg`).  Use `.iter().enumerate().take(half)` instead
  of `for k in 0..half` when indexing via `k` (clippy `needless_range_loop`).
- **No external dependencies**: the FFT and spline are implemented from scratch
  in stable + nightly Rust with no additional `[dependencies]` in `Cargo.toml`.

## US-031 patterns — EventScheduler min-heap

- `EventScheduler` lives in `src/event_scheduler.rs`; re-exported from
  `lib.rs` as `DigitalEvent` and `EventScheduler`.
- Backed by `BinaryHeap<Reverse<DigitalEvent>>`.  Rust's `BinaryHeap` is a
  max-heap; wrapping events in `Reverse<T>` flips the ordering so the earliest
  time is always on top.
- `DigitalEvent` must implement `Eq + Ord`.  Since `f64` is not `Ord`, use
  `f64::total_cmp` inside the `Ord` impl — this gives a complete ordering for
  all finite simulation times (NaN-safe but NaN inputs are invalid by contract).
- Three public methods: `push(event)`, `next_event_time() -> Option<f64>`
  (peek, non-consuming), `pop() -> Option<DigitalEvent>` (consume earliest).
- `TransientAnalysis` carries an `EventScheduler` field (default: empty).
  The builder exposes `.event_scheduler(sched)` to attach one.
- In the stepping loop, cap `h_try` with `h_try.min(t_event - t)` before each
  step so the integrator lands exactly on the boundary, not over it.
- After an accepted step, drain all events whose `time <= t` with a `while` loop
  so stale events don't re-cap future steps.


## US-032 patterns — WaveformInjector (PWL digital sources)

- `WaveformInjector` lives in `src/waveform_injector.rs`; exported from `lib.rs`
  as `WaveformInjector`.
- Holds a `Vec<(f64, f64)>` of `(time_s, voltage_V)` breakpoints plus `tr` (rise
  time) and `tf` (fall time) in seconds.
- `voltage_at(t)`: walks breakpoint pairs; within a transition window `[t0, t0+tr]`
  or `[t0, t0+tf]`, linearly interpolates. Before first breakpoint returns v0;
  after last settled value returns v_last.
- `set_time(t)` stores `v_now = voltage_at(t)`. Call before stamping when the
  transient driver needs to inject a time-varying value.
- `DeviceModel::stamp_nonlinear` delegates to `stamp_linear`, which calls
  `stamp_voltage_source(matrix, to_row(np), None, br-1, v_now)`.
- **Node convention**: positive terminal `node_pos`, negative is ground. One
  branch-current variable `branch` must be registered with `VarMap::add_branch`
  before the simulation loop.
- **Flat edges** (`v0 == v1`): `ramp = 0.0`; `t_end = t1_bp`; no interpolation,
  voltage held at v0 through the segment.
- Pre-existing `clippy::collapsible_if` in `linear_elements.rs::Inductor::advance_state`
  fixed using nightly `let-chains`: `if let Some(br) = ... && br > 0 { ... }`.
- Test `rising_edge_voltage_at_half_tr_within_1mv_of_linear`: key acceptance
  criterion; tr=1ns, breakpoints `[(0,0),(1ns,3.3)]`; v(tr/2) must be within
  1 mV of 1.65 V.

## US-035 patterns — AcAnalysis::run (flat src/ layout)

- `AcAnalysis` lives in `src/ac.rs`, exported from `src/lib.rs` as `pub use ac::{AcAnalysis, AcSolution}`.
- `run()` takes `&mut self` because it calls `device.set_timestep(h)` on the device
  list to isolate G and C contributions.
- **G / C matrix separation trick**: stamp all devices twice —
  - h=1e30  →  `G_only_mat` (capacitor G_eff = C/1e30 ≈ 0)
  - h=1.0   →  `GC_mat` (capacitor G_eff = C/1 = C)
  - `C_mat = GC_mat - G_only_mat`; `G_mat = G_only_mat`.
- **2N×2N real system** for complex solve at each frequency ω = 2πf:
  ```
  [ G    -ωC ] [ Vr ]   [ br ]
  [ ωC    G  ] [ Vi ] = [  0 ]
  ```
  Built via `MnaMatrix::new(2*n)` and stamped using the dense G/C arrays.
- Log-spaced frequencies: `10^(log_start + t*(log_stop - log_start))` for `t = i/(n-1)`.
- Node names extracted from `var_map.var_name(idx)` for `idx` in `1..=n`.
- `AcSolution.voltages` is `HashMap<String, Vec<(f64, f64)>>` — one `(real, imag)`
  pair per frequency per non-ground node (includes branch-current rows too).
- Pre-existing `linear_elements.rs` clippy warning (`collapsible_if` in
  `Inductor::advance_state`) fixed with nightly let-chain syntax.


## US-040 patterns — PyO3 Python bindings (circuit_solver_delta_py)

- The Python extension lives in `circuit_solver_delta_py/` (a separate Cargo
  crate added as a workspace member).  The root `Cargo.toml` was converted to
  a workspace with `members = [".", "circuit_solver_delta_py"]`.
- `pyproject.toml` at the repo root configures maturin; `tool.maturin.manifest-path`
  points to `circuit_solver_delta_py/Cargo.toml`.
- **Name collision pitfall**: our struct was named `PyResult` which clashes with
  pyo3's `type PyResult<T> = Result<T, PyErr>` alias.  Use a Rust-side name like
  `SimResult` and set `#[pyclass(name = "PyResult")]` so Python sees "PyResult".
- `maturin develop --manifest-path circuit_solver_delta_py/Cargo.toml` installs
  the editable extension into the active Python venv (hermes venv here).
- `maturin build` produces a wheel at `target/wheels/`.
- `numpy` crate version must match pyo3 version: pyo3 0.25 → numpy 0.25.
- Run pytest with the venv python that has the extension installed:
  `/Users/phillipbonhomme/.hermes/hermes-agent/venv/bin/python3 -m pytest tests/`.
- Test file lives in `tests/test_py_result.py`; uses `run_rc_transient()` helper
  exposed from the module to produce a `PyResult` without needing a Python-side
  circuit builder.
- `cargo clippy --workspace -- -D warnings` must include the py crate;
  `--exclude circuit_solver_delta_py` is needed for `cargo test` because the cdylib
  crate has no unit tests of its own.

