---
title: "Network-on-Chip (NoC) Model"
type: concept
tags: [simulation, modeling, network-on-chip, verification, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/20-16-model-development-and-execution-process-with-repositories-validation-and-verification.txt"]
confidence: low
---

## Definition

A Network-on-Chip (NoC) Model represents the communication fabric of a multi-processor system on chip as a set of processing nodes, links, and switches that route packets (flits) between them. In the SOC-DEVS verification context (Gholami and Sarjoughian 2017), NoC models exemplify how data exclusion can shrink the state space for model checking.

## How It Works

Flits are routed through the network but their payload data are not processed by the routing logic; only source and destination identifiers determine routing behavior. By excluding flit data from the state space, the model becomes amenable to Constrained DEVS verification. Properties of interest include total flit delay distribution, throughput, and absence of deadlock or livelock.

## Key Parameters

- Number of processing nodes, links, switches
- Routing policy
- Source/destination flit metadata
- Buffer sizes per switch

## When To Use

- Architecture-level verification of NoC designs
- Pre-silicon performance evaluation
- Comparing topologies (mesh, torus, fat-tree)

## Risks & Pitfalls

- Excluding data may hide payload-dependent congestion
- Verification still scales poorly with switch count
- Real systems have analog effects (clock skew) not captured here

## Related Concepts

- [[concepts/constrained-devs]]
- [[concepts/parallel-devs]]
- [[concepts/cosmos-framework]]

## Sources

- [[summaries/modeling-simulation-systems-20-16-model-development-and-execution-process-with-repositories-validation-and-verification]]
