---
title: Network Scaling (Impedance and Frequency)
type: claim
id: concepts/network-scaling
tags:
- foundational
- analog
- well-established
- numerical
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

Network scaling is a transformation of element values such that one resistor becomes 1 ohm (impedance scaling) and one frequency becomes 1 rad/sec (frequency scaling). It eliminates very small and very large numbers from computations.

## How It Works

Impedance scaling by factor k: R → R/k, L → L/k, C → C k. Frequency scaling by omega_0: omega_s = omega_d / omega_0; resistors are unchanged, L_s = L_d omega_0 / k, C_s = C_d omega_0 k. Transducers scale specifically: VVTs and CCTs are unchanged; VCT transconductance g multiplied by k; CVT transresistance r divided by k.

Self-consistent unit sets (Standard, Audio, VHF, UHF) — see Table 1.8.1 — match conventional ranges of voltage, current, resistance, capacitance, inductance, frequency, and time.

## Key Parameters

- k (impedance scaling factor).
- omega_0 (frequency normalization).
- Unit set chosen for the application.

## When To Use

- Preventing overflow/underflow in computer arithmetic.
- Improving numerical conditioning of matrices.
- Matching textbook normalized filter prototypes (1 rad/sec, 1 ohm).

## Risks & Pitfalls

- Forgetting to denormalize at the end of design.
- Inconsistent unit sets across components yields wrong simulations.
- Group-delay tau scales as omega_0 (a subtle effect when reporting tau).

## Related Concepts

- [[concepts/impedance-admittance]]
- [[concepts/network-function]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
