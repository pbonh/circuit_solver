---
title: "Model Continuity"
type: concept
tags: [simulation, modeling, devs, decision-support, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems.txt"]
confidence: low
---

## Definition

Model Continuity (Hu and Zeigler 2005) is the property that the same DEVS model used for research-mode simulation can be re-targeted to real-time execution and operational deployment with minimal modification — only the underlying simulator engine changes. It is enabled by the DEVS Protocol's clean separation of model and simulator.

## How It Works

A DEVS model is developed and validated against research scenarios. When the model is deployed for decision support, it executes on a real-time DEVS simulator engine (Chapter 9's real-time variant) and interfaces with real components via data-distribution middleware. The model's specification — atomic/coupled DEVS — remains unchanged; only the underlying engine and middleware shift.

## Key Parameters

- Model code unchanged between phases
- Simulator engine (research vs. real-time)
- Middleware (DDS, web services) for live integration
- Time-management discipline

## When To Use

- Decision-support systems built on top of research models
- Bridging laboratory experiments to fielded operations
- Long-term lifecycle of SoS models

## Risks & Pitfalls

- Real-time engine may not meet research-mode latency assumptions
- Middleware behavior in deployment may diverge from research-mode stubs
- Recalibration needed when transitioning to real data

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/virtual-build-and-test]]
- [[concepts/real-time-devs-simulation]]

## Sources

- [[summaries/modeling-simulation-systems-21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems]]
- [[summaries/modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems]]
