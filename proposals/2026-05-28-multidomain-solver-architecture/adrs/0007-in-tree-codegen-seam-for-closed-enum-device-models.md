---
adr: 0007
title: In-Tree Codegen Seam for Closed-Enum Device Models
status: accepted
created: 2026-05-28
supersedes: none
---

# ADR 0007: In-Tree Codegen Seam for Closed-Enum Device Models

## Status

accepted

## Y-Statement

In the context of scaling the device-model library to industrial coverage, facing the cost of hand-writing each variant of the closed device-model enum, we decided for a compile-time macro/codegen seam that generates model-family variants into the closed `enum DeviceModel` and
against hand-writing every enum variant, and a runtime-pluggable device-model trait (dynamic dispatch), to achieve a larger model library while preserving zero-cost, monomorphized stamp dispatch, accepting macro/build complexity, with runtime model registration remaining unsupported.

## Context

ADR-0005 ([[decisions/0005-closed-enum-device-model-dispatch]], effective 1.045) chose a closed `enum DeviceModel` for zero-cost monomorphized dispatch in the Newton-Raphson loop, accepting that a new model is a breaking recompile and that there is no runtime extensibility. An industrial model library pressures the cost of hand-writing each variant (grill `cc-adr0005-closed-enum`). The operator confirmed a refinement (not a reversal) at the `write_design` halt (Q2). Supporting KG: [[concepts/enum-type]] (effective 0.988), [[concepts/static-dispatch]] (0.95), [[concepts/zero-cost-abstractions]] (0.95).

## Decision

Introduce an **in-tree compile-time macro/codegen seam** that generates device-model family variants directly into the closed `enum DeviceModel`. Generated variants are ordinary enum members dispatched by static monomorphization. This **refines** ADR-0005 and preserves its core invariant: dispatch stays zero-cost and there is **no runtime model registration**.

## Consequences

Positive: the model library scales without hand-writing each variant, while keeping monomorphized dispatch and predictable memory layout. Negative: macro/codegen build complexity and longer compile times for large families. The ADR-0005 invariant holds — runtime registration is still rejected — so this is a refinement, not a supersession.

## Sources

- `design.md` (Device Model Engine container + C4 L3 component diagram)
- `specs/device-modeling/spec.md` (codegen-seam scenario)
- Refines [[decisions/0005-closed-enum-device-model-dispatch]] (effective 1.045)
- KG: [[concepts/enum-type]] (0.988), [[concepts/static-dispatch]] (0.95), [[concepts/zero-cost-abstractions]] (0.95)
- Inherited confidence (min rollup): 0.95 — presented recommended-accept; operator-confirmed
