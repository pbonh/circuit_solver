---
title: "State-Space Model"
type: concept
tags: [analysis, simulation, well-established, linear-algebra]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt"]
confidence: medium
---

## Definition

A state-space model represents a linear time-invariant system as a first-order matrix system: ẋ = A x + B u and y = C x + D u, where x is the state vector, u is the input, y is the output, and A, B, C, D are constant matrices derived from the system's transfer function or directly from the underlying ODEs.

## How It Works

Transfer-function representation H(s) = N(s) / D(s) is converted to controllable, observable, or balanced state-space forms via standard algorithms (controllable-canonical, modal, balanced realization). Numerical integration libraries (LAPACK, LTITR in MATLAB) advance the state x(t) over a time mesh given input u(t). Compared with repeatedly resolving MNA each time step, state-space simulation amortizes matrix construction.

## Key Parameters

- State dimension (order of system).
- Realization form (canonical, balanced).
- Integration scheme.

## When To Use

- Time-domain simulation of LTI systems in control engineering.
- Linear circuit transient analysis after symbolic Laplace solve.
- Model order reduction (balanced truncation).

## Risks & Pitfalls

- Ill-conditioned realizations cause numerical instability.
- Non-LTI behavior requires piecewise-linear or hybrid extensions.

## Related Concepts

- [[concepts/laplace-transform-simulator]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/power-delivery-exploration]]

## Sources

- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
