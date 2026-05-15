---
title: "Sensitivity"
type: concept
tags: [algorithm, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

Let f : {0, 1}^n → {0, 1} be a Boolean function. The sensitivity of f at input x is the number of indices i for which flipping the i-th bit of x changes f(x). The sensitivity s(f) is the maximum over all x.

The block sensitivity bs(f) at x is the maximum number of disjoint subsets B ⊆ [n] with f(x^B) ≠ f(x); bs(f) is the max over x.

The decision tree depth D(f) is the smallest depth of a decision tree computing f.

## How It Works

Theorem 4.206 (Huang 2019, "Sensitivity Theorem"):
- s(f) ≤ D(f).
- D(f) = O(s(f)^c) for some constant c.

Combined with Tal (bs(f) ≤ δ(f)^2 in degree, where δ(f) is the polynomial degree) and Nisan's sandwich s ≤ bs ≤ D = O(bs^4), this resolves the sensitivity conjecture: s ≤ bs ≤ s^4.

Huang's proof: build {0, -1, +1}-matrix A_n of order 2^n with eigenvalues ±√n of multiplicity 2^(n-1) each. Apply Cauchy's interlace lemma to the principal submatrix on any subgraph of Q_n with > 2^(n-1) vertices; the largest eigenvalue is at least √n. Combined with Gotsman-Linial equivalence theorem, this yields s(f) ≥ √(δ(f)).

## Key Parameters

- n: number of input bits.
- s(f), bs(f), D(f), δ(f).

## When To Use

- Quantum complexity lower bounds.
- Communication complexity.
- Property testing.

## Risks & Pitfalls

- The exact polynomial degree c for D(f) = O(s(f)^c) is currently 4; tighter bounds are open.
- The sensitivity-bs gap is exactly s^4 in the worst case (recently shown).

## Related Concepts

- [[concepts/block-sensitivity]]
- [[concepts/hypercube]]
- [[concepts/cauchy-interlace-lemma]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
