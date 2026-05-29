---
title: Nullor
type: claim
id: concepts/nullor
tags:
- analog
- pathological-element
- foundational
- behavioral
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/04-1-introduction.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A nullor is a two-port pathological element consisting of a nullator (zero voltage and zero current at one port) and a norator (arbitrary voltage and arbitrary current at the other port). It is the idealized building block for representing operational amplifiers, OTAs, and current conveyors at the behavioral level.

## How It Works

Inserting a nullor pair removes two equations and two unknowns from an MNA matrix (one row/column collapse per element), effectively compressing the system. After all active blocks are modeled by nullors, the residual passive network plus interconnects can be symbolically analyzed with reduced complexity.

## Key Parameters

- Mapping rules between real active devices and nullor equivalents.
- Bookkeeping in the two-graph: nullator edges go in the V-graph only; norator edges go in the I-graph only.

## When To Use

- Behavioral modeling of analog filters with op-amps, OTAs, current conveyors.
- Reducing MNA matrix dimension before DDD/GPDD expansion (Chap. 9 of Shi/Tan/Tlelo-Cuautle).

## Risks & Pitfalls

- Nullor is an idealization; finite gain-bandwidth, output impedance, etc., are not modeled.
- Improper nullor placement can create non-uniquely-solvable networks.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/two-graph-method]]
- [[concepts/symbolic-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-04-1-introduction]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-07-part-ii-methods]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-11-7-graph-pair-decision-diagram]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
