---
title: "Error Growth Function"
type: concept
tags: [ode, numerical-integration, stability, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

Hairer–Zennaro's (1996) error growth functions φ_R(x) and φ_B(x) quantify how a numerical method amplifies linear (φ_R) and nonlinear (φ_B) perturbations as a function of the dimensionless step scaling x = h Re λ or x = h ν. They are the sharpest scalar bounds: ‖y_n − z_n‖ ≤ φ(nh)‖y_0 − z_0‖.

## How It Works

For the linear test equation y' = λ y, φ_R(x) = max{|R(z)|^n : Re z ≤ x, n h Re λ = x} — the worst-case linear amplification consistent with a given total dimensionless time. The nonlinear analogue φ_B replaces |R(z)| with the discrete contraction factor under [[concepts/one-sided-lipschitz-condition]]. Hairer–Zennaro proved both functions are *superexponential*: φ_R(x) ≤ e^x for A-stable methods and φ_B(x) ≤ e^x for B-stable methods, with equality only at x = 0 and the asymptotic decay rate matching that of the continuous flow.

## Key Parameters

- Dimensionless argument x = h Re λ (linear) or x = h ν (nonlinear).
- Method-specific [[concepts/stability-function]] R(z).
- One-sided Lipschitz constant ν.

## When To Use

- Sharp long-time error estimates on stiff problems.
- Comparing the contraction quality of different A-stable or B-stable methods.
- Theoretical analysis of asymptotic numerical stability.

## Risks & Pitfalls

- φ_R and φ_B can be expensive to compute exactly; bounds are often used in practice.
- Superexponential bounds imply asymptotic stability but not necessarily small constants for moderate n.

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/b-stability]]
- [[concepts/contractivity]]
- [[concepts/stability-function]]
- [[concepts/dahlquist-test-equation]]
- [[concepts/one-sided-lipschitz-condition]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
