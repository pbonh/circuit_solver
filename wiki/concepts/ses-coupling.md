---
title: SES Coupling
type: claim
id: claim-ses-coupling
tags:
- simulation
- modeling
- ses
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/05-3-system-entity-structure-basics.txt
confidence:
  base: 0.85
---

## Definition

SES Coupling specifies the possibility (not the certainty) of a named message being sent from one entity to another under a perspective. Couplings are the SES-level analog of DEVS coupled-model port-to-port connections.

## How It Works

Coupling sentences take the form "From the X perspective, SENDER sends MESSAGE to RECEIVER!" Three kinds exist: external input coupling (parent → subcomponent), external output coupling (subcomponent → parent), and internal coupling (subcomponent → subcomponent). A consistency rule requires external couplings at one level to use the same message names as the internal couplings at the next level.

## Key Parameters

- Perspective label
- Sender entity (must be in the "made of" list)
- Receiver entity
- Message name

## When To Use

- Wiring up information flow in the M&S process SES example
- Specifying parent/child message routing in hierarchical models
- Generating DEVS coupled-model code that wires ports

## Risks & Pitfalls

- Coupling implies possibility, not actual behavior — modeler must add atomic logic
- Default auto-generated components emit all outputs on any input
- Mismatch of message names across decomposition levels breaks routing

## Related Concepts

- [[concepts/system-entity-structure]]
- [[concepts/ses-decomposition]]
- [[concepts/coupled-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-05-3-system-entity-structure-basics]]
