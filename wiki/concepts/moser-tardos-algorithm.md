---
title: Moser-Tardos Algorithm
type: claim
id: claim-moser-tardos-algorithm
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
  base: 0.85
---

## Definition

The Moser-Tardos algorithm (2009) is a constructive proof of the Lovász Local Lemma. Given a finite set of independent random variables and a finite set of "bad" events A_1, …, A_n satisfying the LLL condition with parameters x_i:

1. Sample a random assignment of all variables.
2. While some bad event A is violated: pick one and resample only its variables.
3. Continue until no bad event is violated.

## How It Works

The expected number of resampling steps for event A is at most x(A) / (1 - x(A)); the total expected number of steps is ∑_A x(A) / (1 - x(A)).

The proof uses "witness trees" τ(t) recording the resampling history at step t. Witness trees are proper (children at the same depth have different labels) and the probability that a given proper tree T appears in the log is bounded by ∏_{a ∈ V(T)} P([a]). A Galton-Watson branching process generates the trees, giving the expected bound.

## Key Parameters

- x_i values from the LLL condition.
- Expected resampling count is a sum over events.
- Bounded-degree dependency graphs admit deterministic derandomization (Moser-Tardos provides this in a separate construction).

## When To Use

- Whenever an LLL-based existence proof needs to be algorithmic.
- For finding 2-colorings of low-intersection hypergraphs, dominating sets in regular graphs, satisfying assignments to k-CNF.

## Risks & Pitfalls

- The witness-tree analysis is intricate; numerical bounds depend on x(A) staying bounded away from 1.
- The algorithm is randomized; deterministic versions exist under stronger conditions.

## Related Concepts

- [[concepts/lovasz-local-lemma]]
- [[concepts/hypergraph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
