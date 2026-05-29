---
title: Multigrid Method
type: claim
id: claim-multigrid-method
tags:
- algorithm
- linear-algebra
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

The multigrid method is a hierarchical iterative algorithm for solving large linear systems (especially discretized PDEs and Laplacian systems) by combining three operations across a hierarchy of grids: smoothing (a few iterations of a basic relaxation method), restriction (coarsening), and prolongation (interpolation). Fedorenko (1960s) introduced the core idea; Brandt (1970s) and Hackbusch (1970s) formalized it.

## How It Works

On the finest grid, a smoothing step (e.g., Gauss-Seidel) damps high-frequency error. The residual is restricted to a coarser grid, where low-frequency error appears high-frequency and is damped again. Recursion descends to the coarsest grid (solved exactly), and prolongation interpolates corrections back up, with smoothing on each return. V-cycle, W-cycle, and F-cycle differ in the recursion pattern. Asymptotic complexity is O(|V|) per solve.

## Key Parameters

- Number of grid levels.
- Smoother (Gauss-Seidel, Jacobi, Chebyshev).
- Cycle type (V, W, F).
- Restriction and prolongation operators.

## When To Use

- Power grid IR drop analysis (PowerRush, etc.).
- Discretized elliptic PDEs.
- Preconditioner in Krylov methods.

## Risks & Pitfalls

- Geometric multigrid requires regular grids; AMG generalizes to irregular ones.
- Smoother choice affects convergence dramatically.
- V-cycle may fail to converge on hard problems; F-cycle is more robust but slower.

## Related Concepts

- [[concepts/algebraic-multigrid]]
- [[concepts/preconditioning]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/sparse-matrix]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
