---
title: Simulation of Analog and Mixed-Signal Circuits
type: source
slug: simulation-analog-mixed-signal-circuits
created: 2026-06-16
updated: 2026-06-16
summary: BCTM 1998 tutorial by Ken Kundert covering SPICE simulation methods, Newton-Raphson convergence, integration methods, timing simulation, and Verilog-AMS.
source_file: Papers/simulation_whitepaper1.pdf
tags: [circuit-simulation, spice, verilog-ams, analog, mixed-signal, numerical-methods]
status: active
---

# Simulation of Analog and Mixed-Signal Circuits

- **Source file:** `sources/Papers/simulation_whitepaper1.pdf`
- **Author / origin:** Dr. Ken Kundert, Cadence Design Systems, Inc.
- **Date:** BCTM 1998 Tutorial on Circuit Simulation

## Summary

A comprehensive tutorial on circuit simulation for analog and mixed-signal circuits, aimed at practicing circuit designers and CAD engineers. Covers DC, AC, transient, and Fourier analyses, timing simulation, hardware description languages, and introduces Verilog-AMS.

### Traditional SPICE Simulation

Circuit simulators formulate nonlinear first-order differential/algebraic equations from structural circuit descriptions and solve them numerically. Different analyses apply different assumptions:
- **DC**: Finds equilibrium points by discarding time-derivatives. Convergence is an issue; Newton-Raphson requires isolated solutions. Gmin (small conductance ~10^-12 S across nonlinear devices) prevents floating nodes.
- **AC/Noise**: Linearizes circuit about DC operating point; computes sinusoidal steady-state (transfer functions). Not suitable for mixers, oscillators, switched-capacitor circuits, or other large-signal or frequency-converting circuits.
- **Transient**: Discretizes time; approximates waveforms with piecewise polynomials; replaces derivatives with finite-difference approximations solved one timepoint at a time.
- **Fourier**: Computes Fourier coefficients of transient waveforms. SPICE's implementation is notoriously inaccurate due to linear interpolation error limiting resolution to 40-60 dB; Spectre controls LTE in voltage (not charge).

### Newton-Raphson Algorithm

Iterative method starting from an initial guess; linearizes nonlinear equations at each step. Convergence guaranteed if: (1) equations are sufficiently smooth, (2) solution is isolated, (3) initial guess is close enough. Two convergence criteria: update criterion (|Δv| < ε) and residue criterion. SPICE uses ΔI check (subject to false convergence); Spectre uses KCL check (more reliable).

### Homotopy / Continuation Methods

When NR fails, simulators use homotopy: solve a series of problems parameterized by λ from an easy case (λ=0) to the desired problem (λ=1). Variants: source stepping, Gmin stepping, pseudo-transient analysis. Fail due to discontinuities (folds, bifurcations, oscillations).

### Integration Methods

| Method | Form | Stability |
|---|---|---|
| Forward Euler (FE) | Explicit | Unstable on stiff circuits |
| Backward Euler (BE) | Implicit | Stiffly stable; adds numerical damping |
| Trapezoidal Rule (TR) | Implicit | Marginally stable; rings on stiff circuits |
| Gear's BDF2 (G2) | Implicit | Stiffly stable; less damping than BE |

LTE (local truncation error) is estimated as difference between extrapolated and computed solution; timestep adjusted to control LTE. Circuits with long time constants (oscillators, integrators, charge-storage) accumulate errors and require tighter tolerances.

### Timing Simulation

Fast, reduced-accuracy simulation for large MOS digital circuits. Uses forward Euler (explicit), simplified models, circuit partitioning into small loosely-coupled subcircuits. 10-100x speedup over SPICE but unsuitable for analog/bipolar; risks incorrect results.

### Top-Down Design Methodology and Mixed-Level Simulation

Key insight: 14x productivity difference between best and worst companies. Top-down design (TDD) verifies architecture before block design. Mixed-level simulation (MLS) runs one block at transistor level while rest of system uses behavioral models—the only feasible approach for verifying complex mixed-signal systems (e.g., PRML disk read channel with >10,000 transistors and 2000 simulation cycles).

### Verilog-AMS

Mixed-signal extension to Verilog-HDL (Verilog-A approved June 1996; Verilog-AMS August 1998). Supports both event-driven (initial/always blocks) and continuous-time (analog block) behavior. Supports conservative models (potential + flow) and signal-flow models (potential only). Additional operators: `idt`/`ddt` (time integration/differentiation), `idtmod` (circular integrator), `transition`/`slew`. Examples shown: VCO, sampler, phase/frequency detector (PFD/CP), N-bit ADC.

## Key takeaways

- SPICE NR convergence requires smooth models, isolated solutions, and a good initial guess; homotopy methods (Gmin stepping preferred) rescue most failures
- Trapezoidal rule rings on stiff circuits; BE/G2 add numerical damping; choice depends on circuit type
- Spectre's KCL check and voltage-domain LTE control outperform SPICE's ΔI check and charge-domain LTE
- Mixed-level simulation (behavioral + transistor-level blocks) is the only practical path to verifying complex mixed-signal systems
- Verilog-AMS unifies analog and digital HDL simulation with conservative and signal-flow models

## Pages updated from this source

- [[circuit-simulation]] - core topic page created
- [[newton-raphson]] - convergence algorithm concept
- [[spice-simulation]] - SPICE analysis modes concept
- [[integration-methods]] - numerical integration methods concept
- [[homotopy-methods]] - convergence homotopy concept
- [[verilog-ams]] - Verilog-AMS language concept
- [[ken-kundert]] - author entity
- [[overview]] - updated with circuit simulation topic
