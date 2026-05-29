---
title: Lovász Local Lemma
type: claim
id: concepts/lovasz-local-lemma
tags:
- algorithm
- advanced
- well-established
- probability
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The Lovász Local Lemma is a probabilistic existence tool. Let A_1, …, A_n be events in a probability space and D a dependency graph (vertices = events, edges = dependencies between events' variables). If there exist x_i ∈ [0, 1) such that

  P(A_i) ≤ x_i · ∏_{{i,j} ∈ E(D)} (1 - x_j)

for every i, then P(∩ A_i^c) ≥ ∏_i (1 - x_i) > 0. In particular, there exists an outcome where no bad event occurs.

A useful symmetric form: if every event has probability ≤ p and depends on at most d others, then e · p · (d + 1) ≤ 1 implies P(no bad event) > 0.

## How It Works

The proof proceeds by induction on |J| to show P(A_i | ∩_{j ∈ J} A_j^c) ≤ x_i for any J ⊆ [n] \ {i}. The constructive proof (Moser-Tardos 2009) provides a randomized resampling algorithm that produces an avoiding assignment efficiently.

Application: A k-uniform hypergraph H in which every hyperedge intersects at most d others is 2-colorable whenever e(d + 1) ≤ 2^(k-1).

## Key Parameters

- d: max degree of the dependency graph.
- p: upper bound on per-event probability.
- x_i: free parameters tuning the bound.

## When To Use

- Proving existence of combinatorial structures (colorings, factorizations, dominating sets).
- Constructing low-degree (a, b)-dominating sets in regular graphs.

## Risks & Pitfalls

- The independence assumption is crucial; the dependency graph must correctly capture all variable-sharing.
- Original Lovász-Erdős proof is non-constructive; use Moser-Tardos when an algorithm is needed.

## Related Concepts

- [[concepts/moser-tardos-algorithm]]
- [[concepts/hypergraph]]
- [[concepts/dominating-set]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
