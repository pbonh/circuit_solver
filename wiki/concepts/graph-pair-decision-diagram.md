---
title: Graph-Pair Decision Diagram (GPDD)
type: claim
id: concepts/graph-pair-decision-diagram
tags:
- gpdd
- bdd
- graph
- symbolic
- advanced
- analog
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/00-preface.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A Graph-Pair Decision Diagram is a BDD-style structure that encodes the cancellation-free symbolic expansion of an admittance two-graph pair (V-graph and I-graph) into spanning-tree pairs, sharing common subexpressions across the enumeration.

## How It Works

GPDD construction extends the classical two-graph method (Mayeda) by formulating spanning-tree enumeration as recursive graph contraction/removal of edges, then encoding decisions as BDD-style branches. Because the underlying two-graph enumeration is cancellation-free, every path corresponds to a non-cancelling product term — making GPDD particularly economical for circuits where determinant cancellations are common.

## Key Parameters

- Edge variable ordering (the contraction/removal sequence).
- Graph-pair construction rules for controlled sources, nullors, etc.
- Choice between full numerator/denominator construction and combined transfer-function form.

## When To Use

- Symbolic transfer functions where DDD suffers from heavy cancellations.
- Cancellation-sensitive analyses (e.g., precise frequency response).
- Hierarchical analysis paired with DDD for matrix-friendly blocks.

## Risks & Pitfalls

- Spanning-tree enumeration explodes for dense graphs; ordering and contraction strategy matter.
- Implementation is more intricate than DDD because two graphs must be tracked synchronously.

## Related Concepts

- [[concepts/two-graph-method]]
- [[concepts/binary-decision-diagram]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/symbolic-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-00-preface]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-04-1-introduction]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-07-part-ii-methods]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-11-7-graph-pair-decision-diagram]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-12-8-hierarchical-analysis-methods]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
