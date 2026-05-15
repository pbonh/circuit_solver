---
title: "Carving Width"
type: concept
tags: [graph, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

The carving width of a graph G is the minimum over all carvings C of max_{X ∈ C} |δ(X)|, where δ(X) is the edge boundary of X. The p-carving width uses a weight function p : E → ℤ_{≥0} and replaces |δ(X)| with p(δ(X)) = ∑_{e ∈ δ(X)} p(e).

## How It Works

Theorem 4.33: for a connected planar graph G with p(δ(x)) < k for every vertex x, the p-carving width is ≥ k iff G has an antipodality of p-range ≥ k. The chapter develops a chain of equivalent characterizations: tilt → slope → antipodality, plus bond carvings.

Robertson-Seymour use carving width to approximate branchwidth and treewidth on planar graphs. There is an O(m^2) algorithm to decide antipodality.

## Key Parameters

- Carving width is at most n - 1 for connected G.
- Branchwidth ≤ carving width + 1 in some normalizations.
- Treewidth is at most (3/2) · branchwidth, so treewidth on planar graphs is (3/2)-approximable.

## When To Use

- Planar graph approximations for treewidth / branchwidth.
- VLSI routing problems.

## Risks & Pitfalls

- Carving width is different from branchwidth; they are related but not equal.
- For non-planar graphs the antipodality characterization may not hold.

## Related Concepts

- [[concepts/carving]]
- [[concepts/antipodality]]
- [[concepts/treewidth]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
