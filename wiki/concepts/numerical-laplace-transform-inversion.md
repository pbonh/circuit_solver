---
title: Numerical Laplace Transform Inversion
type: claim
id: concepts/numerical-laplace-transform-inversion
tags:
- transient
- analog
- foundational
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Numerical Laplace transform inversion (NILT) computes a time-domain response by numerically inverting the Laplace-domain network function F(s) → f(t). In Vlach and Singhal it is presented as a specialized integration method for linear networks (Chapter 10) with capabilities not easily provided by direct numerical integration.

## How It Works

NILT evaluates F(s) at a set of carefully chosen complex s-values and combines the samples via a quadrature formula (e.g., Vlach's method using shifted Chebyshev approximations, or Gaver-Stehfest, or Talbot). The result is f(t) at a chosen time t. Sensitivities are obtained by differentiating F(s) with respect to parameters and inverting in the same way.

## Key Parameters

- Choice of inversion algorithm (Vlach's, Gaver-Stehfest, Talbot, etc.).
- Number of complex sample points.
- Numerical precision (some NILT methods need extended precision).
- Time *t* of interest (NILT computes a single t at a time, not an entire trajectory in one shot).

## When To Use

- Linear networks where the Laplace-domain network function is available analytically or via symbolic analysis.
- Problems involving Dirac impulses, derivatives of impulses, or distributed elements — these are easy in NILT and hard in time-stepping methods.
- Time-domain sensitivity computation for linear networks, where direct time integration is software-intensive.

## Risks & Pitfalls

- Numerical inversion is ill-conditioned; many algorithms require high-precision arithmetic.
- Different inversion methods perform differently on oscillatory, damped, or non-smooth responses.
- Not applicable to inherently nonlinear circuits (no Laplace transform).

## Related Concepts

- [[concepts/laplace-transform]]
- [[concepts/symbolic-analysis]]
- [[concepts/sensitivity-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-13-chapter-10-numerical-laplace-transform-inversion]]
- [[summaries/computer-methods-circuit-analysis-design-23-appendix-c-special-complex-integration-of-a-rational-function]]
- [[summaries/computer-methods-circuit-analysis-design-24-appendix-d-program-for-network-analysis]]
