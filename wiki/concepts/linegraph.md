---
title: Linegraph
type: claim
id: claim-linegraph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
confidence:
  base: 0.85
---

## Definition

The linegraph L(G) of a graph G (with at least one edge) has vertex set E(G); two edges of G are adjacent in L(G) iff they share an endpoint.

## How It Works

Linegraphs are always claw-free (no induced K_{1,3}), because three pairwise-non-adjacent edges sharing a common edge would not be possible. Harary lists nine forbidden induced subgraphs that characterize linegraphs.

Maximum matching in G corresponds to maximum independent set in L(G); this is the gateway by which Minty's algorithm computes maximum matchings via the wider class of claw-free graphs.

## Key Parameters

- |V(L(G))| = |E(G)|.
- L(K_n) is strongly regular and is called the triangle graph T(n).

## When To Use

- To reduce matching to independent-set on a claw-free graph.
- To study edge colorings as proper colorings of L(G) (chromatic index = χ(L(G))).
- To analyze edge dominating set as a dominating set in L(G).

## Risks & Pitfalls

- Linegraphs of bipartite graphs are dominoes; not all linegraphs are dominoes (the 4-wheel W4 is the linegraph of the diamond but is not a domino).
- The empty-graph case (E(G) = ∅) gives no vertices, so L(G) is undefined.

## Related Concepts

- [[concepts/graph]]
- [[concepts/claw-free-graph]]
- [[concepts/matching]]
- [[concepts/chromatic-index]]
- [[concepts/domino-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
