---
title: On-Chip Voltage Regulator
type: claim
id: claim-on-chip-voltage-regulator
tags:
- vlsi
- power-integrity
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt
confidence:
  base: 0.65
---

## Definition

An on-chip voltage regulator is an integrated DC-DC or DC-LDO converter placed within the IC die that supplies and regulates a local supply voltage close to the load circuitry. Three main types are switched-mode power supply (SMPS), switched capacitor (SC), and low-dropout linear regulator (LDO).

## How It Works

SMPS converters use an LC stage with duty-cycle modulation — highest efficiency but largest area. SC converters use capacitor charge transfer — medium efficiency, medium area, but worse regulation. LDO regulators use a pass transistor with error-amplifier feedback — smallest area, fastest regulation, but limited efficiency bounded by V_out/V_in. Modern point-of-load applications favor LDOs near load circuitry.

## Key Parameters

- Maximum supply current.
- Efficiency.
- Settling time / load regulation bandwidth.
- Output voltage range.
- Silicon area.

## When To Use

- Multi-voltage-domain SoCs.
- Dynamic voltage/frequency scaling and power gating.
- Workload-sensitive high-performance digital and analog subsystems.

## Risks & Pitfalls

- Limited current capacity per regulator forces distribution across the die.
- Thermal effects can degrade regulation.
- Stability margins depend on output capacitor characteristics.

## Related Concepts

- [[concepts/voltage-regulator-placement]]
- [[concepts/heterogeneous-power-delivery]]
- [[concepts/power-distribution-network]]
- [[entities/ldo-regulator]]

## Sources

- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
