---
title: "Line Search"
type: concept
tags: [optimization, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt"]
confidence: medium
---

## Definition

A line search determines the step length d in an optimization update x^{k+1} = x^k + d s^k, where s^k is a given descent direction. Methods range from simple backtracking to careful Wolfe-condition-based searches.

## How It Works

Approaches:
- Exact line search: find argmin_d F(x^k + d s^k). Expensive; rarely justified.
- Backtracking (Armijo): start with d = 1, halve until F(x^k + d s^k) < F(x^k) + alpha d s^k^T nabla F.
- Wolfe conditions: combine sufficient decrease with curvature condition for superlinear convergence in quasi-Newton.
- Golden section, bisection, quadratic/cubic interpolation: classical 1D minimization tools.

Most modern codes use a combination — backtracking with Wolfe conditions, sometimes augmented by safeguarded interpolation.

## Key Parameters

- Initial step length (often 1 for quasi-Newton).
- Decrease parameter alpha (typically 1e-4).
- Curvature parameter c (typically 0.9).
- Maximum number of trials.

## When To Use

- Any gradient-based optimization algorithm.
- Trust-region methods alternatively bound step size without explicit line search.

## Risks & Pitfalls

- Too aggressive line search can miss the basin of attraction.
- Too cautious line search slows progress.
- Non-smooth F breaks Wolfe conditions.

## Related Concepts

- [[concepts/optimization-theory]]
- [[concepts/quasi-newton-method]]
- [[concepts/steepest-descent]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory]]
