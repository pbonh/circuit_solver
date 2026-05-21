---
title: "Event-Trace Equivalence"
type: concept
tags: [digital, verification, foundational]
created: 2026-05-18
updated: 2026-05-18
sources: ["wiki/specs/circuit-solver"]
confidence: medium
---

## Definition

*Event-trace equivalence* is a relation between two digital simulation runs that holds when, at every reference-kernel cycle boundary, the two runs agree on the set of (time, signal, value) tuples for all monitored signals. Intra-cycle settling order is ignored: only the observable signal values at each cycle boundary are compared.

## How It Works

Both simulators run the same Verilog source against the same testbench and emit a [[concepts/value-change-dump]] (VCD). A comparison tool walks both traces in lockstep at the reference kernel's cycle boundaries (typically the testbench's clock edges or its `$monitor` sample points). At each boundary it collects the set of `(signal, value)` pairs from each trace; the run passes when the sets are identical at every boundary.

## Key Parameters

- **Reference kernel** — the trusted simulator whose cycle boundaries define the comparison points (here, [[entities/icarus-verilog]]).
- **Boundary definition** — clock edges, `$monitor` ticks, or every `posedge`/`negedge` of a nominated clock.
- **Signal scope** — which hierarchical signals are compared (typically the testbench's observable outputs plus an internal-signal allow-list).
- **Glitch tolerance** — whether intra-cycle transients that settle before the next boundary are ignored (typical) or fail the run (strict).

## When To Use

- Validating a new digital event kernel against an established one ([[entities/icarus-verilog]], Verilator).
- Regression-guarding a mixed-signal cosim against drift in either the analog or digital kernel.
- Asserting functional equivalence in CI without committing to byte-level VCD identity.

## Risks & Pitfalls

- Choosing boundaries too coarsely (e.g., only end-of-test) lets early-cycle divergences slip through; choosing them too finely (every delta cycle) over-constrains and rejects legitimate scheduling differences.
- Hidden state — signals not in the compared scope — can diverge without the equivalence relation catching it, masking real bugs.
- Floating-point or real-valued signals can drift below the relation's resolution; treat them with [[concepts/golden-reference]] tolerance bounds rather than set equality.

## Related Concepts

- [[concepts/value-change-dump]]
- [[concepts/golden-reference]]
- [[concepts/mixed-level-simulation]]

## Sources

- [[specs/circuit-solver]] — Uses event-trace equivalence as the digital-kernel acceptance criterion against [[entities/icarus-verilog]].
