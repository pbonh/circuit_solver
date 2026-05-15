---
title: "Minty's Algorithm"
type: concept
tags: [graph, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

Minty's algorithm computes a maximum independent set α(G) in claw-free graphs by reducing the problem to a maximum-matching problem in an auxiliary graph (Edmonds' graph).

## How It Works

Given a maximal independent set B (black vertices), color other vertices white. Each white vertex has 1 or 2 black neighbors. Define "wings" (sets of white vertices sharing a unique pair of black neighbors), "regular" black vertices, "irregular" black vertices, "tipped wings."

For each pair s, t of white vertices with one black neighbor each, build Edmonds' graph: a matching with one edge per regular vertex (its two partition classes), plus edges s — t through irregular paths. An augmenting path in Edmonds' graph corresponds to an augmenting path in the original graph; the blossom algorithm finds it.

Total runtime: O(n^5). Faenza et al. improved to O(n^3).

## Key Parameters

- O(n^5) over all pairs of white vertices with single-black-neighbor.
- Edmonds' graph has 2N nodes for N regular vertices, plus s and t.

## When To Use

- Computing α on claw-free graphs (which generalize linegraphs, hence maximum matching).
- Demonstrating that maximum matching is polynomial.

## Risks & Pitfalls

- The "irregular paths" subroutine uses dynamic programming and careful bookkeeping.
- Computing ω in claw-free graphs is NP-complete (different problem); only α is polynomial.

## Related Concepts

- [[concepts/claw-free-graph]]
- [[concepts/edmonds-blossom-algorithm]]
- [[concepts/independent-set]]
- [[concepts/matching]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
