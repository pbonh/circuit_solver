---
title: "Peer Message Exchange (DEVS)"
type: concept
tags: [simulation, modeling, devs, distributed, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/12-9-devs-simulation-protocol.txt"]
confidence: medium
---

## Definition

Peer Message Exchange is a DEVS Simulation Protocol implementation variant in which the coupled-model coupling information is partitioned across simulators so that simulators exchange DEVS messages directly with each other, bypassing the coordinator for message routing.

## How It Works

The coordinator still issues GetTN, SendOutput, and DoDelta phases, but each simulator holds its outgoing-coupling segment and pushes outputs directly to peer simulators. After all simulators report MyDone, the coordinator triggers transitions. SES coupling: `all SimulatorPeer sends outMyOutput to all SimulatorPeer as inStoreInput!`.

## Key Parameters

- Per-simulator coupling segment
- MyDone synchronization message
- Reduced coordinator role

## When To Use

- Scaling DEVS simulations to many components without coordinator bottleneck
- Distributed deployments where peer connectivity is faster than star routing
- Service-oriented computing platforms

## Risks & Pitfalls

- Coupling-segment partitioning errors fragment routing
- All-to-all peer connectivity may be costly in some networks
- Synchronization barrier (MyDone) still required

## Related Concepts

- [[concepts/devs-simulation-protocol]]
- [[concepts/devs-coordinator]]
- [[concepts/real-time-devs-simulation]]

## Sources

- [[summaries/modeling-simulation-systems-12-9-devs-simulation-protocol]]
