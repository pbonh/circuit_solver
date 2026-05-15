---
title: "Bramble"
type: concept
tags: [graph, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A bramble B = {B_i} of a graph G is a set of subsets B_i ⊆ V such that:
1. Each B_i induces a connected subgraph of G.
2. Every pair B_i, B_j "touch": B_i ∩ B_j ≠ ∅, or there exist a ∈ B_i, b ∈ B_j with {a, b} ∈ E.

A hitting set for B is a set Z ⊆ V with Z ∩ B_i ≠ ∅ for all i. The order of B is the minimum hitting-set size. The bramble number b(G) is the maximum order over all brambles in G.

## How It Works

Seymour-Thomas (1993): tw(G) + 1 = b(G). Brambles serve as obstructions to small tree-decompositions: a bramble of order k forces tw ≥ k - 1.

For chordal graphs, b(G) = ω(G) by the Helly property of subtree intersections (Lemma 4.15).

## Key Parameters

- Order = min hitting-set size.
- Number of brambles can be exponential.

## When To Use

- Proving lower bounds on treewidth.
- As a "dual" certificate for tree-decomposition optimality.

## Risks & Pitfalls

- Brambles are originally called "screens" by Seymour-Thomas; the name was changed by Reed.
- Order computation is NP-complete in general.

## Related Concepts

- [[concepts/treewidth]]
- [[concepts/tree-decomposition]]
- [[concepts/chordal-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
