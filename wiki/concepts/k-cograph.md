---
title: k-Cograph
type: claim
id: concepts/k-cograph
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

A k-cograph (for k ∈ ℕ) is a graph G that has a decomposition tree (T, f) where:
1. T is a rooted binary tree.
2. Each leaf of T is labeled with an element of [k].
3. f : V(G) → leaves(T) is a bijection.
4. Each internal node is labeled with a symmetric Boolean k × k matrix σ.
5. {x, y} ∈ E iff σ(label(f(x)), label(f(y))) = true at their lowest common ancestor.

A 1-cograph (only one label) is an ordinary cograph.

## How It Works

Theorem 4.122: for every k, the class of k-cographs has a finite set of forbidden induced subgraphs (Sk). Proof via Kruskal's tree theorem: an infinite antichain in S_k would give an infinite antichain of k-cotrees, contradicting wqo.

Theorem 4.124: k-cograph recognition is FPT in k, runnable in O(n^3) using Courcelle MS1 + rankwidth ≤ k.

Cograph-width(G) = min k such that G is a k-cograph; cograph-width is FPT.

## Key Parameters

- k = number of labels.
- |S_k| = number of forbidden induced subgraphs.
- t = max |V(S)| over S ∈ S_k.

## When To Use

- Generalization of cograph algorithms via the recursive label structure.
- Test bed for parameterized recognition.

## Risks & Pitfalls

- |S_k| can grow rapidly; explicit obstruction sets are mostly unknown.
- The recognition algorithm uses Courcelle, so constants can be huge.

## Related Concepts

- [[concepts/cograph]]
- [[concepts/cotree]]
- [[concepts/kruskal-theorem]]
- [[concepts/rankwidth]]
- [[concepts/courcelle-theorem]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
