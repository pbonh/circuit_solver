---
title: In-Tree Codegen Seam for Closed-Enum Device Models
type: claim
id: decisions/0007-in-tree-codegen-seam-for-closed-enum-device-models
tags:
- decision
- circuit-solver
- device-model
- codegen
- macros
- rust
- zero-cost-abstractions
created: '2026-05-28'
updated: '2026-05-28'
refines: decisions/0005-closed-enum-device-model-dispatch
sources:
- architecture/circuit-solver
- grills/circuit-solver
- vision/circuit-solver
- contexts/device-modeling
- contexts/numeric-solver
- concepts/enum-type
- concepts/static-dispatch
- concepts/zero-cost-abstractions
confidence:
  base: 0.95
  source_count: 8
  contradicted: false
  effective: 1.045
  inputs_hash: 297789400ac7e97d
---
"In the context of scaling the device-model library to industrial coverage, facing the cost of hand-writing each variant of the closed device-model enum, we decided for a compile-time macro/codegen seam that generates model-family variants into the closed `enum DeviceModel` and against hand-writing every enum variant and a runtime-pluggable device-model trait (dynamic dispatch), to achieve a larger model library while preserving zero-cost monomorphized stamp dispatch, accepting macro/build complexity and that runtime model registration remains unsupported."

## Status

accepted

Refines [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]] (does not supersede it).

## Context

[[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]] chose a closed `enum DeviceModel` for zero-cost monomorphized dispatch in the Newton-Raphson loop, accepting that a new model is a breaking recompile and that there is no runtime extensibility. An industrial model library pressures the cost of hand-writing each variant. The operator confirmed a refinement (not a reversal) at the design stage (`proposals/2026-05-28-multidomain-solver-architecture/question-for-operator.md`, Q2). Supporting KG: [[concepts/enum-type]], [[concepts/static-dispatch]], [[concepts/zero-cost-abstractions]].

## Decision

Introduce an **in-tree compile-time macro/codegen seam** that generates device-model family variants directly into the closed `enum DeviceModel`. Generated variants are ordinary enum members dispatched by static monomorphization. This **refines** ADR-0005 and preserves its core invariant: dispatch stays zero-cost and there is **no runtime model registration**.

## Consequences

**Positive:**
- The model library scales without hand-writing each variant, while keeping monomorphized dispatch and predictable memory layout.

**Negative:**
- Macro/codegen build complexity and longer compile times for large model families.

**Neutral:**
- The ADR-0005 invariant holds — runtime registration is still rejected — so this is a refinement, not a supersession. ADR-0005 itself anticipated this (its "Neutral" note that proc-macro codegen producing enum variants is permitted under the same ADR).

## Related Decisions

- [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]] — Refined by this ADR; the closed-enum dispatch invariant is preserved.
- [[contexts/device-modeling]] — Bounded context owning the `DeviceModel` enum and stamp generation.
- [[contexts/numeric-solver]] — Receives stamps and drives Newton-Raphson iteration.
- [[concepts/enum-type]], [[concepts/static-dispatch]], [[concepts/zero-cost-abstractions]] — The Rust dispatch concepts the decision rests on.
- [[architecture/circuit-solver]] — Container diagram surfacing the decisions.

## Provenance

Recorded by the scientia pipeline for change `2026-05-28-multidomain-solver-architecture` (design halt Q2, operator-confirmed; inherited confidence 0.95, recommended-accept). See `proposals/2026-05-28-multidomain-solver-architecture/`.
