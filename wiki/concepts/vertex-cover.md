---
title: Vertex Cover
type: claim
id: claim-vertex-cover
tags:
- graph
- foundational
- well-established
- np-hard
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.85
---

## Definition

A vertex cover of a graph G is a set S ⊆ V such that every edge has at least one endpoint in S. Equivalently, V \ S is an independent set. The vertex cover number τ(G) = n - α(G).

## How It Works

Vertex cover is NP-complete in general but is the textbook FPT example: a search tree of depth k that branches on both endpoints of any uncovered edge solves the problem in O(2^k · |G|).

The fastest known parameterized algorithm runs in O(1.2738^k · n^O(1)) (Chen-Kanj-Xia 2010). The 2-approximation by König-Egerváry / matching gives polynomial-time τ(G) on bipartite graphs and a factor-2 approximation on general graphs.

## Key Parameters

- τ(G) = n - α(G).
- FPT parameter k = solution size.
- Optimal kernel size: 2k - O(log k) by Bondy et al.

## When To Use

- Standard parameterized algorithm benchmark.
- Approximation: factor-2 by LP rounding or maximum matching.
- Reduction target for many NP-completeness proofs.

## Risks & Pitfalls

- Vertex cover and dominating set are different: a vertex cover covers all edges; a dominating set covers all vertices.
- Approximating below 1.36 is hard under UGC.

## Related Concepts

- [[concepts/independent-set]]
- [[concepts/fixed-parameter-tractability]]
- [[concepts/matching]]
- [[concepts/edge-dominating-set]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
