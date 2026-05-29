---
title: Integrated Circuit
type: claim
id: concepts/integrated-circuit
tags:
- vlsi
- foundational
- well-established
- semiconductor
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/04-1-introduction.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

An integrated circuit (IC) is a set of electronic circuits (transistors, resistors, capacitors, and interconnects) fabricated on a single substrate (typically silicon) using planar semiconductor processes.

## How It Works

ICs are manufactured by sequentially patterning and doping layers on a silicon wafer to form transistors and metal interconnects. The first IC was demonstrated by Jack Kilby in 1958 using discrete components on a single substrate; Robert Noyce introduced the monolithic IC in 1959 at Fairchild Semiconductor using a planar process. The MOSFET (Atalla and Kahng, 1959) became the dominant device, with self-aligned gates (late 1960s) and ion implantation (1965) enabling rapid integration scaling from SSI to VLSI.

## Key Parameters

- Number of transistors and gate density.
- Feature size / process node.
- Number of metal layers.
- Die area and yield.
- Operating frequency, power, and reliability.

## When To Use

- All modern electronic systems requiring miniaturized, low-power, high-performance functionality.

## Risks & Pitfalls

- Defect density limits yield as die size grows.
- Process variation widens with shrinking feature sizes.
- Power density and thermal management increase in difficulty.

## Related Concepts

- [[concepts/vlsi-design]]
- [[concepts/mosfet]]
- [[concepts/electronic-design-automation]]

## Sources

- [[summaries/graphs-in-vlsi-04-1-introduction]]
