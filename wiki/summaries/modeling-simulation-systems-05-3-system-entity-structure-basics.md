---
title: 'Modeling and Simulation of Systems — Chapter 3: System Entity Structure Basics'
type: source
id: source-modeling-simulation-systems-05-3-system-entity-structure-basics
kind: derived-summary
tags:
- simulation
- modeling
- ses
- devs
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/05-3-system-entity-structure-basics.txt
---

## Key Points

- The M&S process is itself modeled as an SES (MSProcessSystem) with five phase entities: ClarifyObjectivesStep, DataGatherStep, ConstructModelStep, ExecuteModelStep, InterpretResultsStep.
- A simple waterfall formulation (objectives → data → construct → execute → interpret) is used as a teaching scaffold; later chapters revisit with realistic iterative workflows.
- SES has two unique features highlighted here: decomposition (breaking entities into subentities) and coupling (specifying possible information flow between components).
- Decomposition statements take the form "From the X perspective, ENTITY is made of A, B, C!" — order of subentities is irrelevant.
- Coupling statements take the form "From the X perspective, A sends MESSAGE to B!" and define a possibility, not an actuality, of message flow.
- Three coupling kinds: external input coupling (parent → subcomponent), external output coupling (subcomponent → parent), and internal coupling (between subcomponents).
- A model is compatible with an SES entity if it can receive the inferred inputs and produce the inferred outputs; SES alone is insufficient to fully determine which input/output pairs are realized.
- A key decomposition rule: when decomposing an entity, external input/output couplings must agree on message names with the internal couplings of the parent at the next level.
- Hierarchical construction: any entity can itself be decomposed, producing nested coupled models.
- Worked example decompositions: DataGatherPhase (getData, validateData), ClarifyObjectivesPhase (clarifyRequirements, clarifyValues, clarifyWeights), ConstructModelPhase (defineModel, implementModel, calibrateModel, validateModel), ExecuteModelPhase (generateAlternatives, runExperiments), InterpretResultsPhase (evaluateAlternatives, rankAlternatives).
- Default auto-generated component behavior produces all outputs upon any input — modelers must restrict the input/output table to enforce intended semantics.

## Relevant Concepts

- [[concepts/system-entity-structure]] — central to the chapter.
- [[concepts/ses-decomposition]] — breaking an entity into subentities.
- [[concepts/ses-coupling]] — possibilities of message flow.
- [[concepts/ses-perspective]] — labeled axis (e.g., "process", "dataGather") for organizing decompositions.
- [[concepts/coupled-devs-model]] — generated artifact of SES processing.
- [[concepts/discrete-event-system-specification]] — formal target of SES-based code generation.
- [[entities/ms4-me]] — environment used to author and animate the SES.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 3 — System Entity Structure Basics
- File path: `raw/ModelingAndSimulationOfSystems/_txt/05-3-system-entity-structure-basics.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
