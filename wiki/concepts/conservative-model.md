---
title: Conservative Model
type: claim
id: concepts/conservative-model
tags:
- analog
- mixed-signal
- device-model
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A conservative model is an analog/mixed-signal HDL description that relates both potentials and flows at its ports — i.e., it specifies a relationship between V(a,b) and I(a,b). This is the natural style for device models and for blocks that interact with their environment through real loading.

## How It Works

In [[entities/verilog-ams]], conservative ports use a discipline with a flow nature (typically `electrical`, which contributes both `V` and `I`). The analog block expresses the device equation in implicit form, e.g., `V(a,b) <+ r*I(a,b);` for a resistor — both V and I are unknowns and KCL must hold at each node. The simulator places these contributions into [[concepts/modified-nodal-analysis]] alongside transistor-level stamps.

## Key Parameters

- Port discipline (potential + flow)
- Number of conservative nodes (controls loading granularity)
- Multidisciplinary nature support (electrical, thermal, mechanical) — Verilog-AMS allows user-defined disciplines for cross-domain modeling

## When To Use

- Device-level modeling (resistors, capacitors, inductors, ideal transformers, custom physical devices)
- Interface boundaries between behavioral and transistor sections where loading matters
- Any block whose impedance affects neighboring blocks in a non-negligible way
- Multidisciplinary blocks (e.g., a sensor modeled as a coupled electrical + thermal element)

## Risks & Pitfalls

- More expensive to simulate per timestep than [[concepts/signal-flow-model]] blocks — full Newton iteration through the conservative ports.
- Care needed at A↔D interfaces; Verilog-AMS inserts user-specified interface modules automatically, VHDL-AMS does not.

## Related Concepts

- [[concepts/signal-flow-model]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/ahdl-mshdl]]
- [[concepts/mixed-level-simulation]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
