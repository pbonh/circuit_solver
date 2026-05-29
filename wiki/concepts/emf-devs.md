---
title: EMF-DEVS
type: claim
id: claim-emf-devs
tags:
- simulation
- modeling
- devs
- emf
- eclipse
- emerging
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/15-12-languages-for-constructing-devs-models.txt
confidence:
  base: 0.65
---

## Definition

Per Chapter 12: "DEVS simulation model development can also be defined using meta-modeling. The Eclipse Modeling Framework (EMF) is a meta-level modeling environment (Steinberg et al. 2008). It is used to introduce EMF-DEVS approach (Sarjoughian and Markid 2012). EMF-DEVS modeling environment supports meta-level atomic and coupled DEVS model specifications defined in terms of the Parallel DEVS formalism. It can be used to develop domain-specific models. A key advantage of the EMF-DEVS is adding constraints for user-defined models atop pre-defined ones for the generic atomic and coupled EMF-DEVS models. These constraints can help automate validation of structural properties of atomic and coupled model before generation of concrete simulation models for a target tool such as the DEVS-Suite simulator."

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
