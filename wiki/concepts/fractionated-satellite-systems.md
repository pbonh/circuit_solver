---
title: Fractionated Satellite Systems
type: claim
id: claim-fractionated-satellite-systems
tags:
- simulation
- modeling
- applications
- satellites
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/17-13-flexible-modeling-support-environments.txt
confidence:
  base: 0.65
---

## Definition

Fractionated Satellite Systems replace traditional monolithic satellites with networked clusters of smaller modular satellites whose individual components (sensing, communication, processing, propulsion) are distributed across multiple platforms and linked by intersatellite networks. The DARPA System F6 program (Future, Fast, Flexible, Fractionated, Free-Flying Spacecraft) explored this paradigm.

## How It Works

A cluster is defined by which modules each satellite carries and how the satellites communicate. Modules include Sensing (visual, infrared, radar), Communication (uplink/downlink, intersatellite, processing payload), Energizing, Propulsing, Navigating, and Controlling. The benefits sought include adaptability, reliability, in-orbit reconfigurability, and resilience to single-platform failure.

## Key Parameters

- Cluster size (number of satellites)
- Per-satellite module composition
- Intersatellite vs. ground-link communication
- Sensor type and resolution
- Mean-time-to-failure and fault-tolerance levels

## When To Use

- Comparing fractionated vs. monolithic satellite architectures
- Design-space exploration in space-mission planning
- Reliability/resilience analysis under module failures

## Risks & Pitfalls

- Combinatorial explosion in cluster configurations
- Interservice latency and protocol overhead
- Real-world technology readiness for in-space module exchange

## Related Concepts

- [[concepts/master-ses]]
- [[concepts/modeling-support-environment]]
- [[concepts/systems-of-systems]]

## Sources

- [[summaries/modeling-simulation-systems-17-13-flexible-modeling-support-environments]]
