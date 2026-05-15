---
title: "Simplicial Vertex"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A vertex x of a graph G is simplicial if its neighborhood N(x) is either empty or a clique. Equivalently, x is a "leaf" in the chordal-graph sense.

A perfect elimination order of G is an ordering [x_1, …, x_n] of V(G) such that each x_i is simplicial in G[x_i, …, x_n].

## How It Works

A graph is chordal iff it has a perfect elimination order (Corollary 4.5). Every chordal graph has a simplicial vertex (Lemma 4.4); inductively, chordal graphs are exactly those for which every induced subgraph has a simplicial vertex.

For trees, leaves are simplicial; for chordal graphs, simplicials are the chordal analog of leaves.

## Key Parameters

- A chordal graph that is not a clique has at least two simplicial vertices.
- The number of simplicials may be exponential (e.g. K_n has all simplicials).

## When To Use

- Linear-time chordal graph recognition (lex-BFS produces a perfect elimination order if one exists).
- Recursive algorithms: peel simplicials and decompose.

## Risks & Pitfalls

- A simplicial vertex must have a clique (not just an independent set) as neighborhood; isolated vertices count as simplicial.
- "Simple" elimination orders avoid taking simplicial vertices that are noses of bulls, midpoints of P_5, etc. — these are used in strongly chordal graphs.

## Related Concepts

- [[concepts/chordal-graph]]
- [[concepts/clique]]
- [[concepts/tree]]
- [[concepts/clique-tree]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
