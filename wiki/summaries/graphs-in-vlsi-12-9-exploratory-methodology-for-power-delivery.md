---
title: 'Graphs in VLSI — Chapter 9: Exploratory Methodology for Power Delivery'
type: source
id: summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery
kind: publication
tags:
- vlsi
- power-integrity
- optimization
- novel
- simulation
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt
---

## Key Points

- Power delivery is conventionally analyzed late in the design flow (after placement/routing), so failures force expensive redesign iterations. The chapter proposes early exploration that jointly optimizes electrical and non-electrical metrics (cost, area, MTTF) to reduce or eliminate downstream iterations.
- The framework casts power delivery as constrained global optimization: x_opt = argmin f(x) subject to c(x) ≤ 0, where x includes supply voltage, decoupling capacitor values, and interconnect dimensions; f combines metrics such as inverse MTTF and total area.
- Power network is modeled as cascaded RL-RLC stages (PCB → package → die) with a linear voltage source and a load current source profile. MTTF is approximated as MTTF = K_1 W^n H^n / I_rms^n · exp(K_2 W^2 H^2 / I_rms^2) per interconnect segment.
- A custom Laplace-transform-based simulator accelerates the inner loop. Each circuit element is expressed in the s-domain with symbolic variables for variable parameters; MNA yields symbolic transfer functions H(s) = (b_n s^n + ... + b_0) / (a_m s^m + ... + a_0) where coefficients depend on x. The symbolic system is constructed once (high t_setup); subsequent iterations only re-evaluate the coefficients (low t_L), producing speedup that grows with iteration count N.
- State-space simulation (LAPACK / LTITR) extracts time-domain waveforms from the transfer function without resolving MNA each time step.
- Case study 1 (single rail): Three-level RL-RLC power network optimized for cost (decap area-weighted) subject to power, voltage drop, and frequency constraints. Result: supply voltage reduced from 5 V to 3.09 V, total cost reduced 15%, power reduced 38.6%, load voltage fluctuations cut 53%. Completed in 28 s with 66 evaluations.
- Case study 2 (multi-rail): Three configurations (12, 8, 3 rails) for a mobile-SoC power network with 12 functional blocks. Eight-rail configuration minimizes total decoupling capacitance cost. Three-rail merging fails because combined voltage ranges shrink and force larger on-chip decap. Optimization across 23 rail configurations completes in 26 minutes; HSPICE-based simulator takes 265 minutes for the same result (10× speedup).
- Trade-off analysis: early exploration with time t_exp pays off whenever (N − N_new)(t_sim + t_correct) > t_exp, i.e., when even one late-stage iteration is saved.

## Relevant Concepts

- [[concepts/power-delivery-exploration]] — the early-design constrained optimization workflow introduced.
- [[concepts/laplace-transform-simulator]] — fast symbolic linear-circuit simulator built around modified nodal analysis.
- [[concepts/state-space-model]] — used to convert transfer functions into time-domain simulations.
- [[concepts/decoupling-capacitor]] — core design variable in both case studies.
- [[concepts/power-distribution-network]] — system being optimized.
- [[concepts/modified-nodal-analysis]] — foundation of the symbolic Laplace solver.
- [[concepts/mean-time-to-failure]] — reliability metric coupled to current density.
- [[concepts/particle-swarm-optimization]] — global optimizer used in case study 2.
- [[concepts/interior-point-algorithm]] — local optimizer used in case study 1.
- [[concepts/voltage-domain]] — multi-rail granularity considered in case study 2.

## Source Metadata

- Source type: book chapter
- Book title: Graphs in VLSI
- Chapter: 9 — Exploratory methodology for power delivery
- File path: `raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
