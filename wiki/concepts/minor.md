---
title: Minor
type: claim
id: concepts/minor
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A graph H is a minor of a graph G if H can be obtained from G by a sequence of vertex deletions, edge deletions, and edge contractions. Equivalently, V(G) partitions into branch sets V_1, …, V_h with V(H) = [h] such that each G[V_i] is connected and every edge {i, j} ∈ E(H) is realized by some edge between V_i and V_j in G.

## How It Works

The minor relation is a well-quasi-order on graphs (Graph Minor Theorem). Minor-closed classes have finite obstruction sets:
- Planar graphs: {K_5, K_{3,3}}.
- Outerplanar graphs: {K_4, K_{2,3}}.
- Treewidth ≤ k graphs: finite obstruction set T_k.
- K_t-minor-free graphs: directly defined by excluding K_t.

Whether a fixed H is a minor of G is in O(n^3) (Robertson-Seymour). Minor containment H ≤_min G can be checked in MS1 (the property is expressible in monadic second-order logic).

## Key Parameters

- Hadwiger number: max h such that K_h is a minor of G.
- Hadwiger's conjecture: graphs without K_t-minor are (t - 1)-colorable.

## When To Use

- Structural graph theory.
- Defining graph classes by exclusion (planar, K_4-minor-free, etc.).

## Risks & Pitfalls

- "Topological minor" is a stricter relation (subdivision, no contractions); the classes differ.
- Edge contractions can create new cycles, multi-edges, and loops in the minor — different texts handle this differently.

## Related Concepts

- [[concepts/graph-minor-theorem]]
- [[concepts/topological-minor]]
- [[concepts/well-quasi-order]]
- [[concepts/outerplanar-graph]]
- [[concepts/treewidth]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
