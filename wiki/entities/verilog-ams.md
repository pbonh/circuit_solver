---
title: Verilog-AMS
type: entity
id: entity-verilog-ams
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

Verilog-AMS is the analog and mixed-signal extension of Verilog-HDL — a single language that supports the description of both analog (continuous-time) and digital (event-driven) circuits and behaviors. It is one of the two industry-standard [[concepts/ahdl-mshdl]]s. Verilog-A (the analog-only subset) was approved by Open Verilog International (OVI) in June 1996; full Verilog-AMS (Verilog-MS) was approved in August 1998.

## Characteristics

- **Dual kernels**: an event-driven kernel (initial/always blocks, blocking statements) and a continuous-time kernel (analog blocks, evaluated once per timestep, no blocking).
- **Disciplines** declare port types — `electrical` (conservative: potential + flow) and `voltage` (signal-flow: potential only), plus user-defined disciplines for non-electrical domains (thermal, mechanical).
- **Analog operators** — `idt` (time integration), `ddt` (differentiation), `idtmod` (modular integration for phase-like quantities), `transition`/`slew` filters, Laplace and Z filters.
- **Events** — `cross()` (analog signal crossings), `timer()` (periodic or one-shot), `initial_step` / `final_step`.
- **@ blocks** — code executed on events, e.g. `@(posedge clk) hold = V(in);` inside an analog block to model a sampler.
- **Automatic interface element insertion** at analog↔digital port-type mismatches — user-specified interface modules resolve the mismatch.
- **Signal-flow / conservative port compatibility** — they freely interconnect and use the same syntactic style.
- **Automatic back-annotation of parasitics** — extracted resistances and capacitances are merged into the simulation.

## Common Strategies

- [[concepts/signal-flow-model]] for abstract top-level architectural blocks
- [[concepts/conservative-model]] for device models and interface boundaries
- [[concepts/mixed-level-simulation]] coupling between transistor blocks and surrounding Verilog-AMS testbenches
- Ideal switch modeling via `discontinuity(0)` to flag unsmooth events to the integrator

## Related Entities

- [[entities/vhdl-ams]]
- [[entities/spectre]]
- [[entities/ken-kundert]]
- [[entities/cadence]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
