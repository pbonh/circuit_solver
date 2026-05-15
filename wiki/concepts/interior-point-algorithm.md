---
title: "Interior Point Algorithm"
type: concept
tags: [algorithm, optimization, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt"]
confidence: low
---

## Definition

The Interior Point Method (IPM) is a family of nonlinear-programming algorithms that solve constrained optimization problems by traversing the interior of the feasible region using a sequence of barrier-penalty-augmented subproblems. Modern variants (Karmarkar's algorithm, primal-dual interior-point) solve linear, quadratic, and conic programs in polynomial time.

## How It Works

A logarithmic barrier function added to the objective penalizes infeasibility (e.g., -μ Σ log(s_i) where s_i are slack variables for inequality constraints). The barrier parameter μ is reduced gradually as iterations progress, allowing the iterate to approach the constraint boundary. Newton steps are used to solve the resulting KKT system at each value of μ.

## Key Parameters

- Initial barrier parameter μ_0 and reduction schedule.
- Stopping tolerance.
- Choice of step-size and line search.

## When To Use

- Convex nonlinear constrained optimization (LP, QP, SOCP, SDP).
- Local refinement in non-convex problems after global search.
- Inner solver inside MATLAB Optimization Toolbox (used in Ch. 9 case study).

## Risks & Pitfalls

- Can converge to local minima on non-convex problems.
- Performance sensitive to scaling and conditioning.

## Related Concepts

- [[concepts/particle-swarm-optimization]]
- [[concepts/power-delivery-exploration]]
- [[concepts/voltage-regulator-placement]]

## Sources

- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
