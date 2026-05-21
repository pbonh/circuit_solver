---
title: "Spec: Circuit Solver"
type: spec
tags: [spec, circuit-solver, gherkin, analog, digital, mixed-signal]
created: 2026-05-17
updated: 2026-05-18
sources:
  - "vision/circuit-solver"
  - "grills/circuit-solver"
  - "architecture/circuit-solver"
  - "decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph"
  - "decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer"
  - "decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views"
  - "decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler"
  - "decisions/0005-closed-enum-device-model-dispatch"
adr_ids: ["0001", "0002", "0003", "0004", "0005"]
confidence: high
---

## Goal

Define binary acceptance criteria for the circuit-solver v1 release such that its analog, digital, and mixed-signal results are equivalent — under explicit numeric tolerances — to a *golden reference* computed by [[entities/ngspice]] (analog) and [[entities/icarus-verilog]] (digital), using the [[entities/sky130-pdk]] for analog and gate-level digital tests and the [[entities/asap7-pdk]] for additional gate-level digital tests. Acceptance is *functional-correctness lenient* (5 % / 0.5 dB / 100 µV / 2 dB envelopes); tighter envelopes are deferred to a v2 spec. ASAP7's BSIM-CMG analog primitives are out of scope for v1 per [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]].

## Scope

| Actor                       | Impact                                                                                       | Deliverable                                                              |
| --------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| Analog designer (Pria)      | Trusts DC / AC / transient / noise results enough to use them for design exploration         | Per-analysis [[concepts/golden-reference]] conformance harness vs ngspice |
| Digital designer (Devesh)   | Trusts gate-level event traces enough to substitute the simulator for iverilog in mixed-signal flows | Event-trace equivalence harness vs iverilog                              |
| Mixed-signal designer (Mira) | Trusts the analog↔digital boundary enough to run multi-domain testbenches end-to-end          | Three canonical mixed-signal cosim harnesses (digital→analog load, comparator+DFF, level shifter) |
| Solver maintainer (Roya)    | Has a tight, reproducible regression bar for the [[concepts/newton-raphson-method]] core      | Convergence guard scenarios (hybrid ΔI/ΔV + KCL, source / Gmin homotopy) |
| Python user (Pria)          | Builds circuits incrementally from Python and reads results as NumPy arrays without surprise | API-contract scenarios for the [[entities/pyo3]] frontend                |

## User Stories

### Story 1 — Analog conformance to ngspice on Sky130

**Story:** As Pria, an analog designer, I want every analog analysis the simulator advertises (DC, AC, transient, noise) to agree with [[entities/ngspice]] on the [[entities/sky130-pdk]] within a documented tolerance envelope, so that I can use circuit-solver results as a substitute for ngspice without re-deriving design margins.

**Acceptance criteria:**

- For every Sky130 test circuit in the analog conformance harness, the simulator computes a DC operating point whose every node voltage differs from the ngspice reference by no more than 5 % relative or 10 µV absolute, whichever is greater, and whose every branch current differs by no more than 5 % relative or 10 pA absolute.
- For every Sky130 small-signal test circuit, the simulator's AC magnitude differs from the ngspice reference by no more than 0.5 dB at every swept frequency point.
- For every Sky130 transient test circuit, the simulator's waveform differs from the ngspice reference by no more than 5 % relative or 100 µV absolute at every common timepoint, after timestep alignment.
- For every Sky130 small-signal noise test circuit, the simulator's input-referred noise spectral density differs from the ngspice reference by no more than 2 dB at every swept frequency point.
- An analysis run that fails any of the above bounds is reported as `failed: golden-reference deviation`, never silently as `converged`.

### Story 2 — Digital event-trace equivalence with Icarus Verilog

**Story:** As Devesh, a digital designer, I want the simulator's digital event kernel to produce traces that are event-equivalent to [[entities/icarus-verilog]] on a corpus of standard cells and small testbenches drawn from Sky130 and ASAP7 gate-level libraries, so that I can move existing Verilog testbenches into the mixed-signal flow without re-validating digital behaviour.

