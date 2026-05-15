---
title: "DEVS Hardware Synthesis"
type: concept
tags: [simulation, modeling, devs, hardware, fpga, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/22-18-activity-based-implementations-of-systems-of-systems.txt"]
confidence: medium
---

## Definition

DEVS Hardware Synthesis is the methodology of compiling DEVS coupled models into FPGA or ASIC implementations using the Globally Asynchronous Locally Synchronous (GALS) pattern with explicit clock gating and per-domain frequency scaling. The approach exploits DEVS's explicit timing semantics to minimize dynamic power consumption.

## How It Works

Each atomic DEVS model is mapped to a synchronous logic block driven by a Domain Clock Module (DCM). Coupling becomes wired connections with asynchronous handshaking between domains. Activity tracking during simulation provides per-component frequency assignment targets. Frequency-search algorithms enumerate alternative combinations, evaluating each for latency-constraint satisfaction and total energy consumption (Pifer 2012). Multiple atomic models can be co-located on a single DCM when DCM resources are limited.

## Key Parameters

- DCM count and frequency range
- Latency constraints per event pair
- Activity-to-frequency mapping
- Partitioning of atomic models among DCMs

## When To Use

- Low-power FPGA implementation of sensor/decision/action SoS
- Adaptive quantizer hardware
- Sustainable smart-building controllers

## Risks & Pitfalls

- Frequency-search complexity grows combinatorially
- Real-hardware power models must be calibrated against the linear-frequency assumption
- Asynchronous handshaking requires careful verification

## Related Concepts

- [[concepts/gals-design-pattern]]
- [[concepts/clock-gating]]
- [[concepts/activity-based-modeling]]
- [[concepts/activity-tracking]]
- [[concepts/model-continuity]]

## Sources

- [[summaries/modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems]]
