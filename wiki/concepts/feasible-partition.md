---
title: "Feasible Partition"
type: concept
tags: [graph, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

A partition {X, S, C} of V(G) is feasible if:
(a) G[C] is connected.
(b) S = N(C) and S separates X from C.
(c) Every vertex of X is adjacent to every vertex of S.

## How It Works

Feasible partitions decompose G into a "core" C, a "boundary" S, and a "complement" X with strong adjacency between X and S. They are used to find clique separators and to count minimal clique separators inductively.

The greedy procedure (Algorithm 5):
1. Pick a non-universal vertex R; initialize C = {R}, S = N(R), X = V \ N[C].
2. While ∃ y ∈ S with X ⊄ N(y), add y to C, update S and X.
3. Output {X, S, C}.

Runs in O(n^3) worst case.

## Key Parameters

- Sizes of X, S, C.
- The recursion solves the problem in O(n^4) for listing minimal clique separators.

## When To Use

- Whenever a structural decomposition of a graph into "well-connected" parts is needed.
- For proving lemmas about minimal separators by induction on the size of X.

## Risks & Pitfalls

- The choice of starting vertex R matters; choosing a universal vertex fails because no feasible partition exists for a clique.

## Related Concepts

- [[concepts/clique-separator]]
- [[concepts/minimal-separator]]
- [[concepts/separator]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
