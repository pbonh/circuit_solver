---
title: "Closed Enum Device Model Dispatch"
type: claim
id: claim-decision-0005-closed-enum-device-model-dispatch
tags: [decision, circuit-solver, device-model, dispatch, performance, rust, zero-cost-abstractions]
created: 2026-05-17
updated: 2026-05-18
sources:
  - "architecture/circuit-solver"
  - "grills/circuit-solver"
  - "vision/circuit-solver"
  - "contexts/device-modeling"
  - "contexts/numeric-solver"
  - "concepts/zero-cost-abstractions"
  - "concepts/static-dispatch"
  - "concepts/dynamic-dispatch"
  - "concepts/trait-objects"
  - "concepts/memory-safety"
confidence:
  base: 0.85
---

"In the context of the Device Model Engine's stamp evaluation inside tight Newton-Raphson loops, facing the requirement for zero-cost dispatch and cache-friendly data layouts, we decided for a closed enum (`enum DeviceModel { Diode(...), BJT(...), MOSFET(...), ... }`) covering all in-scope core semiconductor models, to achieve compile-time monomorphized dispatch with predictable memory layout and no vtable indirection, accepting that runtime extensibility of device models is excluded from scope and that adding a new model variant is a breaking change requiring recompilation."

## Status

accepted

## Context

Newton-Raphson stamp evaluation must run in a tight loop with zero-cost dispatch and cache-friendly data layouts. This is an [[concepts/architecturally-significant-requirement|architecturally significant requirement]] (ASR) because it constrains the internal architecture of the [[contexts/device-modeling|device-modeling]] context and directly impacts the throughput of the [[contexts/numeric-solver|numeric-solver]] context's nonlinear iteration. Every Newton-Raphson step re-linearizes every nonlinear device; the dispatch from element to stamp function is on the hot path.

The [[vision/circuit-solver|Circuit Solver vision]] explicitly bounds scope to core semiconductor models (diode, BJT, MOSFET Level-1 through BSIM4-level) and does not state a runtime extensibility requirement. The [[grills/circuit-solver|grill Q&A]] explored four dispatch alternatives (trait objects, closed enum, hybrid registry, proc-macro/codegen closed dispatch) and converged on the closed enum because the device scope is fixed and the performance stakes are high.

The [[architecture/circuit-solver|architecture page]] surfaces this decision under `## Decisions Surfaced` as the fifth top-level commitment.

## Decision

We commit to a closed enum dispatch strategy inside the Device Model Engine container:

1. **Closed enum for all core models.** `DeviceModel` is a Rust enum with one variant per in-scope model family: `Diode(...)`, `BJT(...)`, `MOSFET(...)`, and any other model families the vision declares in scope. Each variant carries its own `ModelParameters` payload.

2. **Zero-cost dispatch via `match`.** Stamp evaluation and Jacobian computation dispatch through `match` on the enum discriminant. The compiler monomorphizes (or inlines) the per-variant stamp code, producing direct branches with no vtable lookup and no heap indirection.

3. **No runtime extensibility.** The set of model variants is fixed at compile time. Runtime plugin registration, `Box<dyn DeviceModel>`, or a string-keyed registry are out of scope. If future scope expansion requires runtime extensibility, a superseding ADR must be opened.

4. **Model parameters owned by variants.** Each variant owns its parameter struct directly; no separate heap allocation or pointer chase is required to reach parameter data during stamp evaluation.

This decision means that the `device-modeling` context delivers `LinearizedModel` stamps to the `numeric-solver` context via a single enum type that the numeric solver matches on at each Newton iteration. The numeric solver does not hold trait objects or generic parameters over model types; it holds `Vec<DeviceModel>` (or similar) and matches.

## Consequences

**Positive:**
- Zero-cost dispatch: `match` on a closed enum compiles to a jump table or direct conditional branches with no vtable overhead, preserving the [[concepts/zero-cost-abstractions|zero-cost abstraction]] guarantee.
- Cache-friendly data layout: all model parameters for a given element live inline in the enum's memory footprint, eliminating the pointer-chase latency that `Box<dyn Trait>` or registry lookups introduce.
- Compile-time exhaustiveness: `match` arms on a closed enum are checked for exhaustiveness by the Rust compiler. Adding a model variant forces all stamp sites to be updated, preventing silent omission bugs.
- No heap allocation per model instance: the enum is `Sized` and can live in a dense `Vec` or array, enabling vectorized or cache-line-optimized traversal during MNA assembly.
- Alignment with vision scope: the decision is coherent with the explicit scope bound to core models; it does not over-engineer for extensibility that is not required.

**Negative:**
- No runtime extensibility: third-party or proprietary device models cannot be loaded at run time. Extending the simulator to a new model requires editing the enum, recompiling, and redeploying.
- Breaking change on variant addition: adding a new model variant is a source-breaking change for any downstream `match` that lacks a `_` catch-all. Within the simulator codebase this is a controlled refactor; for any external consumers of the enum it is an API break.
- Enum size bloat: the enum's size equals the largest variant plus discriminant. A small diode model and a large BSIM4 parameter set share the same footprint, so the diode variant pays padding overhead. For circuits dominated by small devices this can waste memory; mitigating this requires `Box`-wrapping the largest variant, which reintroduces a pointer chase.
- Code duplication risk: if multiple model families share similar stamp patterns (e.g., Level-1 and Level-2 MOSFET), the closed-enum approach may duplicate logic across `match` arms unless common helper functions are factored out.

**Neutral:**
- The decision does not prescribe the exact enum layout (`Box`-wrapped large variants vs. raw enum) or whether the enum is nested (e.g., `MOSFET(Level1(...), BSIM4(...))`). Those details are left to the device-modeling context's implementation spec.
- The decision does not preclude compile-time code generation (proc macros) that expands model equations into Rust code; such codegen would produce enum variants and match arms, not replace the enum.
- If the vision later expands to include additional core models (e.g., JFET, GaAs MESFET), the enum is extended with new variants under this same ADR. If the vision expands to *runtime* model loading, a new ADR supersedes this one.

## Related Decisions

- [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]] — Preceding ADR on solver backends; the numeric solver that consumes `LinearizedModel` stamps uses the hybrid `russell` + `faer` stack.
- [[decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views|ADR-0003]] — Preceding ADR on graph flattening; the flattened element list that feeds the device model engine is produced by the two-pass strategy.
- [[architecture/circuit-solver]] — The container diagram that surfaces this decision under `## Decisions Surfaced`.
- [[grills/circuit-solver]] — Q&A log where dispatch architecture alternatives were interrogated.
- [[vision/circuit-solver]] — Scope declaration that bounds device models to core families and excludes runtime extensibility.
- [[contexts/device-modeling]] — Bounded context that owns the `DeviceModel` enum and stamp generation.
- [[contexts/numeric-solver]] — Bounded context that receives stamps and drives Newton-Raphson iteration.
- [[concepts/zero-cost-abstractions]] — Concept page on Rust's zero-cost abstraction guarantee.
- [[concepts/static-dispatch]] — Concept page on compile-time dispatch via monomorphization.
- [[concepts/dynamic-dispatch]] — Concept page on runtime vtable dispatch, which this decision avoids.
- [[concepts/trait-objects]] — Concept page on `dyn Trait`, the alternative dispatch mechanism rejected here.
- [[concepts/memory-safety]] — Concept page on Rust memory safety; the closed enum preserves it by avoiding heap-allocated trait-object lifetimes.