---
title: Complement
type: claim
id: concepts/complement
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The complement Ḡ of a graph G is the graph on the same vertex set V(G) whose edges are exactly the nonedges of G: E(Ḡ) = {{a, b} : {a, b} ⊆ V(G), {a, b} ∉ E(G)}.

## How It Works

Complementation is an involution (G = G̿) that swaps several graph invariants:
- α(G) = ω(Ḡ) (independent set size of G = clique number of Ḡ).
- Cographs are closed under complementation because P4 is self-complementary.
- Many classes are defined as "G and Ḡ both in class X" (e.g. permutation graphs are exactly graphs where G and Ḡ are both comparability graphs; threshold graphs are graphs where G and Ḡ are both trivially perfect).

## Key Parameters

- |E(Ḡ)| = n(n-1)/2 - |E(G)|.
- The Seidel matrix J - I - 2A relates A to the adjacency matrix of Ḡ.

## When To Use

- Reducing problems to / from their dual (e.g. clique to independent set).
- Verifying class closure under complementation (cographs, threshold, perfect).

## Risks & Pitfalls

- The complement of a connected graph may be disconnected and vice versa; this is the basis for the cotree decomposition.
- "Cocomponent" of G means a component of Ḡ.

## Related Concepts

- [[concepts/graph]]
- [[concepts/clique]]
- [[concepts/independent-set]]
- [[concepts/cograph]]
- [[concepts/perfect-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
