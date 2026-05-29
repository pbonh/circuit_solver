---
title: Gmin Stepping
type: claim
id: claim-gmin-stepping
tags:
- analog
- dc
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.85
---

## Definition

Gmin stepping is a [[concepts/homotopy-method]] continuation aid for [[concepts/dc-analysis]] in which the parallel "minimum conductance" `Gmin` placed across every nonlinear device is swept from a large value (≈1 Ω equivalent, where the circuit is dominated by linear conductances and trivially solvable) down to a tiny operational value (≈10⁻¹² S) at which Gmin no longer perturbs the answer.

## How It Works

The simulator inserts a conductance Gmin in parallel with every nonlinear device. With Gmin large, the nonlinear elements are heavily shunted and the system is well-conditioned and nearly linear, so [[concepts/newton-raphson-method]] converges quickly from a zero initial guess. The simulator decreases Gmin in steps, re-solving with the previous solution as the initial guess each time, until Gmin is small enough that its effect on the operating point is negligible.

## Key Parameters

- Initial Gmin value (large enough for trivial convergence)
- Final Gmin value (small enough for accuracy — defaults around 10⁻¹² S)
- Step ratio between successive Gmin values
- Maximum NR iterations per step before shrinking the step ratio

## When To Use

Used as a fallback when plain Newton-Raphson with the user's [[concepts/nodeset]] (or zero) fails to converge on the DC operating point. Generally works better than [[concepts/source-stepping]] because the resulting trajectory has fewer folds.

## Risks & Pitfalls

- Still subject to folds when the circuit has multiple equilibria — fewer than with source stepping, but they exist.
- Cannot resolve genuinely non-isolated solutions like inductor short loops; topology fixes are required.
- A small but nonzero Gmin remains in the final solution, which can affect very high-impedance nodes by a measurable amount.

## Related Concepts

- [[concepts/homotopy-method]]
- [[concepts/source-stepping]]
- [[concepts/pseudo-transient-analysis]]
- [[concepts/dc-analysis]]
- [[concepts/newton-raphson-method]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
