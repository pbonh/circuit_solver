---
title: "Agent-Based Simulation (ABS)"
type: concept
tags: [simulation, modeling, agent-based, devs, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems.txt"]
confidence: medium
---

## Definition

Agent-Based Simulation (ABS), also called Individual-Based Modeling (IBM), simulates a system by modeling autonomous agents that interact with each other and with a shared environment. ABS is widely used in life-science M&S, but suffers from a lack of common formalization; DEVS-based formalizations (Duboz et al. 2006) standardize the agent/environment contract while preserving emergence.

## How It Works

Each agent is wrapped as a DEVS atomic model with state, input/output ports, and time-advance functions. The environment is itself an atomic or coupled model. Agent populations are managed with dynamic-structure DEVS, supporting birth/death/mutation. Outputs of the simulation are time trajectories from which emergent properties (e.g., flocking, epidemic spread) can be identified.

## Key Parameters

- Per-agent state and behavior rules
- Environment dynamics
- Population size and demographics
- Communication topology

## When To Use

- Animal epidemiology and herd dynamics
- Plant-population modeling
- Sociological and economic simulations
- Robot swarms

## Risks & Pitfalls

- Sensitivity to initial conditions
- Calibration requires individual-level data
- Communication of model assumptions across teams

## Related Concepts

- [[concepts/devs-agent-modeling]]
- [[concepts/dynamic-structure-devs]]
- [[concepts/emergence]]
- [[concepts/living-systems-modeling]]

## Sources

- [[summaries/modeling-simulation-systems-21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems]]
