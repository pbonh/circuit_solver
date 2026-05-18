---
title: "Icarus Verilog"
type: entity
tags: [digital, simulator, verilog, open-source]
created: 2026-05-18
updated: 2026-05-18
sources: ["wiki/specs/circuit-solver"]
confidence: low
---

## Overview

[Icarus Verilog](https://steveicarus.github.io/iverilog/) (commonly `iverilog`) is an open-source Verilog HDL compilation and simulation toolchain. It compiles IEEE 1364 Verilog and a working subset of SystemVerilog into an intermediate form executed by the `vvp` runtime, producing a [[concepts/value-change-dump]] (VCD) of every monitored signal.

## Characteristics

- Event-driven digital simulation kernel — advances simulation time to the next scheduled event rather than uniformly.
- Outputs standard VCD traces consumable by GTKWave, Surfer, and programmatic diff tools.
- Open source, BSD-style licence; widely used as a [[concepts/golden-reference]] for digital RTL.
- Maintained by Stephen Williams and contributors; current releases target Verilog-2005 plus a SystemVerilog subset.
- Treated by this project as the v1 reference kernel for [[concepts/event-trace-equivalence]] comparisons of the circuit-solver digital kernel.

## Common Strategies

- [[concepts/event-trace-equivalence]] — VCD-vs-VCD comparison at every cycle boundary.
- [[concepts/mixed-level-simulation]] — As the external digital kernel coordinated by the optimistic mixed-signal scheduler ([[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]]).

## Related Entities

- [[entities/ngspice]] — Analog counterpart used jointly for the lockstep mixed-signal reference.
- [[entities/sky130-pdk]] — Source of gate-level Verilog used in the digital conformance corpus.
- [[entities/asap7-pdk]] — Additional gate-level Verilog source for the digital conformance corpus.
