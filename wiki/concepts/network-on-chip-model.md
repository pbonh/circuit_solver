---
title: Network-on-Chip (NoC) Model
type: claim
id: claim-network-on-chip-model
tags:
- simulation
- modeling
- network-on-chip
- verification
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/20-16-model-development-and-execution-process-with-repositories-validation-and-verification.txt
confidence:
  base: 0.65
---

## Definition

Chapter 16 Sect. 16.5.3-16.5.5 uses the NoC as the canonical example of how data-exclusion shrinks the verification state space: "An example is Network-on-Chip (NoC) model (Gholami and Sarjoughian 2017). In this kind of system, data (called flits) is communicated among processing nodes, links, and switches, but not processed. This kind of data can be excluded which results in lowering the state space size which is a key factor for model verification. For NoC model verification, source and destination nodes (i.e., processing elements and switches) are used to define state space. Inclusion of data can result in unbounded state space (i.e., models can be validated, but verified if appropriately constrained)." Properties of interest include "total delay of flits or the distribution of flit traffic" verifiable via the DEVS-Suite simulator.

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
