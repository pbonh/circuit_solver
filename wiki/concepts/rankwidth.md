---
title: Rankwidth
type: claim
id: claim-rankwidth
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
---

## Definition

The rankwidth of a graph G is the smallest k such that there is a carving (ternary tree-routed decomposition) of V(G) where, for every X ∈ C, the cut matrix (the submatrix of A(G) with rows in X and columns in V \ X) has GF[2]-rank ≤ k.

Distance-hereditary graphs are exactly the graphs of rankwidth ≤ 1.

## How It Works

Hliněný-Oum (2008) give a cubic FPT algorithm to test rankwidth ≤ k. Courcelle's theorem (MS1 version): MS1 problems are solvable in O(n^3) on graphs of bounded rankwidth, generalizing the bounded-treewidth result for MS2.

Lemma 4.72 shows graphs of rankwidth ≤ k are χ-bounded: χ ≤ 2^(k·ω) · 3^(ω-1), giving polynomial-time computation of χ in classes with bounded rankwidth and bounded clique number.

## Key Parameters

- rw(G).
- The carving is a ternary tree of subsets, internal nodes degree 3.
- rw ≤ 2^τ where τ is threshold-width.

## When To Use

- Generalizing treewidth-based algorithms to denser graph classes.
- For graphs that are "dense but structured" (distance-hereditary, cographs, threshold).

## Risks & Pitfalls

- Computing rw(G) is NP-complete in general; only FPT algorithms exist.
- Star routing trees would give rankwidth 1 to every graph, so internal nodes are restricted to degree 3.

## Related Concepts

- [[concepts/carving]]
- [[concepts/distance-hereditary-graph]]
- [[concepts/threshold-width]]
- [[concepts/courcelle-theorem]]
- [[concepts/chi-boundedness]]

## Sources

- [[summaries/guide-to-graph-algorithms-06-problem-formulations]]
- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
