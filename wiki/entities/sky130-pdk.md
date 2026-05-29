---
title: Sky130 PDK
type: entity
id: entities/sky130-pdk
tags:
- pdk
- foundry
- open-source
- sky130
- analog
- digital
created: 2026-05-18
updated: 2026-05-18
sources:
- wiki/specs/circuit-solver
---

## Overview

The [SkyWater Sky130 PDK](https://github.com/google/skywater-pdk) is an open-source 130 nm process design kit jointly released by SkyWater Technology and Google. It ships device models, standard-cell libraries, IO cells, and analog primitives suitable for both analog and digital design, all under permissive open-source licences.

## Characteristics

- 130 nm planar CMOS technology — analog device models are BSIM4-family (compatible with the [[vision/circuit-solver]] device scope and [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]]).
- Multiple standard-cell libraries: `sky130_fd_sc_hd` (high-density), `sky130_fd_sc_hs` (high-speed), `sky130_fd_sc_ms` (medium-speed), `sky130_fd_sc_ls` (low-speed), etc.
- Analog primitives include NMOS / PMOS variants, diodes, resistors, capacitors, and inductors with characterised SPICE models.
- Distributed with both [[entities/ngspice]]-compatible analog model cards and Verilog cell-library descriptions usable by [[entities/icarus-verilog]].
- Designated by this project as the v1 PDK for analog ([[concepts/golden-reference]]) conformance and for the Sky130 portion of the digital conformance corpus.

## Common Strategies

- [[concepts/golden-reference]] — ngspice + Sky130 model cards produce the reference for DC, AC, transient, and noise spec scenarios.
- [[concepts/event-trace-equivalence]] — Sky130 gate-level Verilog produces a reference trace under [[entities/icarus-verilog]].

## Related Entities

- [[entities/ngspice]] — Consumes Sky130 analog model cards for the golden reference.
- [[entities/icarus-verilog]] — Consumes Sky130 gate-level Verilog for the digital reference.
- [[entities/asap7-pdk]] — The other open PDK in the v1 corpus, used digital-only.
