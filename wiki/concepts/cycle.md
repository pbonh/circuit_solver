---
title: Cycle
type: claim
id: concepts/cycle
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

A cycle C in a graph G is an ordered set of at least three distinct vertices [c_1, …, c_t] such that consecutive pairs are adjacent and the first/last pair {c_1, c_t} is also an edge. The cycle on n vertices is C_n. A chord of a cycle is an edge of G with both endpoints on V(C) that is not one of the cycle edges. A chordless cycle (induced cycle) has no chord.

## How It Works

Cycles characterize many graph properties:
- A graph is a tree iff it is connected and has no cycle.
- A graph is bipartite iff all cycles are even.
- A graph is chordal iff it has no induced cycle of length ≥ 4.
- Augmenting cycles / odd cycles drive matching theory.

## Key Parameters

- Length |E(C)| = |V(C)| = t.
- Girth — length of the shortest cycle.

## When To Use

- Proving structural theorems (bipartiteness, chordality, planarity).
- Detecting cycles in dependency graphs (deadlocks, topological sort).

## Risks & Pitfalls

- The Kloks-Xiao text requires cycle length t ≥ 3; some texts allow length 2 multi-edge cycles in multi-graphs.
- "Circuit" in some chapters (e.g. on planar carving width) means an embedding of a cycle, distinguishing topology from combinatorics.

## Related Concepts

- [[concepts/graph]]
- [[concepts/path]]
- [[concepts/tree]]
- [[concepts/bipartite-graph]]
- [[concepts/chordal-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
