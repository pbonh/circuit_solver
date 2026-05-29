---
title: Baker's Method
type: claim
id: claim-bakers-method
tags:
- graph
- algorithm
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

Baker's method (Brenda Baker, 1994) is a PTAS framework for many NP-complete optimization problems on planar graphs. It partitions the vertices into layers by BFS distance from the outerface and removes one layer to leave k-outerplanar subgraphs, on which the problem is solvable in linear time (by Courcelle on bounded treewidth).

## How It Works

For a planar graph G and parameter k:
1. Compute layers L_1, L_2, … via BFS from the outerface.
2. For i ∈ [k], define G_i = G[∪_{j ≢ i (mod k)} L_j]. Each G_i is k-outerplanar.
3. Solve the optimization problem on each G_i using Courcelle / DP.
4. Combine the solutions; by pigeonhole, at least one G_i loses only a 1/k fraction of the optimum.

Result: approximation ratio (k - 1) / k for "subset" objectives like maximum independent set. Approximation can be made arbitrarily good by choosing larger k, at the cost of larger constant in the running time.

## Key Parameters

- k: the layer skip.
- Approximation ratio: (k - 1) / k.

## When To Use

- PTAS for independent set, vertex cover, dominating set, Hamiltonicity check on planar graphs.
- As a meta-algorithm for any MS2 minor-monotone problem on planar graphs.

## Risks & Pitfalls

- The constant hidden in O(f(k) · n) explodes with k.
- Baker's method is restricted to layered separation; non-layered methods (Klein-Plotkin-Rao) handle other minor-closed classes.

## Related Concepts

- [[concepts/k-outerplanar-graph]]
- [[concepts/courcelle-theorem]]
- [[concepts/treewidth]]
- [[concepts/independent-set]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
