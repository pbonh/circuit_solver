---
title: "Value Change Dump (VCD)"
type: concept
tags: [digital, verification, foundational, well-established]
created: 2026-05-18
updated: 2026-05-18
sources: ["wiki/specs/circuit-solver"]
confidence: low
---

## Definition

A *Value Change Dump* (VCD) is a standardised IEEE 1364 ASCII file format that records the time-stamped value changes of every monitored signal during a digital simulation run. It is the lingua franca for digital-simulator output and waveform-viewer input.

## How It Works

The simulation kernel writes a header section declaring the simulation timescale and the set of monitored signals with their hierarchical names. The body then emits, in time order, lines of the form `#<timestamp>` followed by `<value><signal-id>` tokens for every signal that changed at that timestamp. Tools that consume VCD reconstruct the signal-vs-time trajectory by replaying the recorded changes.

## Key Parameters

- **Timescale** — the resolution of timestamps (e.g., `1 ns`).
- **Scope hierarchy** — module / instance path of each signal.
- **Signal identifier** — a short ASCII handle internal to the VCD that maps to the full hierarchical name.
- **Value coding** — `0`, `1`, `x`, `z` for scalars; binary or real for vectors.

## When To Use

- Capturing reference traces from a trusted simulator like [[entities/icarus-verilog]] for [[concepts/golden-reference]] comparison.
- Driving waveform viewers (GTKWave, Surfer) for human inspection.
- Cross-tool digital regression — two kernels that emit VCD on the same testbench can be compared programmatically.

## Risks & Pitfalls

- Different simulators may emit redundant value-change records (a signal "changing" to the same value), which trips naive byte-level diffs; use [[concepts/event-trace-equivalence]] for semantic comparison.
- Float / real-valued signals in VCD lose precision; SPICE-style waveforms should use a real-valued format instead.
- Very long runs produce very large VCD files; tools may need streaming consumers rather than full in-memory parses.

## Related Concepts

- [[concepts/event-trace-equivalence]]
- [[concepts/golden-reference]]

## Sources

- [[specs/circuit-solver]] — Uses VCD as the exchange format between [[entities/icarus-verilog]] and the circuit-solver digital kernel.
