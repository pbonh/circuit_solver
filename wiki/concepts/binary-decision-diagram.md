---
title: Binary Decision Diagram (BDD)
type: claim
id: claim-binary-decision-diagram
tags:
- bdd
- foundational
- graph
- symbolic
- data-structure
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/00-preface.txt
confidence:
  base: 0.85
---

## Definition

A Binary Decision Diagram is a rooted, directed acyclic graph that represents a Boolean function (and, by extension, set-valued or term-valued functions) as a sequence of variable decisions ending in 0/1 (or term) leaves. Reduced Ordered BDDs (ROBDDs) are canonical under a fixed variable order.

## How It Works

Each non-terminal node carries a variable and two outgoing edges (then/else). Two reduction rules — merging isomorphic subgraphs and removing redundant nodes — make ROBDDs canonical and compact. Operations like AND, OR, EXISTS, and substitution are implemented by recursive traversal with memoization (caching by node pointer pairs). Zero-suppressed BDDs (ZBDDs) suppress nodes whose then-edge is the 0 leaf, which is well suited for representing sparse sets of product terms (as needed by symbolic analysis).

## Key Parameters

- Variable order — critical to BDD size; bad orders can cause exponential blow-up.
- Reduction rules (standard vs. zero-suppression).
- Memoization cache size.

## When To Use

- Logic synthesis and verification (the original application).
- Symbolic circuit analysis: as the substrate for DDDs, GPDDs, and other graph-based term representations.
- Compact representation of large sets of monomials.

## Risks & Pitfalls

- Variable ordering can make or break performance; finding a good order is itself NP-hard.
- Memory scales with the number of unique nodes, not the number of represented terms — but a bad order undoes this advantage.

## Related Concepts

- [[concepts/determinant-decision-diagram]]
- [[concepts/graph-pair-decision-diagram]]
- [[concepts/zero-suppressed-bdd]]
- [[concepts/symbolic-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-00-preface]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-03-part-i-fundamentals]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-04-1-introduction]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-08-4-determinant-decision-diagrams]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-09-5-ddd-implementation]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-11-7-graph-pair-decision-diagram]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-15-10-symbolic-moment-computation]]
