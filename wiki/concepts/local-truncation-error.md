---
title: Local Truncation Error
type: claim
id: claim-local-truncation-error
tags:
- analog
- transient
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.85
---

## Definition

Local Truncation Error (LTE) is the per-step error introduced by an [[concepts/integration-method]] when it replaces the continuous derivative d/dt with a finite-difference approximation. **Global truncation error** is the accumulated effect of LTE made on each step and depends on the circuit, not just the method.

## How It Works

For an integration method of order p, the exact solution can be expanded in a Taylor series and the discrete operator's residue at one step is the truncation of that series at order p+1. Simulators estimate LTE in practice by extrapolating from previous timepoints with a polynomial of the same order as the method (the method is exact on such polynomials, so any deviation from the integrator's result is LTE). The estimated LTE is compared to a tolerance derived from `reltol` and `abstol`; if too large, the step is rejected and h shrunk; if well below, h is grown.

## Key Parameters

- `reltol` — relative tolerance, scales the LTE threshold with signal magnitude
- `abstol` — absolute tolerance, prevents tolerance collapse near zero crossings
- `chargetol` (SPICE) — analogous tolerance applied to charge LTE
- LTE quantity controlled: charge in SPICE, voltage in Spectre — see [[concepts/charge-conservation]]

## When To Use

LTE estimation drives the **timestep controller** of every adaptive transient solver. Tighter tolerances → smaller h → smaller LTE per step but more steps, hence longer simulation time. The trade-off is the single largest factor affecting transient runtime and accuracy.

## Risks & Pitfalls

- **Error accumulation depends on the circuit.** Dissipative circuits damp out LTE; circuits with long time constants (integrators, oscillators, switched-capacitor circuits, charge-storage circuits) accumulate it. Oscillators in particular suffer phase drift because integrators systematically underestimate curvature, biasing the LTE in one direction.
- **Charge vs. voltage LTE control matters on stiff circuits.** A tiny LTE in charge on a tiny capacitor can correspond to a large LTE in voltage — SPICE's charge-based control is the textbook source of visibly inaccurate voltage waveforms.
- **Tighten tolerances** when simulating long transients on circuits with long time constants, particularly oscillators; loose default tolerances are the source of many "Spectre and SPICE disagree" reports.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/transient-analysis]]
- [[concepts/charge-conservation]]
- [[concepts/numerical-damping]]
- [[concepts/forward-euler]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
