---
title: "Threshold-Width"
type: concept
tags: [graph, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A graph G has threshold-width ≤ k if it has k independent sets N_1, …, N_k such that there is a threshold graph H having G as a spanning subgraph and every edge in E(H) \ E(G) has both endpoints in some N_i.

τ(G) is the smallest such k.

## How It Works

Theorem 4.105 (Kloks-Xiao): the class of graphs with τ ≤ k has a finite forbidden-induced-subgraph characterization. Proof uses Higman's lemma: graphs are encoded as labeled words; an embedding among words induces an induced subgraph embedding.

Theorem 4.111: threshold-width is NP-complete (reduces from K-width, which Kou-Stockmeyer-Wong showed NP-complete).

Theorem 4.117: there is an O(n^2) FPT algorithm via probe-universal sets, k-probe modules, and greedy extension.

Lemma 4.106: rankwidth ≤ 2^τ, so threshold-width upper-bounds rankwidth.

## Key Parameters

- τ(G) — main parameter.
- k-probe modules (false twin sets of size ≥ 3 or true twin sets of size ≥ k + 3).

## When To Use

- A new width parameter for graph algorithm meta-theorems.
- Test bed for FPT meta-theorems via Higman/Kruskal.

## Risks & Pitfalls

- The forbidden subgraph set is finite but unknown.
- DH-width (analog using distance-hereditary embeddings) is open.

## Related Concepts

- [[concepts/threshold-graph]]
- [[concepts/rankwidth]]
- [[concepts/fixed-parameter-tractability]]
- [[concepts/higmans-lemma]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
