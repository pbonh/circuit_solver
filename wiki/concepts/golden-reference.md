---
title: "Golden Reference"
type: concept
tags: [verification, testing, foundational]
created: 2026-05-18
updated: 2026-05-18
sources: ["wiki/specs/circuit-solver"]
confidence: medium
---

## Definition

A *golden reference* is a precomputed result produced by an independent, trusted tool that defines the expected output of a system under test. The system passes when its output agrees with the golden reference inside a documented tolerance envelope; it fails when it deviates beyond that envelope.

## How It Works

For analog circuit simulation, the golden reference is typically an established SPICE-class simulator ([[entities/ngspice]], commercial Spectre, etc.) producing operating-point voltages, frequency responses, or transient waveforms on the same netlist. For digital simulation, the reference is an established event-driven kernel ([[entities/icarus-verilog]], Verilator) producing a [[concepts/value-change-dump]] (VCD) trace. The system under test is run on the same input under the same stimulus, and the outputs are compared point-by-point inside the agreed envelope.

## Key Parameters

- **Reference tool and version** — exact build, since results can drift across versions.
- **Tolerance envelope** — relative tolerance, absolute tolerance, and (for AC / noise) dB tolerance.
- **Coverage corpus** — the set of test circuits or testbenches over which conformance must hold.
- **Comparison granularity** — every sample, every cycle boundary, every accepted timestep, or only final values.

## When To Use

- Validating a new simulator against an established one before users trust its numbers.
- Guarding against regressions while refactoring the solver, integrator, or device-model engine.
- Bounding subjective claims about "correctness" with reproducible, measurable criteria.

## Risks & Pitfalls

- An overly tight envelope causes false failures when the reference tool's own numerical noise exceeds the bound.
- An overly loose envelope masks real bugs.
- The reference tool itself can be wrong — golden-reference verification is correctness *relative to* the reference, not absolute correctness.
- Tolerance bounds that mix relative and absolute terms must be defined unambiguously (e.g., "5 % relative OR 10 µV absolute, whichever is greater").

## Related Concepts

- [[concepts/event-trace-equivalence]]
- [[concepts/value-change-dump]]

## Sources

- [[specs/circuit-solver]] — Defines the v1 acceptance envelope and corpus.
