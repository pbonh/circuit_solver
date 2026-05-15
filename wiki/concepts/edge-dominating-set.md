---
title: "Edge Dominating Set"
type: concept
tags: [graph, algorithm, well-established, np-hard, matching]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

An edge dominating set in a graph G is a set M ⊆ E(G) such that every edge of E(G) \ M shares an endpoint with some edge in M. Equivalently, M is a dominating set in the linegraph L(G), or E(G - V(M)) = ∅.

A minimum edge dominating set has minimum cardinality.

## How It Works

Every minimum edge dominating set's endpoint set V(M) is a vertex cover. The parameterized problem is FPT: enumerate all minimal vertex covers of size ≤ 2k (there are at most 4^k of them) and extend each to a minimum edge dominating set via maximum matching in G[S].

Current best: 2.2351^k by Iwaide-Nagamochi.

## Key Parameters

- Solution size k.
- Tight bound: V(M) is a vertex cover of size ≤ 2k.

## When To Use

- Network monitoring (every link is either monitored or adjacent to a monitored link).
- Backup planning (every connection has a near backup).

## Risks & Pitfalls

- Edge dominating set ≠ matching: an edge dominating set need not be a matching.
- The reduction to vertex cover + matching is non-trivial; just picking a maximal matching gives a 2-approximation but is not optimal.

## Related Concepts

- [[concepts/dominating-set]]
- [[concepts/vertex-cover]]
- [[concepts/matching]]
- [[concepts/fixed-parameter-tractability]]
- [[concepts/linegraph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
