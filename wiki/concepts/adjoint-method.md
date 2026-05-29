---
title: Adjoint Method (for Sensitivity Computation)
type: claim
id: claim-adjoint-method
tags:
- sensitivity
- foundational
- well-established
- analog
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt
confidence:
  base: 0.65
---

## Definition

The adjoint method (Tellegen's-theorem-based or adjoint-network method) is a technique for computing sensitivities of network responses with respect to many element parameters at the cost of essentially one additional system solve, instead of one solve per parameter.

## How It Works

For a linear network with matrix equation Y v = i, the sensitivity of an output (e.g., a voltage or a scalar function) with respect to a parameter is computed using an adjoint (transposed) network excited at the output port. The product of corresponding original- and adjoint-network branch quantities yields the sensitivity. This stems from Tellegen's theorem, which states that the sum over branches of the product of voltages of one network and currents of a topologically identical network is zero.

## Key Parameters

- Choice of objective: scalar function, single output, multiple outputs.
- Type of network: linear time-invariant (most direct), AC, transient (incremental adjoint), nonlinear (adjoint of the linearized Jacobian).
- Whether the adjoint excitation is at one port (cheap) or many (more expensive).

## When To Use

- Sensitivity of a small number of outputs with respect to many parameters — adjoint is far cheaper than direct differentiation.
- Gradient computation for optimization-based design, where the objective is a small set of scalars.

## Risks & Pitfalls

- Implementation details differ between linear/nonlinear, frequency-domain, and time-domain cases.
- Time-domain adjoint requires reverse-time integration and careful storage of forward solutions.
- Easy to confuse direct and adjoint formulations; sign conventions on the adjoint excitation matter.

## Related Concepts

- [[concepts/sensitivity-analysis]]
- [[concepts/tellegen-theorem]]
- [[concepts/symbolic-analysis]]
- [[concepts/sequential-quadratic-programming]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
- [[summaries/computer-methods-circuit-analysis-design-11-chapter-8-large-change-sensitivity-and-related-topics]]
- [[summaries/computer-methods-circuit-analysis-design-13-chapter-10-numerical-laplace-transform-inversion]]
- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
