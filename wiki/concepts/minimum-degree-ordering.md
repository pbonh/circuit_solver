---
title: Minimum Degree Ordering
type: claim
id: concepts/minimum-degree-ordering
tags:
- sparse-matrix
- foundational
- well-established
- graph
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Minimum-degree ordering is a greedy heuristic for sparse-matrix reordering: at each step, the next pivot is the vertex with the fewest nonzero entries (smallest degree in the elimination graph). Vlach & Singhal call this the "minimum degree rule."

## How It Works

On the elimination graph of a structurally symmetric matrix:
1. Compute the degree of every remaining vertex (number of incident edges).
2. Choose the vertex of minimum degree as the next pivot.
3. Form the clique among its neighbors (representing fill-ins), then delete the vertex.
4. Repeat until all vertices are ordered.

The algorithm produces orderings nearly as good as the more expensive minimum-fill-in algorithm but at much lower computational cost. Most production sparse solvers use minimum degree (or its variants AMD, MMD).

## Key Parameters

- Tie-breaking rule among vertices of equal minimum degree.
- Approximate variants (AMD: approximate minimum degree) for speed.
- Symmetric-pattern assumption (matrix must be structurally symmetric).

## When To Use

- Default ordering choice in most sparse circuit-simulation packages.
- Whenever the matrix structure is fixed and many factor-and-solve cycles are anticipated.

## Risks & Pitfalls

- Heuristic — not globally optimal.
- Performance varies with tie-breaking; randomized tie-breaks can avoid pathological orderings.

## Related Concepts

- [[concepts/reordering]]
- [[concepts/minimum-fill-in]]
- [[concepts/fill-in]]
- [[concepts/elimination-graph]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
