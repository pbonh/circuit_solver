---
title: Simulation of Analog and Mixed-Signal Circuits — Kundert BCTM '98 Tutorial
type: source
id: summaries/kundert-bctm98-simulation-tutorial
kind: publication
tags:
- analog
- mixed-signal
- dc
- ac
- transient
- noise
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
---

## Key Points

- A circuit simulator takes a structural circuit description and formulates a system of nonlinear, first-order differential/algebraic equations (model equations + Kirchhoff's laws), then solves them numerically. Different analyses apply different simplifying assumptions before solving.
- **DC analysis** discards time derivatives and solves the resulting nonlinear algebraic system for equilibrium points via Newton-Raphson; equilibria may be stable, unstable, or non-isolated, and NR cannot find non-isolated solutions (floating nodes, loops of shorts) — `Gmin` (≈10⁻¹² S) is added to every nonlinear device to avoid this.
- **Newton-Raphson convergence** is guaranteed only when (1) model equations are sufficiently smooth, (2) the solution is isolated, and (3) the initial guess is close enough. The third is hard to satisfy, so simulators rely on **homotopy/continuation** methods — source stepping, Gmin stepping, and pseudo-transient — that solve an easy parameterized problem first and step toward the desired one.
- **Convergence criteria** combine an update check (|Δv| < ε) with a residue check. SPICE's ΔI check is susceptible to false convergence when NR stalls; Spectre's KCL check enforces ΣI < δ directly and is more reliable but harder to implement efficiently.
- **AC and noise analysis** linearize the circuit around the DC operating point, producing a linear time-invariant (LTI) model whose sinusoidal steady-state response is computed directly. This is suitable for amplifiers and continuous-time filters but fundamentally cannot model frequency conversion, noise folding, mixers, oscillators, samplers, or switched-capacitor circuits.
- **Transient analysis** discretizes time and approximates the solution trajectory by a piecewise low-order polynomial. The d/dt operator is replaced by a finite-difference approximation — Forward Euler, Backward Euler, Trapezoidal Rule (BE+FE), or Gear's second-order BDF — converting the ODE/DAE into a difference equation solved one timepoint at a time via NR.
- **Stiff circuits** (those whose fastest time constants are much shorter than the desired timestep) make explicit methods like Forward Euler unstable; implicit methods (BE, TR, G2) are stiffly stable. TR is only marginally stable on stiff circuits and exhibits characteristic point-to-point ringing. BE and Gear2 are overly stable and introduce artificial numerical damping — visible as decaying oscillation on an LC tank that should ring forever.
- **Local truncation error (LTE)** is the per-step integration error; **global truncation error** is its cumulative effect, which depends on the circuit. Dissipative circuits damp out errors; non-dissipative circuits (integrators, oscillators, switched-capacitor circuits, charge-storage circuits) accumulate them, so they require tighter tolerances. LTE is estimated by comparing the computed solution to a low-order polynomial extrapolation from previous timepoints, and the timestep is adapted to keep LTE within a threshold.
- **SPICE controls LTE in charge** while **Spectre controls LTE in voltage**; on stiff circuits a small charge error can correspond to a large voltage error, so SPICE can produce noticeably less accurate waveforms with default tolerances.
- **SPICE Fourier analysis is notoriously inaccurate** (typically capped near 40 dB resolution) because linear interpolation onto an equally-spaced grid introduces spurs as high as -54 dB. Tightening Tstep, Tmax, and reltol is required to extract genuine 60-120 dB resolution from a transient-derived spectrum.
- **Initial conditions**: UIC uses specified ICs exactly and zeros the rest (charge conserved on loops/cutsets in one step); non-UIC forces ICs with series/parallel resistors during a DC solve and computes the unspecified ones, but yields unexpected results on parallel LC tanks. Charge conservation requires both charge-based device models (newer MOS models) and a tight enough KCL tolerance (reltol/abstol).
- **Timing simulation** trades accuracy for speed (10-100× SPICE on MOS digital) by partitioning into single-node subcircuits, using explicit integration (forward Euler), and simplified models. Requires loose coupling and non-stiff partitions; fails on memories, busses, analog, and bipolar circuits. Mixed-signal timing simulation extends this with full circuit simulation on identified analog/bipolar partitions but speedup is bounded by the analog fraction.
- The argument: SPICE-level performance gains are incremental (2-4× per decade); real productivity comes from **top-down design + mixed-level simulation** using **AHDLs / MS-HDLs** (Verilog-AMS, VHDL-AMS) that let designers verify blocks in the context of pin-accurate behavioral models of the rest of the system. The Disk Read Channel case study (PRML, >10k transistors, 2000 cycles) was infeasible at the transistor level (>1 month) but tractable under mixed-level simulation (overnight per block).
- **Verilog-AMS** combines event-driven (initial/always) and continuous-time (analog block, evaluated once per timestep) constructs, supports both signal-flow models (potentials only, for abstract top-level blocks) and conservative models (potentials and flows, for device modeling), provides analog operators (`idt`, `ddt`, `transition`, `slew`, Laplace/Z filters), event constructs (`cross`, `timer`, `initial_step`/`final_step`), automatic interface element insertion at A/D port mismatches, and parasitic back-annotation. **VHDL-AMS** (IEEE 1076.1, approved 1998) similarly extends VHDL but lacks automatic interface insertion and parasitic back-annotation.

## Relevant Concepts

- [[concepts/modified-nodal-analysis]] — formulation that SPICE/Spectre use to build the system of equations from a netlist
- [[concepts/newton-raphson-method]] — the iterative root-finding kernel beneath every analysis
- [[concepts/dc-analysis]] — equilibrium-point analysis under constant-waveform assumption
- [[concepts/homotopy-method]] — parameterized continuation used as a convergence aid when NR alone fails
- [[concepts/gmin-stepping]] — sweeping a parallel conductance to track a solution path
- [[concepts/source-stepping]] — sweeping independent-source magnitudes from 0 to nominal
- [[concepts/pseudo-transient-analysis]] — adding 1 F node-to-ground capacitors and running transient as a homotopy
- [[concepts/nodeset]] — user-provided initial guess for DC convergence
- [[concepts/ac-analysis]] — linearized small-signal sinusoidal steady-state
- [[concepts/noise-analysis]] — variation of AC that propagates small noise sources
- [[concepts/small-signal-analysis]] — the umbrella for AC and noise
- [[concepts/transient-analysis]] — time-domain ODE/DAE integration
- [[concepts/integration-method]] — the family of finite-difference approximations to d/dt
- [[concepts/forward-euler]] — explicit first-order method (not used in circuit simulation, used in timing simulation)
- [[concepts/backward-euler]] — implicit first-order method
- [[concepts/trapezoidal-rule]] — second-order implicit, marginally stable on stiff circuits
- [[concepts/gear-bdf]] — Gear's second-order backward differentiation formula
- [[concepts/stiff-circuit]] — circuits with time constants much shorter than the desired timestep
- [[concepts/local-truncation-error]] — per-step integration error; controls the timestep
- [[concepts/numerical-damping]] — artificial loss introduced by overly stable methods like BE and Gear2
- [[concepts/charge-conservation]] — property of MOS device models and KCL approximations that matters for switched-capacitor circuits
- [[concepts/fourier-analysis]] — and its interpolation-error pitfalls in SPICE
- [[concepts/timing-simulation]] — fast reduced-accuracy MOS-digital simulation
- [[concepts/mixed-level-simulation]] — single block at transistor level inside a behavioral testbench
- [[concepts/top-down-design]] — design and verify the system before the blocks
- [[concepts/signal-flow-model]] — abstract block model relating potentials only
- [[concepts/conservative-model]] — device-like model relating potentials and flows
- [[concepts/ahdl-mshdl]] — analog and mixed-signal hardware description languages
- [[entities/spice]] — the canonical circuit simulator
- [[entities/spectre]] — Cadence's circuit simulator family, Kundert's vehicle for KCL convergence and voltage-LTE control
- [[entities/verilog-ams]] — IEEE/OVI-standardized mixed-signal Verilog extension
- [[entities/vhdl-ams]] — IEEE 1076.1 mixed-signal VHDL extension
- [[entities/ken-kundert]] — the paper's author; Cadence fellow, principal architect of Spectre, Verilog-AMS/VHDL-AMS contributor

## Source Metadata

- Source type: tutorial / whitepaper (BCTM 1998 tutorial notes)
- Title: Simulation of Analog and Mixed-Signal Circuits
- Author: Ken Kundert (Cadence Design Systems)
- Venue: BCTM 1998 Tutorial
- File: `raw/simulation_whitepaper_v1/simulation_whitepaper1.txt` (extracted from `simulation_whitepaper1.pdf`)
- Pages: 69
- Companion: Kundert's book *The Designer's Guide to SPICE and Spectre* (Kluwer, 1995)
