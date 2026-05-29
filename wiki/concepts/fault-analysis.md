---
title: Fault Analysis
type: claim
id: claim-fault-analysis
tags:
- analog
- well-established
- fault-analysis
- sensitivity
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/11-chapter-8-large-change-sensitivity-and-related-topics.txt
confidence:
  base: 0.65
---

## Definition

Fault analysis isolates the cause of a circuit's failure to meet specifications from terminal measurements. Two main categories:
- Drift failures: small variations in many components (often due to aging).
- Catastrophic failures: large changes in a few components (open or short circuits).

Catastrophic faults are simpler to isolate and are the typical focus.

## How It Works

A fault directory is precomputed: for each element, the locus of response F as that element varies (delta from -G to infinity) is tabulated. The measured F is compared to all loci; the matching curve identifies the faulty element. The value of the failed component is then recovered from delta.

Large change sensitivity (Chapter 8 of Vlach & Singhal) is the computational engine: a single LU factorization plus m+1 forward/back substitutions precomputes F_hat; sweeping any delta_i only requires solving a 1x1 (single-fault) or 2x2 (two-fault) system.

## Key Parameters

- Number of components in the circuit.
- Number of simultaneous faults considered (typically 1 or 2).
- Resolution of the delta sweep.
- Measurement noise tolerance.

## When To Use

- Production test of small analog circuits.
- Maintenance and repair of field-failed equipment.
- Yield analysis during design.

## Risks & Pitfalls

- Multi-fault coverage is combinatorial; typically restricted to 1-2 faults.
- Multiple faults may produce response values matching single-fault loci spuriously.
- Drift faults are hard to localize because the loci overlap.

## Related Concepts

- [[concepts/large-change-sensitivity]]
- [[concepts/sensitivity-analysis]]
- [[concepts/multiparameter-sensitivity]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-11-chapter-8-large-change-sensitivity-and-related-topics]]
