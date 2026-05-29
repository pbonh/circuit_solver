---
title: Scale Transfer Modeling
type: claim
id: concepts/scale-transfer-modeling
tags:
- simulation
- modeling
- multi-scale
- living-systems
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Scale Transfer Modeling captures the interdependence between two organization levels of a living (or otherwise hierarchical) system by computing emerging properties from a lower scale that become parameters at an upper scale, and computing environmental constraints (initial conditions, global variables) from the upper scale that become inputs at the lower scale.

## How It Works

The modeler identifies a focal level (Aumann 2007) and the immediate scales above and below. Lower-scale DEVS components produce trajectories that are aggregated (statistical summaries, emergent properties) and fed to upper-scale components. Upper-scale components produce environmental constraints (temperature, population size) that propagate downward as inputs or parameters. DEVS hierarchical composition and dynamic-structure capabilities make this bidirectional coupling natural to formalize.

## Key Parameters

- Focal scale
- Aggregation rule (lower → upper)
- Constraint-projection rule (upper → lower)
- Coupling cadence between scales

## When To Use

- Cell → organ → organism transitions in physiology
- Individual → population → ecosystem ecological models
- Atomic-service → composite-service → system QoS aggregation
- Climate-change impact analyses across spatial scales

## Risks & Pitfalls

- Information loss in aggregation
- Cross-scale time-step mismatch
- Emergent properties may not be stable enough to act as parameters

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/emergence]]
- [[concepts/living-systems-modeling]]

## Sources

- [[summaries/modeling-simulation-systems-21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems]]