**Acceptance criteria:**

- For every cell and testbench in the digital conformance corpus, the simulator emits a [[concepts/value-change-dump]] (VCD) trace whose set of (time, signal, value) events at every iverilog cycle boundary is identical to iverilog's VCD, as defined by [[concepts/event-trace-equivalence]].
- Intra-cycle settling order is allowed to diverge; only the observable signal values at each cycle boundary are compared.
- The corpus includes at minimum: an inverter, a 2-input NAND, a D flip-flop, a 4-bit ripple-carry counter, and one Sky130 and one ASAP7 hierarchical gate-level netlist of at least 100 gates.
- A trace divergence that survives until the next cycle boundary fails the run; an intra-cycle glitch that disappears by the next boundary does not.
- Every comparison run records the iverilog version and the exact VCD reference path in its result metadata.

### Story 3 — Mixed-signal cosim correctness on three canonical circuits

**Story:** As Mira, a mixed-signal designer, I want the simulator's [[concepts/mixed-level-simulation]] kernel to produce correct results on three canonical circuits — a digital-driven analog load, a comparator feeding a clocked latch, and a Sky130 level shifter across power domains — so that I can trust the optimistic time-advance and rollback machinery committed in [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]].

**Acceptance criteria:**

- For the digital-driven analog load, every analog node voltage observed at each digital cycle boundary differs from a lockstep-cosim reference (iverilog ↔ ngspice run in synchronised steps) by no more than 5 % relative or 100 µV absolute.
- For the comparator + clocked latch, the captured `Q` value at every clock edge is identical to the lockstep-cosim reference for at least 10⁴ consecutive clock cycles on a randomised analog stimulus.
- For the Sky130 level shifter, the propagation delay from digital input transition to the analog output crossing the receiving-domain threshold differs from the lockstep-cosim reference by no more than 5 % at both rising and falling edges.
- The [[concepts/checkpoint]] rate (rollback events per analog timestep accepted) on every mixed-signal scenario does not exceed 5 % of accepted analog timesteps under the spec's stimulus.
- A mixed-signal run that exceeds the rollback budget is reported as `failed: rollback storm`, not silently slowed.

### Story 4 — Newton-Raphson convergence guard

**Story:** As Roya, a solver maintainer, I want the [[concepts/newton-raphson-method]] hybrid convergence criterion (ΔI/ΔV primary + KCL guard) and the homotopy aids (source stepping, Gmin stepping) to converge robustly on a documented benchmark suite, so that future changes to the [[contexts/numeric-solver]] context cannot silently regress robustness.

**Acceptance criteria:**

- On every circuit in the convergence benchmark suite, a DC operating-point solve converges in no more than 100 Newton iterations at the simulator's default tolerances.
- For every circuit where ΔI/ΔV reports convergence, the KCL residue at the converged point is below the simulator's `ABSTOL` (1 pA by default); if KCL fails, the iteration continues rather than returning a "converged" result.
- For every circuit flagged in the suite as requiring homotopy, either [[concepts/source-stepping]] or [[concepts/gmin-stepping]] succeeds within its configured retry budget; the choice and retry count are recorded in the convergence report.
- A convergence failure produces a `ConvergenceReport` naming which guard fired (ΔI/ΔV, KCL, homotopy retry exhausted), never a silent partial answer.

### Story 5 — Python frontend contract

**Story:** As Pria, a Python user, I want the [[entities/pyo3]] frontend to expose an immutable [[concepts/graph]] builder, zero-copy NumPy results, and predictable error propagation, so that I can build circuits interactively without inheriting C-extension footguns or copying large result arrays.

**Acceptance criteria:**

