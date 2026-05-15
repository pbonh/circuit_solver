---
title: "Drift-Off"
type: concept
tags: [dae, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

Drift-off is the numerical phenomenon in which the discrete solution of an index-reduced DAE wanders away from the original (higher-index) constraint manifold over time. For a [[concepts/constrained-mechanical-system]] reduced by differentiation to index 1, integrating g̈ = 0 numerically lets g(q(t)) grow as O(t^2) and G(q) u(t) grow as O(t) (Hairer–Wanner Theorem VII.2.1, Eq. 2.6).

## How It Works

The drift is essentially the double integration of round-off in the acceleration-level constraint: each step picks up a tiny error in g̈ ≈ 0, the position-level constraint integrates this twice, so position error grows quadratically. The Hairer–Wanner numerical experiments (Section VII.2) show g(q(t)) reaching O(10^{−5}) after a few seconds and worse over hours of simulated time. Remedies: [[concepts/baumgarte-stabilization]] (replace g̈ = 0 with g̈ + 2αġ + β²g = 0, damping the drift exponentially), [[concepts/projection-method-dae]] (after each step, project (q, u) back to the constraint manifold), [[concepts/ggl-formulation]] (extra multiplier maintains both position and velocity constraints), and [[concepts/overdetermined-dae]] formulations that solve all constraint levels by least squares. Numerical experiments show velocity-level projection alone is essentially as good as combined position + velocity projection.

## Key Parameters

- Drift rate in position level ≈ const · t^2.
- Drift rate in velocity level ≈ const · t.
- Integrator step size and accumulated round-off.

## When To Use

- Diagnosing why a constrained-mechanical-system simulation diverges from the constraint over time.
- Comparing index-reduction methods (Baumgarte vs. projection vs. GGL vs. overdetermined).
- Validating long-time integration accuracy.

## Risks & Pitfalls

- Untreated drift makes any naive index-reduction approach unsuitable for long-time integration.
- Baumgarte stabilisation with too-large α, β stiffens the system; too-small fails to damp.
- Projection adds computational cost but is robust.

## Related Concepts

- [[concepts/index-reduction]]
- [[concepts/baumgarte-stabilization]]
- [[concepts/projection-method-dae]]
- [[concepts/ggl-formulation]]
- [[concepts/overdetermined-dae]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/index-3-dae]]
- [[concepts/hidden-constraint]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
