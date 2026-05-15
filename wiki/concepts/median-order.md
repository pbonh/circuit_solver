---
title: "Median Order"
type: concept
tags: [graph, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A median order of a digraph D = (V, A) is an ordering v_1 < v_2 < … < v_n maximizing |{(v_i, v_j) ∈ A : i < j}|. Equivalently, a median order minimizes the number of "back-arcs" (it is a feedback-arc-set-minimizing order).

## How It Works

Havet-Thomassé introduced median orders for tournament work. Key properties:
- Any interval of a median order is a median order of the induced subdigraph (Exercise 4.92).
- For any v_i and interval I to its right, |N^+(v_i) ∩ I| ≥ |N^-(v_i) ∩ I| in tournament case.

These properties drive algorithms for M-embeddings of oriented trees in tournaments (Lemma 4.141 / Lemma 4.144), leading to El Sahili's 3(n-1) bound for Sumner's conjecture.

## Key Parameters

- O(2^|V|) to compute exactly (related to feedback arc set, NP-hard).
- For tournaments, median orders relate to ranking aggregation.

## When To Use

- Tournament-tree embedding algorithms.
- Ranking aggregation (Kemeny rank).
- Feedback arc set heuristics.

## Risks & Pitfalls

- Computing exact median order is NP-hard (equivalent to feedback arc set).
- For tournaments, polynomial-time approximations are known (3-approximation by Coppersmith-Fleischer-Rudra).

## Related Concepts

- [[concepts/tournament]]
- [[concepts/dag]]
- [[concepts/topological-sort]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
