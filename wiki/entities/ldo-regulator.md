---
title: LDO Regulator
type: entity
id: entities/ldo-regulator
tags:
- vlsi
- power-integrity
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt
---

## Overview

A Low-Dropout (LDO) regulator is a linear DC voltage regulator whose minimum input-to-output voltage difference ("dropout") is small (typically tens to hundreds of millivolts). LDOs are widely used as on-chip point-of-load voltage regulators because of their small area and fast load-regulation bandwidth, despite limited power-conversion efficiency bounded by V_out / V_in.

## Characteristics

- Linear regulation via a pass transistor (typically PMOS or NMOS).
- Error amplifier in negative-feedback configuration sets the output voltage.
- Output bypass capacitor for stability and transient response.
- Power efficiency η ≈ V_out / V_in (no boost; lossy at high V_in / V_out ratios).
- Fast load step response — micro to nanosecond range with appropriate compensation.
- Per-LDO maximum current limited by pass-transistor sizing and electromigration.

## Common Strategies

- Distribute many small LDOs across the die rather than relying on one large regulator (see voltage regulator placement, Chapter 8).
- Combine with off-chip SMPS in a heterogeneous power delivery scheme to balance efficiency and regulation quality.
- Use multiple voltage domains with per-domain LDOs.

## Related Entities

- [[concepts/on-chip-voltage-regulator]]
- [[concepts/heterogeneous-power-delivery]]
- [[concepts/voltage-regulator-placement]]
- [[concepts/power-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
