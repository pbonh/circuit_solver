---
title: Cadence Design Systems
type: entity
id: entities/cadence
tags:
- analog
- mixed-signal
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
---

## Overview

Cadence Design Systems is an EDA (electronic design automation) company. Within the circuit-simulation context of this knowledge base, it is the developer of the [[entities/spectre]] simulator family, the home of [[entities/ken-kundert]], and the vendor of the mixed-signal design tool suite cited as the reference 3-month-design / no-respin productivity benchmark in Kundert's BCTM 1998 tutorial.

## Characteristics

- Develops Spectre (and SpectreHDL, SpectreRF), Verilog and VHDL simulators, and analog/digital design environments such as Virtuoso.
- A primary stakeholder in [[entities/verilog-ams]] standardization (OVI).
- Maintains a top-down-design + [[concepts/mixed-level-simulation]] flow integrating its simulator with its schematic / layout / verification tools.

## Common Strategies

- Spectre's KCL-check + voltage-LTE control as defaults
- Verilog-AMS as the canonical mixed-signal language
- High-capacity SPICE-level simulation feeding pin-accurate block models for system-level verification

## Related Entities

- [[entities/spectre]]
- [[entities/ken-kundert]]
- [[entities/verilog-ams]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
