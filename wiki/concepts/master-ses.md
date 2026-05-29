---
title: Master SES
type: claim
id: concepts/master-ses
tags:
- simulation
- modeling
- ses
- sos
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/17-13-flexible-modeling-support-environments.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A Master SES is the all-encompassing System Entity Structure that captures every component option, alternative architecture, and parameter range relevant to a Systems-of-Systems design problem. Pruning scripts derive concrete configurations from the Master SES; downstream simulators map from this master into their own specialized SES.

## How It Works

The Master SES uses multi-aspects and specializations to encode unbounded variety (e.g., SatelliteModule with id-indexed copies, Sensor presence/EMType/band/resolution specializations). Pruning scripts apply selections and count bounds (`set count bounds for SatelliteModule as [3,6] !`). The resulting PES is mapped to specialized model SESs (e.g., a Physical Model SES that distinguishes ImageSat vs. RelaySat).

## Key Parameters

- Multi-aspect cardinality bounds
- Specialization choice constraints
- Variable ranges
- Mappings to specialized PESs

## When To Use

- Capturing the entire design space of a complex SoS
- Seeding multiple downstream simulator abstractions consistently
- DARPA F6 / Frontier-style design environments

## Risks & Pitfalls

- Master SES can grow unwieldy; modular organization is essential
- Mapping yield may be low if domain is too unconstrained
- Versioning across consumers requires discipline

## Related Concepts

- [[concepts/system-entity-structure]]
- [[concepts/automated-pruning]]
- [[concepts/ses-to-ses-mapping]]
- [[concepts/fractionated-satellite-systems]]

## Sources

- [[summaries/modeling-simulation-systems-17-13-flexible-modeling-support-environments]]
