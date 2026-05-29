---
title: Blended Multistep Method
type: claim
id: concepts/blended-multistep-method
tags:
- ode
- numerical-integration
- multistep
- stiff
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

A blended multistep method (Skeel & Kong 1977) combines an Adams and a BDF formula in a weighted linear combination with a Jacobian-dependent weight, so the method automatically behaves like Adams in nonstiff regimes and like BDF on stiff modes. The weight is typically (I − h J γ)^{−1} multiplying the BDF term, producing a Rosenbrock-flavoured multistep hybrid.

## How It Works

The blend ratio is controlled by −h J γ: when h‖J‖ is small (nonstiff) the BDF contribution is suppressed and the method recovers Adams accuracy; when h‖J‖ is large (stiff) the blend approaches BDF and inherits its stability. Skeel–Kong showed the construction yields A-stability and stiffly-stable behaviour together, at the cost of an extra Jacobian-linear system per step (similar to a [[concepts/rosenbrock-method]]'s I − h γ J). Brugnano–Magherini and others extended the family later (BIM, GBE methods).

## Key Parameters

- Blend weight γ (analogous to Rosenbrock γ).
- Adams and BDF orders being blended.
- Per-step LU cost.

## When To Use

- Code unifying nonstiff and stiff regimes without switching algorithms.
- Problems with transient stiffness (e.g. chemical kinetics during ignition / quench).
- Theoretical comparison with predictor–corrector and second-derivative methods.

## Risks & Pitfalls

- Per-step Jacobian + LU cost like Rosenbrock; no longer "pure" LMS.
- Variable-step implementation is delicate.
- For very stiff problems, plain BDF or IRK is more economical.

## Related Concepts

- [[concepts/adams-method]]
- [[concepts/gear-bdf]]
- [[concepts/linear-multistep-methods]]
- [[concepts/rosenbrock-method]]
- [[concepts/general-linear-method]]
- [[concepts/extended-bdf-method]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
