---
title: Clique
type: claim
id: concepts/clique
tags:
- graph
- foundational
- well-established
- np-hard
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

A clique in a graph G is a nonempty set C ⊆ V such that every pair of vertices of C is adjacent in G. The clique number ω(G) is the maximal cardinality of a clique in G. The complete graph on n vertices K_n is itself a clique.

By complementarity, ω(G) = α(Ḡ). A triangle is a clique of size 3.

## How It Works

Cliques are dense substructures. Counting / finding cliques sub-cubically uses fast matrix multiplication: triangles in O(n^α), α < 2.376. The Bron-Kerbosch algorithm lists all maximal cliques in O(n^2 · 3^(n/3)).

The Moon-Moser bound shows any graph on n > 1 vertices has at most 3^(n/3) maximal cliques.

## Key Parameters

- ω(G).
- Number of maximal cliques (Moon-Moser bound).
- Edge clique cover number θ_e(G).

## When To Use

- As a structural feature in interval / chordal / cograph / threshold detection.
- As a complexity baseline: deciding ω ≥ k is NP-complete in general.

## Risks & Pitfalls

- "Maximal" (not contained in a larger clique) ≠ "maximum" (largest by size).
- Computing ω is NP-complete on general graphs, AT-free graphs, and triangle-free graphs (reduces to α).

## Related Concepts

- [[concepts/graph]]
- [[concepts/independent-set]]
- [[concepts/maximal-clique]]
- [[concepts/bron-kerbosch-algorithm]]
- [[concepts/edge-clique-cover]]
- [[concepts/clique-separator]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
