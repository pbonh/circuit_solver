---
title: Bottleneck Domination
type: claim
id: claim-bottleneck-domination
tags:
- graph
- algorithm
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.85
---

## Definition

Given a weighted graph (G, w) where w: V → R, the bottleneck of a vertex set W is max_{x ∈ W} w(x). The bottleneck domination problem asks for a dominating set D with minimum bottleneck.

## How It Works

There is a linear-time algorithm: define m(x) = min_{y ∈ N[x]} w(y) and let ρ = max_{x ∈ V} m(x). Then the minimal bottleneck is ρ. The matching algorithm for total domination uses m'(x) = min over open neighborhood and ρ' = max over m'(x).

This is a sweep over edges and adjacency lists in O(n + m).

## Key Parameters

- ρ for ordinary bottleneck domination.
- ρ' = max_x min_{y ∈ N(x)} w(y) for total bottleneck domination.

## When To Use

- Selecting facility locations where the worst-case (weakest) match matters more than total cost.
- Robustness-oriented network design.

## Risks & Pitfalls

- The bottleneck objective is different from the sum-of-weights objective; both can be solved efficiently here but typically by different algorithms.

## Related Concepts

- [[concepts/dominating-set]]
- [[concepts/graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
