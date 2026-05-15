---
title: "Two-Graph Method"
type: concept
tags: [graph, symbolic, foundational, analog]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/00-preface.txt"]
confidence: high
---

## Definition

The two-graph method is a classical symbolic network-analysis technique that represents a circuit by a pair of graphs (a voltage graph and a current graph) such that the determinant of the admittance matrix equals a signed sum over spanning-tree pairs common to both graphs, with each tree pair contributing a single non-cancelling product term.

## How It Works

For an admittance-matrix network, every nonzero matrix entry maps to an edge in each of the two graphs (possibly with different orientation). The Binet–Cauchy formula gives the determinant as a sum of products of matching minors, which combinatorially corresponds to enumerating common spanning trees of the two graphs. Controlled sources, nullors, and other multi-terminal elements introduce different edge sets in each graph.

## Key Parameters

- Graph-construction rules per element type (R, C, L, VCCS, nullor, etc.).
- Enumeration / contraction order over edges.

## When To Use

- Cancellation-free symbolic generation for analog circuits.
- As a foundation for GPDD construction.
- Theoretical analyses where each product term must correspond to a physical signal path.

## Risks & Pitfalls

- Naïve enumeration is super-exponential in graph size; compaction via decision diagrams (GPDD) is what makes it practical.
- Graph construction for nonreciprocal or active elements is bookkeeping-heavy.

## Related Concepts

- [[concepts/graph-pair-decision-diagram]]
- [[concepts/symbolic-analysis]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/spanning-tree-enumeration]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-00-preface]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-04-1-introduction]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-11-7-graph-pair-decision-diagram]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-12-8-hierarchical-analysis-methods]]
