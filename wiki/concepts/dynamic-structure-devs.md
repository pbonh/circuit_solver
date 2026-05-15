---
title: "Dynamic Structure DEVS"
type: concept
tags: [simulation, modeling, devs, dynamic-structure, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/13-10-dynamic-structure-agent-modeling-and-publish-subscribe.txt"]
confidence: medium
---

## Definition

Dynamic Structure DEVS extends classical DEVS by allowing components and couplings of a coupled model to be added, removed, or modified at run time, while still respecting the underlying time-base and event semantics.

## How It Works

MS4 Me exposes the dynamic structure via methods callable in tagged code blocks: `addChildModel(model)`, `removeChildModel(model)`, and `addCoupling(source, srcport, destination, destport)`. An existing component invokes these on its parent (cast to `CoupledModelImpl`). The new component is initialized and participates in the next protocol cycle; new couplings take effect immediately.

## Key Parameters

- Component-add / component-remove operations
- Coupling-add / coupling-remove operations
- Caller identity (the existing component performing the structural change)
- Parent coupled model reference

## When To Use

- Agent-and-Actor patterns where agents accept and release mobile objects
- Publish/Subscribe routers wiring topic ports for new subscribers
- Open systems whose membership varies over the run
- Modeling cellular birth/death and reconfigurable architectures

## Risks & Pitfalls

- Structural changes can invalidate other components' assumptions
- Adding components during an event resolution requires careful ordering
- Save/restore of dynamic-structure state for re-runs is non-trivial

## Related Concepts

- [[concepts/coupled-devs-model]]
- [[concepts/devs-agent-modeling]]
- [[concepts/publish-subscribe]]
- [[concepts/atomic-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-13-10-dynamic-structure-agent-modeling-and-publish-subscribe]]
- [[summaries/modeling-simulation-systems-21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems]]
