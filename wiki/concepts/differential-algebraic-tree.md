---
title: "Differential-Algebraic Tree"
type: concept
tags: [dae, runge-kutta, order-conditions, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

Differential-algebraic trees (DATs) are the rooted-tree analogues of Butcher trees adapted for DAE order conditions. They have two kinds of vertices — *meagre* (corresponding to differential components) and *fat* (corresponding to algebraic components) — and the order conditions for a Runge–Kutta / Rosenbrock method on a DAE are indexed by elementary differentials evaluated on DAT_y, DAT_z, and (for index 2) the larger set LDAT.

## How It Works

For an [[concepts/index-1-dae]] y' = f(y, z), 0 = g(y, z), order conditions on f are computed via the classical Butcher tree τ(t) over y plus its z-derivative branches; the DAT y / z partition keeps track of which vertices contribute differential-state derivatives and which contribute algebraic-state derivatives. For [[concepts/rosenbrock-method]]s on DAEs (Section VI.4) the tree class is enriched with the γ-vertex marking. For [[concepts/index-2-dae]]s (Section VII.5) the set DAT_2 generalises further; new trees introduce additional order conditions. RODAS satisfies these new conditions automatically by design.

## Key Parameters

- Meagre / fat vertex partition.
- Tree-set size as a function of order (grows factorially).
- DAE index.

## When To Use

- Designing high-order RK / Rosenbrock methods for DAEs.
- Theoretical analysis of order reduction on DAEs.
- Verifying order conditions of new method families.

## Risks & Pitfalls

- The combinatorics explodes faster than for ODE trees; computer-algebra support is essential beyond order 4.
- Different conventions (Hairer–Wanner vs. Brasey–Hairer) for fat / meagre marking; cross-reference carefully.

## Related Concepts

- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/runge-kutta-method]]
- [[concepts/rosenbrock-method]]
- [[concepts/index-1-dae]]
- [[concepts/index-2-dae]]
- [[concepts/order-reduction]]
- [[entities/rodas]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
