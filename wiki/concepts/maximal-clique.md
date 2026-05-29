---
title: Maximal Clique
type: claim
id: concepts/maximal-clique
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A clique C in a graph G is maximal if no vertex outside C is adjacent to every vertex of C: ∀x ∉ C, ∃y ∈ C with {x, y} ∉ E. A maximum clique is the largest by cardinality.

## How It Works

Maximal cliques are listed by the Bron-Kerbosch algorithm. By Moon-Moser, the maximum number of maximal cliques in an n-vertex graph (n > 1) is:
- 3^(n/3) when n ≡ 0 (mod 3).
- 4 · 3^(⌊n/3⌋ - 1) when n ≡ 1 (mod 3).
- 2 · 3^(⌊n/3⌋) when n ≡ 2 (mod 3).

The extremal graphs are unions of triangles (with two edges or one edge as exceptions). Many graph classes have polynomially many maximal cliques: chordal graphs (≤ n), interval graphs, dominoes, AT-free graphs.

## Key Parameters

- Count of maximal cliques |Ω(G)|.
- ω(G) is the size of the maximum clique.

## When To Use

- Compact representation: a chordal graph's clique tree has the maximal cliques as nodes.
- Graph-class recognition (interval, chordal): scan maximal cliques in a consecutive arrangement.

## Risks & Pitfalls

- "Maximal" ≠ "maximum"; the net graph illustrates this with maximal cliques of different sizes.

## Related Concepts

- [[concepts/clique]]
- [[concepts/bron-kerbosch-algorithm]]
- [[concepts/clique-tree]]
- [[concepts/edge-clique-cover]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
