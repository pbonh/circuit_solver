---
title: "Property C"
type: concept
tags: [ode, numerical-integration, stability, mathematical-tool, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

Property C (Jeltsch–Nevanlinna 1982) is a structural condition on a numerical method allowing pointwise comparison of stability domains: a method has Property C if its principal root carries the entire instability, i.e. the characteristic polynomial's principal root determines the [[concepts/stability-region]] boundary completely while all auxiliary roots stay strictly inside the unit disk.

## How It Works

Methods with Property C admit *comparison theorems*: no method's scaled stability domain can strictly contain another's. The theorem rules out the possibility that a higher-order method could simultaneously have a strictly larger stability region than a lower-order method in the same class — formalising the trade-off between order and stability. Property C generalises the order-star finger-counting machinery (which assumes a single rational R) to multistep methods whose stability is governed by a multi-valued algebraic function on a [[concepts/riemann-surface]].

## Key Parameters

- Principal vs. auxiliary root distribution.
- Stability-region scaling factor.

## When To Use

- Theoretical comparison of methods in the same family (BDF orders, Adams orders).
- Proving stability-region domination theorems for multistep methods.
- Extension of order-star theory to multi-valued characteristic equations.

## Risks & Pitfalls

- Methods without Property C escape the comparison theorems and can have anomalous stability shapes.
- The condition is technical and rarely needed in routine use; mainly a research tool.

## Related Concepts

- [[concepts/order-star]]
- [[concepts/riemann-surface]]
- [[concepts/linear-multistep-methods]]
- [[concepts/daniel-moore-conjecture]]
- [[concepts/stability-region]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
