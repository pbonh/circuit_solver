---
title: "Linear Multistep Methods"
type: concept
tags: [numerical-integration, transient, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt"]
confidence: medium
---

## Definition

Linear multistep methods are a family of numerical integration formulas that compute the next time-step solution as a linear combination of values and derivatives at the current and several previous time steps. They are central to modern transient circuit simulation, where the network equations form an algebraic-differential system.

## How It Works

A general linear multistep formula has the form
    sum_{i=0..k} alpha_i x_{n-i} = h * sum_{i=0..k} beta_i f_{n-i}
where *h* is the step size, *x* the state, and *f = dx/dt*. Specific families include:
- Backward differentiation formulas (BDF / Gear), well suited to stiff systems.
- Adams-Bashforth (explicit) and Adams-Moulton (implicit) methods.
- The trapezoidal rule, a one-step implicit method also covered by Vlach and Singhal.

## Key Parameters

- Order *k* (number of past steps used).
- Step size *h* (controlled adaptively via local-truncation-error estimates).
- Stability properties (A-stability, stiff-stability).
- Implicit vs. explicit (implicit methods are required for stiff circuits).

## When To Use

- Transient analysis of circuits with widely separated time constants (stiff systems).
- Time-domain solution of algebraic-differential systems arising from modified-nodal or tableau formulations.

## Risks & Pitfalls

- Higher-order methods can be unstable for stiff problems unless A-stable or stiffly-stable.
- Variable-step variable-order implementations are required in practice but add complexity.
- Local-truncation-error estimates may be unreliable near rapid transitions.

## Related Concepts

- [[concepts/numerical-integration-odes]]
- [[concepts/algebraic-differential-equations]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/tableau-formulation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
- [[summaries/hairer-ode-ii-01-preface]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
