---
title: 'Modeling and Simulation of Systems — Chapter 5: Specialization and Pruning'
type: source
id: source-modeling-simulation-systems-07-5-specialization-and-pruning
kind: derived-summary
tags:
- simulation
- modeling
- ses
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/07-5-specialization-and-pruning.txt
---

## Key Points

- Decomposition alone produces a single hierarchical model; specialization adds choice points so an SES generates a family of possible models.
- Specialization syntax: "ENTITY can be CHOICE1, CHOICE2, or CHOICE3 in SPECIALIZATION_LABEL!" (e.g., `getData can be immediateAccess, findInDataBase, or startResearch in meansToGetData!`).
- The pruning interface lets the modeler select one alternative at each specialization to produce a fully Pruned Entity Structure (PES) that maps to a concrete coupled DEVS model.
- With 3 specializations of 3 choices, 27 PES variants exist; the combinatorial space scales rapidly.
- Multiple-occurrence specializations: the same specialization label can appear under different entities and be pruned differently for each occurrence — this is the SES's "diversity through uniformity" property.
- Pruning order is flexible: top-down (brand first, then computePower) or bottom-up (computePower first, which implies a brand choice) generally converge to the same PES.
- There are no syntactic restrictions on where a specialization may be added: under the root, under an entity in an aspect (decomposition child), or under an entity in another specialization (taxonomies).
- Special specialization name "presence" (with choices Present/NotPresent) controls whether the parent entity appears in the PES — a structural toggle.
- Multiple sibling specializations under the same entity combine combinatorially (e.g., location × accessRights gives 4 combinations).
- Variables attached to entities (`A boy has height, weight, and eyeColor!`) with declared ranges (`The range of a boy's height is double with values [20, 72]!`) can be constrained per specialization choice (`Set short_boy's height to [20, 40]!`).
- During PES interpretation, variable values may be sampled randomly from the allowed range, enabling parameter-sweep experiments.

## Relevant Concepts

- [[concepts/system-entity-structure]] — the host formalism.
- [[concepts/ses-specialization]] — newly introduced construct.
- [[concepts/ses-pruning]] — selection of alternatives to derive a PES.
- [[concepts/pruned-entity-structure]] — outcome of pruning, ready for transformation to coupled model.
- [[concepts/ses-presence-specialization]] — Present/NotPresent toggle for structural inclusion.
- [[concepts/ses-variables]] — entity-level numeric and discrete value carriers.
- [[concepts/coupled-devs-model]] — eventual target of PES transformation.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 5 — Specialization and Pruning
- File path: `raw/ModelingAndSimulationOfSystems/_txt/07-5-specialization-and-pruning.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
