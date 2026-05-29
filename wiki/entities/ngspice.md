---
title: Ngspice
type: entity
id: entity-ngspice
tags:
- eda
- simulation
- spice
- open-source
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/_reconciled
---

> No raw source in this knowledge base discusses ngspice substantively; this page exists because [[entities/spice]] and [[entities/hspice]] mention ngspice as a SPICE derivative. The content below is general-knowledge background consistent with those cross-references; treat the page as low-confidence until a raw source ingests ngspice-specific material.

## Overview

Ngspice is the open-source descendant of Berkeley SPICE3f5, maintained by a community at ngspice.sourceforge.io. It is one of the standard SPICE derivatives — alongside HSPICE, Spectre, LTspice, Eldo — used as a free, scriptable, batch-driven circuit simulator for analog, mixed-signal, and digital-as-mixed-signal designs.

## Characteristics

- Direct descendant of the Berkeley SPICE3f5 code base.
- DC, AC, transient, noise, distortion, pole/zero, sensitivity, and Monte Carlo analyses.
- BSIM (3, 4, 6) and EKV MOSFET models, MEXTRAM, HICUM, VBIC BJTs.
- Mixed-signal / digital co-simulation via XSPICE / event-driven extensions.
- Python and TCL scripting; KiCad bundles ngspice as its default simulator.

## Common Strategies

- Open-source counterpart to commercial HSPICE for academic / hobbyist analog design.
- Reference simulator for cross-checking commercial tool results.
- Backend for higher-level Python frameworks (e.g., PySpice).

## Related Entities

- [[entities/spice]] — parent simulator family.
- [[entities/hspice]] — commercial counterpart.

## Sources

- (No raw source — content drawn from public ngspice documentation referenced by SPICE/HSPICE wiki pages.)