- After a `Circuit` is built by the Python builder API, any attempt to mutate an element or node from Python raises a Python `TypeError` rather than silently editing the underlying Rust graph.
- A successful DC, AC, transient, or noise analysis returns a result object whose voltage and current arrays are `numpy.ndarray` instances backed by the Rust solution buffer (verified by `arr.flags.owndata is False` and `arr.flags.writeable is False`).
- A panic in the Rust core surfaces as a `RuntimeError` in Python with the panic message preserved; the Python interpreter does not abort.
- A long-running analysis releases the [[concepts/global-interpreter-lock]] (GIL) during the [[entities/russell]] / [[entities/faer]] solve phases; a concurrent Python thread observes ≥ 80 % CPU utilisation while the solver is hot.

## Scenarios

### Analog conformance — DC operating point

```gherkin
Feature: DC operating point matches ngspice on Sky130

  Scenario: Pria validates an NMOS biased in saturation
    Given Pria has a Sky130 NMOS test circuit "nmos_sat" with W=1µm, L=150nm, Vgs=Vds=1.8V
    And ngspice has computed the operating point as the golden reference
    When Pria runs the DC analysis through circuit-solver against the same netlist
    Then every node voltage agrees with the ngspice reference to within 5% relative or 10µV absolute
    And every device terminal current agrees with the ngspice reference to within 5% relative or 10pA absolute
    And the analysis result reports "converged"
```

### Analog conformance — AC small-signal

```gherkin
Feature: AC magnitude matches ngspice on Sky130

  Scenario: Pria sweeps a common-source amplifier from 1 Hz to 1 GHz
    Given Pria has a Sky130 common-source amplifier "cs_amp" biased at Id=100µA with a 10kΩ load
    And ngspice has produced the small-signal frequency response as the golden reference
    When Pria runs an AC sweep from 1Hz to 1GHz with 10 points per decade through circuit-solver
    Then the magnitude in dB agrees with the ngspice reference to within 0.5 dB at every swept frequency point
```

### Analog conformance — Transient

```gherkin
Feature: Transient waveforms match ngspice on Sky130

  Scenario: Pria simulates a five-stage ring oscillator
    Given Pria has a Sky130 five-stage CMOS ring oscillator "ring_5"
    And ngspice has produced a transient waveform from 0 to 10ns as the golden reference
    When Pria runs a transient analysis from 0 to 10ns with adaptive timestepping through circuit-solver
    Then every common timepoint's node voltages agree with the ngspice reference to within 5% relative or 100µV absolute
    And the oscillation period agrees with the ngspice reference to within 5%
```

### Analog conformance — Noise

```gherkin
Feature: Noise spectral density matches ngspice on Sky130

  Scenario: Pria measures input-referred noise of the common-source amplifier
    Given Pria has the Sky130 "cs_amp" circuit biased at its nominal operating point
    And ngspice has produced the input-referred noise spectral density from 1Hz to 1MHz as the golden reference
    When Pria runs a noise analysis over the same frequency range through circuit-solver
    Then the input-referred noise spectral density agrees with the ngspice reference to within 2 dB at every swept frequency point
```

### Digital conformance — Event-trace equivalence

```gherkin
Feature: Digital event traces are equivalent to Icarus Verilog

  Scenario Outline: Devesh validates a digital cell or testbench against iverilog
    Given Devesh has the Verilog source "<source>" from the <pdk> gate-level corpus
    And iverilog has produced the reference VCD for testbench "<bench>"
    When Devesh runs the same Verilog source under circuit-solver's digital kernel with testbench "<bench>"
    Then the (time, signal, value) event set at every iverilog cycle boundary is identical to the iverilog reference
    And any intra-cycle glitch that disappears before the next cycle boundary does not fail the run

    Examples:
      | source            | pdk    | bench              |
      | inv_x1.v          | sky130 | tb_inv_x1.v        |
      | nand2_x1.v        | sky130 | tb_nand2_x1.v      |
      | dff_x1.v          | sky130 | tb_dff_x1.v        |
      | counter_4bit.v    | sky130 | tb_counter_4bit.v  |
      | hier_100gate.v    | sky130 | tb_hier_100gate.v  |
      | hier_100gate_a7.v | asap7  | tb_hier_100gate.v  |
```

