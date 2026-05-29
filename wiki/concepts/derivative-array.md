---
title: Derivative Array
type: claim
id: claim-derivative-array
tags:
- dae
- mathematical-tool
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.65
---

## Definition

For a general implicit DAE F(u', u, x) = 0 the derivative array (Campbell 1985, 1989) is the stacked system [F; dF/dx; d^2F/dx^2; …; d^m F/dx^m] obtained by repeatedly differentiating F with respect to x. The smallest m for which the array determines u' explicitly (after algebraic manipulation) is the [[concepts/differentiation-index]].

## How It Works

Each differentiation introduces higher derivatives u^{(k)} as new unknowns while also adding equations. The arrays at successive m form a growing rectangular linear (in derivatives) system; QR or SVD factorisation of the array's Jacobian with respect to (u', u'', …, u^{(m+1)}) yields an underlying ODE u' = φ(u, x) once the rank stabilises. Campbell's unstructured-higher-index approach uses the derivative array to compute the underlying ODE directly, bypassing the need for closed-form differentiation steps. The construction is the formal substrate for [[concepts/overdetermined-dae]] formulations and Pantelides' algorithm in Modelica-style tools.

## Key Parameters

- Differentiation depth m.
- Jacobian-array rank.
- Smoothness class of F.

## When To Use

- Index determination of arbitrary implicit DAEs.
- Constructing underlying ODEs for non-standard DAE forms.
- Pantelides-style index-reduction in equation-based modelling tools (Dymola, OpenModelica).

## Risks & Pitfalls

- The derivative array grows large quickly; m = 3 already produces a system of size O(m · n).
- Rank determination is numerically delicate; SVD-based ranks are sensitive to tolerance.
- Construction is symbolic / automatic-differentiation heavy.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/differentiation-index]]
- [[concepts/index-reduction]]
- [[concepts/overdetermined-dae]]
- [[concepts/index-of-a-dae]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
