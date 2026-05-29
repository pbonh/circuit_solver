---
title: Order of Integration and Truncation Error
type: claim
id: concepts/order-of-integration
tags:
- transient
- numerical-integration
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The order p of a numerical integration formula is the highest integer such that the formula exactly integrates polynomials of degree p. The truncation error coefficient c_{p+1} multiplies the leading omitted term in the Taylor expansion of the exact solution.

## How It Works

For a generic two-point LMS formula a_1 x_1 + a_0 x_0 - h (b_1 x'_1 + b_0 x'_0) = 0, Taylor expansion of x(t_0 + h) and x'(t_0 + h) about t_0 yields conditions on (a_0, a_1, b_0, b_1) for each derivative coefficient to vanish:
- a_1 + a_0 = 0 (zeroth derivative).
- a_1 - b_1 - b_0 = 0 (first derivative).
- a_1/2 - b_1 = 0 (second derivative, for order 2).
- a_1/6 - b_1/2 = 0 (third derivative, for order 3).

Forward and backward Euler satisfy the first two only (p=1, c_2 = -1/2 or +1/2). Trapezoidal satisfies the first three (p=2, c_3 = -1/12).

## Key Parameters

- Order p (number of Taylor coefficients matched).
- Truncation error c_{p+1} (leading error term).
- Stability properties (often inversely related to order in LMS methods).

## When To Use

- Choosing between integrators for a given accuracy requirement.
- Local truncation error estimation for adaptive step-size control.
- Theoretical analysis of integration methods.

## Risks & Pitfalls

- Higher order doesn't guarantee smaller error at finite h; absolute error depends on h^p.
- The Dahlquist barrier: no A-stable LMS method has order > 2.

## Related Concepts

- [[concepts/linear-multistep-methods]]
- [[concepts/a-stability]]
- [[concepts/trapezoidal-rule]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
