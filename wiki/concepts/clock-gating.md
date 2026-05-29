---
title: Clock Gating
type: claim
id: concepts/clock-gating
tags:
- hardware
- low-power
- fpga
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

Clock Gating is the low-power digital design technique of disabling the clock signal to logic blocks that are inactive, eliminating their dynamic switching power consumption. In DEVS-based hardware synthesis, clock gating is driven by the explicit phase structure of the DEVS atomic model — passive phases have their clocks gated off.

## How It Works

A gating logic element AND-combines the clock with an enable signal derived from the component's current phase. When in a passive phase, the enable is de-asserted and the clock to that domain is held low, preventing register-level toggling and the associated dynamic power. Hand-shaking re-asserts the clock when a new external input arrives.

## Key Parameters

- Enable derivation policy (per-phase, per-port)
- Clock-gate latency
- Glitch-free gating design (latch-based clock-enable)

## When To Use

- FPGA and ASIC low-power designs derived from DEVS models
- Sensor packages with bursty activity
- Adaptive quantizers and event-driven processing pipelines

## Risks & Pitfalls

- Improperly gated clocks can introduce glitches
- Coarse gating leaves savings on the table; fine-grained gating adds complexity
- Tool-chain support varies across FPGA vendors

## Related Concepts

- [[concepts/gals-design-pattern]]
- [[concepts/devs-hardware-synthesis]]
- [[concepts/activity-based-modeling]]

## Sources

- [[summaries/modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems]]
