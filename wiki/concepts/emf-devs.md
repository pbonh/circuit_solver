---
title: "EMF-DEVS"
type: concept
tags: [simulation, modeling, devs, emf, eclipse, emerging]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/15-12-languages-for-constructing-devs-models.txt"]
confidence: low
---

## Definition

EMF-DEVS is a meta-level Parallel DEVS modeling environment built on the Eclipse Modeling Framework (EMF). It supports meta-level atomic and coupled DEVS model specifications and lets domain modelers add structural constraints on top of the predefined generic atomic and coupled meta-models.

## How It Works

EMF provides a meta-modeling language (Ecore) and code-generation facilities. EMF-DEVS defines Parallel DEVS atomic and coupled meta-classes; users derive domain-specific meta-models that constrain port types, allowed state transitions, and coupling topologies. These constraints are checked automatically before generating concrete simulation models for a target tool such as DEVS-Suite.

## Key Parameters

- Parallel DEVS meta-classes (atomic, coupled)
- Domain-specific constraint language
- EMF Ecore meta-model
- Generation target (DEVS-Suite, MS4 Me, etc.)

## When To Use

- Developing domain-specific DEVS modeling languages
- Enforcing modeling rules across a team or organization
- Automating validation before simulation-model generation

## Risks & Pitfalls

- Steep ramp-up for users not familiar with EMF
- Tool coupling to Eclipse
- Constraint expressiveness limited to what Ecore/OCL supports

## Related Concepts

- [[concepts/parallel-devs]]
- [[concepts/discrete-event-system-specification]]
- [[concepts/devs-uml-mapping]]

## Sources

- [[summaries/modeling-simulation-systems-15-12-languages-for-constructing-devs-models]]
