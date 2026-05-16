---
title: "Emergence (Simulation)"
type: concept
tags: [simulation, modeling, complex-systems, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems.txt"]
confidence: medium
---

## Definition

Per Chapter 17, Sect. 17.2.1 ("A Systemic Approach: Emergence and Scale Transfer"): "One very important notion ABS [Agent-Based Simulation] brought to the simulation domain is emergence. Indeed, emergence is very important in life sciences where emergent properties are central to understand living organisms' behaviors. The emerging properties 'appear' when we simulate the model. They can be observed as the outputs of different types of simulators. When simulating ABS, DEVS simulators generate output trajectories for which emergence can be identified by the user."

## How It Works

The modeler defines local interaction rules at the agent or component level; emergent properties become visible when output trajectories of the composite model are analyzed. Chapter 17 anchors the DEVS-specific link to scale: "Emergence and scale transfer are related since the latter is a model of the interdependence of one organization level upon one other (Duboz et al. 2003). Discrete-events formalisms can be used to specify, and then to simulate, fast and slow processes of the same system in the same model. This ability is critical for asynchronous system modeling." However, the chapter also flags a hard case: "the interactions between fast and slow processes are not easily modeled when timescales are very different."

## Key Parameters

- Local interaction rules
- Population size and demography
- Observation/aggregation methods
- Simulation time horizon

## When To Use

- Life-sciences modeling where collective behavior matters
- Sociological/economic systems with feedback loops
- Studying robustness of patterns to parameter perturbations

## Risks & Pitfalls

- "Emergence" used loosely can obscure understanding
- Patterns may be artifacts of model approximations
- Quantifying emergence requires careful statistical framing

## Related Concepts

- [[concepts/agent-based-simulation]]
- [[concepts/scale-transfer-modeling]]
- [[concepts/living-systems-modeling]]

## Sources

- [[summaries/modeling-simulation-systems-21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems]]
