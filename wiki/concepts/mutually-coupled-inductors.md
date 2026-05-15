---
title: "Mutually Coupled Inductors"
type: concept
tags: [foundational, analog, ac, transient, well-established, device-model]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: high
---

## Definition

Two inductors L1 and L2 with mutual inductance M form a two-port governed by V1 = sL1 I1 ± sM I2 and V2 = ±sM I1 + sL2 I2 (sign chosen according to winding orientation). The coefficient of coupling is k = M / sqrt(L1 L2), and 0 ≤ k ≤ 1.

## How It Works

The sign of M depends on dot conventions: dots on the same side give +M, dots on opposite sides give -M. In MNA, each inductor contributes its own branch-current unknown and an algebraic-differential equation including the mutual term.

## Key Parameters

- L1, L2 (self-inductances).
- M (mutual inductance).
- k = M / sqrt(L1 L2): coupling coefficient (1 = perfect coupling, 0 = no coupling).
- Sign of M (dot convention).

## When To Use

- Power and signal transformers, where ideal-transformer approximation is too restrictive.
- Coupled-coil filters.
- Modeling parasitic mutual coupling between traces.

## Risks & Pitfalls

- k near 1 produces a nearly-singular matrix in MNA — careful numerics required.
- Wrong dot convention flips signs and gives wrong responses.

## Related Concepts

- [[concepts/inductor]]
- [[concepts/ideal-transformer]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
