---
title: "ISAAC"
type: entity
tags: [tool, symbolic, analog, historical]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/05-2-symbolic-analysis-techniques-in-a-nutshell.txt"]
confidence: medium
---

## Overview

ISAAC (Interactive Symbolic Analysis of Analog Circuits) is a classical symbolic circuit analyzer developed by Gielen, Walscharts, and Sansen at K.U. Leuven, frequently cited alongside ASAP, SCAPP, SYNAP, and RAINIER as a foundational pre-BDD-era tool.

## Characteristics

- Matrix-based symbolic engine with classical simplification (SAG).
- Targeted analog ICs and active filters.
- Predates the BDD-based generation (DDD, GPDD).

## Common Strategies

- Sum-of-product expansion with magnitude-based pruning.
- Closed-form transfer functions of medium-size analog blocks.

## Related Entities

- [[entities/scad3]]
- [[entities/spice]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
