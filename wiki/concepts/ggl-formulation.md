---
title: "GGL Formulation"
type: concept
tags: [dae, mechanical, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The Gear–Gupta–Leimkuhler (GGL) formulation (Eq. 1.48 in Hairer–Wanner Chapter VII) augments the index-2 form of a [[concepts/constrained-mechanical-system]] with an extra Lagrange multiplier μ to keep *both* the position-level g(q) = 0 and the velocity-level G(q) u = 0 constraints exactly satisfied. The augmented system is q' = u + G^T μ, M u' = f − G^T λ, 0 = g(q), 0 = G u.

## How It Works

The system is index 2 — but unlike a plain index-2 reduction it has both position and velocity constraints, so the constraint manifold {(q, u) : g(q) = 0, G u = 0} is exactly preserved by the continuous flow and (with proper discretisation) by the numerical scheme too. The extra multiplier μ is non-zero only when discretisation error would otherwise push the state off the position manifold; in exact arithmetic μ = 0. GGL avoids [[concepts/drift-off]] without the parameter-tuning of [[concepts/baumgarte-stabilization]] or the post-step cost of [[concepts/projection-method-dae]] — it is closer in spirit to projection but uses an extra differential variable instead of a separate projection step.

## Key Parameters

- Original multiplier λ (constraint force).
- GGL multiplier μ (position-correction).
- Solve cost: an extra block in the augmented linear system.

## When To Use

- Multibody dynamics where drift is unacceptable and explicit projection is awkward.
- Long-time integration with mid-range accuracy requirements.
- Compatibility with index-2 BDF / RK convergence theory.

## Risks & Pitfalls

- Larger augmented system per step than plain index-2; cost per step grows by ≈ rank(G).
- The hidden constraint G u = 0 must be enforced exactly — and is, by construction; this is what makes GGL drift-free.

## Related Concepts

- [[concepts/drift-off]]
- [[concepts/index-reduction]]
- [[concepts/baumgarte-stabilization]]
- [[concepts/projection-method-dae]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/index-2-dae]]
- [[concepts/overdetermined-dae]]
- [[concepts/lagrange-multiplier]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
