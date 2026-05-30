---
change-id: 2026-05-28-multidomain-solver-architecture
created: 2026-05-28
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

### Netlist graph & frontend foundations

<!-- component: netlist -->
<!-- touches: project/src/netlist/graph.rs -->
<!-- produces-contract: netlist.CircuitGraph -->
- [ ] **1.** Immutable `CircuitGraph` with a Rust-backed builder API exposed to Python. <!-- traces-spec: frontend-contract#circuit-graph-immutable --> <!-- traces-adr: ADR-0001 -->

<!-- component: netlist -->
<!-- touches: project/src/netlist/flatten.rs -->
<!-- uses-contract: netlist.CircuitGraph -->
<!-- produces-contract: netlist.FlattenedView -->
- [ ] **2.** Two-pass graph flattening producing per-analysis sub-views (full matrix incl. ground built once). (depends on #1) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden --> <!-- traces-adr: ADR-0003 -->

### Numeric solver (pure-Rust sparse-direct)

<!-- component: numeric -->
<!-- touches: project/src/numeric/mna.rs -->
<!-- uses-contract: netlist.FlattenedView -->
<!-- produces-contract: numeric.StampInterface -->
- [ ] **3.** MNA assembly via branch stamping over the flattened graph. (depends on #2) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden --> <!-- traces-adr: ADR-0002 -->

<!-- component: numeric -->
<!-- touches: project/src/numeric/lu_real.rs -->
- [ ] **4.** russell real-valued sparse LU for DC/transient solves. (depends on #3) <!-- traces-spec: analog-engine#transient-integration-matches-golden --> <!-- traces-adr: ADR-0002 -->

<!-- component: numeric -->
<!-- touches: project/src/numeric/lu_complex.rs -->
- [ ] **5.** faer complex-valued sparse LU for AC small-signal (G + jwC). (depends on #3) <!-- traces-spec: analog-engine#ac-uses-pure-rust-complex-backend --> <!-- traces-adr: ADR-0002 -->

<!-- component: numeric -->
<!-- touches: project/src/numeric/newton.rs -->
<!-- uses-contract: devices.DeviceModel -->
- [ ] **6.** Newton-Raphson loop with gmin-stepping then source-stepping continuation and a structured non-convergence error. (depends on #3, #7) <!-- traces-spec: analog-engine#non-convergence-guarded --> <!-- traces-adr: ADR-0002 -->

### Device model engine (closed enum + codegen seam)

<!-- component: devices -->
<!-- touches: project/src/devices/model.rs, project/src/devices/stamp.rs -->
<!-- uses-contract: numeric.StampInterface -->
<!-- produces-contract: devices.DeviceModel -->
- [ ] **7.** Closed `enum DeviceModel` with per-variant stamp evaluator; diode and BJT variants matching the reference models. (depends on #3) <!-- traces-spec: device-modeling#diode-bjt-stamps-match-reference --> <!-- traces-adr: ADR-0005 -->

<!-- component: devices -->
<!-- touches: project/src/devices/mosfet.rs -->
<!-- uses-contract: devices.DeviceModel -->
- [ ] **8.** Add a MOSFET model as an in-tree enum variant (monomorphized dispatch). (depends on #7) <!-- traces-spec: device-modeling#mosfet-in-tree-enum-variant --> <!-- traces-adr: ADR-0005 -->

<!-- component: devices -->
<!-- touches: project/tests/devices/no_runtime_registration.rs -->
<!-- uses-contract: devices.DeviceModel -->
- [ ] **9.** Ensure no runtime model-registration API exists (compile-time-only extensibility). <!-- traces-spec: device-modeling#runtime-registration-rejected --> <!-- traces-adr: ADR-0005 -->

<!-- component: devices -->
<!-- touches: project/src/devices/codegen.rs -->
<!-- uses-contract: devices.DeviceModel -->
- [ ] **10.** In-tree macro/codegen seam generating model-family variants into the closed enum. (depends on #7) <!-- traces-spec: device-modeling#model-family-via-codegen-seam --> <!-- traces-adr: ADR-0007 -->

### Native event-driven digital kernel (supersedes ADR-0004)

<!-- component: digital -->
<!-- touches: project/src/digital/event_queue.rs -->
<!-- produces-contract: digital.DigitalKernel -->
- [ ] **11.** Event queue with an in-process `run-until` API. <!-- traces-spec: digital-engine#native-kernel-event-queue --> <!-- traces-adr: ADR-0006 -->

<!-- component: digital -->
<!-- touches: project/src/digital/settle.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **12.** Delta-cycle combinational settling with oscillation detection (report, never hang). (depends on #11) <!-- traces-spec: digital-engine#zero-delay-combinational-settling --> <!-- traces-adr: ADR-0006 -->

<!-- component: digital -->
<!-- touches: project/src/digital/checkpoint.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **13.** Checkpoint/restore of the event queue and net state for rollback. (depends on #11) <!-- traces-spec: digital-engine#native-kernel-optimistic-rollback --> <!-- traces-adr: ADR-0006 -->

### Analysis orchestration & mixed-signal

<!-- component: orch -->
<!-- touches: project/src/orchestration/transient.rs -->
- [ ] **14.** Transient analysis driver using A-stable backward-Euler / trapezoidal integration. (depends on #4, #6) <!-- traces-spec: analog-engine#transient-integration-matches-golden --> <!-- traces-adr: ADR-0002 -->

<!-- component: orch -->
<!-- touches: project/src/orchestration/ac_noise.rs -->
- [ ] **15.** AC small-signal and noise analysis drivers. (depends on #5) <!-- traces-spec: analog-engine#ac-uses-pure-rust-complex-backend --> <!-- traces-adr: ADR-0002 -->

<!-- component: orch -->
<!-- touches: project/src/orchestration/scheduler.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **16.** Mixed-Signal Scheduler: optimistic time advance + checkpoint/rollback across the analog/digital boundary. (depends on #13, #14) <!-- traces-spec: mixed-signal-cosim#digital-driven-analog-load-rollback --> <!-- traces-adr: ADR-0006 -->

<!-- component: orch -->
<!-- touches: project/src/orchestration/run_until.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **17.** Scheduler drives the native digital kernel via in-process run-until (no IPC). (depends on #11, #16) <!-- traces-spec: mixed-signal-cosim#scheduler-drives-native-kernel --> <!-- traces-adr: ADR-0006 -->

<!-- component: orch -->
<!-- touches: project/tests/orchestration/comparator_dff.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **18.** Mixed-signal corpus: comparator + D flip-flop testbench. (depends on #16) <!-- traces-spec: mixed-signal-cosim#comparator-plus-dff -->

<!-- component: orch -->
<!-- touches: project/tests/orchestration/level_shifter.rs -->
<!-- uses-contract: digital.DigitalKernel -->
- [ ] **19.** Mixed-signal corpus: level shifter across supply domains. (depends on #16) <!-- traces-spec: mixed-signal-cosim#level-shifter -->

### Digital correctness (event-trace equivalence)

<!-- component: digital -->
<!-- touches: project/src/digital/equivalence.rs -->
- [ ] **20.** Event model + event-trace-equivalence checker over ordered (time, net, value) tuples within tolerance. <!-- traces-spec: digital-equivalence#ordered-events-not-vcd -->

<!-- component: digital -->
<!-- touches: project/src/digital/vcd.rs -->
- [ ] **21.** VCD parser into the event model (interchange only; no acceptance depends on VCD bytes). (depends on #20) <!-- traces-spec: digital-equivalence#vcd-interchange-only -->

### Python frontend contract (PyO3)

<!-- component: frontend -->
<!-- touches: project/src/frontend/pymodule.rs -->
<!-- uses-contract: netlist.CircuitGraph -->
- [ ] **22.** PyO3 module exposing analysis results as zero-copy NumPy views over Rust-owned memory. (depends on #1) <!-- traces-spec: frontend-contract#results-zero-copy-numpy --> <!-- traces-adr: ADR-0001 -->

<!-- component: frontend -->
<!-- touches: project/src/frontend/gil.rs -->
- [ ] **23.** Release the GIL during long solves; verify a concurrent Python thread progresses. (depends on #14, #22) <!-- traces-spec: frontend-contract#gil-released-during-solve --> <!-- traces-adr: ADR-0001 -->

### Validation harnesses (golden references)

<!-- component: orch -->
<!-- touches: project/tests/orchestration/golden_ngspice.rs -->
- [ ] **24.** ngspice golden-reference harness on sky130 for DC/AC/transient/noise within tolerance. (depends on #14, #15) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden -->

<!-- component: digital -->
<!-- touches: project/tests/digital/golden_icarus.rs -->
- [ ] **25.** Icarus Verilog golden-trace harness wired to the event-trace-equivalence checker. (depends on #20, #21) <!-- traces-spec: digital-equivalence#ordered-events-not-vcd -->
