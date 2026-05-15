---
title: "Balanced Truncation (TBR / PMTBR)"
type: concept
tags: [mor, interconnect, ac, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/05-2-symbolic-analysis-techniques-in-a-nutshell.txt"]
confidence: medium
---

## Definition

Truncated Balanced Realization (TBR) reduces a linear system by simultaneously diagonalizing its controllability and observability Gramians and discarding modes with small Hankel singular values. Approximate variants (PMTBR / Poor Man's TBR, SBPOR, SOGA) approximate the Gramians to avoid the cubic cost of solving Lyapunov equations.

## How It Works

Standard TBR solves two Lyapunov equations for `P` and `Q`, computes the SVD of `L_P^T L_Q`, and forms a balancing transformation `T` such that the balanced Gramians are equal and diagonal. Modes corresponding to small singular values are truncated. PMTBR replaces Gramian computation with sampled approximations; SBPOR works in second-order form to retain symmetry and positive-definiteness needed for passivity in RLCK networks.

## Key Parameters

- Hankel singular value truncation threshold.
- Sampling strategy for approximate Gramians.
- First- vs. second-order formulation.

## When To Use

- RC interconnect modeling (PMTBR works well).
- RLCK on-chip interconnects requiring passivity + accuracy (SBPOR/SOGA).
- Frequency-band-targeted reduction (UiMOR).

## Risks & Pitfalls

- Direct TBR is `O(n^3)` and infeasible for large IC problems.
- Loss of passivity in some Gramian-approximation variants.
- Requires expertise in control-theoretic numerics.

## Related Concepts

- [[concepts/model-order-reduction]]
- [[concepts/krylov-subspace-mor]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
