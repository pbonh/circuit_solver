---
change-id: 2026-05-28-multidomain-solver-architecture
created: 2026-05-28
---

# Tasks

Ordered, dependency-aware checklist generated from `design.md` and the ADRs.
Every task is traceable to the scenario it satisfies via a `traces-spec`
comment, and grouped by ADR where applicable via a `traces-adr` comment.

### Netlist graph & frontend foundations

- [ ] **1.** Immutable `CircuitGraph` with a Rust-backed builder API exposed to Python. <!-- traces-spec: frontend-contract#circuit-graph-immutable --> <!-- traces-adr: ADR-0001 -->
- [ ] **2.** Two-pass graph flattening producing per-analysis sub-views (full matrix incl. ground built once). (depends on #1) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden --> <!-- traces-adr: ADR-0003 -->

### Numeric solver (pure-Rust sparse-direct)

- [ ] **3.** MNA assembly via branch stamping over the flattened graph. (depends on #2) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden --> <!-- traces-adr: ADR-0002 -->
- [ ] **4.** russell real-valued sparse LU for DC/transient solves. (depends on #3) <!-- traces-spec: analog-engine#transient-integration-matches-golden --> <!-- traces-adr: ADR-0002 -->
- [ ] **5.** faer complex-valued sparse LU for AC small-signal (G + jwC). (depends on #3) <!-- traces-spec: analog-engine#ac-uses-pure-rust-complex-backend --> <!-- traces-adr: ADR-0002 -->
- [ ] **6.** Newton-Raphson loop with gmin-stepping then source-stepping continuation and a structured non-convergence error. (depends on #3, #7) <!-- traces-spec: analog-engine#non-convergence-guarded --> <!-- traces-adr: ADR-0002 -->

### Device model engine (closed enum + codegen seam)

- [ ] **7.** Closed `enum DeviceModel` with per-variant stamp evaluator; diode and BJT variants matching the reference models. (depends on #3) <!-- traces-spec: device-modeling#diode-bjt-stamps-match-reference --> <!-- traces-adr: ADR-0005 -->
- [ ] **8.** Add a MOSFET model as an in-tree enum variant (monomorphized dispatch). (depends on #7) <!-- traces-spec: device-modeling#mosfet-in-tree-enum-variant --> <!-- traces-adr: ADR-0005 -->
- [ ] **9.** Ensure no runtime model-registration API exists (compile-time-only extensibility). <!-- traces-spec: device-modeling#runtime-registration-rejected --> <!-- traces-adr: ADR-0005 -->
- [ ] **10.** In-tree macro/codegen seam generating model-family variants into the closed enum. (depends on #7) <!-- traces-spec: device-modeling#model-family-via-codegen-seam --> <!-- traces-adr: ADR-0007 -->

### Native event-driven digital kernel (supersedes ADR-0004)

- [ ] **11.** Event queue with an in-process `run-until` API. <!-- traces-spec: digital-engine#native-kernel-event-queue --> <!-- traces-adr: ADR-0006 -->
- [ ] **12.** Delta-cycle combinational settling with oscillation detection (report, never hang). (depends on #11) <!-- traces-spec: digital-engine#zero-delay-combinational-settling --> <!-- traces-adr: ADR-0006 -->
- [ ] **13.** Checkpoint/restore of the event queue and net state for rollback. (depends on #11) <!-- traces-spec: digital-engine#native-kernel-optimistic-rollback --> <!-- traces-adr: ADR-0006 -->

### Analysis orchestration & mixed-signal

- [ ] **14.** Transient analysis driver using A-stable backward-Euler / trapezoidal integration. (depends on #4, #6) <!-- traces-spec: analog-engine#transient-integration-matches-golden --> <!-- traces-adr: ADR-0002 -->
- [ ] **15.** AC small-signal and noise analysis drivers. (depends on #5) <!-- traces-spec: analog-engine#ac-uses-pure-rust-complex-backend --> <!-- traces-adr: ADR-0002 -->
- [ ] **16.** Mixed-Signal Scheduler: optimistic time advance + checkpoint/rollback across the analog/digital boundary. (depends on #13, #14) <!-- traces-spec: mixed-signal-cosim#digital-driven-analog-load-rollback --> <!-- traces-adr: ADR-0006 -->
- [ ] **17.** Scheduler drives the native digital kernel via in-process run-until (no IPC). (depends on #11, #16) <!-- traces-spec: mixed-signal-cosim#scheduler-drives-native-kernel --> <!-- traces-adr: ADR-0006 -->
- [ ] **18.** Mixed-signal corpus: comparator + D flip-flop testbench. (depends on #16) <!-- traces-spec: mixed-signal-cosim#comparator-plus-dff -->
- [ ] **19.** Mixed-signal corpus: level shifter across supply domains. (depends on #16) <!-- traces-spec: mixed-signal-cosim#level-shifter -->

### Digital correctness (event-trace equivalence)

- [ ] **20.** Event model + event-trace-equivalence checker over ordered (time, net, value) tuples within tolerance. <!-- traces-spec: digital-equivalence#ordered-events-not-vcd -->
- [ ] **21.** VCD parser into the event model (interchange only; no acceptance depends on VCD bytes). (depends on #20) <!-- traces-spec: digital-equivalence#vcd-interchange-only -->

### Python frontend contract (PyO3)

- [ ] **22.** PyO3 module exposing analysis results as zero-copy NumPy views over Rust-owned memory. (depends on #1) <!-- traces-spec: frontend-contract#results-zero-copy-numpy --> <!-- traces-adr: ADR-0001 -->
- [ ] **23.** Release the GIL during long solves; verify a concurrent Python thread progresses. (depends on #14, #22) <!-- traces-spec: frontend-contract#gil-released-during-solve --> <!-- traces-adr: ADR-0001 -->

### Validation harnesses (golden references)

- [ ] **24.** ngspice golden-reference harness on sky130 for DC/AC/transient/noise within tolerance. (depends on #14, #15) <!-- traces-spec: analog-engine#dc-operating-point-matches-golden -->
- [ ] **25.** Icarus Verilog golden-trace harness wired to the event-trace-equivalence checker. (depends on #20, #21) <!-- traces-spec: digital-equivalence#ordered-events-not-vcd -->

