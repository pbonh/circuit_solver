---
title: "Zero-Suppressed BDD (ZBDD)"
type: concept
tags: [bdd, zbdd, foundational, data-structure, sparse]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt"]
confidence: high
---

## Definition

A Zero-suppressed BDD is a BDD variant in which a node whose solid (then) edge points to the 0 terminal is removed and incoming edges are redirected to its dashed (else) child. ZBDDs are well suited to representing sparse subsets of a universe — and, in algebraic BDDs, to discarding product terms multiplied by zero factors.

## How It Works

Introduced by Minato (1993) for subset-system representation. The reduction rule differs from standard BDD: in ROBDD a node is removed when its two children are equal; in ZBDD a node is removed when its then-child is 0. Operations (subset union, intersection, sub-set, multiplication-style products) are defined recursively and memoized.

## Key Parameters

- Variable order.
- Coupling with arithmetic BDDs: zero-suppression is the algebraic analog of don't-care suppression.

## When To Use

- Combinatorial set families (covers, paths, spanning trees).
- DDD and GPDD construction: solid arrow to 0 means the contribution is multiplied by zero and is dropped.

## Risks & Pitfalls

- Mixing ROBDD and ZBDD nodes naively yields incorrect results; the package must be ZBDD-consistent.
- Variable order still matters.

## Related Concepts

- [[concepts/binary-decision-diagram]]
- [[concepts/robdd]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/graph-pair-decision-diagram]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-08-4-determinant-decision-diagrams]]
