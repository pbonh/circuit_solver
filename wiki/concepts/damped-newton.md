---
title: Damped Newton Method
type: claim
id: claim-damped-newton
tags:
- dc
- numerical
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/15-chapter-12-dc-solution-of-networks.txt
confidence:
  base: 0.65
---

## Definition

A damped (or relaxed) Newton-Raphson iteration multiplies the Newton step Delta x by a damping factor zeta in (0, 1] to ensure global convergence: x^{k+1} = x^k + zeta^k Delta x. The factor is chosen so the residual norm decreases: ||f(x^{k+1})|| < ||f(x^k)||.

## How It Works

A line-search procedure selects zeta^k by trying values (e.g., 1, 1/2, 1/4, ...) until the decrease condition is met. More sophisticated methods use the Armijo condition or backtracking line search. In circuit simulation, additional clamps may limit |delta V_BE| per iteration (e.g., to a few V_T) to prevent exponential overflow.

For very hard DC problems, source stepping (gradually ramping bias from 0 to its final value) or gmin stepping (adding tiny shunt conductances that decrease toward zero) supplements damping.

## Key Parameters

- Damping factor zeta (typically halved if not accepted).
- Per-device step limits (for diodes/transistors).
- Backtracking line-search parameters.

## When To Use

- DC analysis where pure Newton-Raphson diverges or oscillates.
- Globalization of any nonlinear solver.

## Risks & Pitfalls

- Damping slows convergence near the solution; should be turned off once quadratic convergence regime is reached.
- Over-aggressive damping can stall the iteration.

## Related Concepts

- [[concepts/newton-raphson-method]]
- [[concepts/source-stepping]]
- [[concepts/dc-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks]]
