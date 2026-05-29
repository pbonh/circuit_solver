---
title: Symbolic Sensitivity Analysis
type: claim
id: claim-symbolic-sensitivity-analysis
tags:
- symbolic
- analog
- sensitivity
- advanced
- design-centering
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/11-7-graph-pair-decision-diagram.txt
confidence:
  base: 0.65
---

## Definition

Symbolic sensitivity analysis computes `∂H/∂p` of a transfer function `H(s, p1, p2, ...)` with respect to one or more circuit parameters `p_i` in closed form, exposing which parameters dominate variations in a chosen performance metric.

## How It Works

For a BDD-based symbolic engine the derivative is computed as the cofactor of the parameter symbol: `∂(BDD)/∂p = D_p` (the 1-cofactor) where `p` is the BDD variable. For GPDD the symbols are primitive device parameters so derivatives are direct cofactors with no chain-rule expansion. For DDD the symbols are composite MNA-matrix entries, so chain-rule expansion is needed and re-introduces cancellable pairs.

## Key Parameters

- Parameter variable selected (single or batch).
- Order parameter is placed in the BDD (affects derivative-DDD/GPDD size).

## When To Use

- Design centering and yield optimization.
- Ranking parameters by effect for tolerance budgeting.
- Process-variation-aware design.

## Risks & Pitfalls

- DDD-based sensitivity suffers from secondary cancellation; GPDD is preferred for sensitivity-heavy workflows.
- For nonlinear circuits, linearization assumptions must hold over the parameter range.

## Related Concepts

- [[concepts/graph-pair-decision-diagram]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/symbolic-cancellation]]
- [[concepts/process-variation]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-11-7-graph-pair-decision-diagram]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
