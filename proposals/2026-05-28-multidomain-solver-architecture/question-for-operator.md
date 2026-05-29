---
change-id: 2026-05-28-multidomain-solver-architecture
stage: write_design
mode: pause_and_ask
status: answered   # operator: set to `answered` (and fill answers) or delete this file to resume
created: 2026-05-28
---

# Question for Operator — design stage halt

The `write_design` stage is `pause_and_ask`. The specs and the accepted ADRs
strongly determine most of the architecture, but **three design boundaries are
durable commitments the KG does not settle for an *industrial-strength,
all-three-domain* target**. Per ADR-0010 I will not silently pick. Each option
cites its KG support with effective confidence inline.

---

## Q1 (blocking) — Digital engine boundary: external co-simulation vs. native engine

The v1 architecture co-simulates with an **external** event-driven simulator
([[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler]],
effective 1.045; golden ref [[entities/icarus-verilog]]). The proposal reopens
this for "industrial-strength digital" (grill `oq-native-digital-scope`,
`cc-adr0004-external-cosim`). The KG supports **both** directions at high
confidence, so this is a genuine branch:

- **Option A — Keep external co-simulation (conservative; honors ADR-0004).**
  Digital stays an external container; the Mixed-Signal Scheduler issues
  `run-until`. No superseding ADR needed.
  - KG: [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler]] (effective 1.045).
  - Cost: "industrial digital" performance/perf-coupling bounded by IPC with the external tool.

- **Option B — Build a native event-driven digital engine (industrial).**
  Adds a digital-kernel container inside the solver built on
  [[concepts/discrete-event-system-specification]] (effective 0.95) /
  [[concepts/event-driven-architecture]] (effective 0.95).
  - Requires a **superseding ADR** for 0004 (durable reversal).
  - Benefit: single-process, no IPC; tighter mixed-signal sync.

- **Option C — Hybrid: native fast-path, external fallback for full Verilog.**
  Native engine for gate-level primitives; external for full HDL.
  - Highest complexity; two digital paths to validate.

This choice changes a **top-level C4 container** (external system vs. internal
container), so it must be settled before `design.md` is written.

## Q2 — Device-model extensibility seam

ADR-0005 ([[decisions/0005-closed-enum-device-model-dispatch]], effective 1.045)
chose a **closed enum** (no runtime extensibility; new model = recompile). An
"industrial model library" pressures this (grill `cc-adr0005-closed-enum`).

- **Option A — Keep the closed enum, add models in-tree** (specs currently assume this).
  No ADR change.
- **Option B — Add a controlled in-tree codegen/macro seam** for model families
  (still compile-time, still monomorphized) to scale the library without hand-writing each variant.
  Refinement of ADR-0005, not a reversal.
- **Option C — Introduce a runtime-pluggable model trait** (dynamic dispatch).
  **Reverses** ADR-0005; requires a superseding ADR and accepts vtable cost in the Newton loop.

## Q3 (confirm) — Steady-state / RF engine scope

Specs currently scope **out** harmonic-balance / shooting
([[concepts/shooting-method]], effective 0.85) per grill `oq-steady-state-scope`.

- **Option A — Confirm out of scope** for this change (recommended; keeps the spec surface bounded).
- **Option B — In scope** — adds an analysis-engine container and ~3+ scenarios.

---

### How to resume

Set `status: answered` in the frontmatter and write your picks below (or just
tell me in chat and I will record them and continue). I will not write
`design.md` or advance the design stage until this is resolved.

**Answers:**
- Q1: **Option B — native event-driven digital engine** (in-process DEVS/event-driven kernel). Supersedes [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler]]; record-adr must emit a superseding ADR. Specs revised to drop the external-boundary assumption; new `digital-engine` capability spec added. Icarus Verilog is retained as the **golden reference** for digital validation, not as the runtime engine.
- Q2: **Option B — in-tree codegen/macro seam** for device-model families. Refines (does not reverse) [[decisions/0005-closed-enum-device-model-dispatch]]: still compile-time monomorphized, runtime registration still rejected.
- Q3: **Option A — steady-state / RF out of scope** for this change (follow-on).
