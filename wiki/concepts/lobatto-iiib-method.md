---
title: Lobatto IIIB Method
type: claim
id: claim-lobatto-iiib-method
tags:
- ode
- numerical-integration
- runge-kutta
- symplectic
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

The s-stage Lobatto IIIB method is the discrete-derivative companion of [[concepts/lobatto-iiia-method]] on the same Lobatto nodes. It has classical order 2s − 2 and stage order s − 1, with a *singular* coefficient matrix A (first column zero), so it is not a [[concepts/collocation-method]] but rather defined by the dual / left-handed simplifying assumption D(s).

## How It Works

The singularity of A means the stage equations are not all "self-implicit": the first stage is essentially explicit and successive stages are coupled. This structure is what makes Lobatto IIIB the natural velocity-update partner of Lobatto IIIA in the [[concepts/lobatto-iiia-iiib-pair]] symplectic integrator for constrained mechanical systems: position stages use IIIA, velocity stages use IIIB, and the pair preserves the symplectic structure on the constraint manifold (Jay 1994/96). Lobatto IIIB is A-stable but not L-stable (R(∞) ≠ 0), not algebraically stable, and not [[concepts/b-convergence|B-convergent]] — Hairer–Wanner observe that the singular A leads to an unbounded local error, ruling out stiffness-uniform convergence.

## Key Parameters

- Number of stages s ≥ 2.
- Singular A (first column zero).
- Order 2s − 2, stage order s − 1.
- R(∞) = (−1)^{s−1}.

## When To Use

- Velocity-update step in symplectic Lobatto IIIA–IIIB integrators for constrained Hamiltonian systems.
- Theoretical analysis of partitioned RK pairs.

## Risks & Pitfalls

- A is singular; standard IRK convergence theorems do not apply directly.
- Cannot be used as a stand-alone stiff integrator — it is meant as a pair.
- Not B-convergent and not L-stable.

## Related Concepts

- [[concepts/lobatto-iiia-method]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/lobatto-iiic-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/symplectic-method]]
- [[concepts/constrained-hamiltonian-system]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
