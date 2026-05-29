---
title: SES Multi-aspect
type: claim
id: claim-ses-multi-aspect
tags:
- simulation
- modeling
- ses
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/08-6-aspects-and-multi-aspects.txt
confidence:
  base: 0.85
---

## Definition

An SES Multi-aspect is a variable-cardinality aspect that has exactly one child — its multi-entity — which generates a specified number of replicated copies at pruning time. Multi-aspects let an SES describe systems with an unbounded number of similar components without enumerating each by name.

## How It Works

Syntax: `From the LABEL perspective, PARENT are made of more than one CHILD!` plus an index specialization `CHILD can be id in index!`. The pruning script directs `restructure multiaspects using index!` and `set multiplicity of index as [N] for CHILD!`, producing N replicated entities id0_CHILD, id1_CHILD, ..., id(N-1)_CHILD that are injected as siblings of the multi-aspect's parent. Specializations attached to the multi-entity are inherited by each copy and can be pruned independently per-copy.

## Key Parameters

- Multi-entity name
- Index specialization
- Multiplicity per multi-aspect
- Per-copy specialization choices
- Nested multi-aspects for hierarchical replication

## When To Use

- Modeling populations (people, dogs, satellites, computers)
- Cellular and grid-based simulations
- Database enclaves with variable number of clients per enclave

## Risks & Pitfalls

- All copies share the same template — variable cardinality per parent requires context-sensitive pruning
- Multiplicity must be set before transformation
- Bottom-up restructuring order required when nested multi-aspects use different index names

## Related Concepts

- [[concepts/ses-aspect]]
- [[concepts/ses-uniform-coupling]]
- [[concepts/ses-coupling-specification]]
- [[concepts/system-entity-structure]]

## Sources

- [[summaries/modeling-simulation-systems-08-6-aspects-and-multi-aspects]]
