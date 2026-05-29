---
title: Mixed-Level Simulation
type: claim
id: concepts/mixed-level-simulation
tags:
- mixed-signal
- analog
- digital
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

Mixed-level simulation (MLS) is the practice of simulating one circuit block at the transistor level while representing the rest of the surrounding system with behavioral (pin-accurate) models. The system acts as an executable testbench for the block, and the simulator handles both descriptions concurrently inside one run.

## How It Works

The designer describes the full system at a high level using an analog/mixed-signal HDL ([[entities/verilog-ams]] or [[entities/vhdl-ams]]). Each block is initially a behavioral model. When a block is ready for verification, its transistor-level [[concepts/modified-nodal-analysis]] description is substituted in place of its behavioral model. The simulator integrates the two domains — discrete-event for digital and behavioral abstractions, continuous-time for the transistor-level block — coupled at the interface signals.

## Key Parameters

- Block boundary definition and pin accuracy of the behavioral models
- Interface-element insertion at A↔D boundaries (Verilog-AMS does this automatically; VHDL-AMS does not)
- Time-step coupling between the discrete and analog kernels

## When To Use

- Verifying a transistor-level block in the context of the system it must work in
- Replacing impractical full-transistor runs on large designs (the Disk Read Channel case: >10k transistors, 2000 cycles, transient predicted >1 month; under MLS, one block overnight)
- Verifying inter-block interfaces during top-down design
- Generating signal-flow testbenches for incremental block release

## Risks & Pitfalls

- Behavioral models must be pin-accurate; sloppy abstractions hide integration bugs that only surface at full integration.
- Requires a simulator architected to handle both high-level and transistor-level descriptions efficiently in the same run; without that, the "high capacity" requirement for SPICE-level simulation forces unnatural partition boundaries.
- Discrete/analog timestep coordination can introduce subtle artifacts at boundary crossings.

## Related Concepts

- [[concepts/top-down-design]]
- [[concepts/ahdl-mshdl]]
- [[concepts/signal-flow-model]]
- [[concepts/conservative-model]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
