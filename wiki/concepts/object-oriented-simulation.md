---
title: Object-Oriented Simulation
type: claim
id: claim-object-oriented-simulation
tags:
- simulation
- modeling
- object-oriented
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/04-2-devs-integrated-development-environments.txt
confidence:
  base: 0.65
---

## Definition

Object-oriented simulation is the implementation paradigm that maps simulation entities (atomic models, coupled models, messages, ports, schedulers) to objects in an object-oriented programming language. DEVS and OO are described as an "ideal marriage" in the Zeigler/Sarjoughian guide, with DEVS providing the formal framework and OO providing a steadily evolving wealth of implementation platforms.

## How It Works

Each atomic DEVS model becomes a class with state instance variables and transition/output/time-advance methods. Coupled models become container objects holding component objects and coupling tables. Messages and ports are typed objects. Modern DEVS implementations exist in Java, C++, .NET, web-service environments, and increasingly cloud-native runtimes.

## Key Parameters

- Class-per-atomic-model mapping
- Inheritance for model families and specializations
- Message and port types
- Container/composite pattern for coupled models

## When To Use

- All practical DEVS implementations today
- Bridging DEVS theory to enterprise software stacks
- Reusing object-oriented libraries (collections, network, persistence) inside simulation code

## Risks & Pitfalls

- Inheritance-heavy designs can obscure DEVS semantics
- Polymorphism can hide cost in performance-critical inner loops
- Object-identity vs. value-semantics mismatch with mathematical DEVS definitions

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/atomic-devs-model]]
- [[concepts/coupled-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-04-2-devs-integrated-development-environments]]
- [[summaries/modeling-simulation-systems-06-4-devs-natural-language-models-and-elaborations]]
- [[summaries/modeling-simulation-systems-09-7-managing-inheritance-in-pruning]]
