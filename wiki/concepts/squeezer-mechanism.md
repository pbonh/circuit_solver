---
title: Squeezer Mechanism
type: claim
id: concepts/squeezer-mechanism
tags:
- mechanical
- dae
- benchmark
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The Andrews squeezer mechanism is a 7-body planar multibody benchmark with 3 closed kinematic loops, 6 algebraic constraints, and a driving torque on one body. Originally proposed by P. E. Nikravesh / B. Andrews for testing multibody integrators, it is the canonical [[concepts/multibody-system]] benchmark in Hairer–Wanner VII.

## How It Works

The mechanism is a 2D linkage that compresses a workpiece when driven. The 7 bodies have ≈ 21 position coordinates; the 6 loop-closure constraints reduce the effective DOF. The chapter provides a complete Fortran reference implementation of M(q), f(q, q̇), g(q), and G(q), enabling head-to-head comparisons of DAE codes ([[entities/phem56]] half-explicit, [[entities/radau5]] IRK, [[entities/rodas]] Rosenbrock, [[entities/dassl]] BDF). Two variants are commonly run: the nonstiff baseline and a stiff variant with very stiff springs.

## Key Parameters

- 7 bodies, ≈ 21 generalised coordinates.
- 3 closed loops, 6 algebraic constraints.
- Stiff variant: spring constants O(10^4) or higher.

## When To Use

- Standardised comparison of DAE codes for multibody dynamics.
- Pedagogical example showing constraint handling, drift control, and method-of-choice for stiff vs. nonstiff.

## Risks & Pitfalls

- Half-explicit methods dominate the nonstiff variant; implicit codes are essential for stiff.
- Constraint drift over long simulations needs projection or Baumgarte stabilisation.
- Different DAE formulations (index 3 vs. index 2 vs. index 1) give different numerical behaviour for the same physical problem.

## Related Concepts

- [[concepts/multibody-system]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/index-3-dae]]
- [[concepts/half-explicit-method]]
- [[entities/phem56]]
- [[entities/radau5]]
- [[entities/rodas]]
- [[entities/dassl]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
