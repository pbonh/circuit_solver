---
title: Circuit Simulation
type: topic
slug: circuit-simulation
created: 2026-06-16
updated: 2026-06-16
summary: The field of numerically solving circuit equations to predict analog, digital, and mixed-signal circuit behavior.
tags: [circuit-simulation, eda, vlsi, analog, mixed-signal]
sources: [simulation-analog-mixed-signal-circuits, computer-methods-circuit-analysis-design]
status: active
---

# Circuit Simulation

The computational discipline of formulating and numerically solving systems of differential-algebraic equations derived from circuit topology and device models. Covers DC, AC, transient, and steady-state analyses; numerical methods; device modeling; and hardware description languages.

## Overview

- Circuit simulators translate netlists into nonlinear DAEs and solve them numerically
- Core algorithms: [[newton-raphson]] for nonlinear algebraic solve, [[integration-methods]] for time-domain
- [[spice-simulation]] is the dominant paradigm (DC, AC, noise, transient, Fourier)
- [[homotopy-methods]] rescue convergence failures
- [[verilog-ams]] and VHDL-AMS enable mixed-level and top-down design flows
- Mixed-level simulation (behavioral + transistor-level blocks simultaneously) is the only feasible approach to verifying complex mixed-signal systems

## Entities and concepts in this topic

- [[spice-simulation]] - DC, AC, transient, Fourier analysis modes
- [[newton-raphson]] - nonlinear solver at the core of all SPICE analyses
- [[integration-methods]] - FE, BE, TR, G2 for transient time-stepping
- [[homotopy-methods]] - convergence recovery via source stepping, Gmin stepping, pseudo-transient
- [[verilog-ams]] - analog/mixed-signal HDL for behavioral modeling
- [[ken-kundert]] - architect of Spectre; author of key simulation literature
- [[computer-methods-circuit-analysis-design]] - algorithmic foundation of SPICE: MNA, LU, NR, BDF, adjoint

## Open threads

- Parallel/distributed circuit simulation approaches and their actual speedup limits
- RF simulation (harmonic balance, envelope methods) — referenced in source but not covered
- Formal verification vs. simulation for mixed-signal correctness
