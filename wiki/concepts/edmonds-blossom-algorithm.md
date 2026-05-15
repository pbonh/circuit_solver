---
title: "Edmonds' Blossom Algorithm"
type: concept
tags: [graph, algorithm, foundational, well-established, matching]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

Edmonds' blossom algorithm (1965) finds a maximum matching in a general (possibly non-bipartite) graph in polynomial time. It generalizes Berge's augmenting-path search by handling odd cycles ("blossoms") via contraction.

## How It Works

Starting from any matching M:
1. Search for an M-augmenting path P (alternating M / non-M edges, starting and ending at unmatched vertices).
2. When the search encounters a flower (alternating chain that returns to an even-position vertex), contract its blossom to a single vertex in G/B, continue the search.
3. If an augmenting path is found, flip M along P and repeat.
4. If no augmenting path exists, M is maximum.

Edmonds' original runtime: O(n^2 · m). Micali-Vazirani: O(√n · m). Faenza et al. improved max independent set in claw-free graphs (a generalization) to O(n^3).

## Key Parameters

- Number of augmenting paths is at most n/2.
- Each search runs in O(n + m) after blossom-contraction bookkeeping.

## When To Use

- Maximum matching in general graphs.
- As a sub-routine in Minty's algorithm for independent sets in claw-free graphs.

## Risks & Pitfalls

- Blossom contractions create multigraphs; careful bookkeeping is required to lift augmenting paths back to the original graph.
- Implementations are intricate; common bugs involve blossom expansion and end-of-path detection.

## Related Concepts

- [[concepts/matching]]
- [[concepts/claw-free-graph]]
- [[concepts/mintys-algorithm]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