### Mixed-signal cosim — Digital-driven analog load

```gherkin
Feature: Digital stimulus drives an analog load correctly

  Scenario: Mira drives a Sky130 inverter chain into an RC load
    Given Mira has a Sky130 five-stage inverter chain whose output sees a 1pF capacitor to ground
    And a Verilog testbench drives the chain's input with a 100MHz square wave
    And a lockstep-cosim reference (iverilog ↔ ngspice) has produced the analog output trace
    When Mira runs the same circuit under circuit-solver's optimistic mixed-signal scheduler
    Then the analog output voltage at every digital cycle boundary agrees with the lockstep reference to within 5% relative or 100µV absolute
    And the ratio of rollback events to accepted analog timesteps does not exceed 5%
```

### Mixed-signal cosim — Comparator + clocked latch

```gherkin
Feature: Analog threshold crossings produce correct digital events

  Scenario: Mira feeds a comparator output into a Verilog D flip-flop
    Given Mira has an analog comparator with a 100µV input-referred offset and a randomised differential stimulus
    And the comparator output feeds a Verilog D flip-flop clocked at 100MHz
    And a lockstep-cosim reference (iverilog ↔ ngspice) has produced the captured Q sequence over 10000 clock cycles
    When Mira runs the same circuit under circuit-solver's optimistic mixed-signal scheduler for 10000 clock cycles
    Then the captured Q value at every clock edge is identical to the lockstep reference for all 10000 cycles
```

### Mixed-signal cosim — Level shifter across power domains

```gherkin
Feature: Cross-domain timing matches lockstep reference

  Scenario: Mira drives a Sky130 1.8V-to-3.3V level shifter
    Given Mira has a Sky130 level shifter that translates from a 1.8V digital domain to a 3.3V analog domain
    And a Verilog testbench drives the 1.8V input with a 50MHz square wave under a 1pF receiving-domain load
    And a lockstep-cosim reference (iverilog ↔ ngspice) has produced rising and falling propagation delays
    When Mira runs the same level shifter under circuit-solver's optimistic mixed-signal scheduler
    Then the rising-edge propagation delay agrees with the lockstep reference to within 5%
    And the falling-edge propagation delay agrees with the lockstep reference to within 5%
```

### Newton-Raphson — Hybrid convergence guard

```gherkin
Feature: KCL guard rejects false ΔI/ΔV convergence

  Scenario: Roya verifies a near-discontinuity bias point
    Given Roya has a parallel-diode-and-resistor circuit at a bias point where the diode I-V curve is steep
    And the simulator's default ABSTOL is 1pA
    When Roya runs a DC operating-point analysis through circuit-solver
    Then the Newton-Raphson iteration converges only when both the ΔI/ΔV update is below tolerance and the KCL residue at every node is below ABSTOL
    And the convergence report names "hybrid: dIdV+KCL" as the criterion that fired
```

### Newton-Raphson — Homotopy fallback

```gherkin
Feature: Source / Gmin stepping unlocks pathological DC bias

  Scenario: Roya runs a CMOS inverter at its metastable point
    Given Roya has a Sky130 CMOS inverter biased exactly at its switching threshold so plain Newton-Raphson diverges from zero initial guess
    And the simulator's homotopy retry budget is set to the default 10 steps
    When Roya runs a DC operating-point analysis through circuit-solver
    Then either source stepping or Gmin stepping succeeds within the retry budget
    And the convergence report names which homotopy fired and the number of steps consumed
```

### Frontend — Immutability and zero-copy

```gherkin
Feature: PyO3 frontend preserves Rust ownership and avoids result copying

  Scenario: Pria attempts to mutate a built circuit and reads a result array
    Given Pria has built a Sky130 common-source amplifier via the Python builder API
    And the resulting Circuit handle is held in a Python variable
    When Pria attempts to assign a new value to one of the Circuit's element attributes from Python
    Then Python raises a TypeError and the underlying Rust graph is unchanged
    And the result arrays returned by a subsequent DC analysis are numpy.ndarray instances with owndata=False and writeable=False
```

