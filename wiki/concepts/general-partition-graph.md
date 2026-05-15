---
title: "General Partition Graph"
type: concept
tags: [graph, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A graph G is a general partition graph if there exists a set S and a map V → 2^S, x → S_x, such that:
1. {x, y} ∈ E ⇔ S_x ∩ S_y ≠ ∅.
2. S = ∪_x S_x.
3. For every maximal independent set M, {S_m : m ∈ M} is a partition of S.

Equivalently (Exercise 4.86), G has a clique cover C such that every maximal independent set hits every clique of C.

## How It Works

General partition graphs satisfy the "triangle condition": for every maximal independent set M and every edge {x, y} in G - M, there is m ∈ M with {x, y, m} a triangle in G.

Lemma 4.132: for any class with ω ≤ k, recognizing general partition graphs is polynomial. By the Graph Minor Theorem, every minor-closed class (e.g. planar) has bounded clique number, so general partition graph recognition is polynomial on each.

## Key Parameters

- Clique cover size and structure.
- ω(G) (the recognition is polynomial when ω is bounded).

## When To Use

- Identifying graphs with rich combinatorial structure.
- Test bed for "clique cover meets independent set" duality.

## Risks & Pitfalls

- Not every "triangle condition" graph is a general partition graph (e.g. AT-free C_5-style models).
- "Intolerable" cliques are those missed by some maximal independent set; they cannot be in any valid cover.

## Related Concepts

- [[concepts/independent-set]]
- [[concepts/clique]]
- [[concepts/edge-clique-cover]]
- [[concepts/at-free-graph]]
- [[concepts/graph-minor-theorem]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
