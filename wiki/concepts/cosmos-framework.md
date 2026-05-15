---
title: "CoSMoS Framework"
type: concept
tags: [simulation, modeling, devs, persistence, ide, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/20-16-model-development-and-execution-process-with-repositories-validation-and-verification.txt"]
confidence: medium
---

## Definition

The CoSMoS framework (Component-based System Modeler and Simulator) is a DEVS-based modeling environment that unifies logical, visual, and persistent model specifications. Logical models follow the DEVS formalism or XML Schema; visual models are hierarchical component-based diagrams; persistent models live in relational databases. CoSMoS automates partial code generation for the DEVS-Suite simulator.

## How It Works

A modeler authors templates with composition and specialization relationships in the database, then composes them visually into Instance Template Models and concrete Instance Models. Persistent storage lets the same template appear in many instance hierarchies. Code is generated for DEVS-Suite or XML Schema targets; transition/output/time-advance behaviors must be filled in for atomic models. CoSMoS supports XML data modeling and Cellular Automata extensions.

## Key Parameters

- Logical/visual/persistent model triad
- Template/Instance Template/Instance abstraction levels
- Composition and specialization constraints
- Database-backed persistence
- Code-generation target

## When To Use

- Large-scale SoS modeling with many model variants
- Collaborative model authoring across teams
- Disciplined model-validation and verification workflows
- SW/HW co-design with persistent specification

## Risks & Pitfalls

- Database schema migrations needed when the meta-model evolves
- Coupling to DEVS-Suite limits code-generation targets
- Visual-modeling productivity depends on diagram complexity

## Related Concepts

- [[concepts/template-instance-template-instance]]
- [[concepts/constrained-devs]]
- [[concepts/parallel-devs]]
- [[concepts/system-entity-structure]]

## Sources

- [[summaries/modeling-simulation-systems-20-16-model-development-and-execution-process-with-repositories-validation-and-verification]]
