---
title: Nyström Method
type: claim
id: claim-nystrom-method
tags:
- ode
- numerical-integration
- multistep
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

A Nyström method (Nyström 1925) is a [[concepts/linear-multistep-methods|linear multistep]] integrator that integrates f over *two* steps instead of one: y_{n+2} − y_n = h ∑_{j=0}^k β_j f_{n+j}. The simplest case (k = 1) is the explicit midpoint rule y_{n+2} = y_n + 2 h f_{n+1}; the Milne–Simpson rule is the implicit order-4 cousin y_{n+2} = y_n + h/3(f_n + 4 f_{n+1} + f_{n+2}).

## How It Works

The first characteristic polynomial is ρ(ζ) = ζ^2 − 1, with roots ζ = ±1. The extra root at ζ = −1 is a *spurious* root that lies exactly on the unit circle; tiny perturbations can drive it outside. This is "weak instability" (Dahlquist 1951): the method is zero-stable in the strict sense but its [[concepts/stability-region]] degenerates to a single point (explicit midpoint) or to an interval on the imaginary axis (Milne–Simpson, Hamming 1959). Used in practice only for special classes of problems (oscillatory Hamiltonian systems where the imaginary-axis stability matters), and for theoretical comparison with Adams and BDF families.

## Key Parameters

- Step count k.
- Spurious root distribution.
- Stability set (typically a single point or interval).

## When To Use

- Oscillatory Hamiltonian systems where imaginary-axis stability is required (Störmer–Verlet is a related second-derivative Nyström variant).
- Classroom-illustrative examples of weak instability.
- The starting / restart literature for multistep codes.

## Risks & Pitfalls

- Weakly unstable for general dissipative problems.
- Degenerate stability set is brittle — round-off can de-stabilise.
- Should not be used as the primary integrator on general nonstiff problems; Adams is preferred.

## Related Concepts

- [[concepts/linear-multistep-methods]]
- [[concepts/adams-method]]
- [[concepts/stability-region]]
- [[concepts/dahlquist-barrier]]
- [[concepts/symplectic-method]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
