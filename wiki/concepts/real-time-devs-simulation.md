---
title: "Real-Time DEVS Simulation"
type: concept
tags: [simulation, modeling, devs, real-time, distributed, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/12-9-devs-simulation-protocol.txt"]
confidence: medium
---

## Definition

Real-Time DEVS Simulation is a DEVS Simulation Protocol implementation variant in which simulators self-schedule next events to occur in wall-clock real time and exchange messages peer-to-peer; the coordinator's role reduces to starting and stopping the run.

## How It Works

Each simulator holds in its current state for the time interval returned by its model's time-advance function. When that interval elapses (in real time), it invokes the output function, sends messages to peer simulators, and triggers its transition. External inputs from peers cause immediate external transitions. Logical-time coordination is replaced by physical-time elapsing.

## Key Parameters

- Wall-clock time base
- Per-simulator self-scheduling
- StartUp / Stop control from coordinator
- Real-time time-advance semantics

## When To Use

- Hardware-in-the-loop simulation
- Network-on-Chip and cyber-physical DEVS deployments
- Interactive simulations where physical time matters

## Risks & Pitfalls

- Cannot recover lost real time if simulator falls behind
- Drift across distributed hosts requires careful clock synchronization
- Not suitable when logical-time speedups are desired

## Related Concepts

- [[concepts/devs-simulation-protocol]]
- [[concepts/peer-message-exchange]]
- [[concepts/devs-coordinator]]

## Sources

- [[summaries/modeling-simulation-systems-12-9-devs-simulation-protocol]]
