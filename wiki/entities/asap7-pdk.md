---
title: ASAP7 PDK
type: entity
id: entity-asap7-pdk
tags:
- pdk
- predictive
- open-source
- asap7
- finfet
- digital
created: 2026-05-18
updated: 2026-05-18
sources:
- wiki/specs/circuit-solver
---

## Overview

The [ASAP7 PDK](https://github.com/The-OpenROAD-Project/asap7) is an open-source predictive 7 nm FinFET process design kit developed at Arizona State University in collaboration with ARM, distributed via the OpenROAD project. It is *predictive* — calibrated against published 7 nm node characteristics rather than a real silicon process — and intended for academic research and tool development.

## Characteristics

- 7 nm FinFET technology — analog device models are **BSIM-CMG** (common multi-gate), which is **out of v1 scope** for circuit-solver per [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]] (closed-enum dispatch capped at BSIM4-level).
- Ships standard-cell libraries and Verilog gate-level netlists suitable for digital flows.
- Used in this project for the **gate-level digital** portion of the conformance corpus only — its analog primitives are deferred to a future v2 spec.
- Distributed under permissive open-source licences; widely used by academic VLSI courses.

## Common Strategies

- [[concepts/event-trace-equivalence]] — ASAP7 gate-level Verilog produces a reference trace under [[entities/icarus-verilog]] for the v1 digital conformance corpus.

## Related Entities

- [[entities/icarus-verilog]] — Consumes ASAP7 gate-level Verilog for the digital reference.
- [[entities/sky130-pdk]] — The other open PDK in the v1 corpus, used for both analog and digital.
- [[entities/ngspice]] — *Not* used with ASAP7 in v1; the BSIM-CMG analog flow is deferred.