### Frontend — GIL release during solve

```gherkin
Feature: Long solves do not block the Python interpreter

  Scenario: Pria runs a 100k-timestep transient while a Python thread spins
    Given Pria has a Sky130 transient analysis expected to take longer than one second of wall-clock time
    And a second Python thread is running a CPU-bound loop in parallel
    When Pria submits the transient analysis through circuit-solver
    Then the second Python thread observes at least 80% CPU utilisation throughout the solve
    And the transient result returns successfully without raising a deadlock
```

## Glossary

- `Golden reference` — A precomputed result from a trusted external tool (here, [[entities/ngspice]] for analog and [[entities/icarus-verilog]] for digital) against which the simulator's output is compared bit- or tolerance-bounded; see [[concepts/golden-reference]].
- `Conformance harness` — The automated test suite that runs the simulator and the golden-reference tool on the same input and compares outputs under the spec's tolerance bounds.
- `Tolerance envelope` — The triple (relative tolerance, absolute tolerance, dB tolerance) bounding allowed deviation; v1 uses 5 % / 10 µV / 0.5 dB for analog magnitudes and 2 dB for noise spectral density.
- `Operating point` — The DC steady-state solution vector used as the linearisation base for AC, noise, and transient analyses ([[concepts/dc-analysis]]).
- `Event-trace equivalence` — Equality of the (time, signal, value) event set at every reference-kernel cycle boundary, ignoring intra-cycle settling order ([[concepts/event-trace-equivalence]]).
- `Lockstep-cosim reference` — A mixed-signal reference computed by running iverilog and ngspice in fully synchronised steps, used as the ground truth for the simulator's optimistic mixed-signal output ([[concepts/mixed-level-simulation]]).
- `Rollback storm` — A pathological condition where the optimistic mixed-signal scheduler ([[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]]) rolls back more than 5 % of accepted analog timesteps, indicating a misprediction failure mode.
- `Homotopy aid` — A continuation strategy ([[concepts/source-stepping]], [[concepts/gmin-stepping]]) that gradually morphs an unsolvable problem into the target problem when plain Newton-Raphson cannot converge from the initial guess.
- `Convergence report` — The named-criterion record returned by every analysis describing which guard fired (ΔI/ΔV, KCL, homotopy retry exhausted) or which homotopy succeeded.
- `Zero-copy result` — A NumPy array backed directly by Rust-owned memory, distinguished by `owndata=False` and `writeable=False` flags (see [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph|ADR-0001]]).
- `Sky130 PDK` — The SkyWater 130 nm open process design kit, used for all v1 analog and most digital corpus circuits ([[entities/sky130-pdk]]).
- `ASAP7 PDK` — The ASAP 7 nm predictive PDK, used in v1 for **gate-level digital corpus circuits only**; its BSIM-CMG analog primitives are deferred per [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]] ([[entities/asap7-pdk]]).

## Sources

