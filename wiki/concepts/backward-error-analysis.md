---
title: "Backward Error Analysis"
type: concept
tags: [numerical-integration, mathematical-tool, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

Backward error analysis (BEA) of a numerical ODE integrator views the discrete map y_{n+1} = Φ_h(y_n) as the *exact* flow of a *modified* differential equation y' = f̃_h(y), with f̃_h = f + h f_1 + h² f_2 + … a formal series in h. Instead of bounding the *forward* error y_n − y(x_n) directly, BEA bounds the modified field f̃_h − f and uses continuous-dynamical-systems theory on the modified system.

## How It Works

For a method of order p, f̃_h = f + h^p f_p + h^{p+1} f_{p+1} + … Solving y' = f̃_h gives a continuous trajectory that the discrete iterates lie on *exactly* (in formal-power-series sense; truncation gives exponentially small remainders). For a [[concepts/symplectic-method]] applied to a Hamiltonian system, f̃_h is *itself* Hamiltonian — the discrete map is the exact flow of a modified Hamiltonian H̃ = H + O(h^p). This is the deep reason symplectic integrators conserve energy nearly exactly: they exactly conserve a *modified* energy that differs from the true H by O(h^p). Benettin–Giorgilli (1994), Hairer–Lubich (1997), and Reich (1999) made BEA rigorous for symplectic and reversible methods.

## Key Parameters

- Order p of the method.
- Modified field f̃_h or modified Hamiltonian H̃.
- Truncation order N of the formal series (optimal N depends on h).

## When To Use

- Explaining long-time conservation of energy / momentum / phase-space volume by symplectic methods.
- Theoretical analysis of geometric integrators.
- Proving long-time stability of structure-preserving discretisations.

## Risks & Pitfalls

- The modified field is a *formal* series — convergence is generally only asymptotic, with exponentially small remainder over exponentially long times for analytic problems.
- BEA does not by itself improve the per-step accuracy; it explains the *qualitative* long-time behaviour.
- For non-symplectic methods, BEA gives a modified equation but not a modified Hamiltonian; energy will drift.

## Related Concepts

- [[concepts/backward-error-analysis-manifolds]]
- [[concepts/symplectic-method]]
- [[concepts/symplectic-integrator]]
- [[concepts/kepler-problem]]
- [[concepts/composition-method]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
