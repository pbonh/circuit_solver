---
title: "Minimum Local Fill-In Ordering"
type: concept
tags: [sparse-matrix, foundational, well-established, graph]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt"]
confidence: medium
---

## Definition

Minimum local fill-in (also Markowitz, for asymmetric matrices) ordering chooses, at each step, the vertex whose elimination would create the fewest new fill-in edges. It generally yields better orderings than minimum-degree but is more expensive to compute.

## How It Works

For each candidate pivot vertex k, count f_k = number of fill-ins that would be introduced if k were eliminated next (this equals the number of edges added to the clique of k's neighbors). Choose k with minimum f_k. When a fill-in occurs, update f-counts for affected vertices by the bookkeeping rules in Section 2.8 of Vlach & Singhal (subtractions and additions based on adjacency overlaps).

Berry (1971) gave the original implementation. Wing and Huang (1975, used in Appendix E of the textbook) introduced fill-information updating that makes the algorithm competitive in speed with minimum-degree.

## Key Parameters

- Fill counts f_i maintained for every remaining vertex.
- Update rules upon insertion of a fill-edge between i and j.
- Tie-breaking rule.

## When To Use

- When the same matrix structure is factored many times and the additional ordering cost is amortized.
- When ordering quality matters more than ordering speed.

## Risks & Pitfalls

- Significantly more bookkeeping than minimum-degree.
- Without fill-information updating, the algorithm is orders of magnitude slower than minimum-degree.

## Related Concepts

- [[concepts/minimum-degree-ordering]]
- [[concepts/reordering]]
- [[concepts/fill-in]]
- [[concepts/elimination-graph]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
- [[summaries/computer-methods-circuit-analysis-design-25-appendix-e-sparse-matrix-solver]]
