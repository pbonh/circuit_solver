---
title: 'Modeling and Simulation of Systems — Chapter 6: Aspects and Multi-aspects'
type: source
id: summaries/modeling-simulation-systems-08-6-aspects-and-multi-aspects
kind: publication
tags:
- simulation
- modeling
- ses
- multi-aspect
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/08-6-aspects-and-multi-aspects.txt
---

## Key Points

- The same entity can have multiple aspects (decompositions) — e.g., MSProcessSystem has both `process` (full waterfall) and `fastProcess` (skip data gathering, retrieve model from repository) aspects.
- During pruning, exactly one aspect must be chosen per entity, generating distinct families of PESs.
- Aspects often correspond to different perspectives or abstractions (e.g., game-from-player view vs. host-from-network view); a third aspect can compose the others for cross-cutting analysis.
- Aspects require fixed entity lists; multi-aspects address the need for variable cardinality with: `From the multiPerson perspective, People are made of more than one Person!`.
- Multi-aspect restructuring uses an index specialization (`Person can be id in index!`) plus pruning directives (`restructure multiaspects using index!`, `set multiplicity of index as [3] for Person!`) to generate id0_Person, id1_Person, id2_Person and inject them into the parent aspect.
- Specializations on the multi-entity are inherited by each generated copy; per-copy pruning is allowed (`select medium from height for id1_Person!`).
- Uniform-coupling syntax: `all Person sends Hello to all Person!` (all-to-all, excluding self), `each Boy sends Invitation to each Girl!` (each-to-each with prefix matching), one-to-all, and all-to-one. These drastically reduce coupling data entry.
- Multi-aspects can be nested hierarchically — example MultiEnclaveNet with multiple Enclaves each containing a DBServer and multiple Clients, all coupled with all-to-all DBUpdate broadcast across enclaves.
- Predefined coupling specifications (`circle`, `cyclic`, `cellular`, `tree`) generate complex network digraphs from a single statement; cellular supports multidimensional Moore neighborhoods (`set multiplicity of location as [10,10] for cell!`).
- Custom coupling specifications can be declared by adding a sibling entity with suffix `CouplingSpecification`, listing edges with `add coupling NAME from id_i to id_j in index!`, and assigning uniform port-to-port couplings with `for NAME leftnode sends MSG to rightnode!`.
- A Monorail example illustrates cyclic-coupling specification with leftnode/rightnode car/GoAhead message pairing.

## Relevant Concepts

- [[concepts/ses-aspect]] — multiple decompositions per entity.
- [[concepts/ses-multi-aspect]] — variable-cardinality decomposition with multi-entity.
- [[concepts/ses-uniform-coupling]] — all/each-based coupling that scales without manual enumeration.
- [[concepts/ses-coupling-specification]] — pluggable network digraph specifications (circle, cellular, tree).
- [[concepts/cellular-automaton-coupling]] — multidimensional Moore-neighborhood pattern via cellular specification.
- [[concepts/system-entity-structure]] — overall host.
- [[concepts/ses-pruning]] — chooses aspect and multi-aspect cardinality.
- [[concepts/pruned-entity-structure]] — output of pruning.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 6 — Aspects and Multi-aspects
- File path: `raw/ModelingAndSimulationOfSystems/_txt/08-6-aspects-and-multi-aspects.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
