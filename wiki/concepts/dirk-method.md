---
title: DIRK Method
type: claim
id: claim-dirk-method
tags:
- ode
- numerical-integration
- runge-kutta
- stiff
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

A Diagonally Implicit Runge–Kutta (DIRK) method has a lower-triangular Butcher matrix A with non-zero diagonal entries: a_{ij} = 0 for j > i, a_{ii} ≠ 0. Each stage Y_i is then determined by a scalar (n-dimensional) implicit equation involving only previously computed stages, avoiding the coupled (s n)-dimensional system of a fully implicit IRK.

## How It Works

DIRK methods reduce the per-step cost of an [[concepts/implicit-runge-kutta]] from one block (s n) × (s n) Newton solve to s sequential n × n Newton solves. If the diagonal entries differ, each stage needs its own LU factorisation; if they are all equal ([[concepts/sdirk-method]], "singly diagonal"), one LU per step suffices and the cost approaches that of backward Euler. DIRK families typically have low stage order (q = 1) and therefore suffer [[concepts/order-reduction]] on stiff problems; design effort centres on achieving high classical order (Crouzeix 1975, Nørsett 1974, Hairer–Wanner SDIRK4) while keeping the stability function L-stable and the error constants small.

## Key Parameters

- Number of stages s.
- Diagonal entries a_{ii} (equal for SDIRK).
- Classical order p, stage order q.
- Stability function R(z) — typically A-stable or L-stable.

## When To Use

- Stiff problems where fully implicit RK is too expensive but Rosenbrock methods are not desired.
- Codes that want one LU factorisation per step (then prefer SDIRK).
- Embedded pairs for error estimation with moderate implementation complexity.

## Risks & Pitfalls

- Low stage order causes severe order reduction on stiff and DAE problems.
- Distinct diagonals incur multiple LU factorisations per step.
- Not L-stable by default; check R(∞) in the chosen family.

## Related Concepts

- [[concepts/sdirk-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/runge-kutta-method]]
- [[concepts/order-reduction]]
- [[concepts/stiffly-accurate-method]]
- [[concepts/rosenbrock-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
