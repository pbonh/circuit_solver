---
change-id: 2026-05-28-multidomain-solver-architecture
topic: Rust solver architecture spanning industrial-strength analog, digital, and mixed-signal device modeling and simulation engines (all three domains)
created: 2026-05-28
---

# Why

The `circuit-solver` project already has a ratified v1 architecture: five accepted ADRs
and a v1 acceptance spec validated against golden references. The knowledge graph's
textbook corpus — Hairer & Wanner (*Solving ODEs II*), Sze (*Physics of Semiconductor
Devices*), *Computer Methods for Circuit Analysis and Design*, *Advanced Symbolic
Analysis for VLSI Systems*, *Graphs in VLSI*, and *The Rust Programming Language* —
supplies the full method knowledge needed to take the solver from a v1 acceptance binary
to **industrial-strength coverage across all three domains** (continuous-time analog,
event-driven digital, mixed-signal co-simulation) for both the **device-modeling** layer
and the **simulation-engine** layer.

This proposal is KG-seeded: its context, prior art, and constraints are drawn from the
wiki with each claim's effective confidence shown inline. After the pre-seed KG
remediation (id reconciliation + Policy-A textbook-trust re-scoring; see
`development/log.md` 2026-05-28), every cited claim is sourced from a `kind: publication`
academic textbook, so confidence no longer discriminates *relevance* — topical proximity
to the architecture entity does. Context is therefore scoped to the curated 1-hop
architectural core plus the load-bearing domain methods, not the full (now fully
connected) corpus.

## Context (from KG)

**Accepted architecture (1-hop core of [[architecture/circuit-solver | supports]]).**
The five ratified decisions are the load-bearing context; all are accepted and carry
multi-source textbook backing (effective clamped high by the source-count multiplier):

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph | supports]] (effective 1.045) — in-process PyO3 binding, immutable `CircuitGraph` + per-request mutable analysis state.
- [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer | supports]] (effective 1.045) — pure-Rust hybrid sparse-direct LU: russell (real DC/transient) + faer (complex AC).
- [[decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views | supports]] (effective 1.045) — two-pass flattening with per-analysis sub-views.
- [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler | supports]] (effective 1.045) — optimistic analog/digital co-simulation via a shared Mixed-Signal Scheduler.
- [[decisions/0005-closed-enum-device-model-dispatch | supports]] (effective 1.045) — closed `enum DeviceModel` for zero-cost monomorphized stamp dispatch.

**Bounded contexts** (entities directly composing the architecture):
[[contexts/numeric-solver | supports]], [[contexts/device-modeling | supports]],
[[contexts/analysis-orchestration | supports]], [[contexts/netlist-graph | supports]],
[[contexts/application-frontend | supports]].

**Load-bearing methods directly linked from those contexts** (the curated technical core):

- [[concepts/modified-nodal-analysis | supports]] (effective 0.95) and [[concepts/nodal-analysis | supports]] (effective 0.95) — matrix formulation.
- [[concepts/newton-raphson-method | supports]] (effective 0.95) — nonlinear DC/transient inner loop.
- [[concepts/backward-euler | supports]] (effective 0.988) and [[concepts/trapezoidal-rule | supports]] (effective 0.95) — transient integration.
- [[concepts/ownership | supports]] (effective 0.95), [[concepts/zero-cost-abstractions | supports]] (effective 0.95), [[concepts/enum-type | supports]] (effective 0.988) — the Rust core discipline underpinning ADR-0005.
- [[concepts/mixed-level-simulation | supports]] (effective 0.95) — the mixed-signal premise behind ADR-0004.

_Scope note: the full 2-hop neighborhood of these hub entities is the entire 1166-claim
corpus (all now ≥ `proposal_seed_min` 0.70). Per the threshold rule that is technically
"context"; it is curated to the architectural core above to remain a usable proposal._

## Prior Art (from KG)

