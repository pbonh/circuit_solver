---
title: Shooting Method (for Steady State)
type: claim
id: claim-shooting-method
tags:
- transient
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/19-chapter-16-time-domain-sensitivities-and-steady-state.txt
confidence:
  base: 0.65
---

## Definition

The shooting method finds the steady-state periodic solution of a nonlinear network by Newton-Raphson on the initial-condition fixed point: find q_0 such that q(T; q_0) = q_0, where q(T; q_0) is the result of integrating one period starting from q_0. The Jacobian dq(T)/dq_0 - I is obtained from the sensitivity network.

## How It Works

1. Choose initial estimate q_0^{(0)}.
2. Integrate the nonlinear DAE one period to get q(T; q_0^{(k)}) and the sensitivity matrix Phi = dq(T)/dq_0 (the "monodromy matrix").
3. Newton update: q_0^{(k+1)} = q_0^{(k)} - (Phi - I)^{-1} [q(T; q_0^{(k)}) - q_0^{(k)}].
4. Repeat until ||q(T; q_0) - q_0|| < epsilon.

## Key Parameters

- Initial estimate q_0^{(0)}.
- Period T.
- Convergence tolerance.
- Integration tolerance per period.

## When To Use

- Steady-state analysis of oscillators (with care for trivial DC solutions).
- Forced periodic response in nonlinear circuits.
- Bifurcation analysis.

## Risks & Pitfalls

- Convergence rate depends on the spectral radius of Phi; stiff circuits can have Phi nearly singular.
- Multiple periodic solutions need multiple starting points to discover.
- Each Newton iteration requires one full period integration plus sensitivity-network integration.

## Related Concepts

- [[concepts/steady-state-analysis]]
- [[concepts/sensitivity-network]]
- [[concepts/newton-raphson-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
