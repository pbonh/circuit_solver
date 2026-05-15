---
title: "Tournament"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A tournament is an orientation of a complete graph K_n: each pair of vertices has exactly one arc between them.

## How It Works

Fisher-Ryan: every tournament has a winning probability distribution w : V → [0, 1] with ∑ w(x) = 1 satisfying w(I(x)) ≥ w(O(x)) for all x ∈ V. Proof via Farkas's lemma on the payoff matrix K (skew-symmetric: k_{ij} = -1 if i → j, +1 if i ← j).

Sumner's conjecture: every tournament on 2(n-1) vertices contains every oriented tree on n vertices. El Sahili: 3(n-1) suffices.

Chudnovsky-Seymour: tournaments are well-quasi-ordered by strong immersion. Proof via codewords for cutwidth-k tournaments and Higman/Kruskal-style wqo.

Bousquet-Lochet-Thomassé: complete multi-digraphs whose arcs are a union of k quasi-orders have γ(T) = O(k^(k+2) · ln(2k)) (Erdős-Sands-Sauer-Woodrow conjecture).

## Key Parameters

- |V(T)| (tournament size).
- Cutwidth k bounds the structural complexity.

## When To Use

- Game-theoretic comparisons (paper-scissors-stone).
- Voting systems (Condorcet).
- Sorting and majority decisions.

## Risks & Pitfalls

- Tournaments need not be transitive; the absence of cycles (transitive tournament) is the strong assumption that makes domination trivial.
- Median orders ≠ transitive orders.

## Related Concepts

- [[concepts/dag]]
- [[concepts/median-order]]
- [[concepts/immersion]]
- [[concepts/strong-immersion]]
- [[concepts/dominating-set]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
