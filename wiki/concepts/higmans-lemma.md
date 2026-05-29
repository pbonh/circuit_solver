---
title: Higman's Lemma
type: claim
id: concepts/higmans-lemma
tags:
- algorithm
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

Higman's lemma states that the set A* of finite nonempty sequences ("words") over a finite alphabet A is well-quasi-ordered by the subsequence relation: for any infinite sequence w_1, w_2, … in A*, there exist i < j such that w_i is a subsequence of w_j.

## How It Works

Nash-Williams's "bad sequence" proof: assume an infinite antichain exists. Pick a minimal-length bad sequence (each x_i has minimal length extending the bad prefix). Some letter appears as the first letter of infinitely many x_i; remove that letter from each to get a shorter bad sequence — contradiction.

Applications:
- k-cograph recognition: forbidden induced subgraphs are finite via Higman applied to k-cotrees.
- Threshold-width characterization: vertex orderings as labeled words.
- Encoding tournaments of cutwidth k by codewords; Higman + gap embeddings show tournaments of cutwidth k are wqo.

## Key Parameters

- A: finite alphabet.
- "Subsequence" relation embeds a shorter word into a longer one preserving order.

## When To Use

- Proving finiteness of obstruction sets.
- Encoding combinatorial structures as words.

## Risks & Pitfalls

- The alphabet must be finite (or well-quasi-ordered).
- The lemma gives no explicit bound on the length until i < j is reached.

## Related Concepts

- [[concepts/well-quasi-order]]
- [[concepts/kruskal-theorem]]
- [[concepts/k-cograph]]
- [[concepts/threshold-width]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
