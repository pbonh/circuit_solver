---
title: Baumgarte Stabilization
type: claim
id: claim-baumgarte-stabilization
tags:
- dae
- mechanical
- numerical-integration
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
---

## Definition

Baumgarte stabilisation (Baumgarte 1972) is the classical drift-control technique for index-reduced DAEs: instead of enforcing g̈(q) = 0 (the acceleration-level constraint after two differentiations), enforce g̈ + 2α ġ + β² g = 0 with α, β > 0. The constraint becomes a damped second-order ODE that drives g(q) → 0 exponentially.

## How It Works

For the [[concepts/constrained-mechanical-system]] q' = u, M u' = f − G^T λ, g(q) = 0, the index-1 acceleration-level equation G u' + Ġ u = 0 is replaced by G u' + Ġ u + 2α(G u) + β² g(q) = 0. The error g(q) and its rate ġ = G u now satisfy the damped second-order ODE g̈ + 2αġ + β² g = 0 with characteristic roots −α ± √(α² − β²). For α = β > 0 (critical damping), drift dies as e^{−α t}. Practical parameter choices balance: large α, β damp drift fast but stiffen the system; small α, β fail to control drift.

## Key Parameters

- Damping parameter α > 0.
- Stiffness parameter β > 0.
- Critical-damping condition α = β.
- Time scale 1/α for drift decay.

## When To Use

- Long-time multibody / mechanical-system simulations.
- Real-time / interactive simulations where projection is too expensive.
- Quick prototyping of index-reduction approaches.

## Risks & Pitfalls

- Too-large α, β stiffens the system, forcing the ODE solver to take smaller steps.
- Too-small α, β fails to damp drift in reasonable time.
- Stabilisation is asymptotic; instantaneous constraint violation is not eliminated, only driven to zero.
- Projection ([[concepts/projection-method-dae]]) is preferable for tight constraint accuracy.

## Related Concepts

- [[concepts/drift-off]]
- [[concepts/index-reduction]]
- [[concepts/projection-method-dae]]
- [[concepts/ggl-formulation]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/index-3-dae]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
