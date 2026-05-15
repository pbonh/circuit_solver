---
title: "Simplified Newton Iteration"
type: concept
tags: [ode, numerical-integration, stiff, solver, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

Simplified (or modified) Newton iteration for an implicit Runge–Kutta stage system uses a *frozen* Jacobian J̄ — typically J(y_n) computed once at the start of the step (or even once across multiple steps) — in place of the freshly-evaluated J(Y_i^{(k)}) at each iteration. The iteration becomes Y^{(k+1)} = Y^{(k)} − (I_s ⊗ I − h A ⊗ J̄)^{−1} G(Y^{(k)}), which lets the LU factorisation of the iteration matrix be reused.

## How It Works

For an s-stage IRK method on dim(y) = n, the full Newton iteration would require factoring an (s n) × (s n) matrix at every iteration. Reusing J̄ allows the LU to be computed once per step (or once per several steps); the iteration still converges quadratically near the solution provided h ν < α_0(A^{−1}) (the [[concepts/coercivity-coefficient]] bound), where ν is the [[concepts/one-sided-lipschitz-condition]] constant. Practical IRK codes (RADAU5, RODAS) combine: (i) starting values from the previous step's interpolation polynomial, (ii) eigendecomposition of A to diagonalise the block solve (one real + one complex n × n system instead of one 3n × 3n), (iii) convergence-monitoring tolerance based on contraction-factor estimates, and (iv) Jacobian update / refactorisation triggered only when the contraction factor degrades.

## Key Parameters

- Frozen Jacobian J̄.
- Iteration matrix (I − h γ J̄) (for SDIRK / Rosenbrock) or (I_s ⊗ I − h A ⊗ J̄) (for full IRK).
- Convergence tolerance / contraction-factor threshold.
- Maximum iterations before Jacobian refactor.

## When To Use

- Stiff IRK / SDIRK solvers where Jacobian evaluation is expensive.
- Long stiff integrations where J̄ changes slowly between steps.
- Any setting where reusing the LU is more economical than recomputing.

## Risks & Pitfalls

- The contraction factor degrades when J̄ drifts from the current J; refactor when it crosses a threshold.
- Without good starting values (extrapolated polynomial), the first iteration can fail and force a step rejection.
- For very stiff transients the iteration may diverge if h ν exceeds α_0(A^{−1}).

## Related Concepts

- [[concepts/implicit-runge-kutta]]
- [[concepts/coercivity-coefficient]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/rosenbrock-method]]
- [[concepts/w-method]]
- [[concepts/sdirk-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
