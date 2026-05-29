---
title: Algebraic-Differential Equations (DAEs)
type: claim
id: claim-algebraic-differential-equations
tags:
- foundational
- transient
- numerical-integration
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt
confidence:
  base: 0.85
---

## Definition

Algebraic-differential equations (also called differential-algebraic equations or DAEs) combine differential equations dx/dt = f(x, y, t) with algebraic constraints g(x, y, t) = 0. Modified-nodal and tableau formulations of circuits naturally produce DAEs: the algebraic equations come from KCL/KVL for voltage sources and ideal elements, while the differential equations come from capacitors and inductors.

## How It Works

A general DAE has the form F(x', x, t) = 0. The index of a DAE is the number of differentiations of the algebraic constraints required to convert the system to pure ODEs. Index-1 DAEs (most circuit problems after careful formulation) are tractable with BDF or Gear-type methods. Higher-index DAEs are problematic and require special care.

Numerical integration:
- Backward Euler on F(x', x, t) = 0: F((x_{n+1} - x_n)/h, x_{n+1}, t_{n+1}) = 0 — solved by Newton-Raphson.
- BDF formulas extend the same idea to higher order.
- Consistent initial conditions: g(x_0, y_0, 0) = 0 must hold; usually computed by a DC-like initial-condition analysis.

## Key Parameters

- DAE index (1 for most circuit DAEs).
- Algebraic vs. differential variable split.
- Step size h.
- Initial conditions (consistency required).

## When To Use

- Any modern circuit simulator: SPICE-family, Spectre, Eldo, etc.
- Multibody dynamics, chemical reaction networks.
- Constrained mechanical systems.

## Risks & Pitfalls

- High-index DAEs (>1) are ill-posed for standard BDF; reformulation is required.
- Inconsistent initial conditions produce large initial transients (slow convergence).
- Algebraic variable derivatives are not defined; care needed when using BDF formulas.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/tableau-formulation]]
- [[concepts/gear-bdf]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
