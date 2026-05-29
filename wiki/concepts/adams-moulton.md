---
title: Adams-Moulton Method
type: claim
id: claim-adams-moulton
tags:
- transient
- numerical-integration
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt
confidence:
  base: 0.65
---

## Definition

The Adams-Moulton method is a family of implicit linear multistep formulae for ODEs. The k-step Adams-Moulton formula uses past derivative values plus the current (unknown) derivative: x_{n+k} = x_{n+k-1} + h sum_{j=0..k} beta_j x'_{n+k-j}. The trapezoidal rule is the 1-step Adams-Moulton; the 2-step variant is x_{n+k} = x_{n+k-1} + (h/12)(5 x'_{n+k} + 8 x'_{n+k-1} - x'_{n+k-2}).

## How It Works

Higher-order Adams-Moulton has smaller truncation error than its explicit Adams-Bashforth counterpart but requires solving an implicit equation each step. The Adams-Moulton order is p = k + 1 (one higher than Adams-Bashforth of the same step count).

Used as correctors paired with Adams-Bashforth predictors in classical PECE (Predict-Evaluate-Correct-Evaluate) codes.

## Key Parameters

- k (number of past samples + current).
- Order p = k + 1.
- Step size h.

## When To Use

- Corrector stage in Adams predictor-corrector codes.
- Non-stiff ODE problems requiring high accuracy.

## Risks & Pitfalls

- Adams-Moulton beyond order 2 is not A-stable; BDF is preferred for stiff systems.
- Requires iterative solution of the implicit equation each step.

## Related Concepts

- [[concepts/adams-bashforth]]
- [[concepts/trapezoidal-rule]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
