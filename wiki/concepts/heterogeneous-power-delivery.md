---
title: "Heterogeneous Power Delivery"
type: concept
tags: [vlsi, power-integrity, architecture]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt"]
confidence: medium
---

## Definition

Heterogeneous power delivery is a multi-stage power architecture that combines a high-efficiency off-chip power management IC (PMIC) with multiple on-chip voltage regulators placed close to the load. The off-chip PMIC handles bulk power conversion; on-chip regulators handle fine-grained regulation at the point of load.

## How It Works

The PMIC supplies one or several intermediate voltages through the board and package to the IC. Inside the IC, multiple on-chip regulators (typically LDOs or small SC/SMPS) re-regulate to the final supply voltages for individual voltage domains. The shorter distance and lower impedance between on-chip regulators and load circuitry suppress dynamic noise, improve load regulation bandwidth, and enable fine-grained voltage scaling and power gating.

## Key Parameters

- Number and locations of on-chip regulators.
- Intermediate vs. final supply voltages.
- Per-domain current demand and noise budget.

## When To Use

- Modern high-performance SoCs with many voltage domains.
- Designs requiring fast dynamic voltage scaling or power gating.

## Risks & Pitfalls

- Increased on-chip area overhead.
- Stability of cascaded regulation requires careful loop design.

## Related Concepts

- [[concepts/on-chip-voltage-regulator]]
- [[concepts/voltage-regulator-placement]]
- [[concepts/power-distribution-network]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
