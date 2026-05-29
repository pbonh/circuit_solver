---
title: Differential-Algebraic Equation
type: claim
id: concepts/differential-algebraic-equation
tags:
- dae
- ode
- modeling
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

A differential-algebraic equation (DAE) is a system F(u', u, x) = 0 in which ∂F/∂u' is singular — some unknowns are constrained by algebraic relations rather than ODEs. Semi-explicit form: y' = f(y, z), 0 = g(y, z). General implicit form: F(u', u) = 0 with rank-deficient ∂F/∂u'. The structural classification is by [[concepts/index-of-a-dae]], measuring how far the constraints are from being explicit ODEs.

## How It Works

DAEs arise whenever a model combines dynamic laws with algebraic constraints (conservation, Kirchhoff laws, kinematic constraints). For linear constant-coefficient pencils B u' + A u = d, the [[concepts/weierstrass-kronecker-form]] decomposes the system into a regular ODE part and a nilpotent algebraic part of size = [[concepts/index-of-nilpotency]]. For semi-explicit nonlinear DAEs the [[concepts/differentiation-index]] (Gear–Petzold 1983/84) counts how many analytic differentiations of g are needed to express the constraints as ODEs. Index 1 (g_z invertible) is tractable by IRK / Rosenbrock / BDF directly; index 2 needs hidden-constraint analysis; index ≥ 3 (constrained mechanical systems) needs [[concepts/index-reduction]] plus stabilisation.

## Key Parameters

- Differentiation index di.
- Perturbation index pi (can differ from di — [[concepts/perturbation-index]]).
- Differential / algebraic variable counts.
- Sensitivity / drift-off rate under index reduction.

## When To Use

- Modelica / multibody dynamics / circuit simulation (modified nodal analysis).
- Constrained Lagrangian / Hamiltonian systems.
- Limit of [[concepts/singular-perturbation-problem]]s.
- Boundary-value problems and control problems with algebraic constraints.

## Risks & Pitfalls

- Wrong choice of method for the index causes order reduction or divergence; check the index first.
- Higher-index DAEs need [[concepts/index-reduction]], [[concepts/baumgarte-stabilization]], or [[concepts/projection-method-dae]] to avoid [[concepts/drift-off]].
- Consistent initialisation is non-trivial (consistent (y_0, z_0) must satisfy the hidden constraints, not just g(y_0, z_0) = 0).

## Related Concepts

- [[concepts/index-of-a-dae]]
- [[concepts/differentiation-index]]
- [[concepts/perturbation-index]]
- [[concepts/index-of-nilpotency]]
- [[concepts/index-1-dae]]
- [[concepts/index-2-dae]]
- [[concepts/index-3-dae]]
- [[concepts/singular-perturbation-problem]]
- [[concepts/weierstrass-kronecker-form]]
- [[concepts/algebraic-differential-equations]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
