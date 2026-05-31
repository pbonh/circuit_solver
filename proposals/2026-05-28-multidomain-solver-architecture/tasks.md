---
change-id: 2026-05-28-multidomain-solver-architecture
created: 2026-05-30
---

# Tasks

Ordered, dependency-aware checklist generated from `design.md` and the ADRs.
Every task is traceable to the scenario it satisfies via a `traces-spec`
comment, and grouped by ADR where applicable via a `traces-adr` comment.

Each task also carries the conflict-prevention ownership markers the execution
layer (`scientia-hermes-emit`) reads to compute file-collision waves and
shared-contract ratification (`hermes.conflict_prevention: true`):
`component` (the C4 component from `design.md`'s Component Map), `touches` (the
files it modifies, constrained to that component's owned globs), and
`produces-contract` / `uses-contract` for the Shared Contracts it defines or
consumes.

_Note: tasks 1-4 are workspace-level infrastructure (ADR-0008). Their `touches`
are Cargo manifest files, which sit outside the source-component globs by design;
`component: workspace` marks them as cross-component infrastructure.
All subsequent tasks presuppose the workspace scaffold (#1-#4) is complete._

### Cargo workspace scaffold (ADR-0008)

<!-- component: workspace -->
<!-- touches: Cargo.toml -->
- [ ] **1.** Create the workspace-root `Cargo.toml` declaring all 7 crates as `[workspace] members`. <!-- traces-spec: workspace#each-bounded-context-container-is-workspace-member-crate --> <!-- traces-adr: ADR-0008 -->

<!-- component: workspace -->
<!-- touches: crates/frontend/Cargo.toml, crates/netlist/Cargo.toml, crates/orchestration/Cargo.toml, crates/numeric/Cargo.toml, crates/devices/Cargo.toml, crates/digital/Cargo.toml -->
- [ ] **2.** Scaffold each domain crate skeleton (`Cargo.toml` + `src/lib.rs`) under `crates/<name>/`; verify `cargo build --workspace` compiles clean. (depends on #1) <!-- traces-spec: workspace#workspace-builds-all-crates-from-root, workspace#unrelated-crate-not-recompiled-when-peer-changes --> <!-- traces-adr: ADR-0008 -->

<!-- component: frontend -->
<!-- touches: Cargo.toml -->
- [ ] **3.** Wire the PyO3 binding crate (`Cargo.toml` at workspace root) to list only `circuit-solver-frontend` as a direct `[dependency]`; confirm no other domain crate appears as a direct dep. (depends on #2) <!-- traces-spec: frontend-contract#pyo3-binding-crate-declares-only-frontend-as-direct-dep --> <!-- traces-adr: ADR-0008 -->

<!-- component: workspace -->
<!-- touches: crates/frontend/Cargo.toml, crates/orchestration/Cargo.toml, crates/numeric/Cargo.toml -->
- [ ] **4.** Wire all remaining inter-crate Cargo dep edges (frontend→netlist+orchestration; orchestration→netlist+numeric+digital; numeric→devices+netlist); add a compile-fail test confirming an undeclared peer access is rejected by the compiler. (depends on #2) <!-- traces-spec: workspace#inter-crate-access-requires-explicit-cargo-dependency, analog-engine#orchestration-crate-declares-explicit-cargo-dep-on-numeric, device-modeling#numeric-crate-declares-explicit-cargo-dep-on-devices, digital-engine#orchestration-crate-declares-explicit-cargo-dep-on-digital --> <!-- traces-adr: ADR-0008 -->

### Netlist graph & frontend foundations

<!-- component: netlist -->
<!-- touches: crates/netlist/src/graph.rs -->
<!-- produces-contract: netlist.CircuitGraph -->
- [ ] **5.** Immutable `CircuitGraph` with a Rust-backed builder API exposed to Python. (depends on #2) <!-- traces-spec: frontend-contract#circuit-graph-immutable-from-python --> <!-- traces-adr: ADR-0001 -->

<!-- component: netlist -->
<!-- touches: crates/netlist/src/flatten.rs -->
<!-- uses-contract: netlist.CircuitGraph -->
<!-- produces-contract: netlist.FlattenedView -->
- [ ] **6.** Two-pass graph flattening producing per-analysis sub-views (full matrix incl. ground built once). (depends on #5) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden --> <!-- traces-adr: ADR-0003 -->

### Numeric solver (pure-Rust sparse-direct)

<!-- component: numeric -->
<!-- touches: crates/numeric/src/mna.rs -->
<!-- uses-contract: netlist.FlattenedView -->
<!-- produces-contract: numeric.StampInterface -->
- [ ] **7.** MNA assembly via branch stamping over the flattened graph. (depends on #6) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden --> <!-- traces-adr: ADR-0002 -->

<!-- component: numeric -->
<!-- touches: crates/numeric/src/lu_real.rs -->
- [ ] **8.** russell real-valued sparse LU for DC/transient solves. (depends on #7) <!-- traces-spec: analog-engine#transient-integration-matches-golden --> <!-- traces-adr: ADR-0002 -->

<!-- component: numeric -->
<!-- touches: crates/numeric/src/lu_complex.rs -->
- [ ] **9.** faer complex-valued sparse LU for AC small-signal (G + jwC). (depends on #7) <!-- traces-spec: analog-engine#ac-uses-pure-rust-complex-backend --> <!-- traces-adr: ADR-0002 -->

<!-- component: numeric -->
<!-- touches: crates/numeric/src/newton.rs -->
<!-- uses-contract: devices.DeviceModel -->
- [ ] **10.** Newton-Raphson loop with gmin-stepping then source-stepping continuation and a structured non-convergence error. (depends on #7, #11) <!-- traces-spec: analog-engine#non-convergence-guarded --> <!-- traces-adr: ADR-0002 -->

### Device model engine (closed enum + codegen seam)

<!-- component: devices -->
<!-- touches: crates/devices/src/model.rs, crates/devices/src/stamp.rs -->
<!-- uses-contract: numeric.StampInterface -->
<!-- produces-contract: devices.DeviceModel -->
- [ ] **11.** Closed `enum DeviceModel` with per-variant stamp evaluator; diode and BJT variants matching the reference models. (depends on #7) <!-- traces-spec: device-modeling#diode-bjt-stamps-match-reference --> <!-- traces-adr: ADR-0005 -->

<!-- component: devices -->
<!-- touches: crates/devices/src/mosfet.rs -->
<!-- uses-contract: devices.DeviceModel -->
- [ ] **12.** Add a MOSFET model as an in-tree enum variant (monomorphized dispatch). (depends on #11) <!-- traces-spec: device-modeling#mosfet-in-tree-enum-variant --> <!-- traces-adr: ADR-0005 -->

<!-- component: devices -->
<!-- touches: crates/devices/tests/no_runtime_registration.rs -->
<!-- uses-contract: devices.DeviceModel -->
- [ ] **13.** Confirm no runtime model-registration API exists (compile-time-only extensibility). <!-- traces-spec: device-modeling#runtime-registration-rejected --> <!-- traces-adr: ADR-0005 -->

<!-- component: devices -->
<!-- touches: crates/devices/src/codegen.rs -->
<!-- uses-contract: devices.DeviceModel -->
- [ ] **14.** In-tree macro/codegen seam generating model-family variants into the closed enum. (depends on #11) <!-- traces-spec: device-modeling#model-family-via-codegen-seam --> <!-- traces-adr: ADR-0007 -->

### Native event-driven digital kernel (ADR-0006)

<!-- component: digital -->
<!-- touches: crates/digital/src/event_queue.rs -->
<!-- produces-contract: digital.DigitalKernel -->
- [ ] **15.** Event queue with an in-process `run-until` API. (depends on #2) <!-- traces-spec: digital-engine#native-kernel-event-queue --> <!-- traces-adr: ADR-0006 -->

<!-- component: digital -->
<!-- touches: crates/digital/src/settle.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **16.** Delta-cycle combinational settling with oscillation detection (report, never hang). (depends on #15) <!-- traces-spec: digital-engine#zero-delay-combinational-settling --> <!-- traces-adr: ADR-0006 -->

<!-- component: digital -->
<!-- touches: crates/digital/src/checkpoint.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **17.** Checkpoint/restore of the event queue and net state for rollback. (depends on #15) <!-- traces-spec: digital-engine#native-kernel-optimistic-rollback --> <!-- traces-adr: ADR-0006 -->

### Analysis orchestration & mixed-signal

<!-- component: orch -->
<!-- touches: crates/orchestration/src/transient.rs -->
- [ ] **18.** Transient analysis driver using A-stable backward-Euler / trapezoidal integration. (depends on #8, #10) <!-- traces-spec: analog-engine#transient-integration-matches-golden --> <!-- traces-adr: ADR-0002 -->

<!-- component: orch -->
<!-- touches: crates/orchestration/src/ac_noise.rs -->
- [ ] **19.** AC small-signal and noise analysis drivers. (depends on #9) <!-- traces-spec: analog-engine#ac-uses-pure-rust-complex-backend --> <!-- traces-adr: ADR-0002 -->

<!-- component: orch -->
<!-- touches: crates/orchestration/src/scheduler.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **20.** Mixed-Signal Scheduler: optimistic time advance + checkpoint/rollback across the analog/digital boundary. (depends on #17, #18) <!-- traces-spec: mixed-signal-cosim#digital-driven-analog-load-rollback --> <!-- traces-adr: ADR-0006 -->

<!-- component: orch -->
<!-- touches: crates/orchestration/src/run_until.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **21.** Scheduler drives the native digital kernel via in-process run-until (no IPC); interface constrained to the `digital` crate's public API only. (depends on #15, #20) <!-- traces-spec: mixed-signal-cosim#scheduler-drives-native-kernel, mixed-signal-cosim#scheduler-accesses-native-kernel-only-via-digital-crate-public-api --> <!-- traces-adr: ADR-0006 -->

<!-- component: orch -->
<!-- touches: crates/orchestration/tests/comparator_dff.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **22.** Mixed-signal corpus: comparator + D flip-flop testbench. (depends on #20) <!-- traces-spec: mixed-signal-cosim#comparator-plus-dff -->

<!-- component: orch -->
<!-- touches: crates/orchestration/tests/level_shifter.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **23.** Mixed-signal corpus: level shifter across supply domains. (depends on #20) <!-- traces-spec: mixed-signal-cosim#level-shifter -->

### Digital correctness (event-trace equivalence)

<!-- component: digital -->
<!-- touches: crates/digital/src/equivalence.rs -->
- [ ] **24.** Event model + event-trace-equivalence checker over ordered (time, net, value) tuples within tolerance. (depends on #2) <!-- traces-spec: digital-equivalence#ordered-events-not-vcd -->

<!-- component: digital -->
<!-- touches: crates/digital/src/vcd.rs -->
- [ ] **25.** VCD parser into the event model (interchange only; no acceptance depends on VCD bytes). (depends on #24) <!-- traces-spec: digital-equivalence#vcd-interchange-only -->

### Python frontend contract (PyO3)

<!-- component: frontend -->
<!-- touches: crates/frontend/src/pymodule.rs -->
<!-- uses-contract: netlist.CircuitGraph -->
- [ ] **26.** PyO3 module exposing analysis results as zero-copy NumPy views over Rust-owned memory. (depends on #5) <!-- traces-spec: frontend-contract#results-zero-copy-numpy --> <!-- traces-adr: ADR-0001 -->

<!-- component: frontend -->
<!-- touches: crates/frontend/src/gil.rs -->
- [ ] **27.** Release the GIL during long solves; verify a concurrent Python thread progresses. (depends on #18, #26) <!-- traces-spec: frontend-contract#gil-released-during-solve --> <!-- traces-adr: ADR-0001 -->

### Validation harnesses (golden references)

<!-- component: orch -->
<!-- touches: crates/orchestration/tests/golden_ngspice.rs -->
- [ ] **28.** ngspice golden-reference harness on sky130 for DC/AC/transient/noise within tolerance. (depends on #18, #19) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden -->

<!-- component: digital -->
<!-- touches: crates/digital/tests/golden_icarus.rs -->
- [ ] **29.** Icarus Verilog golden-trace harness wired to the event-trace-equivalence checker. (depends on #24, #25) <!-- traces-spec: digital-equivalence#ordered-events-not-vcd -->
