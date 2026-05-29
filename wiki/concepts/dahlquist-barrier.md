---
title: Dahlquist Barrier
type: claim
id: concepts/dahlquist-barrier
tags:
- ode
- numerical-integration
- multistep
- stability
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Dahlquist's second barrier theorem (1963): any A-stable [[concepts/linear-multistep-methods|linear multistep]] method has order p ≤ 2, and at order 2 the error constant satisfies C ≤ −1/12, with the [[concepts/trapezoidal-rule]] as the unique extremal A-stable method of order 2. This is a fundamental obstruction to high-order A-stable multistep integration.

## How It Works

The proof relies on the Riesz–Herglotz characterisation of functions with positive real part on |ζ| > 1: an A-stable LMS method (ρ, σ) requires Re(ρ(ζ)/σ(ζ)) > 0 for |ζ| > 1; this constrains the relationship between ρ and σ so tightly that p > 2 is impossible. The error constant bound C ≤ −1/12 follows by minimising over the admissible set. The barrier explains why BDF formulas (which abandon A-stability for orders k ≥ 3 in favour of [[concepts/a-alpha-stability]]) are the only practically viable high-order stiff multistep methods.

## Key Parameters

- Method order p (the bound says p ≤ 2 under A-stability).
- Error constant C.
- (ρ, σ) characteristic polynomials.

## When To Use

- Theoretical justification for using BDF rather than higher-order A-stable LMS.
- Method-design constraint when constructing new stiff multistep families.
- Theoretical companion to the [[concepts/daniel-moore-conjecture]] barrier for RK / general-linear methods.

## Risks & Pitfalls

- Generalisations (e.g. [[concepts/multistep-collocation]], [[concepts/extended-bdf-method]], [[concepts/general-linear-method]]) can sidestep the barrier — but each pays a cost (extra storage, super-future points, multi-derivative information).
- The barrier applies to *A-stability*; weaker stability adjectives (A(α), stiff-stability) allow higher order.

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/linear-multistep-methods]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/daniel-moore-conjecture]]
- [[concepts/order-star]]
- [[concepts/error-constant]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
