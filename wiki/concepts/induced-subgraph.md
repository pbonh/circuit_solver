---
title: Induced Subgraph
type: claim
id: claim-induced-subgraph
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

Given a graph G and a nonempty set W ⊆ V(G), the subgraph of G induced by W, written G[W], is the graph with vertex set W and edges all pairs {a, b} ⊆ W that belong to E(G). A subgraph H of G is induced iff E(H) = {{a, b} : {a, b} ⊆ V(H), {a, b} ∈ E(G)}.

A spanning subgraph satisfies V(H) = V(G) and E(H) ⊆ E(G); a spanning tree is a spanning subgraph that is a tree.

## How It Works

Induced subgraphs preserve all edges between selected vertices, in contrast to spanning subgraphs which may omit some edges. The notion supports both forbidden-subgraph characterizations ("H-free graphs are those without H as an induced subgraph") and recursive decompositions (modular, cograph, chordal).

## Key Parameters

- Size of W.
- For an edge e = {x, y}, G - e denotes (V, E \ {e}); for a vertex x, G - x denotes G[V \ {x}].

## When To Use

- Forbidden-subgraph characterizations (claw-free, P4-free cographs, gem/house/hole/domino-free distance-hereditary, etc.).
- Recursion in dynamic-programming algorithms over decomposition trees.

## Risks & Pitfalls

- Confusing "subgraph" with "induced subgraph" leads to errors in characterizations (e.g. minor-closed vs. induced-subgraph-closed classes differ).
- Removing a vertex changes V; some algorithms maintain V and add an edge instead (contractions).

## Related Concepts

- [[concepts/graph]]
- [[concepts/spanning-tree]]
- [[concepts/minor]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
