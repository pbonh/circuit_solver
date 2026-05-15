---
title: "Szemerédi's Regularity Lemma"
type: concept
tags: [graph, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

For any ε > 0 and t ∈ N there exist N, T ∈ N such that every graph with at least N vertices has an ε-regular partition {V_0, …, V_k} with t ≤ k ≤ T. An ε-regular partition is equitable (|V_i| = |V_j| for 1 ≤ i < j ≤ k), with |V_0| ≤ ε · n, and with all but at most ε·k^2 of the pairs {V_i, V_j} being ε-regular.

A pair {X, Y} is ε-regular if, for every X' ⊆ X and Y' ⊆ Y with |X'| ≥ ε|X| and |Y'| ≥ ε|Y|, |d(X', Y') - d(X, Y)| ≤ ε, where d(X, Y) = e(X, Y) / (|X| |Y|).

## How It Works

The standard proof starts with an arbitrary equitable partition and refines it whenever there are too many irregular pairs. Each refinement increases the index ∑ (|V_i| |V_j| / n^2) · d^2(V_i, V_j) by at least ε^5 / 4; since the index is bounded by 1/2, the process terminates in at most 2ε^(-5) iterations.

Alon et al. (1994) provide an O(M(n))-time constructive algorithm using neighborhood-deviation σ(p, q) and matrix multiplication. Deciding ε-regularity for given partition is co-NP-complete.

## Key Parameters

- ε: regularity tolerance.
- t: lower bound on the partition size.
- T: upper bound, growing as a tower function of 1/ε.

## When To Use

- Extremal combinatorics (Turán-type theorems, triangle-removal).
- Property testing for large graphs.
- Foundational tool for analyzing dense graphs at a "macroscopic" level.

## Risks & Pitfalls

- The tower-of-twos bound on T is not artificial: Gowers proved it is essentially tight.
- The lemma is essentially useless for sparse graphs (one needs sparse-regularity versions).

## Related Concepts

- [[concepts/graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
