---
title: Large Change Sensitivity
type: claim
id: claim-large-change-sensitivity
tags:
- sensitivity
- foundational
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/11-chapter-8-large-change-sensitivity-and-related-topics.txt
confidence:
  base: 0.85
---

## Definition

Large change sensitivity computes the exact response F of a network after arbitrary (non-infinitesimal) perturbations of m elements, without re-solving the full system. The technique exploits the fact that element perturbations enter the system matrix as a rank-m update T = T_0 + P delta Q^T.

## How It Works

Define the (m+1) x (m+1) matrix F_hat = [F W_hat; d_hat^T F_0] where F = Q^T T_0^{-1} P, W_hat = Q^T T_0^{-1} W, d_hat = T_0^{-T} d, F_0 = d^T T_0^{-1} W. Computed once via m+1 forward/back substitutions on the nominal LU factorization.

For any new set of perturbations delta:
- Solve the m_a x m_a sub-system (delta_a^{-1} + F_aa) z_a = W_a (m_a = number of nonzero perturbations).
- Output: F = F_0 - d_a^T z_a.

Element changes can be arbitrary including delta = -G (open circuit) and delta^{-1} = 0 (short circuit), enabling unified fault-condition handling.

## Key Parameters

- m (number of potentially perturbable elements; controls F_hat size).
- m_a (number of actually perturbed elements; controls per-query cost).
- Choice of which elements to allow as perturbations.

## When To Use

- Design iteration with element-value changes.
- Fault analysis (open/short of single or pairs of elements).
- Sensitivity computation under non-infinitesimal perturbation.
- Symbolic analysis via the F matrix subdeterminants.

## Risks & Pitfalls

- Preprocessing cost grows linearly with m; impractical for very large m.
- (delta^{-1} + F) may itself be ill-conditioned for some perturbation sets.
- Requires structured perturbation: each delta_i appears as a rank-1 update — sufficient for R, L, C, transducers, but not arbitrary topology changes.

## Related Concepts

- [[concepts/low-rank-matrix-update]]
- [[concepts/fault-analysis]]
- [[concepts/sensitivity-analysis]]
- [[concepts/symbolic-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-11-chapter-8-large-change-sensitivity-and-related-topics]]
- [[summaries/computer-methods-circuit-analysis-design-17-chapter-14-digital-and-switched-capacitor-networks]]
