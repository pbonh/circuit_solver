---
title: Strong Immersion
type: claim
id: concepts/strong-immersion
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

H strongly immerses in G if there is an immersion map η : H → G (vertex-injective, edges to edge-disjoint paths) with the additional property: for every x ∈ V(H) and e ∈ E(H) such that x is not an endpoint of e, η(x) is not on the path η(e).

## How It Works

Theorem 4.154 (Chudnovsky-Seymour): tournaments are well-quasi-ordered by strong immersion. The proof goes through:
- Linked layouts of cutwidth-k tournaments (Lemma 4.150).
- Encoding by codewords with gap sequences.
- Marches and equivalence classes; Higman/Kruskal-style induction.

Theorem 4.166 (Liu-Muzi 2020): digraphs without k-alternating threads are well-quasi-ordered by strong immersion. The proof uses series-parallel triples, F-series parallel trees, portraits, and forward Ramsey factorization.

## Key Parameters

- Cutwidth k in tournament theory.
- Number of alternating directions in a thread.

## When To Use

- Structural classification of tournaments and digraphs.
- Proving finite-obstruction-set theorems for tournament classes.

## Risks & Pitfalls

- Strong immersion is strictly stronger than weak (Robertson-Seymour) immersion.
- Whether all (non-directed) graphs are wqo by strong immersion is open.

## Related Concepts

- [[concepts/immersion]]
- [[concepts/tournament]]
- [[concepts/well-quasi-order]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
