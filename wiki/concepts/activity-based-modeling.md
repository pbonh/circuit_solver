---
title: Activity-Based Modeling
type: claim
id: claim-activity-based-modeling
tags:
- simulation
- modeling
- devs
- energy
- information
- emerging
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/00-preface.txt
confidence:
  base: 0.65
---

## Definition

Activity-based modeling is a DEVS-intrinsic approach to system design that integrates both energy and information-processing requirements into a unified simulation framework. The "activity" notion serves as a common currency for tracking computational work and the resource expenditure required to perform it.

## How It Works

Inside DEVS atomic and coupled models, activity counters measure how often state transitions fire, how many messages flow, and how much computation each component performs. These activity measurements are mapped to energy/resource cost models, enabling joint optimization of information-processing fidelity against energy budgets — analogous to how biological systems balance their cognitive functions against metabolic cost.

## Key Parameters

- Activity counters per DEVS component
- Energy/resource cost mapping functions
- Information vs. energy trade-off metric
- Aggregation across hierarchical model levels

## When To Use

- Designing systems that must emulate biological-style information/energy balancing
- Sensor networks and embedded SoS with tight energy budgets
- Cloud system co-design where computational load drives energy usage
- Any SoS scenario where treating compute as "free" is invalid

## Risks & Pitfalls

- Resource cost mappings can be coarse if not calibrated against real hardware
- Activity metrics depend on level of model abstraction
- Premature optimization for energy can distort functional fidelity

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/systems-of-systems]]

## Sources

- [[summaries/modeling-simulation-systems-00-preface]]
- [[summaries/modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems]]
