---
title: Forest
type: claim
id: concepts/forest
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A forest is a graph whose components are all trees; equivalently, a graph with no cycles. A forest with k components on n vertices has exactly n - k edges.

## How It Works

Forests are the disjoint-union generalization of trees. Algorithms that run on trees often generalize to forests with a single pass over each component. The feedback vertex set problem asks for the smallest vertex set whose removal yields a forest.

## Key Parameters

- Number of trees (components) k.
- Total vertices n; total edges n - k.

## When To Use

- As the "no cycle" relaxation of trees.
- As a building block in tree-of-cycles and related structural decompositions.

## Risks & Pitfalls

- Forests need not be connected; the empty graph (E = ∅) on n vertices is a forest with n components.

## Related Concepts

- [[concepts/tree]]
- [[concepts/feedback-vertex-set]]
- [[concepts/graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
