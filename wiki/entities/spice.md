---
title: SPICE
type: entity
id: entities/spice
tags:
- tool
- analog
- simulator
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/00-preface.txt
---

## Overview

SPICE (Simulation Program with Integrated Circuit Emphasis) is the de facto family of numerical circuit simulators used throughout the IC industry. It performs DC, AC, transient, noise, and parametric analyses by numerically solving the circuit's nonlinear DAE system at each operating point.

## Characteristics

- Numerical, not symbolic: outputs single-point values for a given parameter set.
- Modified Nodal Analysis (MNA) is the dominant matrix formulation.
- Many derivatives exist: HSPICE, ngspice, Spectre, LTspice, Eldo, and so on.
- The Shi/Tan/Tlelo-Cuautle book positions symbolic analysis as a complement to SPICE, not a replacement.

## Common Strategies

- Use SPICE for final-sign-off numerical verification.
- Pair with symbolic engines for design insight, sensitivity, and statistical sweeps.
- Co-validate symbolic-derived expressions against SPICE for selected operating points.

## Related Entities

- [[entities/ngspice]]
- [[entities/hspice]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-00-preface]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/graphs-in-vlsi-04-1-introduction]]
- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
