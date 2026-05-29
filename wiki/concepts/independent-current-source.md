---
title: Independent Current Source
type: claim
id: concepts/independent-current-source
tags:
- foundational
- analog
- well-established
- device-model
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

An independent current source delivers a prescribed current j through itself regardless of the voltage across its terminals. Its i-v characteristic is a vertical line at current j; setting j = 0 yields an open circuit.

## How It Works

Real sources are modeled by an ideal current source in parallel with internal resistance Rs (Norton form). In nodal analysis, current sources contribute directly to the right-hand side (KCL) and do not introduce extra unknowns.

## Key Parameters

- Prescribed current j(t) or J (DC).
- Parallel internal resistance Rs (for realistic-source modeling).

## When To Use

- Modeling current-mode signal injection (bias currents, DC analysis stimulus).
- Norton representation of any network seen between two nodes.
- Stamping into the nodal RHS vector directly.

## Risks & Pitfalls

- An ideal current source would require infinite voltage if open-circuited.
- Two current sources connected in series with different values produce an inconsistent equation.

## Related Concepts

- [[concepts/independent-voltage-source]]
- [[concepts/thevenin-norton-equivalents]]
- [[concepts/nodal-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
