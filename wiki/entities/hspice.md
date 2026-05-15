---
title: "HSPICE"
type: entity
tags: [tool, analog, simulator, industrial]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors.txt"]
confidence: medium
---

## Overview

HSPICE is a commercial circuit simulator (Synopsys) widely used as the de facto numerical sign-off tool for analog/mixed-signal IC design. The Shi/Tan/Tlelo-Cuautle book uses it as the reference against which symbolic AC and noise results are validated.

## Characteristics

- Industry-standard accuracy for transistor-level transient, AC, noise, harmonic-balance.
- Wide MOSFET model coverage (BSIM3/4/6, PSP, EKV, etc.) and NLEV 0/1/2 noise models.
- Netlist-compatible with the broader SPICE ecosystem.

## Common Strategies

- Run for final-sign-off; pair with symbolic analyzers for insight and statistical sweeps.
- Used for cross-validation of symbolic noise/AC predictions in the book's CMOS amplifier examples.

## Related Entities

- [[entities/spice]]
- [[entities/ngspice]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
