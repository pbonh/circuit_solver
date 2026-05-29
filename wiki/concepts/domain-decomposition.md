---
title: Domain Decomposition
type: claim
id: claim-domain-decomposition
tags:
- algorithm
- linear-algebra
- vlsi
- well-established
- parallel
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt
confidence:
  base: 0.85
---

## Definition

Domain Decomposition (DD) is a divide-and-conquer strategy for solving large linear systems that partitions the underlying graph G = (V, E) into subgraphs G_i with a shared interface graph G_0. The system matrix takes an "arrowhead" block structure that admits parallel local solves followed by a small interface solve.

## How It Works

The block system has m diagonal blocks A_i (intra-subdomain), interface blocks E_i, F_i, and a central interface block A_0. Solving locally A_i x_i + E_i x_0 = b_i yields P_i = A_i^{-1} E_i and q_i = A_i^{-1} b_i. The reduced system (A_0 − F P) x_0 = b_0 − F q is solved on the small interface; local solutions x_i = q_i − P_i x_0 follow. Overlapping (Schwarz) variants allow partitions to overlap, trading communication for convergence speed; non-overlapping (Schur complement) variants minimize redundant work.

## Key Parameters

- Number and size of subdomains.
- Interface size.
- Overlap (Schwarz alternating method) vs. non-overlap.
- Subdomain solver choice.

## When To Use

- Massive power-grid analysis (millions of nodes) on parallel machines.
- Finite-element PDE solvers.
- Any structured linear system amenable to graph partitioning.

## Risks & Pitfalls

- Excessive subdomain count grows interface size and negates parallel benefit.
- Load imbalance among subdomains hurts parallel scaling.

## Related Concepts

- [[concepts/graph-partitioning]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/sparse-matrix]]
- [[concepts/power-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
