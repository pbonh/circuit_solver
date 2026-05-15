---
title: "Signal-Flow Model"
type: concept
tags: [mixed-signal, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

A signal-flow model is an analog/mixed-signal HDL description that relates only potentials (no flows). Its outputs are written directly as functions of its inputs without enforcing Kirchhoff's-current-law conservation at the port. Useful for abstract behavioral blocks at the top of [[concepts/top-down-design]].

## How It Works

In [[entities/verilog-ams]], a signal-flow port is declared with a discipline that has no flow nature (e.g., `voltage` rather than `electrical`). The analog block expresses outputs as contribution statements such as `V(out) <+ a*V(in);` — there is no current term, no loading effect at the input, and no impedance at the output. Multiple signal-flow blocks compose by simple expression substitution.

## Key Parameters

- Port discipline (potential-only vs. potential-and-flow)
- Use of analog operators (`transition`, `slew`, `idt`, `ddt`, Laplace/Z filters) that act on the potentials directly

## When To Use

- Top-level architectural models where loading effects and impedance interactions are not part of the abstraction.
- Behavioral testbenches and abstract building blocks (mixers as ideal multipliers, oscillators as `cos(2π·idt(K·V_ctrl))`, ADC/DAC quantizers, control-loop filters described by Laplace transfer functions).
- Bridging to a [[concepts/conservative-model]] block via interface modules — Verilog-AMS freely interconnects signal-flow and conservative ports.

## Risks & Pitfalls

- Hides loading effects that may matter once the block descends to a real-world implementation. Cross-validation against a conservative or transistor-level model is part of the discipline.
- Mismatched expectations between author and consumer about the model's fidelity can mask interface issues that only show up when the model is replaced by a transistor block.

## Related Concepts

- [[concepts/conservative-model]]
- [[concepts/ahdl-mshdl]]
- [[concepts/mixed-level-simulation]]
- [[concepts/top-down-design]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
