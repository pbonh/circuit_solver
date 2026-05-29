---
title: Cellular Automaton Coupling
type: claim
id: claim-cellular-automaton-coupling
tags:
- simulation
- modeling
- cellular-automata
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/08-6-aspects-and-multi-aspects.txt
confidence:
  base: 0.65
---

## Definition

A cellular automaton coupling is a predefined SES coupling specification that connects each cell to its immediate North/South/East/West neighbors (Moore neighborhood). It generates the canonical grid digraph used in cellular-automata simulations.

## How It Works

The modeler declares two coupling specifications (cellEW for East-West and cellNS for North-South) and uses multidimensional multiplicity `set multiplicity of location as [10,10] for cell!`. The cellular builder `write cellular specification for cell and location based on cellEW/cellNS!` generates the row/column edges; uniform port couplings attach East/West and North/South messages to every edge.

## Key Parameters

- Two-dimensional multiplicity
- East-West and North-South coupling specifications
- Moore-neighborhood digraph
- Per-direction port pairs

## When To Use

- Grid-based simulations (forest fires, fluid flows, urban dynamics)
- Cellular DEVS / Cell-DEVS variants
- Multi-dimensional sensor or processor arrays

## Risks & Pitfalls

- Boundary cells have fewer neighbors — handle edge effects explicitly
- Only Moore neighborhoods are predefined; richer kernels need custom edge lists
- Performance scales with grid size squared in 2D

## Related Concepts

- [[concepts/ses-coupling-specification]]
- [[concepts/ses-multi-aspect]]

## Sources

- [[summaries/modeling-simulation-systems-08-6-aspects-and-multi-aspects]]
