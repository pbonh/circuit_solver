---
title: DEVS Simulation Protocol
type: claim
id: concepts/devs-simulation-protocol
tags:
- simulation
- modeling
- devs
- distributed
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/12-9-devs-simulation-protocol.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The DEVS Simulation Protocol is the extension of the Abstract DEVS Simulator to networked environments. It specifies the interface that simulators present to a coordinator, and the iterative cycle by which a coordinator orchestrates a coupled-model simulation across one or more simulators (potentially distributed across hosts).

## How It Works

Four interface operations: `OperationGetTN()` (returns time of next event), `OperationGetOutput(t)` (returns output bag if imminent at `t`), `OperationStoreInput(m)` (delivers a composed input message), `OperationDoDelta()` (causes state transition). The coordinator iterates: collect TNs, compute minimum (advance global time), request outputs from imminent simulators, apply coupling specification to compose per-target input bags, deliver inputs, request transitions. The protocol allows multiple correct implementations (standard, peer-exchange, real-time).

## Key Parameters

- Coordinator coupling specification
- Per-simulator time of next event
- Composed input bag per simulator
- Implementation variant (standard, peer, real-time)

## When To Use

- All DEVS-based distributed simulations
- Federating DEVS and non-DEVS simulators in one experiment
- Building service-oriented or DDS-based simulation deployments

## Risks & Pitfalls

- Distributed implementations must preserve global-time discipline
- Real-time variant cannot recover lost time
- Message-bag ordering at simultaneous events impacts confluent semantics

## Related Concepts

- [[concepts/abstract-devs-simulator]]
- [[concepts/devs-coordinator]]
- [[concepts/peer-message-exchange]]
- [[concepts/real-time-devs-simulation]]
- [[concepts/simulation-interoperability]]

## Sources

- [[summaries/modeling-simulation-systems-12-9-devs-simulation-protocol]]
- [[summaries/modeling-simulation-systems-13-10-dynamic-structure-agent-modeling-and-publish-subscribe]]
