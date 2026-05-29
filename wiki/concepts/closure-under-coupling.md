---
title: Closure Under Coupling
type: claim
id: claim-closure-under-coupling
tags:
- simulation
- modeling
- devs
- foundational
- well-established
- theory
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/04-2-devs-integrated-development-environments.txt
confidence:
  base: 0.85
---

## Definition

Closure under coupling is the formal property of DEVS stating that any coupled DEVS model is behaviorally equivalent to an atomic DEVS model. The dynamic system specified by composing atomic components and couplings is itself in the class of dynamic systems specifiable by a single atomic DEVS.

## How It Works

The theory constructs an equivalent atomic model whose state aggregates the states and elapsed times of all components and whose transition functions encode the internal/external coupling-routing behavior. Because of this, a coupled model can be substituted as a component in a larger coupled model without loss of expressiveness — the bedrock of hierarchical DEVS modeling.

## Key Parameters

- Equivalent aggregated state
- Aggregated time-advance (minimum of components' next-event times)
- Routed transitions via internal couplings

## When To Use

- Justifying hierarchical composition of DEVS models
- Foundation for the Abstract DEVS Simulator algorithm
- Reasoning about correctness of nested SoS models

## Risks & Pitfalls

- Property does not eliminate the need for select functions in classic DEVS for simultaneous events
- Equivalent atomic model is mathematical; not necessarily efficient to materialize

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/atomic-devs-model]]
- [[concepts/coupled-devs-model]]
- [[concepts/devs-universality-and-uniqueness]]

## Sources

- [[summaries/modeling-simulation-systems-04-2-devs-integrated-development-environments]]
