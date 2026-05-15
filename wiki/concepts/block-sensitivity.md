---
title: "Block Sensitivity"
type: concept
tags: [algorithm, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

For a Boolean function f : {0, 1}^n → {0, 1} and input x, the block sensitivity at x is the maximum number of pairwise disjoint subsets B_1, …, B_k ⊆ [n] such that f(x^{B_i}) ≠ f(x) for each i, where x^B denotes x with the bits in B flipped.

The block sensitivity bs(f) is the maximum over x. bs(f) ≥ s(f) since singletons {i} are disjoint.

## How It Works

Nisan's sandwich (1989): s(f) ≤ bs(f) ≤ D(f) = O(bs(f)^4), where D(f) is decision tree depth. Block sensitivity captures the worst-case "parallelism" of input perturbations.

Tal's lemma: bs(f) ≤ δ(f)^2, where δ(f) is the polynomial degree of f. Combined with Huang's hypercube theorem, this yields the sensitivity-block-sensitivity polynomial relation s ≤ bs ≤ s^4.

## Key Parameters

- bs(f).
- δ(f) polynomial degree.
- D(f) decision tree depth.

## When To Use

- Lower bounds for Boolean function complexity.
- Quantum query complexity (related to bs).

## Risks & Pitfalls

- bs(f) ≥ s(f) but not always equal.
- The exact s-vs-bs gap was open before Huang 2019.

## Related Concepts

- [[concepts/sensitivity]]
- [[concepts/hypercube]]
- [[concepts/cauchy-interlace-lemma]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
