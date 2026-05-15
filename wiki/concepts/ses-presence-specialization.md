---
title: "SES Presence Specialization"
type: concept
tags: [simulation, modeling, ses, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/07-5-specialization-and-pruning.txt"]
confidence: medium
---

## Definition

A presence specialization is a special-purpose SES specialization with the label `presence` and choices `Present` and `NotPresent`. Pruning the choice to `NotPresent` removes the parent entity from the resulting PES.

## How It Works

When the modeler writes `ENTITY can be Present or NotPresent in presence!`, the pruning interface treats this as a structural toggle. Selecting `NotPresent` deletes the entity (and its subtree) from the PES; selecting `Present` keeps it. This is the canonical way to express optional components in an SoS architecture without rewriting the SES.

## Key Parameters

- Special label name `presence`
- Two-choice alternatives: Present, NotPresent

## When To Use

- Modeling optional subsystems (e.g., optional sensor packages)
- Encoding feature flags in design-space exploration
- Toggling test stubs vs. real components

## Risks & Pitfalls

- Removing an entity may invalidate couplings that reference it; consistency checks may be needed
- Easy to misuse for runtime presence — `presence` is static
- Naming the specialization anything other than `presence` loses the structural-toggle semantics

## Related Concepts

- [[concepts/ses-specialization]]
- [[concepts/ses-pruning]]
- [[concepts/pruned-entity-structure]]

## Sources

- [[summaries/modeling-simulation-systems-07-5-specialization-and-pruning]]