KG claims sourced from `kind: publication` textbooks (relaxed floor
`prior_art_floor` 0.60), grouped by domain. Each is an authoritative establishment of a
method the industrial-strength extension will build on.

**Analog engine & formulation** — [[concepts/branch-stamping | supports]] (effective 0.95), [[concepts/companion-model | supports]] (effective 0.95), [[concepts/lu-decomposition | supports]] (effective 0.95) and [[concepts/sparse-matrix | supports]] (effective 0.95), [[concepts/dc-analysis | supports]] / [[concepts/ac-analysis | supports]] / [[concepts/transient-analysis | supports]] (effective 0.95), [[concepts/noise-analysis | supports]] (effective 0.95), [[concepts/gmin-stepping | supports]] (effective 0.95) and [[concepts/source-stepping | supports]] (effective 0.95) — convergence aids. (*Computer Methods for Circuit Analysis and Design*, *Graphs in VLSI*, simulation whitepaper.)

**Integration theory** — [[concepts/a-stability | supports]] (effective 0.95), [[concepts/dahlquist-test-equation | supports]] (effective 0.95), [[concepts/algebraic-differential-equations | supports]] (effective 0.95). (*Solving ODEs II*, *Computer Methods*.)

**Device models** — [[concepts/diode-model | supports]] (effective 0.95), [[concepts/bjt-model | supports]] (effective 0.95), [[concepts/charge-flux-formulation | supports]] (effective 0.95). (*Computer Methods for Circuit Analysis and Design*.)

**Digital engine** — [[concepts/discrete-event-system-specification | supports]] (effective 0.95), [[concepts/event-driven-architecture | supports]] (effective 0.95), [[concepts/digital-network-analysis | supports]] (effective 0.85), [[concepts/and-inverter-graph | supports]] (effective 0.95). (*Modeling and Simulation of Systems*, *Graphs in VLSI*.)

**Mixed-signal** — [[concepts/mixed-level-simulation | supports]] (effective 0.95), [[concepts/switched-capacitor-network | supports]] (effective 0.95).

**Symbolic / sensitivity (industrial analysis breadth)** — [[concepts/sensitivity-analysis | supports]] (effective 0.95), [[concepts/determinant-decision-diagram | supports]] (effective 0.95), [[concepts/binary-decision-diagram | supports]] (effective 0.95). (*Advanced Symbolic Analysis for VLSI Systems*.)

**Rust core** — [[concepts/static-dispatch | supports]] (effective 0.95), [[concepts/memory-safety | supports]], [[concepts/trait-objects | supports]]. (*The Rust Programming Language*.)

**External golden references** (entities, not confidence-scored) —
[[entities/ngspice | mentions]] (analog reference), [[entities/icarus-verilog | mentions]]
(digital reference), validated on [[entities/sky130-pdk | mentions]] and
[[entities/asap7-pdk | mentions]].

## Candidate Problems

Per the seed rules, candidate problems are (i) claims below `low_confidence_floor` 0.45,
(ii) claims with active `contradicts` edges, and (iii) Question pages within 2 hops.

- **Open architectural question (KG-native).** [[grills/circuit-solver | mentions]]
  (type: `question`) — the decision tree behind ADRs 0001–0005, several branches of which
  bound *v1* scope and are the natural re-open points for an industrial-strength target.

_After the operator-confirmed Policy-A re-scoring, **no claim sits below 0.45** and there
are **zero `contradicts` edges**, so (i) and (ii) are empty by construction. The remaining
candidate problems below are coverage gaps surfaced from the authoring analysis, not
low-confidence KG signals:_

- **Device-model coverage gap.** ADR-0005's closed enum names MOSFET as in-scope, but the
  KG has no `concepts/mosfet-model` claim and BSIM-CMG is explicitly deferred in
  [[specs/circuit-solver | mentions]]. Industrial analog needs compact MOS models.
