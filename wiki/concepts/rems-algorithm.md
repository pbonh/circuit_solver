---
title: Rem's Algorithm
type: claim
id: concepts/rems-algorithm
tags:
- graph
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Rem's algorithm (Kloks-Xiao Algorithm 1) is an incremental procedure that, given a graph G with V(G) = [n], computes a representative function δ : V → [n] such that δ(k) ≤ k and δ(k) = k iff k is the representative of its component. After processing all edges, the number of components equals |{x : δ(x) = x}|.

## How It Works

1. Initialize δ as the identity function.
2. For each edge {p, q} ∈ E(G):
   - Set (p_0, q_0) ← (p, q), (p_1, q_1) ← (δ(p_0), δ(q_0)).
   - While p_1 ≠ q_1:
     - If p_1 < q_1: δ(q_0) ← p_1; (q_0, q_1) ← (q_1, δ(q_1)).
     - Else: δ(p_0) ← q_1; (p_0, p_1) ← (p_1, δ(p_1)).

The invariant is that δ decreases by at least one at every iteration of the inner loop, giving O(n^2) total time. This makes it a classic "union by index" structure, closely related to disjoint-set union.

## Key Parameters

- n = |V|.
- Worst-case running time O(n^2), optimal in the sense that δ decreases at every pass.

## When To Use

- Streaming or online connectivity where edges arrive incrementally.
- A teaching example for invariants and pointer compression.

## Risks & Pitfalls

- The "efficiency" of Rem's algorithm (number of δ applications) depends on the edge insertion order; ordering matters for paths.
- Modern union-find with path compression and union by rank achieves nearly O(α(n)) per operation, asymptotically better; Rem's algorithm is mainly pedagogical / historical.

## Related Concepts

- [[concepts/component]]
- [[concepts/connectedness]]
- [[concepts/graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
