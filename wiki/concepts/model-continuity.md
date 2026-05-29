---
title: Model Continuity
type: claim
id: claim-model-continuity
tags:
- simulation
- modeling
- devs
- decision-support
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems.txt
confidence:
  base: 0.65
---

## Definition

Sect. 17.5: "Model continuity refers to the ability to transition as much as possible a model specification through the stages of a system of systems development process (Hu and Zeigler 2005). Its usefulness is clear in engineering, where it supports consistent artifacts among the design stages, from modeling and simulation to a concrete implementation of the system in hardware. Ideally, the same DEVS models employed in the testing of the controls can be transferred to implementation with only a change in the simulation engine." Chapter 17 extends this from engineering to living-systems contexts: "model continuity can be viewed as a back-and-forth dialog between the model and the real system. On the one hand, the modeler wants to learn about the system, on the other hand, the modeler wants to control or modify the system."

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
