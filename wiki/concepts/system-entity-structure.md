---
title: "System Entity Structure (SES)"
type: concept
tags: [simulation, modeling, ontology, foundational, well-established, ses]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/00-preface.txt"]
confidence: high
---

## Definition

System Entity Structure (SES) is a high-level ontology framework targeted to modeling, simulation, systems design, and engineering. An SES is a formal structure governed by a small number of axioms that provide clarity and rigor to its models, supporting hierarchical and modular compositions so that large complex structures can be built stepwise from smaller simpler ones.

## How It Works

SES encodes alternative system architectures and component variants as a labeled tree-like structure with entities, aspects, specializations, and multi-aspects. Pruning selects from the alternative space to produce a Pruned Entity Structure (PES) that can then be transformed into an actual simulation model by associating leaf entities with DEVS atomic models and internal nodes with DEVS coupled-model compositions. SES is the companion ontology to DEVS in the integrated modeling pipeline.

## Key Parameters

- Entities (system components)
- Aspects (decompositions)
- Specializations (alternatives)
- Multi-aspects (variable cardinality)
- Axioms for well-formedness (uniformity, strict hierarchy, alternating mode, valid brothers, etc.)
- Pruning operations to derive simulatable PES

## When To Use

- Organizing model repositories for reuse and combinatorial design exploration
- Capturing families of alternative architectures for a Systems-of-Systems study
- Bridging requirements analysis to DEVS atomic/coupled simulation models
- Driving model-development workflows in MS4 Me and related DEVS environments

## Risks & Pitfalls

- The axioms require care; ill-formed SES can produce invalid pruned structures
- Without good tooling, SES authoring can be tedious
- Naming conventions and inheritance rules require discipline to remain comprehensible

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/systems-of-systems]]
- [[concepts/virtual-build-and-test]]

## Sources

- [[summaries/modeling-simulation-systems-00-preface]]
- [[summaries/modeling-simulation-systems-02-basic-concepts]]
- [[summaries/modeling-simulation-systems-03-1-modeling-and-simulation-of-systems-of-systems]]
- [[summaries/modeling-simulation-systems-04-2-devs-integrated-development-environments]]
- [[summaries/modeling-simulation-systems-05-3-system-entity-structure-basics]]
- [[summaries/modeling-simulation-systems-07-5-specialization-and-pruning]]
- [[summaries/modeling-simulation-systems-08-6-aspects-and-multi-aspects]]
- [[summaries/modeling-simulation-systems-11-advanced-concepts]]
- [[summaries/modeling-simulation-systems-14-11-interest-based-information-exchange-mappings-and-models]]
- [[summaries/modeling-simulation-systems-15-12-languages-for-constructing-devs-models]]
- [[summaries/modeling-simulation-systems-16-applications]]
- [[summaries/modeling-simulation-systems-17-13-flexible-modeling-support-environments]]
- [[summaries/modeling-simulation-systems-20-16-model-development-and-execution-process-with-repositories-validation-and-verification]]
