---
title: Clique Separator
type: claim
id: claim-clique-separator
tags:
- graph
- algorithm
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.85
---

## Definition

A clique separator in a graph G is a set S ⊆ V which is either empty or a clique and whose removal disconnects G. A minimal clique separator is a clique separator that is also a minimal separator.

## How It Works

Whitesides gave an O(n^3) algorithm to find a clique cutset. The Kloks-Xiao variant lists all minimal clique separators in O(n^4) using feasible partitions and the lemma σ(G) < n (the number of minimal clique separators is < n).

Algorithm 5 computes a feasible partition {X, S, C} by initializing C = {R} for some non-universal vertex R and growing C with any y ∈ S that is not adjacent to all of X.

## Key Parameters

- σ(G): number of minimal clique separators, σ < n.
- O(n^4) algorithm to list all minimal clique separators.
- O(n^3) per feasible partition computation.

## When To Use

- Decomposing a graph for divide-and-conquer along clique separators.
- Computing treewidth (Bouchitté-Todinca framework relies on minimal separators).
- Recognition of structured graph classes (chordal: all minimal separators are clique separators).

## Risks & Pitfalls

- The number of clique separators (non-minimal) can be exponential (e.g. clique + path glued together).
- Disconnected graphs have ∅ as their (only) minimal clique separator.

## Related Concepts

- [[concepts/separator]]
- [[concepts/minimal-separator]]
- [[concepts/clique]]
- [[concepts/feasible-partition]]
- [[concepts/chordal-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
