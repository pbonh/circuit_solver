---
title: Sequence of Expressions (SOE)
type: claim
id: concepts/sequence-of-expressions
tags:
- symbolic
- hierarchical
- analog
- foundational
- soe
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/12-8-hierarchical-analysis-methods.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A Sequence-of-Expressions is a nested chain of intermediate symbolic assignments `v_1 = f_1(...)`, `v_2 = f_2(v_1, ...)`, etc., that together compute a circuit transfer function. SOEs were the dominant representation in pre-BDD hierarchical symbolic analysis.

## How It Works

SOEs are produced by symbolic Gaussian / LU elimination, Coates / Mason flow-graph reductions, or RMNA-style block reduction. Each intermediate variable captures a partial result and may involve divisions. For special structures (sequential ladders), SOE can be division-free and directly mappable to a BDD; for general analog circuits divisions reappear.

## Key Parameters

- Pivot order during elimination.
- Whether divisions are deferred or applied immediately.
- Common-subexpression caching.

## When To Use

- Loosely-coupled or ladder-style topologies where SOEs are short.
- When numerical evaluation can tolerate division.

## Risks & Pitfalls

- Small-magnitude divisors propagate large numerical errors.
- Post-processing (e.g., sensitivity) lengthens expressions and worsens stability.
- Pole-zero / s-expansion is hard on SOE form.

## Related Concepts

- [[concepts/hierarchical-symbolic-analysis]]
- [[concepts/gaussian-elimination]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/graph-pair-decision-diagram]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-12-8-hierarchical-analysis-methods]]
