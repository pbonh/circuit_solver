---
title: Differentiation Index
type: claim
id: concepts/differentiation-index
tags:
- dae
- classification
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

The differentiation index di of a DAE F(u', u) = 0 (Gear 1983, Gear–Petzold 1984) is the smallest non-negative integer m such that differentiating the system m times and combining with the original equations yields a system from which an explicit ODE u' = φ(u) can be extracted by algebraic manipulation alone (the "underlying ODE").

## How It Works

For a semi-explicit index-1 DAE y' = f(y, z), 0 = g(y, z) with g_z invertible, one differentiation of g gives g_y f + g_z z' = 0 ⇒ z' = −g_z^{−1} g_y f, completing the underlying ODE; di = 1. For an index-2 problem y' = f(y, z), 0 = g(y) with g_y f_z invertible, the first differentiation produces the [[concepts/hidden-constraint]] g_y f(y, z) = 0; a second differentiation yields the z-equation; di = 2. For index-3 [[concepts/constrained-mechanical-system]]s q' = u, M u' = f − G^T λ, g(q) = 0, three differentiations are needed (constraint → velocity-level → acceleration-level → λ-equation). The differentiation index gives the right diagnostic for choosing methods and for diagnosing solvability of the system.

## Key Parameters

- Number of differentiations required.
- Rank of the Jacobian chain (g_z, g_y f_z, g_y f_y f_z, …).

## When To Use

- Classifying DAEs by structural complexity.
- Determining how many [[concepts/index-reduction]] differentiations are needed.
- Diagnosing whether a method's convergence theorem applies (BDF / IRK have order-loss past index 1).

## Risks & Pitfalls

- The differentiation index can differ from the perturbation index — the two measure different aspects of well-posedness.
- For implicit DAEs F(u', u) = 0 the construction is more delicate; Campbell's [[concepts/derivative-array]] systematises it.
- Index reduction by differentiation propagates [[concepts/drift-off]] errors; pair with projection.

## Related Concepts

- [[concepts/index-of-a-dae]]
- [[concepts/perturbation-index]]
- [[concepts/index-reduction]]
- [[concepts/hidden-constraint]]
- [[concepts/derivative-array]]
- [[concepts/index-1-dae]]
- [[concepts/index-2-dae]]
- [[concepts/index-3-dae]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
