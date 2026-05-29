---
title: Moving Finite Elements
type: claim
id: claim-moving-finite-elements
tags:
- pde
- dae
- adaptive
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.65
---

## Definition

Moving finite elements (Keith Miller and R. N. Miller 1981) is an adaptive PDE method that lets the mesh nodes themselves be solution variables: instead of fixed nodes x_i, the unknowns are (x_i(t), u_i(t)) — node positions plus solution values. Nodes migrate toward regions of high solution gradient, producing automatic mesh refinement.

## How It Works

The semi-discrete system is a [[concepts/quasilinear-dae]] of the form C(y) y' = f(y), where y bundles the node positions and values and C(y) is the mass matrix coupling them. C(y) becomes singular at points where two nodes attempt to coalesce or where the local gradient flattens — the classical pitfall of MFE. The Hairer–Wanner discussion uses MFE on [[concepts/burgers-equation]] as a model of how quasilinear-DAE solvers (LIMEX, Rosenbrock RODAS) cope with state-dependent mass matrices of varying rank. Regularisation strategies (Petzold, Carlson) add penalty / spring terms preventing node coalescence.

## Key Parameters

- Number of moving nodes.
- Spring / penalty regularisation coefficients.
- Singularity threshold for the mass matrix.

## When To Use

- Sharp-front PDE problems ([[concepts/burgers-equation]], reaction–diffusion fronts).
- Moving boundary problems (Stefan problems).
- Adaptive mesh refinement when remeshing is too expensive.

## Risks & Pitfalls

- Node coalescence causes singular C(y); regularisation is mandatory.
- Variable-rank mass matrices stress the underlying DAE solver.
- Higher-dimensional MFE (2D, 3D) is significantly more complex than 1D.

## Related Concepts

- [[concepts/quasilinear-dae]]
- [[concepts/burgers-equation]]
- [[concepts/method-of-lines]]
- [[concepts/differential-algebraic-equation]]
- [[concepts/linearly-implicit-euler]]
- [[entities/limex]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
