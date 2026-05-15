---
title: "SES Aspect"
type: concept
tags: [simulation, modeling, ses, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/08-6-aspects-and-multi-aspects.txt"]
confidence: high
---

## Definition

An SES Aspect is a labeled decomposition of an entity into named subentities, expressing one viewpoint or perspective on that entity. An entity may have multiple aspects, capturing alternative ways of decomposing the same real-world thing.

## How It Works

Each aspect is introduced with the natural-language form "From the LABEL perspective, ENTITY is made of A, B, C!". A single SES can host many aspects per entity (e.g., MSProcessSystem with `process` and `fastProcess` aspects); during pruning, exactly one aspect must be selected per entity to generate a concrete model. Aspects combine with specializations to form a family of pruned structures.

## Key Parameters

- Aspect label (perspective name)
- Subentity list
- Per-perspective coupling statements

## When To Use

- Modeling the same real-world system from multiple viewpoints
- Capturing abstraction levels (player view vs. network view) in one SES
- Trading off fidelity vs. speed (full process vs. fastProcess)

## Risks & Pitfalls

- Conflating aspects with specializations
- Forgetting that exactly one aspect must be selected at pruning
- Cross-aspect couplings cannot be specified within a single perspective label

## Related Concepts

- [[concepts/system-entity-structure]]
- [[concepts/ses-specialization]]
- [[concepts/ses-decomposition]]
- [[concepts/ses-multi-aspect]]

## Sources

- [[summaries/modeling-simulation-systems-08-6-aspects-and-multi-aspects]]
