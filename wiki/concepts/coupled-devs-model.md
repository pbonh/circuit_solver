---
title: Coupled DEVS Model
type: claim
id: claim-coupled-devs-model
tags:
- simulation
- modeling
- devs
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/04-2-devs-integrated-development-environments.txt
confidence:
  base: 0.85
---

## Definition

A Coupled DEVS Model specifies a composite dynamic system by listing its component models and the couplings (port-to-port connections) between them. Components may themselves be atomic or coupled, enabling hierarchical composition to arbitrary depth.

## How It Works

The coupled model declares external input couplings (from the parent's input ports to children's input ports), internal couplings (between children), and external output couplings (from children's output ports to the parent's output ports). The DEVS abstract simulator then carries out state transitions and routes messages across the coupling graph. Closure under coupling guarantees the coupled model is behaviorally equivalent to some atomic DEVS, supporting recursive hierarchy.

## Key Parameters

- Component set
- Internal couplings (Z functions / port maps)
- External input couplings
- External output couplings
- Tie-breaking / select function (classic DEVS)

## When To Use

- Building hierarchical SoS models
- Composing reusable atomic components into larger systems
- Generating structure from SES pruned entity structures

## Risks & Pitfalls

- Port-type mismatches between coupled components
- Forgetting select/tie-break semantics in classic DEVS
- Excessive nesting depth degrading readability

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/atomic-devs-model]]
- [[concepts/closure-under-coupling]]
- [[concepts/system-entity-structure]]

## Sources

- [[summaries/modeling-simulation-systems-04-2-devs-integrated-development-environments]]
- [[summaries/modeling-simulation-systems-05-3-system-entity-structure-basics]]
- [[summaries/modeling-simulation-systems-06-4-devs-natural-language-models-and-elaborations]]
- [[summaries/modeling-simulation-systems-07-5-specialization-and-pruning]]
- [[summaries/modeling-simulation-systems-12-9-devs-simulation-protocol]]
- [[summaries/modeling-simulation-systems-15-12-languages-for-constructing-devs-models]]
- [[summaries/modeling-simulation-systems-18-14-service-based-software-systems]]
