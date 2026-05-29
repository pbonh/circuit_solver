---
title: Activity Tracking
type: claim
id: concepts/activity-tracking
tags:
- simulation
- modeling
- devs
- activity
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/22-18-activity-based-implementations-of-systems-of-systems.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Activity Tracking is the DEVS-simulator technique of measuring, per component, the spatial-temporal distribution of state transitions. It enables performance optimization by focusing computation on the most active components and is also the basis for activity-based energy estimation.

## How It Works

The simulator increments per-component counters on each internal, external, and confluent transition. Aggregate activity statistics (rates, regional sums) are exposed through transducer ports. Used originally to speed up large spatial DEVS models with heterogeneous activity (crowds, fires; Ntaimo et al. 2008), now also used to estimate dynamic power consumption in hardware-synthesized DEVS implementations.

## Key Parameters

- Per-component transition count
- Sampling window
- Aggregation policy (regional, global)
- Activity-to-energy mapping (linear in frequency)

## When To Use

- Speeding up large spatial simulations (DEVS-FIRE, crowd dynamics)
- Pre-deployment energy estimation
- Driving low-power hardware synthesis decisions

## Risks & Pitfalls

- Counter overhead may distort simulation performance
- Coarse aggregation misses local hotspots
- Calibration to real hardware requires measurement

## Related Concepts

- [[concepts/activity-based-modeling]]
- [[concepts/discrete-event-system-specification]]
- [[concepts/devs-hardware-synthesis]]

## Sources

- [[summaries/modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems]]
