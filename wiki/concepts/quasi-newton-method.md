---
title: Quasi-Newton Method (BFGS, DFP)
type: claim
id: concepts/quasi-newton-method
tags:
- optimization
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Quasi-Newton methods approximate Newton's method without explicitly computing the Hessian matrix. They iteratively update an approximation B^k to the inverse Hessian using only gradient differences between successive iterates. The most common variants are BFGS (Broyden-Fletcher-Goldfarb-Shanno) and DFP (Davidon-Fletcher-Powell).

## How It Works

Given x^k, gradient g^k, and approximate inverse Hessian B^k:
1. Search direction: s^k = -B^k g^k.
2. Line search: find d^k minimizing F(x^k + d^k s^k).
3. Update: x^{k+1} = x^k + d^k s^k.
4. Update B: B^{k+1} = update_rule(B^k, s^k, g^{k+1} - g^k).

BFGS is widely considered the most robust update. Superlinear convergence is achieved without forming or inverting the Hessian — only matrix-vector products and rank-1 or rank-2 updates per step.

## Key Parameters

- Initial approximation B^0 (typically identity).
- Line-search algorithm.
- Convergence tolerance on gradient norm.

## When To Use

- Unconstrained smooth nonlinear optimization.
- Building block for SQP and other constrained methods.
- Default choice when Newton is too expensive.

## Risks & Pitfalls

- L-BFGS variant for large n (only stores last m updates).
- BFGS update can fail with insufficient curvature; damped variants exist.
- Performance depends on objective scaling.

## Related Concepts

- [[concepts/optimization-theory]]
- [[concepts/sequential-quadratic-programming]]
- [[concepts/gradient]]
- [[concepts/line-search]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory]]
