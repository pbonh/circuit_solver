---
title: Dirac Impulse (Unit Impulse)
type: claim
id: concepts/dirac-impulse
tags:
- foundational
- math
- well-established
- transient
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The Dirac impulse delta(t) is a distribution that is zero for t ≠ 0 and whose integral over the real line equals one. Its Laplace transform is L{delta(t)} = 1. The unit step u(t) is its integral and has L{u(t)} = 1/s.

## How It Works

Within the Laplace formalism for circuits, an initial condition on a capacitor (V0) appears as a current impulse of value C V0 in the admittance description, while an initial inductor current (I0) appears as a voltage impulse L I0 in the impedance description (see Fig. 1.3.1 in the chapter).

## Key Parameters

- Strength (weight) of the impulse.
- Position (t = 0 for unit impulse; can be shifted to t = t0).
- Derivatives of the impulse (used for higher-order initial conditions).

## When To Use

- Encoding initial conditions in the Laplace domain.
- Representing idealized switching events.
- Numerical Laplace transform inversion handles impulses naturally — a key reason Chapter 10's NILT is preferred for distributed-element networks.

## Risks & Pitfalls

- Time-stepping numerical integrators cannot represent delta functions directly; the source must be smoothed or handled as an initial condition.
- Distributional manipulations require care: products of impulses are not defined.

## Related Concepts

- [[concepts/laplace-transform]]
- [[concepts/numerical-laplace-transform-inversion]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
