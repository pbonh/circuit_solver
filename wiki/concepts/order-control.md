---
title: "Order Control (Variable-Order Integration)"
type: concept
tags: [transient, numerical-integration, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt"]
confidence: medium
---

## Definition

Order control varies the integration order k during the simulation. Higher orders allow larger step sizes for smooth solutions; lower orders are more stable for sharp transitions and at start-up. Gear's BDF codes (1971) and modern DASSL/IDA use both step-size and order control.

## How It Works

The integrator compares estimated truncation errors at order k-1, k, and k+1:
1. If order k+1 gives smaller LTE / larger step: increase order.
2. If order k-1 gives smaller LTE / larger step: decrease order.
3. Otherwise stay at order k.

Start-up: begin with order 1 (backward Euler) for one step, since higher-order methods need history. Build up to the maximum order over the first several steps.

Maximum order: typically 5 or 6 for BDF (orders > 6 are not stiffly stable).

## Key Parameters

- Maximum allowed order.
- Order-change frequency (some codes change at most once per step).
- Cost of step-size changes (forces re-factoring at variable order).

## When To Use

- Production transient solvers.
- Long-time integration of stiff systems.

## Risks & Pitfalls

- Frequent order changes destabilize the integrator.
- Order increase too early in startup can break convergence.

## Related Concepts

- [[concepts/step-size-control]]
- [[concepts/gear-bdf]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
