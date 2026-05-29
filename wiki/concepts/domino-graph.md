---
title: Domino Graph
type: claim
id: concepts/domino-graph
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

A graph G is a domino if every vertex is in at most two maximal cliques. Equivalently (Theorem 2.105), G is {W_4, claw, gem}-free.

## How It Works

Dominoes are recognized in linear time. The representative graph R(G), obtained by collapsing closed-neighborhood equivalence classes, must be the linegraph of a triangle-free graph where every vertex is adjacent to at most one pendant. This gives a structural characterization that drives linear-time recognition.

Dominoes generalize linegraphs of bipartite graphs (which are exactly the (claw, K_4, K_4 - e)-free graphs). Every domino has at most n maximal cliques.

## Key Parameters

- Maximal cliques per vertex ≤ 2.
- Total maximal cliques ≤ n.

## When To Use

- Models where each vertex participates in at most two "cliques of conflict."
- Generalization of bipartite linegraphs in matching algorithms.

## Risks & Pitfalls

- Distinct from a "domino piece" (the 6-vertex graph that is the obstruction for distance-hereditary).
- Not all linegraphs are dominoes: the linegraph of the diamond is W_4 (not a domino).

## Related Concepts

- [[concepts/linegraph]]
- [[concepts/claw-free-graph]]
- [[concepts/maximal-clique]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
