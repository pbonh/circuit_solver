---
title: Optimization Theory
type: claim
id: concepts/optimization-theory
tags:
- optimization
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Optimization theory deals with minimizing (or maximizing) a scalar objective function F(x) of n variables, optionally subject to equality and inequality constraints. In CAD design, the variables are element values, the objective is some performance measure (or its deviation from spec), and the constraints encode physical realizability and design boundaries.

## How It Works

Modern algorithms iteratively generate x^{k+1} = x^k + d^k s^k where s^k is a descent direction and d^k a step length. Convergence is monitored via F(x^{k+1}) < F(x^k). The two main algorithmic ingredients:
1. Search direction (steepest descent, conjugate gradient, quasi-Newton, full Newton).
2. Line search (golden section, quadratic interpolation, Armijo backtracking).

For constrained problems: penalty methods, augmented Lagrangian, sequential quadratic programming (SQP), interior-point methods. Vlach & Singhal highlight SQP as a "CAD-enabling" innovation.

## Key Parameters

- Number of variables n.
- Number of constraints.
- Choice of algorithm.
- Convergence tolerance.

## When To Use

- Circuit element-value selection to meet specifications.
- Many engineering, scientific, and operations-research problems.
- Machine learning training.

## Risks & Pitfalls

- Local minima trap descent algorithms in non-convex problems.
- Numerical conditioning affects convergence speed.
- Gradient inaccuracy (especially from finite differences) destabilizes algorithms.

## Related Concepts

- [[concepts/objective-function]]
- [[concepts/gradient]]
- [[concepts/quasi-newton-method]]
- [[concepts/sequential-quadratic-programming]]
- [[concepts/sensitivity-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory]]
- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
