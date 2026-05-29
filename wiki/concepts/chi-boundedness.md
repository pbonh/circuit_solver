---
title: χ-Boundedness
type: claim
id: concepts/chi-boundedness
tags:
- graph
- advanced
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

A class G of graphs is χ-bounded if there exists a function f : ℕ → ℕ such that χ(G) ≤ f(ω(G)) for every G ∈ G. The class is polynomially χ-bounded if f can be chosen polynomial.

## How It Works

Lemma 4.72: graphs of rankwidth ≤ k are χ-bounded with f(s) = 2^(k·s) · 3^(s-1). The proof partitions V into ≤ 3·2^k classes such that no class contains a maximum clique, then recursively colors each class.

Examples of χ-bounded classes:
- Cographs (χ = ω since perfect).
- Distance-hereditary graphs.
- Intersection graphs of axis-parallel boxes in d-space.
- Graphs without odd holes (or long holes).
- Graphs without an induced subdivision of a tree.
- AT-free graphs (Kierstead-Penrice).
- Circle graphs (polynomially χ-bounded).

Scott-Seymour (2016): if ω ≤ κ and χ > 2^(2^(κ+2)), then G has an odd hole.

## Key Parameters

- The function f.
- Sharper bounds for restricted parameters (rankwidth, treewidth).

## When To Use

- Verifying tractability of coloring on a class.
- As a structural property linking ω and χ.

## Risks & Pitfalls

- Erdős: triangle-free graphs can have arbitrarily high χ (so they are not χ-bounded with bounded ω).
- Gyárfás-Sumner conjecture (for every tree T, the class of T-free graphs is χ-bounded) is open in general.

## Related Concepts

- [[concepts/chromatic-number]]
- [[concepts/clique]]
- [[concepts/rankwidth]]
- [[concepts/perfect-graph]]
- [[concepts/at-free-graph]]
- [[concepts/circle-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
