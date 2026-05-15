---
title: "Matching"
type: concept
tags: [graph, foundational, well-established, matching]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

A matching in a graph G is a set M ⊆ E(G) of edges no two of which share an endpoint. Equivalently, M is an independent set in the linegraph L(G).

ν(G) = α(L(G)) is the matching number, the maximum cardinality of a matching. A perfect matching covers all vertices (|M| = n/2).

## How It Works

Berge: M is maximum iff no M-augmenting path exists. Edmonds' blossom algorithm (1965) finds maximum matchings in general graphs in O(n^2 · m). Micali-Vazirani improved this to O(√n · m).

For bipartite graphs, Hopcroft-Karp achieves O(√n · m). König's theorem links max matching to min vertex cover in bipartite graphs.

## Key Parameters

- ν(G).
- Existence of perfect matching characterized by Tutte's theorem.

## When To Use

- Assignment problems (workers to jobs, students to courses).
- Network flow / transportation.
- Hidden in: stable marriage, Chinese postman, edge cover, edge dominating set.

## Risks & Pitfalls

- The blossom algorithm handles odd-cycle "blossoms" by contraction; naive augmenting-path search fails.
- Maximum-weight matching is a related but harder problem.

## Related Concepts

- [[concepts/edmonds-blossom-algorithm]]
- [[concepts/independent-set]]
- [[concepts/linegraph]]
- [[concepts/vertex-cover]]
- [[concepts/bipartite-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
