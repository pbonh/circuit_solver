---
title: Transistor Amplifier (Hairer–Wanner DAE)
type: claim
id: claim-transistor-amplifier
tags:
- dae
- circuit
- benchmark
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
---

## Definition

The Hairer–Wanner "transistor amplifier" is a benchmark DAE (Equation 1.14, Section VI.1 of *Solving Ordinary Differential Equations II*) modelling a two-stage common-emitter audio amplifier with capacitors, resistors, and Ebers–Moll-like transistor models. The system has the linear-implicit form M u' = φ(u) with constant *singular* mass matrix M — the capacitor topology makes some node equations algebraic and others differential.

## How It Works

After Gaussian elimination M = S · diag(I, 0) · T (Eq. 1.19), the system decomposes into a differential part (rank-(m)) and an algebraic part (rank-(n − m)), the standard semi-explicit reduction. RADAU5 with its M-option handles the constant singular M directly — without explicit reduction — because the ε-embedding diagram (1.23) commutes with the Gaussian decomposition. The amplifier is the canonical stiff-DAE benchmark across the Hairer–Wanner II treatment, the IFAC test set, and standard SPICE-comparison literature.

## Key Parameters

- Capacitor values (set the algebraic / differential split).
- Transistor Ebers–Moll parameters.
- Input sine-wave amplitude / frequency.
- Time scale: tens of milliseconds for an audio-band input.

## When To Use

- Benchmarking DAE solvers (RADAU5, DASSL, RODAS) against a realistic circuit problem.
- Cross-checking circuit-simulation tools (SPICE family).
- Pedagogical example of an implicit-form DAE arising from modified nodal analysis.

## Risks & Pitfalls

- The Ebers–Moll model is exponential; nonlinear-solver damping is essential.
- Stiffness is moderate; results discriminate between equally stable but differently efficient methods, not catastrophic failures.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-1-dae]]
- [[concepts/stiff-circuit]]
- [[concepts/modified-nodal-analysis]]
- [[entities/spice]]
- [[entities/radau5]]
- [[entities/rodas]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
