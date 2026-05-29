---
title: Backward Error Analysis on Manifolds
type: claim
id: claim-backward-error-analysis-manifolds
tags:
- numerical-integration
- dae
- mechanical
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

Backward error analysis on manifolds extends [[concepts/backward-error-analysis]] to numerical methods for ODEs / DAEs constrained to a manifold. For a [[concepts/symplectic-integrator]] of order p applied to a [[concepts/constrained-hamiltonian-system]] (q' = H_p, p' = −H_q − G^T λ, g(q) = 0), the discrete map is shown to be the exact flow of a modified Hamiltonian H̃ on a *modified* manifold M̃, with both H̃ − H = O(h^p) and M̃ − M = O(h^p).

## How It Works

The modified Hamiltonian H̃ and modified manifold M̃ are constructed as formal power series in h. The key result is that on the modified manifold the modified energy H̃ is exactly conserved, so the true energy H oscillates around H̃ but does not drift secularly — explaining why symplectic methods like [[concepts/shake-algorithm]] / [[concepts/rattle-algorithm]] / [[concepts/lobatto-iiia-iiib-pair]] preserve constraints and energy almost exactly over astronomical / molecular-dynamics timescales (the perturbed [[concepts/kepler-problem]] Fig. 2.3 in Hairer–Wanner VII). Hairer–Lubich (1997), Reich (1999), and Leimkuhler–Reich (2004) developed the rigorous theory.

## Key Parameters

- Method order p.
- Modified Hamiltonian H̃ (formal series).
- Modified manifold M̃ (formal series).
- Truncation order of the series (exponentially small remainder over exponentially long times for analytic problems).

## When To Use

- Theoretical explanation of long-time energy near-conservation in constrained symplectic integration.
- Proving long-time stability of geometric integrators on constraint manifolds.
- Analysing structure-preserving discretisations of mechanical systems.

## Risks & Pitfalls

- Asymptotic only; for non-analytic problems the bounds degrade.
- Does not improve per-step accuracy.
- Requires the integrator to be genuinely symplectic-on-manifold; pseudo-symplectic methods do not enjoy the theorem.

## Related Concepts

- [[concepts/backward-error-analysis]]
- [[concepts/symplectic-method]]
- [[concepts/symplectic-integrator]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/manifold-differential-equation]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/shake-algorithm]]
- [[concepts/rattle-algorithm]]
- [[concepts/composition-method]]
- [[concepts/kepler-problem]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
