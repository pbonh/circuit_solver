---
title: Hedetniemi's Conjecture
type: claim
id: concepts/hedetniemi-conjecture
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Hedetniemi's conjecture (1966): for any two graphs G and H, χ(G × H) = min(χ(G), χ(H)), where × denotes the tensor (categorical) product.

The upper bound is trivial: a coloring of G induces a coloring of G × H of the same number of colors.

## Status

The conjecture was refuted by Yaroslav Shitov in 2019 ("Counterexamples to Hedetniemi's conjecture," arXiv:1905.02167). Shitov constructed graphs G and H both with χ ≥ q + 1 (for some q) but χ(G × H) ≤ q.

The fractional version (Hedetniemi's conjecture for fractional chromatic numbers) was proved true by X. Zhu (2011): χ_f(G × H) = min(χ_f(G), χ_f(H)).

## How It Works

For perfect graphs (Theorem 4.226 in the Kloks-Xiao text), the equality holds: when G and H are perfect, ω(G × H) ≥ min(ω(G), ω(H)) = min(χ(G), χ(H)), and the upper bound χ(G × H) ≤ min(χ(G), χ(H)) is always valid, so equality follows.

## Key Parameters

- min(χ(G), χ(H)) is always an upper bound.
- The fractional version always holds.

## When To Use

- As a benchmark for tensor-product coloring problems.
- For verifying that certain product constructions cannot reduce chromatic number.

## Risks & Pitfalls

- The conjecture fails in general (Shitov 2019), so it cannot be used as a tool without verifying the special case (e.g. perfect graphs).
- The fractional version still applies broadly.

## Related Concepts

- [[concepts/tensor-product]]
- [[concepts/chromatic-number]]
- [[concepts/perfect-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
