---
title: Intersection Graph
type: claim
id: concepts/intersection-graph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A graph G is the intersection graph of a family {S_x : x ∈ V(G)} of sets if V(G) is the index set and {x, y} ∈ E(G) iff S_x ∩ S_y ≠ ∅. Every graph is the intersection graph of some set system (Exercise 4.33 in Kloks-Xiao: use maximal cliques as the universe).

## How It Works

Many graph classes are defined as intersection graphs of structured set systems:
- Interval graphs: intervals on a line.
- Chordal graphs: subtrees of a tree.
- Circle graphs: chords of a circle.
- Permutation graphs: line segments between two parallel lines.
- Boxicity-d graphs: axis-parallel boxes in d-space.

The edge clique cover number θ_e(G) equals the minimum size of a universe such that G has an intersection representation.

## Key Parameters

- Size of the universe.
- Structure of the set system (intervals, subtrees, etc.).

## When To Use

- Modeling overlap relations in scheduling, layout, biology.
- Recognition of structured graph classes.

## Risks & Pitfalls

- Every graph is an intersection graph of SOME system; the class is characterized by the structure of the sets.
- Boxicity is NP-complete to compute.

## Related Concepts

- [[concepts/interval-graph]]
- [[concepts/chordal-graph]]
- [[concepts/circle-graph]]
- [[concepts/permutation-graph]]
- [[concepts/edge-clique-cover]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
