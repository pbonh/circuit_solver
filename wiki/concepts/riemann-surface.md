---
title: "Riemann Surface"
type: concept
tags: [mathematical-tool, ode, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

A Riemann surface is the natural geometric domain on which a multi-valued holomorphic function becomes single-valued: it is a connected complex manifold (one-dimensional over ℂ) covering some open subset of the complex plane, with sheets that correspond to the branches of the function. For an algebraic function Q(μ, ζ) = 0 of degree k in ζ, the surface has k sheets that ramify at the discriminant locus.

## How It Works

In Hairer–Wanner Chapter V, the characteristic equation ρ(ζ) − μ σ(ζ) = 0 of a [[concepts/linear-multistep-methods]] method defines an algebraic function ζ(μ) with k branches (one per principal/auxiliary root). Lifting the [[concepts/order-star]] machinery from a single rational R(z) to the Riemann surface of ζ(μ) lets the finger-counting argument apply to multistep methods, recovering the [[concepts/dahlquist-barrier]] and proving the [[concepts/daniel-moore-conjecture]] for multistep methods. The *sheet structure* — branch points, monodromy, sheet connectivity — encodes the interaction between the principal root (= numerical solution) and the auxiliary roots (= parasitic / spurious modes).

## Key Parameters

- Number of sheets k (= multistep step count).
- Branch points (discriminant locus).
- Monodromy group.

## When To Use

- Order-star theory for multistep methods.
- Theoretical study of multistep stability barriers.
- Disambiguating principal vs. auxiliary root behaviour in characteristic-equation analysis.

## Risks & Pitfalls

- The geometry is intricate; concrete computations need explicit branch cuts.
- Not a tool for everyday method comparison — mainly used in the theoretical proofs of barrier results.

## Related Concepts

- [[concepts/order-star]]
- [[concepts/linear-multistep-methods]]
- [[concepts/property-c]]
- [[concepts/daniel-moore-conjecture]]
- [[concepts/dahlquist-barrier]]
- [[concepts/root-locus-curve]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
