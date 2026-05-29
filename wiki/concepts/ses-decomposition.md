---
title: SES Decomposition
type: claim
id: concepts/ses-decomposition
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
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

SES Decomposition is the structural operation in a System Entity Structure that breaks an entity into a set of named subentities under a labeled perspective. Decomposition states that, viewed through this perspective, the parent entity is composed of the listed children.

## How It Works

The constrained natural-language form is "From the X perspective, ENTITY is made of A, B, C!" where the order of the children is immaterial. The labeled perspective (e.g., "process", "structural") allows one entity to have multiple alternative decompositions, supporting different modeling viewpoints. Decomposition is recursive — each child can be decomposed further to build hierarchical SES trees.

## Key Parameters

- Parent entity name
- Perspective label
- Subentity list (unordered)
- Hierarchical depth

## When To Use

- Breaking a system into its workflow phases (M&S process example)
- Decomposing physical systems into structural parts (car → engine, transmission, chassis)
- Building reusable nested entity hierarchies for SoS modeling

## Risks & Pitfalls

- Treating ordering of children as meaningful (it is not in SES)
- Re-using the same perspective label inconsistently across siblings
- Forgetting to update couplings when subentities change

## Related Concepts

- [[concepts/system-entity-structure]]
- [[concepts/ses-coupling]]
- [[concepts/ses-perspective]]

## Sources

- [[summaries/modeling-simulation-systems-05-3-system-entity-structure-basics]]
