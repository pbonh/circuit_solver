---
title: Threshold Graph
type: claim
id: concepts/threshold-graph
tags:
- graph
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

A graph G is a threshold graph if every induced subgraph has an isolated or universal vertex. Equivalent characterizations:
- No induced P_4, C_4, or 2K_2.
- G's vertices can be linearly ordered so each successive vertex is either adjacent to all later vertices or none.
- N(x) ⊆ N[y] or N(y) ⊆ N[x] for every pair x, y.

The class is closed under complementation.

## How It Works

Recognition is linear time via vertex elimination by isolated/universal vertices. Threshold graphs are trivially perfect: in any induced subgraph, α = number of maximal cliques.

The threshold dimension θ(G) is the min k such that G's edges are a union of k threshold graphs. θ(G) ≤ 2 is polynomial; θ(G) ≤ 3 is NP-complete.

## Key Parameters

- O(n) recognition.
- The graph is a split graph; threshold ⇒ split.

## When To Use

- Modeling "tipping point" social phenomena.
- As a tractable subclass for splitgraph problems.

## Risks & Pitfalls

- Threshold graphs are very restricted; they form a small subclass of cographs.
- Distinct from "threshold-width," which generalizes the class.

## Related Concepts

- [[concepts/cograph]]
- [[concepts/splitgraph]]
- [[concepts/threshold-width]]
- [[concepts/perfect-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
