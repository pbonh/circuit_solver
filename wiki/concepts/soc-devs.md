---
title: "SOC-DEVS"
type: concept
tags: [simulation, modeling, devs, soa, cloud, co-design, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/19-15-cloud-system-simulation-modeling.txt"]
confidence: medium
---

## Definition

SOC-DEVS (Service-Oriented Computing DEVS) is the extension of the SOAD framework with explicit hardware modeling for cloud-system co-design. Software services and hardware components are modeled separately as DEVS atomic models, then composed with a service-to-hardware assignment specification.

## How It Works

Each software service maintains a list of operations, communicates via messages, and submits jobs (with CPU cycle and memory requirements) to its assigned hardware. Hardware models consume jobs at rates governed by CPU speed and memory limits; communication links between hardware nodes impose bandwidth and delay. SOC-DEVS supports both single-resource service co-location and networked multi-resource distribution.

## Key Parameters

- Service-context lifecycle
- Per-operation CPU cycle / memory load
- Per-link communication bandwidth and delay
- Mapping function from services to hardware
- Hierarchical software and hardware composition

## When To Use

- Cloud-system architecture exploration before deployment
- Voice communication or video streaming simulations
- Studying interactions between software adaptation policies and hardware contention

## Risks & Pitfalls

- Coarse hardware abstractions can mislead on real-system performance
- Mapping specification can be brittle when service/hardware counts vary
- DEVS-Suite SOC-DEVS package required for execution

## Related Concepts

- [[concepts/soad-framework]]
- [[concepts/cloud-system]]
- [[concepts/software-hardware-co-design]]
- [[concepts/service-context]]

## Sources

- [[summaries/modeling-simulation-systems-19-15-cloud-system-simulation-modeling]]
