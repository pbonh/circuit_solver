---
title: Runge–Kutta Collocation
type: claim
id: concepts/runge-kutta-collocation
tags:
- ode
- numerical-integration
- runge-kutta
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

Runge–Kutta collocation refers to the equivalence (Wright 1970, Guillou–Soulé 1969) between [[concepts/collocation-method]]s on s nodes and s-stage [[concepts/implicit-runge-kutta]] methods. Every collocation method *is* an RK method whose Butcher tableau is determined by Lagrange interpolation on the nodes; conversely, an s-stage IRK method is a collocation method iff its tableau satisfies the simplifying assumption C(s).

## How It Works

The equivalence formalises one direction (collocation → RK) by integrating the interpolating polynomial via the s-point quadrature induced by the nodes: a_{ij} = ∫_0^{c_i} ℓ_j(τ) dτ, b_i = ∫_0^1 ℓ_i(τ) dτ. The reverse direction holds when C(s) is exactly satisfied: the IRK stages Y_i form an order-s polynomial fitting the derivative constraints. This identifies Gauss, Radau IA, Radau IIA, and Lobatto IIIA as collocation methods; Lobatto IIIB and IIIC are *not* (they have stage order s − 1 with extra structure conditions that prevent collocation-polynomial existence). The Hairer–Wanner treatment uses the equivalence to lift superconvergence and stage-order arguments cleanly between the two viewpoints.

## Key Parameters

- Node count s.
- Node positions c_i (determine which collocation family).
- Stage order q = s.

## When To Use

- Constructing high-order IRK methods from quadrature nodes (Gauss / Radau / Lobatto IIIA).
- Generating natural [[concepts/dense-output]] from the collocation polynomial.
- Lifting boundary-value-problem collocation theory to initial-value-problem stiff integration.

## Risks & Pitfalls

- The equivalence is one-way for Lobatto IIIB / IIIC; do not assume every IRK method has a collocation interpretation.
- Stage order ≤ s; classical order can be up to 2s but only with extra structure (Gauss).

## Related Concepts

- [[concepts/collocation-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/multistep-collocation]]
- [[concepts/dense-output]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
