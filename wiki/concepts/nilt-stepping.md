---
title: "NILT Stepping (Time-Origin Reset)"
type: concept
tags: [transient, analog, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/13-chapter-10-numerical-laplace-transform-inversion.txt"]
confidence: medium
---

## Definition

NILT stepping extends the basic Vlach numerical Laplace transform inversion (which loses accuracy at large t) to long simulation intervals by resetting the time origin at each step and continuing with the previous state as initial condition. The stepped algorithm is equivalent to a very high-order absolutely-stable integration method.

## How It Works

At each step:
1. Use the current state vector as initial condition (in the Laplace-domain network equations).
2. Apply basic NILT to compute the state at time t + Delta t.
3. Take this as the new initial condition and repeat.

Initial conditions on capacitors and inductors are represented as equivalent independent sources in the Laplace domain (see Chapter 1 of Vlach & Singhal). Linear networks make this stepping straightforward because the system equations remain linear at each step.

The resulting algorithm has the stability properties of the underlying Pade approximation — typically very high order and A-stable when M, N are chosen well.

## Key Parameters

- Pade order (M, N) — chosen for accuracy.
- Step size Delta t — chosen for accuracy at large times.
- Initial condition representation method.

## When To Use

- Long-time transient simulation of linear networks.
- Distributed-element networks where direct time-stepping is infeasible.
- Time-domain sensitivity computation via adjoint method.

## Risks & Pitfalls

- More expensive per step than simple LMS methods (multiple complex frequency-domain solves).
- Restricted to linear networks (or piecewise-linear with re-formulation per piece).

## Related Concepts

- [[concepts/numerical-laplace-transform-inversion]]
- [[concepts/pade-approximation]]
- [[concepts/a-stability]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-13-chapter-10-numerical-laplace-transform-inversion]]
