---
title: Perturbed Asymptotic Expansion
type: claim
id: concepts/perturbed-asymptotic-expansion
tags:
- ode
- dae
- singular-perturbation
- extrapolation
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

The perturbed asymptotic expansion (Deuflhard–Hairer–Zugck 1987) for a numerical method on a DAE or SPP includes *localised perturbation terms* α_i^j, β_i^j supported near the initial step indices, on top of the smooth expansion: y_i − y(x_i) = ∑_j h^j (a_j(x_i) + α_i^j) + O(h^{M+1}). Unlike the classical [[concepts/asymptotic-expansion]], the perturbations do not vanish far from the boundary — they decay only with i or with the extrapolation column index.

## How It Works

The perturbation terms reflect the way IRK / LMS / linearly-implicit-Euler methods handle initial inconsistency or boundary-layer transients in DAE limits: each step propagates a residue from the previous step's algebraic constraint, and these residues form a slowly-decaying sequence. [[concepts/extrapolation-method]] tableaux on the perturbed expansion produce *differential-algebraic orders* r_jk, s_jk (Hairer–Wanner Tables VI.5.3–5.4) that grow more slowly than classical orders. For SPPs at finite ε, the expansion picks up ε^2 perturbations T_{jj}(H/ε) · b_2(0) that decay exponentially for H/ε → ∞ — the analytical basis for SEULEX's effectiveness on stiff and DAE problems.

## Key Parameters

- Localised perturbation terms α_i^j, β_i^j.
- Differential-algebraic orders r_jk, s_jk.
- Step-size scaling H, sub-step h_j.
- For SPPs, the ratio H/ε.

## When To Use

- Convergence analysis of extrapolation codes (SEULEX, SODEX, LIMEX) on DAEs and SPPs.
- Designing [[concepts/dense-output]] schemes that don't amplify the localised perturbations.
- Theoretical understanding of why DAE orders saturate below the classical extrapolation tableau orders.

## Risks & Pitfalls

- Perturbation terms make some entries of the extrapolation tableau effectively useless beyond a finite column index.
- Mishandled dense output amplifies the boundary-layer perturbations; use Hairer–Ostermann's right-end Hermite construction.

## Related Concepts

- [[concepts/asymptotic-expansion]]
- [[concepts/extrapolation-method]]
- [[concepts/linearly-implicit-euler]]
- [[concepts/singular-perturbation-problem]]
- [[concepts/differential-algebraic-equation]]
- [[concepts/dense-output]]
- [[concepts/order-reduction]]
- [[entities/seulex]]
- [[entities/limex]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
