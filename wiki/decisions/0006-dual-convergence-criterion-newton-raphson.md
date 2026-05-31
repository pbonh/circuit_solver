---
title: "Dual Convergence Criterion for Newton-Raphson"
type: decision
tags: [decision, circuit-solver, numeric-solver, newton-raphson, convergence, dc-analysis, transient-analysis]
created: 2025-07-18
sources:
  - "openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0006-dual-convergence-criterion-newton-raphson.md"
confidence: high
---

"In the context of the numeric solver's Newton-Raphson iteration loop, facing the risk of false convergence where both Δv and ΔI appear small while the residue norm remains large (a stall condition caused by device-model derivative errors or near-singular Jacobians), we decided for a dual convergence criterion requiring both the update norm ‖Δx‖ and the residue norm ‖F(x)‖ to fall below their respective tolerances, and against the single-criterion alternatives (update-only or residue-only), to achieve reliable convergence detection that catches stalls and mis-reported convergence, accepting that the dual check adds one extra norm computation per iteration and may reject some iterations that update-only would have accepted."

## Status

accepted

## Architecturally Significant Requirement

Newton-Raphson convergence detection gates every DC operating-point and transient timestep solve. A false-positive convergence report produces silently wrong results that propagate through subsequent analyses. The [[concepts/newton-raphson-method]] pitfall calls out: "False convergence under the ΔI check: if NR stalls, Δv and ΔI both look small even though the residue is large."

## Options Considered

- **Update-only** — cheapest, but cannot detect stall.
- **Residue-only** — catches stall, but can miss oscillation between nearly-correct iterates.
- **Dual criterion (chosen)** — catches both stall and oscillation; strongest guarantee that F(x) = 0.

## Consequences

- Every converged OperatingPoint and transient timestep satisfies both KCL/KVL (residue) and update stability.
- Two tolerance parameters (`reltol`, `abstol`) must be configured.
- `ConvergenceStatus` must report both norms for diagnostics.

## Source

- OpenSpec ADR: `openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0006-dual-convergence-criterion-newton-raphson.md`
