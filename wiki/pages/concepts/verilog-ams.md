---
title: Verilog-AMS
type: concept
slug: verilog-ams
created: 2026-06-16
updated: 2026-06-16
summary: IEEE analog and mixed-signal extension to Verilog-HDL, unifying event-driven digital and continuous-time analog simulation in a single language.
tags: [hdl, mixed-signal, analog, verilog, circuit-simulation, behavioral-modeling]
sources: [simulation-analog-mixed-signal-circuits]
status: active
---

# Verilog-AMS

Verilog-AMS is the merger of Verilog-HDL (digital event-driven simulation) and Verilog-A (continuous-time analog). Verilog-A was approved by OVI in June 1996; Verilog-AMS (the combined language) in August 1998. It enables mixed-level simulation by allowing behavioral models and transistor-level netlists to co-exist and interoperate.

## Language Architecture

| Kernel | Blocks | Evaluation |
|---|---|---|
| Event-driven (digital) | `initial`, `always` blocks | `initial`: once; `always`: continuously |
| Continuous-time (analog) | `analog` block | once per timestep; no blocking |

## Model Types

- **Conservative models**: relate both potential (V) and flow (I) — used for device modeling and correct loading at interfaces
- **Signal-flow models**: relate only potentials (V → V) — used for abstract, high-level behavioral descriptions

Both types are freely interconnectable in the same Verilog-AMS description.

## Analog Operators

- `idt(x)`, `ddt(x)`: time integration and differentiation
- `idtmod(x, ic, modulus)`: circular integrator (for VCO phase accumulation)
- `transition(x)`: waveform smoothing with specified rise/fall times
- `cross(expr, dir)`: generates events at analog signal crossings
- `timer(start, period)`: generates events at periodic times

## Capabilities Beyond Verilog-HDL + Verilog-A

- Reads analog signals in digital blocks and digital signals in analog blocks
- `cross()` generates discrete events from analog signal crossings
- Automatic interface element insertion at analog/digital port mismatches
- Automatic back-annotation of parasitics
- Multi-disciplinary modeling (non-electrical physical domains via user-defined disciplines)
- Ideal switch modeling via `discontinuity(0)` annotation

## Example Applications

| Circuit | Key language features |
|---|---|
| VCO | `idtmod` for circular phase accumulation, `cos()` for output |
| Sampler | `timer()` events, state variable `real hold`, `transition` output |
| Phase/Frequency Detector | `always` with `posedge` events on two clocks, integer state, `transition` current output |
| N-bit ADC | `posedge conv` event, iterative bit extraction in `always` |

## Why it matters

- Enables top-down design: architecture verified at behavioral level before transistor-level implementation
- Provides pin-accurate block models for mixed-level simulation — verifying blocks in system context
- VHDL-AMS (IEEE 1076.1, approved summer 1998) is a parallel standard without the analog-only subset; lacks automatic interface insertion

## Related concepts and entities

- [[circuit-simulation]] - parent topic
- [[spice-simulation]] - transistor-level simulation Verilog-AMS behavioral models coexist with
- [[ken-kundert]] - made substantial contributions to Verilog-AMS and VHDL-AMS standards
