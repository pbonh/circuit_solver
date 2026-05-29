---
title: Voltage Domain
type: claim
id: concepts/voltage-domain
tags:
- vlsi
- power-integrity
- architecture
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A voltage domain (or power domain / rail) is a region of an integrated circuit supplied by a distinct power network at a specific voltage level. Modern SoCs typically partition into several voltage domains so that low-activity blocks can use lower supply voltages, reducing dynamic power quadratically.

## How It Works

Each voltage domain has its own VDD network, level-shifters at domain boundaries, and possibly its own clock distribution and decoupling capacitance. Domain partitioning is decided based on functional block voltage requirements, activity profiles, and power/noise trade-offs. Inter-domain communication requires level shifters and isolation cells when domains can be independently power-gated.

## Key Parameters

- Number of domains.
- Voltage and current per domain.
- Level-shifter delay and area overhead.
- Per-domain decap budget.

## When To Use

- High-performance SoCs with diverse functional blocks (CPU, GPU, memory, RF).
- Power-managed designs with dynamic voltage and frequency scaling per block.

## Risks & Pitfalls

- Merging incompatible voltage ranges shrinks the usable margin and forces larger decaps.
- Crossing-domain signal paths require careful isolation in power-gated scenarios.

## Related Concepts

- [[concepts/power-distribution-network]]
- [[concepts/heterogeneous-power-delivery]]
- [[concepts/voltage-regulator-placement]]
- [[concepts/power-delivery-exploration]]

## Sources

- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
