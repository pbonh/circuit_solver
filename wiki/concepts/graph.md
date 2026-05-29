---
title: Graph
type: claim
id: concepts/graph
tags:
- graph
- foundational
- well-established
- netlist
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

A graph G is an ordered pair (V, E) where V is a nonempty finite set of vertices (also called points) and E is a set of unordered pairs of vertices called edges (also called lines). Every edge has two endpoints; the endpoints of an edge are said to be incident with that edge, and two vertices that share an edge are adjacent (neighbors).

## How It Works

A graph captures a binary symmetric relation on a finite ground set: each vertex is an element, each edge is a related pair. The two pillars used to study graphs are local information (neighborhoods, degrees) and global structure (connectedness, components, separators, cycles). All graphs in the Kloks-Xiao text are finite, with V nonempty; the case V = ∅ is called a "null graph" and is excluded.

## Key Parameters

- n = |V| — number of vertices.
- m = |E| — number of edges.
- For each vertex x, the degree d(x) = |N(x)|.
- A graph is regular when all degrees are equal.

## When To Use

- As the basic data structure for any graph algorithm: BFS, DFS, shortest path, matching, flow, coloring.
- As the input model for circuit netlists, dependency graphs, social networks, and many combinatorial problems.

## Risks & Pitfalls

- Loops and multiple edges are excluded by the definition in this text (E is a set of unordered pairs of distinct vertices); some chapters explicitly relax this when working with dual graphs of planar embeddings.
- "Empty graph" means E = ∅ but V is nonempty; this is different from a "null graph" (V = ∅).

## Related Concepts

- [[concepts/adjacency-matrix]]
- [[concepts/neighborhood]]
- [[concepts/path]]
- [[concepts/cycle]]
- [[concepts/component]]
- [[concepts/clique]]
- [[concepts/independent-set]]
- [[concepts/graph-data-model]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/guide-to-graph-algorithms-01-preface]]
- [[summaries/guide-to-graph-algorithms-04-graphs]]
