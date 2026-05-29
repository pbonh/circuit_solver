---
title: SES Coupling Specification
type: claim
id: claim-ses-coupling-specification
tags:
- simulation
- modeling
- ses
- multi-aspect
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/08-6-aspects-and-multi-aspects.txt
confidence:
  base: 0.65
---

## Definition

An SES Coupling Specification is a named, pluggable scheme that combines a network-connectivity digraph (which pairs of entities are connected) with a uniform port-to-port coupling rule (what messages flow between connected pairs). Coupling specifications are introduced as sibling entities of the multi-aspect's parent, with a name ending in `CouplingSpecification`.

## How It Works

The modeler declares an entity such as `circleCouplingSpecification` as a sibling of the multi-aspect's parent. In the pruning script, edges are listed via `add coupling NAME from id_i to id_j in index!`, or a predefined builder is invoked via `write cyclic/cellular/tree specification for X and index based on NAME!`. Port-to-port couplings are then attached uniformly with `for NAME leftnode sends MSG to rightnode!` (or the reverse), and the rule is applied to every edge in the digraph.

## Key Parameters

- Specification name (suffix `CouplingSpecification`)
- Network connectivity (explicit edges or predefined topology)
- Predefined topologies: cyclic, cellular, tree
- Uniform leftnode-rightnode port couplings

## When To Use

- Cellular automata with regular grids and Moore neighborhoods
- Tree-structured agent networks
- Cyclic processes like a monorail loop
- Custom digraphs declared edge-by-edge

## Risks & Pitfalls

- Manual edge lists do not scale; predefined specs preferred when applicable
- Multidimensional multiplicity required for cellular ([10,10])
- Misspelled specification suffixes are silently ignored

## Related Concepts

- [[concepts/ses-multi-aspect]]
- [[concepts/ses-uniform-coupling]]
- [[concepts/cellular-automaton-coupling]]

## Sources

- [[summaries/modeling-simulation-systems-08-6-aspects-and-multi-aspects]]