- [[vision/circuit-solver]] — In-scope analyses (DC / AC / transient / noise) and bounded device model set (diode / BJT / MOSFET L1–BSIM4).
- [[grills/circuit-solver]] — Resolved design questions whose commitments drive these scenarios (hybrid solver backends, optimistic mixed-signal sync, closed-enum dispatch, hybrid Newton convergence guard).
- [[architecture/circuit-solver]] — Container responsibilities (Numeric Solver, Mixed-Signal Scheduler, Python Frontend) referenced by the scenarios.
- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph|ADR-0001]] — Source of the immutable-graph and zero-copy-result invariants in Story 5. *Status: accepted (2026-05-18).*
- [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]] — Source of the [[entities/russell]] / [[entities/faer]] backend split assumed in Story 5's GIL-release scenario. *Status: accepted (2026-05-18).*
- [[decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views|ADR-0003]] — Source of the assumption that switching analysis types reuses the flattened structure (implicit in Story 1's multi-analysis tolerance envelope). *Status: accepted (2026-05-18).*
- [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]] — Source of Story 3's optimistic-cosim and rollback-storm criteria. *Status: accepted (2026-05-18).*
- [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]] — Source of the v1 device scope (no BSIM-CMG), which is the reason ASAP7 is digital-only in Story 2. *Status: accepted (2026-05-18).*
- [[concepts/modified-nodal-analysis]], [[concepts/newton-raphson-method]], [[concepts/dc-analysis]], [[concepts/ac-analysis]], [[concepts/transient-analysis]], [[concepts/noise-analysis]], [[concepts/mixed-level-simulation]], [[concepts/source-stepping]], [[concepts/gmin-stepping]], [[concepts/checkpoint]], [[concepts/graph]].
- [[concepts/golden-reference]], [[concepts/event-trace-equivalence]], [[concepts/value-change-dump]], [[concepts/global-interpreter-lock]] — Created as `confidence: low` stubs alongside this spec.
- [[entities/ngspice]], [[entities/icarus-verilog]], [[entities/sky130-pdk]], [[entities/asap7-pdk]], [[entities/pyo3]], [[entities/russell]], [[entities/faer]] — The entities named in scenarios; the four not previously in the wiki are created as `confidence: low` stubs alongside this spec.
- **ADR-status note:** All five governing ADRs were promoted from `proposed` to `accepted` on 2026-05-18, satisfying the Spec workflow's lint rule (`Every spec's frontmatter.adr_ids entry must resolve to an existing ADR file with ## Status: accepted`). This spec is now eligible for `/wiki-kanban-emit` under the P2 pipeline pattern.

## Kanban Tasks

**Board:** `circuit-solver`

**Idempotency Keys:**
- Parent: `circuit-solver:0001+0002+0003+0004+0005:8f2bb9a98937aad9e3df672478aabd493d18d4cbdf8ef6ef3206726dfafb96b3`
- Scenario — Pria validates an NMOS biased in saturation: `circuit-solver:0001+0002+0003+0004+0005:pria-validates-an-nmos-biased-in-saturation:ccf842c441bb15d3069fd0de5e689a54c42f5cf1494359333c4ef2559434ba6f`
- Scenario — Pria sweeps a common-source amplifier from 1 Hz to 1 GHz: `circuit-solver:0001+0002+0003+0004+0005:pria-sweeps-a-common-source-amplifier-from-1-hz-to-1-ghz:afe49fc0b2764cae43f536263b747e3b1f702c70116da53c11b322f0c3637830`
- Scenario — Pria simulates a five-stage ring oscillator: `circuit-solver:0001+0002+0003+0004+0005:pria-simulates-a-five-stage-ring-oscillator:73e4918084abe2b5b824a46a096f334f03221d09d79896b6743cbb25f08ea8e6`
- Scenario — Pria measures input-referred noise of the common-source amplifier: `circuit-solver:0001+0002+0003+0004+0005:pria-measures-input-referred-noise-of-the-common-source-amplifier:92c0dfb7576369ec1edf057ca777ffc62c85f8b513de68bfbc80d639b2da463f`
- Scenario — Devesh validates a digital cell or testbench against iverilog: `circuit-solver:0001+0002+0003+0004+0005:devesh-validates-a-digital-cell-or-testbench-against-iverilog:946772d4c59ce350d530a486ca965392050e05925f68f8b94f618dde987ac0b0`
- Scenario — Mira drives a Sky130 inverter chain into an RC load: `circuit-solver:0001+0002+0003+0004+0005:mira-drives-a-sky130-inverter-chain-into-an-rc-load:3ebfaf2bec1dbedfbc8e3066cd60d33047353f0cf9cbb63004f4a2afaaf2df43`
- Scenario — Mira feeds a comparator output into a Verilog D flip-flop: `circuit-solver:0001+0002+0003+0004+0005:mira-feeds-a-comparator-output-into-a-verilog-d-flip-flop:730cf569cabffb91a4fcdaa24391330094d000fc5074d80082c8d36fecce86f0`
- Scenario — Mira drives a Sky130 1.8V-to-3.3V level shifter: `circuit-solver:0001+0002+0003+0004+0005:mira-drives-a-sky130-18v-to-33v-level-shifter:394f11c795fcaaf77436064b100f4b5a5082f079e3cfc2dc1b0b9e554b98f0ca`
- Scenario — Roya verifies a near-discontinuity bias point: `circuit-solver:0001+0002+0003+0004+0005:roya-verifies-a-near-discontinuity-bias-point:242778f20d9e6da12e6aeee55295b993d4643ba3b72e6093faac266f29768e14`
- Scenario — Roya runs a CMOS inverter at its metastable point: `circuit-solver:0001+0002+0003+0004+0005:roya-runs-a-cmos-inverter-at-its-metastable-point:42b3c745570b96c869bea6ecaf63cfe9c4335e295db8b3d51e8fdc45f56cf2b3`
- Scenario — Pria attempts to mutate a built circuit and reads a result array: `circuit-solver:0001+0002+0003+0004+0005:pria-attempts-to-mutate-a-built-circuit-and-reads-a-result-array:f7a895621c1317053115fb1252f9f429d120c49d2f1fe6dd9d6f2e1196735430`
- Scenario — Pria runs a 100k-timestep transient while a Python thread spins: `circuit-solver:0001+0002+0003+0004+0005:pria-runs-a-100k-timestep-transient-while-a-python-thread-spins:9099a5754ed68c1647a3c0edb159aa3e7f86a635a64970da35fd998dc7d2c991`
- Aggregator: `circuit-solver:0001+0002+0003+0004+0005:aggregator:8e0549487ea897c877fe5ae528812b963f1857313c5815babb587a2787d73abc`

