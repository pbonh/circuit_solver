---
title: "DC Analysis"
type: concept
tags: [analog, dc, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

DC analysis computes the steady-state equilibrium points of a circuit — the node voltages and branch currents that satisfy Kirchhoff's laws and the device models when all waveforms are constant (time derivatives are zero). It is the foundational analysis: AC, noise, and transient analyses all start from a DC operating point.

## How It Works

The simulator discards every d/dt term from the [[concepts/modified-nodal-analysis]] equations, yielding a nonlinear algebraic system. [[concepts/newton-raphson-method]] solves the system iteratively, using either a user-provided [[concepts/nodeset]] or zero as the initial guess. When NR fails to converge from the initial guess, the simulator switches to a [[concepts/homotopy-method]] continuation aid such as [[concepts/source-stepping]], [[concepts/gmin-stepping]], or [[concepts/pseudo-transient-analysis]].

## Key Parameters

- `Gmin` — a small (~10⁻¹² S) conductance added across every nonlinear device to keep nodes from floating
- Nodesets — user hints for the initial guess
- Continuation strategy selection — which homotopy method to try and in what order
- Convergence tolerances — `reltol`, `abstol`, KCL/ΔI check

## When To Use

- To find the operating point that AC and noise analyses linearize around.
- To set the initial condition for a transient analysis (the default unless UIC is set).
- To verify bias-point correctness before time- or frequency-domain runs.
- As a sweep (DC sweep) to characterize transfer curves, transistor I-V plots, etc.

## Risks & Pitfalls

- Three equilibrium types exist: stable, unstable, and non-isolated. DC analysis does not distinguish stable from unstable and will report whichever NR converges to (e.g., the unstable balanced point of a latch).
- Non-isolated equilibria (floating nodes, loops of shorts, parallel LC tanks) are not reachable by NR — topology checkers and `Gmin` are the defensive tools.
- Convergence failures are the dominant SPICE complaint; common causes are bad model parameters, disconnected MOSFET back-gates, and circuits genuinely lacking a stable equilibrium.
- If DC won't converge, a transient with UIC can be used to settle the circuit instead (then saved as a nodeset for subsequent runs).
- Verify large-circuit results by composite metrics (total power, supply currents) — a converged but wrong answer is easy to miss.

## Related Concepts

- [[concepts/newton-raphson-method]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/homotopy-method]]
- [[concepts/source-stepping]]
- [[concepts/gmin-stepping]]
- [[concepts/pseudo-transient-analysis]]
- [[concepts/nodeset]]
- [[concepts/transient-analysis]]
- [[concepts/ac-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
