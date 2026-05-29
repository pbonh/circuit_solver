---
title: GALS Design Pattern
type: claim
id: claim-gals-design-pattern
tags:
- simulation
- modeling
- hardware
- fpga
- low-power
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/22-18-activity-based-implementations-of-systems-of-systems.txt
confidence:
  base: 0.65
---

## Definition

The Globally Asynchronous, Locally Synchronous (GALS) design pattern partitions a digital system into clock domains, each running on its own local clock, with asynchronous handshaking between domains. In activity-based DEVS hardware synthesis, each DEVS atomic component becomes a GALS clock domain, enabling per-component frequency assignment and clock gating.

## How It Works

Each DEVS atomic model is mapped to a synchronous logic block driven by a Domain Clock Module (DCM). Inter-domain communication uses asynchronous handshaking protocols. Clock gating disables clocks during passive phases; frequency scaling assigns lower clock rates to less-active components. The methodology exploits DEVS's explicit phase structure to identify clock-gating opportunities automatically.

## Key Parameters

- Number of clock domains
- Per-domain clock frequency
- Inter-domain handshake protocol
- Clock-gating policy

## When To Use

- FPGA implementation of DEVS-modeled SoS
- ASIC and SoC low-power designs
- Cyber-physical systems with mixed-activity components

## Risks & Pitfalls

- Asynchronous handshake correctness requires careful timing analysis
- Domain crossing introduces metastability concerns
- Limited DCM availability requires multi-component-per-DCM partitioning

## Related Concepts

- [[concepts/clock-gating]]
- [[concepts/devs-hardware-synthesis]]
- [[concepts/activity-based-modeling]]

## Sources

- [[summaries/modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems]]
