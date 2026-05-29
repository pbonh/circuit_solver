---
title: Krylov Subspace Model Order Reduction
type: claim
id: concepts/krylov-subspace-mor
tags:
- mor
- interconnect
- ac
- foundational
- vlsi
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/05-2-symbolic-analysis-techniques-in-a-nutshell.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Krylov subspace MOR projects a large linear system onto a Krylov subspace `K_q(M, v) = span{v, M v, M^2 v, ...}` so that the reduced system matches a chosen number of moments (or block moments) of the original transfer function.

## How It Works

Arnoldi or Lanczos iteration generates an orthonormal basis for the Krylov subspace; congruence projection yields the reduced matrices. PRIMA uses a single Arnoldi pass on the MNA matrix and preserves passivity for RLC networks; SPRIM additionally preserves block structure to retain reciprocity and second-order properties. AWE used explicit moment matching and suffers numerical instability — Krylov methods fix that by implicit matching.

## Key Parameters

- Number of moments to match (reduced order).
- Expansion point in `s` (single or multi-point).
- Block size for multi-port inputs.

## When To Use

- Large RLC interconnect macromodels.
- Repeated AC analysis where solving the full system is too expensive.

## Risks & Pitfalls

- Loss of accuracy outside the moment-matching frequency window.
- Passivity not guaranteed for plain Arnoldi without PRIMA-style structure preservation.

## Related Concepts

- [[concepts/model-order-reduction]]
- [[concepts/balanced-truncation]]
- [[concepts/symbolic-moment-computation]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-15-10-symbolic-moment-computation]]
