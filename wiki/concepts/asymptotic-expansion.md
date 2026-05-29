---
title: Asymptotic Expansion
type: claim
id: concepts/asymptotic-expansion
tags:
- mathematical-tool
- ode
- singular-perturbation
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

An asymptotic expansion in a small parameter ε is a formal series y(x; ε) ∼ ∑_{j=0}^N ε^j y_j(x) + O(ε^{N+1}) whose partial sums approximate the true y to better and better accuracy as ε → 0 for *fixed* x, but typically diverge if N → ∞ at fixed ε > 0. For SPPs (Vasil'eva 1963), the expansion of the smooth (outer) solution is supplemented by boundary-layer terms ε^j η_j((x − x_0)/ε) that decay exponentially in the stretched variable.

## How It Works

Substituting the formal series into the ODE and matching powers of ε determines the y_j recursively: y_0 satisfies the [[concepts/reduced-system]]; y_1, y_2, … are obtained as solutions of linear systems involving the previous y_j and the perturbed right-hand sides. Theorem 3.2 in Hairer–Wanner gives a rigorous remainder estimate O(ε^{N+1}) for the truncated expansion of an SPP under μ(g_z) ≤ −1. Numerical methods (IRK, multistep) applied to the SPP admit a *parallel* expansion (Hairer–Lubich–Roche 1988): the numerical y_n^j, z_n^j are the same recursive structure with the IRK / LMS solution of each cascade step.

## Key Parameters

- Small parameter ε.
- Truncation order N.
- Smooth (outer) and singular (inner / [[concepts/boundary-layer]]) terms.

## When To Use

- Theoretical analysis of [[concepts/singular-perturbation-problem]]s.
- Constructing reduced-order models with quantitative error bounds.
- Numerical-analysis convergence proofs (matched discrete / continuous expansions).

## Risks & Pitfalls

- The series typically diverges for fixed ε > 0; optimal truncation N* depends on ε.
- Inconsistent initial conditions force inclusion of boundary-layer terms — pure outer expansions miss them.
- For [[concepts/perturbed-asymptotic-expansion]]s (DAE extrapolation context), additional localised perturbation terms appear that must be tracked carefully.

## Related Concepts

- [[concepts/singular-perturbation-problem]]
- [[concepts/boundary-layer]]
- [[concepts/perturbed-asymptotic-expansion]]
- [[concepts/reduced-system]]
- [[concepts/extrapolation-method]]
- [[concepts/order-reduction]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