**ADR IDs:** `0001`, `0002`, `0003`, `0004`, `0005`

**Collaboration Pattern:** P2 pipeline (`worker` → `reviewer`)

**Profile Mapping:**
- orchestrator → `default`
- worker → `default`
- reviewer → `default`

**Tenant:** `default`

**Workspace:** `dir:/home/phillip/Boxes/Homes/RustDev/Code/github.com/pbonh/circuit_solver/project`

**Skills:** `wiki-maintainer`, `kanban-worker`

**Task List:**

| ID | Role | Assignee | Scenario / Description |
|---|---|---|---|
| `t_e3d1ebe9` | orchestrator | default | Parent (spec orchestration) |
| `t_8ca9027e` | worker | default | Scenario: Pria validates an NMOS biased in saturation |
| `t_0dac134b` | worker | default | Scenario: Pria sweeps a common-source amplifier from 1 Hz to 1 GHz |
| `t_2c3eb911` | worker | default | Scenario: Pria simulates a five-stage ring oscillator |
| `t_7c3875fb` | worker | default | Scenario: Pria measures input-referred noise of the common-source amplifier |
| `t_6ec40808` | worker | default | Scenario: Devesh validates a digital cell or testbench against iverilog |
| `t_99a70ad2` | worker | default | Scenario: Mira drives a Sky130 inverter chain into an RC load |
| `t_69000f20` | worker | default | Scenario: Mira feeds a comparator output into a Verilog D flip-flop |
| `t_79ecbefd` | worker | default | Scenario: Mira drives a Sky130 1.8V-to-3.3V level shifter |
| `t_af0770ea` | worker | default | Scenario: Roya verifies a near-discontinuity bias point |
| `t_8e71a163` | worker | default | Scenario: Roya runs a CMOS inverter at its metastable point |
| `t_5931e2d9` | worker | default | Scenario: Pria attempts to mutate a built circuit and reads a result array |
| `t_a073ec79` | worker | default | Scenario: Pria runs a 100k-timestep transient while a Python thread spins |
| `t_10406b92` | reviewer | default | Aggregator (Implementation Evidence) |
