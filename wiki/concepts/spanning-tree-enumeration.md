---
title: Spanning-Tree Enumeration
type: claim
id: concepts/spanning-tree-enumeration
tags:
- graph
- foundational
- symbolic
- enumeration
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Spanning-tree enumeration lists all spanning trees of a connected graph. In symbolic analog analysis it provides product terms for the determinant (via the two-graph method) and forms the basis of topological symbolic analyzers.

## How It Works

Minty's 1965 algorithm makes binary In/Out decisions per edge. The compactable variant used for BDD/GPDD construction replaces "In/Out" with "edge-Collapse / edge-Remove", so the intermediate subgraphs shrink monotonically and can be compared/shared via a graph isomorphism canonical form. Termination conditions: only one node remains (tree complete), too few edges remain (impossible), or graph is disconnected (dead branch).

## Key Parameters

- Edge ordering (analog of BDD variable order).
- Subgraph canonicalization for sharing.

## When To Use

- Topological symbolic analysis (signed sum over spanning-tree pairs for `det(A)`).
- GPDD construction.
- Power-grid topology analyses, network reliability, etc.

## Risks & Pitfalls

- Number of spanning trees can be exponential.
- Subgraph isomorphism is hard in general; canonical-form heuristics are critical.

## Related Concepts

- [[concepts/two-graph-method]]
- [[concepts/graph-pair-decision-diagram]]
- [[concepts/binary-decision-diagram]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-11-7-graph-pair-decision-diagram]]
