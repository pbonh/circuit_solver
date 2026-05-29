---
title: Minimum Spanning Tree (MST)
type: claim
id: claim-minimum-spanning-tree
tags:
- graph
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt
confidence:
  base: 0.85
---

## Definition

A minimum spanning tree (MST) of a weighted connected graph G = (V, E, w) is a spanning tree whose total edge weight is minimum over all spanning trees of G.

## How It Works

Three classic greedy algorithms produce an optimal MST:
- Borůvka's algorithm (1926): repeatedly add the minimum-weight external edge for each connected component; O(|E| log |V|).
- Prim/Jarník (1929-1957): grow a single tree by adding minimum-weight external edges; O(|E| log |V|) with binary heap, O(|E| + |V| log |V|) with Fibonacci heap.
- Kruskal (1956): sort edges and add the next smallest that does not create a cycle; O(|E| log |E|).
Advanced algorithms include Fredman-Tarjan O(|E| + |V| log* |V|) (1987), Chazelle's O(|E| α(|E|,|V|)) (1997), and Karger et al. expected O(|E|) randomized (1995).

## Key Parameters

- Number of vertices and edges.
- Edge-weight distribution.
- Sparse vs dense graph.

## When To Use

- Network design and infrastructure layout (telecom, transportation).
- Approximation algorithms for harder problems (e.g., 2-approximation of Steiner tree via metric closure).
- Clustering (single-link clustering uses MST).
- VLSI routing as a baseline before Steiner improvements.

## Risks & Pitfalls

- MST is not unique when ties in edge weights exist; downstream algorithms may need a tiebreaker.
- MST minimizes total weight, not depth or other costs; alternative spanning structures may be preferable for delay-driven routing.

## Related Concepts

- [[concepts/spanning-tree]]
- [[concepts/steiner-minimal-tree]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
