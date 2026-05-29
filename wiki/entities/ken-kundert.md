---
title: Ken Kundert
type: entity
id: entity-ken-kundert
tags:
- analog
- mixed-signal
- rf
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
---

## Overview

Ken Kundert is a Cadence Design Systems fellow and a foundational figure in modern analog/mixed-signal circuit simulation. He is the principal architect of the [[entities/spectre]] circuit simulator family — Spectre, SpectreHDL, and SpectreRF — and made substantial contributions to the development of both [[entities/verilog-ams]] and [[entities/vhdl-ams]]. He also played a key role in the development of Hewlett-Packard's harmonic balance simulator.

## Characteristics

- **Books:**
  - *Steady-State Methods for Simulating Analog and Microwave Circuits* (1990)
  - *The Designer's Guide to SPICE and Spectre* (Kluwer Academic Publishers, 1995) — the canonical practitioner reference on getting reliable results out of SPICE-class simulators.
- **Academic background:** Ph.D., M.Eng., and B.S. in electrical engineering and computer sciences from UC Berkeley (1989, 1983, 1979) — specialized in circuit simulation and analog circuit design.
- **Author** of the BCTM 1998 tutorial *Simulation of Analog and Mixed-Signal Circuits* — the source for the foundational discussions of [[concepts/newton-raphson-method]] convergence, [[concepts/homotopy-method]] continuation aids, [[concepts/local-truncation-error]] control choices, [[concepts/numerical-damping]], and Spectre's design rationale.

## Common Strategies

- KCL-based convergence checking over SPICE's ΔI check (championed in Spectre)
- Voltage-domain LTE control over SPICE's charge-domain LTE
- [[concepts/top-down-design]] + [[concepts/mixed-level-simulation]] as the productivity lever for mixed-signal design

## Related Entities

- [[entities/spectre]]
- [[entities/cadence]]
- [[entities/verilog-ams]]
- [[entities/vhdl-ams]]
- [[entities/spice]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
