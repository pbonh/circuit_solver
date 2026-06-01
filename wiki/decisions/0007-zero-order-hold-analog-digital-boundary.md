---
title: "Zero-Order Hold Default at Analog-Digital Boundary"
type: decision
tags: [decision, circuit-solver, mixed-signal, boundary-interpolation, scheduler]
created: 2025-07-18
sources:
  - "openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0007-zero-order-hold-analog-digital-boundary.md"
confidence: high
---

"In the context of mixed-signal co-simulation where the analog solver's adaptive timesteps rarely align with digital event times, facing the need for a deterministic, charge-conserving signal exchange scheme at the analog-digital boundary, we decided for zero-order hold (ZOH) as the default interpolation method — the analog value at the last accepted timestep is held constant until the digital event time — with linear interpolation available as a per-request option, and against always using linear interpolation or requiring exact timestep alignment, to achieve charge conservation at the boundary and simplicity of implementation, accepting that ZOH introduces stairstep artifacts on fast digital edges."

## Status

accepted

## Architecturally Significant Requirement

The boundary interpolation scheme determines the accuracy and charge conservation of every mixed-signal simulation. The spec/mixed-signal-cosim requires signal exchange at synchronization points but does not specify interpolation. The [[concepts/mixed-level-simulation]] pitfall warns: "Discrete/analog timestep coordination can introduce subtle artifacts at boundary crossings."

## Options Considered

- **Linear interpolation** — more accurate on edges, but does not guarantee charge conservation.
- **Force timestep alignment** — no interpolation needed, but constrains adaptive timestepping and conflicts with ADR-0004.
- **Zero-order hold with linear opt-in (chosen)** — charge-conserving by construction; matches SPICE convention; linear available as option.

## Consequences

- ZOH preserves charge conservation at the boundary.
- Stairstep artifacts on fast edges; conformance harness must account for scheme.
- `AnalysisRequest` must expose `boundary_interpolation` parameter.

## Source

- OpenSpec ADR: `openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0007-zero-order-hold-analog-digital-boundary.md`
