---
title: Interference Graph
type: claim
id: claim-interference-graph
tags:
- graph
- compiler
- vlsi
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt
confidence:
  base: 0.85
---

## Definition

An interference graph encodes conflict relations between resource-consuming entities — most commonly program variables in register allocation. Each node is a variable (or live range); an edge connects two variables whose live ranges overlap and therefore cannot share a register.

## How It Works

Build the interference graph by computing live ranges (program intervals between definition and last use of each variable). For each program point, every pair of simultaneously-live variables receives an interference edge. Graph coloring with K colors then assigns one of K registers to each variable.

## Key Parameters

- Number of nodes (variables/live ranges).
- Graph density (depends on program structure).
- Chromatic number (must not exceed register count for no-spill allocation).

## When To Use

- Compiler register allocation.
- Channel routing in VLSI (analogous horizontal constraint graphs).
- Any resource-conflict problem reducible to coloring.

## Risks & Pitfalls

- Live-range overestimation produces unnecessary interference edges and forces spills.
- Highly connected interference graphs may exceed K colors and require spill cost analysis.

## Related Concepts

- [[concepts/graph-coloring]]
- [[concepts/register-allocation]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
