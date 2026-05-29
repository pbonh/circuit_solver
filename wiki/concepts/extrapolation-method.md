---
title: Extrapolation Method
type: claim
id: claim-extrapolation-method
tags:
- ode
- numerical-integration
- stiff
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
---

## Definition

An extrapolation method (Bulirsch–Stoer 1966; Bader–Deuflhard 1983 for stiff problems) integrates over a fixed macro-step H using several different micro-step sizes h_j = H/n_j, then applies Aitken–Neville recursion on the resulting endpoint values to eliminate successive terms in the asymptotic expansion of the basic discretisation. Doubling the sub-step sequence n_j cancels the leading h^j error term at level j of the tableau, giving an adaptive-order method whose effective order grows with the number of columns.

## How It Works

For nonstiff problems the basic discretisation is usually the explicit midpoint rule, whose asymptotic expansion is in h^2 (Gragg 1965), producing the GBS method. For stiff problems Bader–Deuflhard (1983) use a *linearly implicit* base scheme — either the [[concepts/linearly-implicit-euler]] method (giving an h-expansion; the SEULEX code) or the linearly implicit midpoint rule (giving an h^2-expansion; the SODEX code, equivalent to Bader–Deuflhard's METAN1). The Aitken–Neville tableau T_{jk} accumulates higher-order corrections; smoothing (Gragg / Lindberg) is required to recover L-stability at the right edge. For [[concepts/differential-algebraic-equation]]s, Deuflhard–Hairer–Zugck (1987) prove a *perturbed* asymptotic expansion with localised perturbation terms near the initial values, which the dense-output construction (Hairer–Ostermann 1990) must avoid amplifying.

## Key Parameters

- Macro-step H.
- Sub-step sequence {n_j} (e.g. n_j = 2, 4, 6, 8, ...).
- Tableau depth (highest column k_max).
- Base discretisation (midpoint, linearly implicit Euler, linearly implicit midpoint).

## When To Use

- High-precision integration with adaptive order (1 to ~12).
- Stiff problems via SEULEX (linearly implicit Euler) or SODEX (linearly implicit midpoint).
- Quasilinear DAEs via LIMEX-style linearly-implicit Euler extrapolation (Deuflhard–Nowak).
- Long-time / event-rich problems where high-order steps are economical.

## Risks & Pitfalls

- For DAEs and [[concepts/singular-perturbation-problem]]s the asymptotic expansion is *perturbed*; the effective orders r_jk, s_jk grow more slowly than classical orders.
- Smoothing is mandatory for L-stability in stiff variants.
- Dense output is delicate near boundary layers; use Hermite interpolation with extrapolated derivatives only at the right end.

## Related Concepts

- [[concepts/linearly-implicit-euler]]
- [[concepts/perturbed-asymptotic-expansion]]
- [[concepts/asymptotic-expansion]]
- [[concepts/dense-output]]
- [[concepts/singular-perturbation-problem]]
- [[concepts/quasilinear-dae]]
- [[entities/seulex]]
- [[entities/sodex]]
- [[entities/limex]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
