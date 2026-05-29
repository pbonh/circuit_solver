---
title: Variable Ordering (BDD)
type: claim
id: claim-variable-ordering
tags:
- bdd
- foundational
- np-complete
- optimization
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt
confidence:
  base: 0.85
---

## Definition

The total order in which variables are introduced from BDD root to leaves. Variable ordering determines the size of an ROBDD (or DDD/GPDD) representing a given function — sometimes by exponential factors.

## How It Works

Static heuristics (interleaving, topological, FANIN/FANOUT-based, weight-based) pick an order at construction time. Dynamic reordering (Rudell's sifting algorithm) repeatedly slides each variable to its best position while observing BDD size. For DDD/GPDD, the order is over matrix entries or graph edges and is typically driven by topology (e.g., sparsity, node connectivity).

## Key Parameters

- Heuristic family (interleaving, weight-based, MOSFET-stage-based).
- Whether dynamic reordering is enabled.
- Cost function (node count, evaluation count).

## When To Use

- All BDD-based work; finding a good order is often the difference between a tractable and intractable problem.

## Risks & Pitfalls

- Finding the optimal order is NP-complete (Bollig and Wegener).
- Some functions (e.g., integer multiplier outputs) have no good order.
- Dynamic reordering is expensive during heavy operation streams.

## Related Concepts

- [[concepts/binary-decision-diagram]]
- [[concepts/robdd]]
- [[concepts/determinant-decision-diagram]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-08-4-determinant-decision-diagrams]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-09-5-ddd-implementation]]
