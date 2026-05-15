---
title: "Emergence (Simulation)"
type: concept
tags: [simulation, modeling, complex-systems, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems.txt"]
confidence: low
---

## Definition

Emergence in simulation refers to system-level properties that "appear" only when individual components are simulated together — they are not directly programmed into any component but arise from the interactions among components. Examples include flocking, epidemic curves, and morphogenesis.

## How It Works

The modeler defines local interaction rules at the agent or component level; emergent properties become visible when output trajectories of the composite model are analyzed. In DEVS-based ABS, the simulator generates trajectories from which the user (or post-processing algorithms) identifies emergent patterns. Emergence is closely tied to scale-transfer modeling, since emergent properties at one scale can serve as parameters at the next.

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
