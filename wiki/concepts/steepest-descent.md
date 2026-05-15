---
title: "Steepest Descent"
type: concept
tags: [optimization, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt"]
confidence: medium
---

## Definition

The steepest-descent method is the simplest gradient-based optimization algorithm: search direction is the negative gradient s^k = -nabla F(x^k), with a line search to determine step length. Update: x^{k+1} = x^k - d^k nabla F(x^k).

## How It Works

At each iteration:
1. Compute the gradient g^k = nabla F(x^k).
2. Use s^k = -g^k as the search direction.
3. Line-search d^k.
4. Update x^{k+1} = x^k + d^k s^k.

Convergence is linear, with the rate depending on the condition number of the Hessian. For ill-conditioned problems (very elongated level sets), steepest descent zigzags slowly.

## Key Parameters

- Step length (line-search).
- Gradient tolerance.

## When To Use

- Educational introduction to gradient methods.
- When only the gradient is available and a robust simple method is sufficient.
- Stochastic gradient descent variants for very large machine-learning problems.

## Risks & Pitfalls

- Very slow on ill-conditioned problems (the classic "elephant" problem).
- Conjugate gradient and quasi-Newton are preferred for almost all practical applications.

## Related Concepts

- [[concepts/optimization-theory]]
- [[concepts/quasi-newton-method]]
- [[concepts/gradient]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory]]
