---
title: "Spectre"
type: entity
tags: [analog, mixed-signal, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Overview

Spectre is a circuit simulator family developed by Cadence Design Systems and architected by [[entities/ken-kundert]]. It is the principal analog/mixed-signal simulator in Cadence's tool suite and is widely used in commercial mixed-signal design flows.

## Characteristics

- **Voltage-domain LTE control** instead of SPICE's charge-domain — produces visibly more accurate waveforms on stiff circuits with default tolerances.
- **KCL convergence check** (`|ΣI| < δ`) by design — robust against the false-convergence pathology of SPICE's ΔI check. Implemented efficiently in Spectre because the simulator was architected for it from the start; SPICE-derived simulators that bolt on a KCL check pay a substantial performance penalty.
- **Spectre family members:**
  - **Spectre** — classical SPICE-class analog simulator
  - **SpectreHDL** — proprietary analog HDL (predates Verilog-A)
  - **SpectreRF** — periodic steady-state, harmonic balance, PAC, PNoise for RF circuits
- First-class support for [[entities/verilog-ams]] and the broader [[concepts/mixed-level-simulation]] workflow.
- High-capacity SPICE-level simulation as required for [[concepts/top-down-design]] mixed-level runs.

## Common Strategies

- [[concepts/modified-nodal-analysis]] formulation, voltage-LTE-controlled timestep, Trapezoidal Rule default
- KCL-based [[concepts/newton-raphson-method]] convergence
- Standard [[concepts/homotopy-method]] cascade ([[concepts/gmin-stepping]], [[concepts/pseudo-transient-analysis]]) for stubborn [[concepts/dc-analysis]] cases
- Tight integration with Cadence Virtuoso analog design environment

## Related Entities

- [[entities/spice]]
- [[entities/ken-kundert]]
- [[entities/cadence]]
- [[entities/verilog-ams]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
