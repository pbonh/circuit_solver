---
title: Layered Expansion Diagram (LED)
type: claim
id: claim-layered-expansion-diagram
tags:
- ddd
- bdd
- symbolic
- advanced
- implementation
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/04-1-introduction.txt
confidence:
  base: 0.65
---

## Definition

A Layered Expansion Diagram is a standalone (non-BDD-package-dependent) implementation of the Determinant Decision Diagram, organizing the determinant expansion into layers that correspond to matrix rows or columns and yielding a natural complexity analysis.

## How It Works

The expansion proceeds layer by layer, each layer corresponding to a row/column choice in Laplace-style cofactor expansion. Within a layer, nodes share children across the cofactor minors, achieving the same compression as a BDD-based DDD but without using an external BDD package. The layered structure makes the worst-case node count analyzable for dense matrices.

## Key Parameters

- Layer ordering (row vs. column, and the order within).
- Cofactor sharing/caching strategy.
- Sparse vs. dense matrix handling.

## When To Use

- Standalone DDD implementations where adding a BDD-package dependency is undesirable.
- Theoretical complexity analysis of DDD on dense matrices.

## Risks & Pitfalls

- Loses some of the very mature optimizations of dedicated BDD packages (dynamic reordering, parallel ops).
- Layer ordering critically determines compactness.

## Related Concepts

- [[concepts/determinant-decision-diagram]]
- [[concepts/binary-decision-diagram]]
- [[concepts/symbolic-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-04-1-introduction]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-09-5-ddd-implementation]]
