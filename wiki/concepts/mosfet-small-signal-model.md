---
title: "MOSFET Small-Signal Model"
type: concept
tags: [mosfet, analog, device-model, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors.txt"]
confidence: medium
---

## Definition

The small-signal model of a MOSFET represents the device near a bias point as a linear two-/three-/four-terminal network composed of a transconductance `gm`, output conductance `go`, body transconductance `gmb`, and parasitic capacitances (Cgs, Cgd, Cds, Cbs, Cbd, etc.).

## How It Works

For symbolic analysis, the active core is a VCCS `i_DS = gm v_GS`, naturally captured by a nullor pair: nullator at gate-source (forces `v_GS` to control), norator at drain-source (delivers `i_DS`). Parasitic capacitors are added as admittance edges between the corresponding terminals. The four-terminal model adds `gmb v_BS` and bulk capacitances.

## Key Parameters

- `gm`, `go = 1/r_o`, `gmb`.
- `Cgs`, `Cgd`, `Cds`, `Cbs`, `Cbd`.
- Bias point (sets all of the above).

## When To Use

- AC, noise, and stability symbolic analysis of analog CMOS circuits.
- Sensitivity analysis with respect to device sizing.

## Risks & Pitfalls

- Linearization valid only near the bias point.
- High-frequency behavior may need additional non-quasi-static elements.

## Related Concepts

- [[concepts/nullor]]
- [[concepts/dependent-source]]
- [[concepts/symbolic-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
