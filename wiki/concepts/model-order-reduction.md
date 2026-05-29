---
title: Model Order Reduction (MOR)
type: claim
id: concepts/model-order-reduction
tags:
- interconnect
- foundational
- ac
- numerical
- vlsi
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/05-2-symbolic-analysis-techniques-in-a-nutshell.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Model Order Reduction approximates a large linear (or linearized) dynamical system by a much smaller one that preserves selected transfer-function characteristics (poles, residues, moments, passivity) within a frequency band of interest.

## How It Works

Two principal families: (i) Krylov subspace / moment-matching methods (AWE, PRIMA, SPRIM, SOMOR) that project the system onto a subspace spanned by moments of the input; (ii) Gramian-based balanced truncation methods (TBR, PMTBR, SBPOR, SOGA) that identify weakly controllable/observable modes and truncate them. Parameterized/variational MOR retains process-variation parameters as symbols.

## Key Parameters

- Reduction order.
- Expansion frequency / moment match count.
- Passivity preservation (PRIMA, SPRIM).
- Symmetry / second-order structure preservation (SBPOR).

## When To Use

- Interconnect modeling (power grids, on-chip wiring, package).
- Embedded system-level analog macro-models.
- As a symbolic-analysis special case when only `s` is symbolic.

## Risks & Pitfalls

- Passivity loss in some Gramian-approximation methods.
- Cubic complexity of standard balanced truncation.
- Variational MOR can suffer moment-term explosion with parameter count.

## Related Concepts

- [[concepts/krylov-subspace-mor]]
- [[concepts/balanced-truncation]]
- [[concepts/variational-mor]]
- [[concepts/symbolic-analysis]]
- [[concepts/symbolic-moment-computation]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-15-10-symbolic-moment-computation]]
