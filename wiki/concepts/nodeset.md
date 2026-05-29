---
title: Nodeset
type: claim
id: claim-nodeset
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

A nodeset is a user-supplied hint for the initial guess used by [[concepts/newton-raphson-method]] at the start of [[concepts/dc-analysis]]. It assigns a starting voltage to one or more nodes (and optionally branch currents). Unlike an initial condition, a nodeset does not constrain the final solution — it merely seeds NR.

## How It Works

The simulator initializes the NR state with the nodeset values for the listed nodes and zero (or default) for the rest, then proceeds with iteration as usual. If NR converges, the nodeset values have no effect on the final answer; if NR diverges from zero but converges from the nodeset, the nodeset has rescued an otherwise stuck DC.

## Key Parameters

- Per-node voltage / current values
- Optional flag to save the final operating point back to a file for reuse as a nodeset in subsequent runs

## When To Use

- After a [[concepts/dc-analysis]] convergence failure on a circuit whose approximate operating point the designer knows (digital-style nodes that should be near rails, bias voltages of analog stages).
- To seed the simulator from a previously-solved DC point of a closely related circuit or operating condition (parameter sweeps, corner runs).
- As an alternative to a UIC transient run for circuits that won't solve in DC.

## Risks & Pitfalls

- Garbage nodesets can push NR into a different basin of attraction than expected — yielding an unstable or unintended equilibrium that converges fine.
- Nodesets do not enforce the listed values in the converged solution — those are only initial guesses. Designers sometimes confuse this with initial conditions (`.IC`/UIC).

## Related Concepts

- [[concepts/dc-analysis]]
- [[concepts/newton-raphson-method]]
- [[concepts/homotopy-method]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
