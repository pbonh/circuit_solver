---
title: SPICE Simulation
type: concept
slug: spice-simulation
created: 2026-06-16
updated: 2026-06-16
summary: The dominant circuit simulation paradigm providing DC, AC, noise, transient, and Fourier analyses via formulation and numerical solution of nonlinear DAEs.
tags: [spice, circuit-simulation, dc-analysis, ac-analysis, transient-analysis]
sources: [simulation-analog-mixed-signal-circuits]
status: active
---

# SPICE Simulation

SPICE (Simulation Program with Integrated Circuit Emphasis) formulates a system of nonlinear first-order differential/algebraic equations from a structural circuit description and solves them numerically. Model equations (built-in or user-specified behavioral models) are combined with Kirchhoff's laws.

## Analysis Modes

### DC Analysis
- Discards time-derivatives; solves for equilibrium (constant-valued) points
- Uses [[newton-raphson]]; requires isolated solutions
- Multiple equilibrium points possible (stable, unstable, non-isolated)
- Gmin (≈10^-12 S across nonlinear devices) prevents floating-node singularities
- Non-isolated solutions (floating nodes, inductor/voltage-source loops) require topology checking

### AC and Noise Analysis
- Linearizes circuit about DC operating point; assumes small sinusoidal stimulus
- Computes transfer functions; frequency-domain only
- **Not suitable for**: mixers, oscillators, VCOs, samplers, switched-capacitor filters, chopper-stabilized amps, parametric amplifiers — all require large periodic stimuli or perform frequency conversion
- Noise analysis: variation computing response to distributed noise sources

### Transient Analysis
- Approximates differential equations with difference equations via [[integration-methods]]
- Timestep chosen to control local truncation error (LTE)
- SPICE controls LTE in charge domain (can give poor answers on stiff circuits)
- Spectre controls LTE in voltage domain — more accurate on stiff circuits
- Initial conditions: UIC (exact as specified, unspecified = 0) vs. non-UIC (DC-forced via V/I sources)

### Fourier Analysis
- Computes Fourier coefficients from transient waveforms
- SPICE accuracy limited to 40-60 dB by linear interpolation error (unequally-spaced points must be resampled)
- Spectre: controlled LTE in voltage + no interpolation aliasing → better Fourier accuracy

## Convergence

SPICE uses two convergence checks:
- **ΔV check**: |update| < ε — important at high-impedance nodes
- **ΔI check**: |change in current between iterations| < δ — subject to false convergence

Spectre uses **KCL check** (|sum of currents| < δ) — not subject to false convergence; no performance penalty in Spectre's design.

## Why it matters

- Foundation of all analog and mixed-signal IC design verification
- Understanding analysis limitations prevents misuse (e.g., using AC analysis on a switched-capacitor filter)
- Spectre's improvements over SPICE (KCL check, voltage LTE, charge-based models) reflect decades of convergence reliability work

## Related concepts and entities

- [[newton-raphson]] - the nonlinear algebraic solver used in every SPICE analysis
- [[integration-methods]] - time-discretization schemes for transient analysis
- [[homotopy-methods]] - recovery when NR convergence fails
- [[verilog-ams]] - behavioral models used in mixed-level SPICE simulation
- [[ken-kundert]] - principal Spectre architect; introduced KCL check and voltage LTE
- [[circuit-simulation]] - parent topic
