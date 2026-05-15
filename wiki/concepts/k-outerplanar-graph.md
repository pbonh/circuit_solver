---
title: "k-Outerplanar Graph"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A plane graph G has layers L_1, L_2, … defined by L_i = set of vertices in the outerface of G - (L_1 ∪ … ∪ L_{i-1}). G is k-outerplanar if its plane embedding has at most k nonempty layers.

1-outerplanar = outerplanar.

## How It Works

Lemma 4.244 (Bodlaender): a k-outerplanar graph has treewidth ≤ 3k - 1. Contracting each layer to a clique (after suitable rotations) gives a chordal graph of clique size ≤ 3k.

Baker's method: for any planar graph G and k ∈ ℕ, removing every k-th layer separates G into k-outerplanar subgraphs. By Courcelle, MS2 problems are linear-time on k-outerplanar graphs. This yields a PTAS for many planar problems: e.g. independent set within factor k / (k+1) in linear time (Theorem 4.246).

Computing the smallest outerplanarity of a graph is NP-complete.

## Key Parameters

- k = number of layers.
- Treewidth ≤ 3k - 1.

## When To Use

- PTAS for NP-complete problems on planar graphs.
- Modeling layered planar networks.

## Risks & Pitfalls

- The treewidth bound is sharp: 3k - 1 is achieved by certain examples.
- The PTAS uses Courcelle; concrete algorithms may be more efficient.

## Related Concepts

- [[concepts/outerplanar-graph]]
- [[concepts/treewidth]]
- [[concepts/bakers-method]]
- [[concepts/courcelle-theorem]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
