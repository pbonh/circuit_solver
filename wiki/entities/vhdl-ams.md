---
title: VHDL-AMS
type: entity
id: entities/vhdl-ams
tags:
- mixed-signal
- analog
- digital
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
---

## Overview

VHDL-AMS is the analog and mixed-signal extension of VHDL, standardized as IEEE 1076.1 in the summer of 1998. Unlike Verilog, there is no analog-only subset for VHDL — VHDL-AMS is a single language supporting both analog and digital descriptions. It is one of the two industry-standard [[concepts/ahdl-mshdl]]s.

## Characteristics

- **Dual kernels**: discrete-event for digital, continuous-time for analog.
- **Single unified language** — analog statements and digital statements coexist; there is no separate analog-only profile.
- **Lacks** Verilog-AMS's automatic interface-element insertion at A↔D port-type mismatches — designers must declare bridging components explicitly.
- **Lacks** Verilog-AMS's automatic back-annotation of parasitics.
- Otherwise broadly comparable expressive power for circuit and system modeling.

## Common Strategies

- [[concepts/signal-flow-model]] and [[concepts/conservative-model]] coexist as in Verilog-AMS, but interface bridging is more verbose.
- [[concepts/mixed-level-simulation]] in flows that prefer VHDL as the primary HDL.

## Related Entities

- [[entities/verilog-ams]]
- [[entities/spectre]]
- [[entities/ken-kundert]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
