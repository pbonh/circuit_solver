---
title: "Extrapolation Steady-State Method"
type: concept
tags: [transient, advanced, emerging]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/19-chapter-16-time-domain-sensitivities-and-steady-state.txt"]
confidence: medium
---

## Definition

The extrapolation steady-state method (Vlach & Singhal Section 16.5) accelerates convergence to a periodic solution by applying sequence-acceleration techniques (Shanks transformation, Aitken's Delta^2 process) to the sequence of states at successive periods q(0), q(T), q(2T), ... without requiring derivative information.

## How It Works

1. Integrate the nonlinear DAE through several periods, recording q(k T) for k = 0, 1, 2, ....
2. Apply sequence extrapolation to this sequence to predict the fixed point q_infty.
3. Restart integration from q_infty; refine as needed.

Compared to shooting methods, this approach:
- Does not require the sensitivity network.
- Is simple to program.
- Vlach & Singhal report excellent results.

## Key Parameters

- Number of periods initially integrated.
- Extrapolation order (Aitken, Wynn epsilon, Shanks).
- Convergence tolerance.

## When To Use

- Steady-state analysis when sensitivity computation is impractical or expensive.
- When the user wants a quick steady-state estimate without elaborate setup.

## Risks & Pitfalls

- Extrapolation may not converge if the underlying sequence is not regular enough.
- More periods of integration may be needed than shooting (which converges quadratically when working).

## Related Concepts

- [[concepts/steady-state-analysis]]
- [[concepts/shooting-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