- **Native digital engine gap.** ADR-0004 co-simulates with an *external* event-driven
  simulator ([[entities/icarus-verilog | mentions]]). "Industrial-strength digital" may
  require a *native* event-driven kernel built on
  [[concepts/discrete-event-system-specification | mentions]] /
  [[concepts/event-driven-architecture | mentions]], reopening the external-vs-native boundary.
- **Steady-state / RF engine gap.** [[concepts/shooting-method | mentions]] (effective 0.85)
  exists in the KG but no harmonic-balance/PSS engine is in scope; industrial analog/RF
  typically requires one.

## Constraints (from KG)

The wiki has no claims with `kind: constraint` and no `refines`-chain constraints-root,
so this subsection is **derived from the accepted ADR claims' explicit "accepting that…"
clauses** (each an [[concepts/architecturally-significant-requirement | refines]]). These
bound the design space for any industrial-strength extension:

- **Pure-Rust dependency tree, no C/C++ FFI** — [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer | refines]] (effective 1.045). Rules out wrapping KLU/SuperLU; new solver capability must be pure-Rust.
- **Closed-enum device dispatch; no runtime model extensibility** — [[decisions/0005-closed-enum-device-model-dispatch | refines]] (effective 1.045). Every new device model is a breaking change requiring recompilation — a hard limit on an "industrial model library."
- **In-process PyO3 ABI coupling** — [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph | refines]] (effective 1.045). Python runtime must be ABI-compatible; cross-language debugging cost accepted.
- **Optimistic sync cost** — [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler | refines]] (effective 1.045). Checkpoint memory overhead and full re-solve on digital misprediction are accepted.
- **One-time full-matrix flatten incl. ground** — [[decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views | refines]] (effective 1.045). Per-solve sub-view masking cost accepted to avoid re-flattening.

## Proposed Change

Extend the solver to industrial-strength coverage along **three domains × two layers**,
within the constraints above:

1. **Analog (continuous-time).**
   - *Modeling:* add compact MOS models (resolving the MOSFET gap) inside the closed
     `enum DeviceModel`; extend charge/flux handling for reactive nonlinearities.
   - *Engine:* harden DC (continuation via gmin/source-stepping), transient (variable-order
     integration on the russell backend), AC and noise; evaluate adding a steady-state
     (harmonic-balance/shooting) engine.
2. **Digital (event-driven).**
   - *Modeling:* gate-/logic-level primitives.
   - *Engine:* decide native DEVS-based event kernel vs. continued external co-simulation
     (ADR-0004) — the central scope question below.
3. **Mixed-signal.**
   - Generalize the shared Mixed-Signal Scheduler (ADR-0004) from the v1 corpus
     (digital-driven analog load, comparator+DFF, level shifter) to arbitrary
     analog↔digital boundaries, keeping the two kernels decoupled.

Cross-cutting: a symbolic/sensitivity analysis surface
([[concepts/sensitivity-analysis | supports]], DDD/BDD) for industrial design-space work.

This is a seed for the grill stage; scope boundaries (especially native-digital and
steady-state engines) are deliberately left open for interrogation.

## Open Questions

- **Native digital engine vs. external co-simulation?** Does "industrial-strength digital"
  justify a native event-driven kernel, or does the optimistic co-sim boundary (ADR-0004)
  suffice? This reopens an accepted decision.
- **Device-model extensibility under the closed-enum constraint.** How large can the
  `enum DeviceModel` (ADR-0005) grow before the recompile-per-model constraint blocks an
  industrial model library? Is a controlled extensibility seam warranted?
- **Steady-state / RF scope.** Is a harmonic-balance/shooting engine in scope for "all
  three domains, industrially," or deferred?
- **Pure-Rust solver ceiling.** Can russell + faer (ADR-0002) meet industrial matrix sizes
  and conditioning without a C solver, or does the pure-Rust constraint need revisiting?
- **Compact-model fidelity.** Can BSIM-class models be expressed within the pure-Rust,
  closed-enum design at acceptable performance?
