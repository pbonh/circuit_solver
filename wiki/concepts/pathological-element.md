---
title: "Pathological Element"
type: concept
tags: [analog, foundational, behavioral, two-graph]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/10-6-generalized-two-graph-theory.txt"]
confidence: medium
---

## Definition

A pathological element is an idealized circuit primitive that constrains either voltage or current independently of the other. The four canonical types are: nullator and norator (forming the nullor pair) and voltage mirror (VM) and current mirror (CM). Combinations of these model op-amps, current conveyors, ICCIIs, DXCCIIs, and other analog building blocks.

## How It Works

- Nullator (NL): zero terminal voltage AND zero current.
- Norator (NR): arbitrary voltage AND arbitrary current.
- Voltage mirror (VM): terminal voltages of equal magnitude, opposite sign; zero current.
- Current mirror (CM): output current equals input current in magnitude, opposite sign; arbitrary voltages.

In two-graph analysis: NL and VM go into the V-graph (precollapsed); NR and CM go into the I-graph (precollapsed). VM and CM introduce bidirectional edges and result in oppositely-signed node-set indices that affect admittance-sign stamping.

## Key Parameters

- Reference orientation (matters for sign flipping in NAM stamping).
- Which pairs are combined to model a real active block (NL-NR = nullor = ideal op-amp; VM-CM = ICCII+; NL-CM = CCII+; etc.).

## When To Use

- Behavioral-level modeling of current conveyors and complex active blocks before symbolic analysis.
- Compact two-graph and reduced NAM construction.

## Risks & Pitfalls

- Incorrect reference orientation flips a sign and changes the transfer function.
- Loop equations may become ill-posed if pathological-element placements over-constrain the network.

## Related Concepts

- [[concepts/nullor]]
- [[concepts/two-graph-method]]
- [[concepts/nodal-admittance-matrix]]
- [[concepts/symbolic-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
