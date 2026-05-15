---
title: "Clustered Coloring"
type: concept
tags: [graph, algorithm, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A cluster coloring of G with k colors and cluster number c is a map f : V → [k] such that each monochromatic subgraph has every connected component of size ≤ c. The class G of graphs has cluster chromatic number k if there is c ∈ ℕ such that all G ∈ G can be cluster-colored with k colors and cluster number c.

A defective coloring uses k colors and defect d: each monochromatic component has max degree ≤ d.

## How It Works

Van den Heuvel-Wood (2018) Theorem 4.88: every K_t-minor-free graph admits a (2t - 2)-cluster coloring with cluster number ⌈(t-2)/2⌉.

Proof tools include:
- BFS-trees with few leaves yielding bandwidth ≤ k - 1 bounds.
- A "connected partition" greedily built so that each block has bounded degree and 2-coloring with small cluster.
- The recursion uses Lemma 4.91 / 4.92 to span A by a minimal connected subgraph with cutvertices and bandwidth control.

## Key Parameters

- k (number of colors).
- c (cluster size).
- t (forbidden minor K_t).

## When To Use

- Hadwiger-conjecture-relaxed colorings.
- Distributed coloring with locality constraints.

## Risks & Pitfalls

- Cluster coloring is weaker than proper coloring; many problems become tractable but lose strict properness.
- Liu-Oum: cluster chromatic number of K_t-minor-free graphs is at most 3(t - 1) but cluster sizes are very large.

## Related Concepts

- [[concepts/chromatic-number]]
- [[concepts/minor]]
- [[concepts/graph-minor-theorem]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
