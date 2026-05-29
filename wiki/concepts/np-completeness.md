---
title: NP-Completeness
type: claim
id: concepts/np-completeness
tags:
- algorithm
- foundational
- well-established
- np-hard
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

NP is the class of decision problems P for which an answer supplied by an oracle can be tested in polynomial time. A problem is NP-complete if it is in NP and every other NP problem reduces to it in polynomial time.

A polynomial-time reduction from problem A to problem B is an algorithm that transforms an instance of A into an instance of B in polynomial time, preserving "yes" answers.

## How It Works

To prove a problem P is NP-complete:
1. Show P ∈ NP (an oracle answer is verifiable in polynomial time).
2. Reduce some known NP-complete problem to P in polynomial time.

Classic examples in the Kloks-Xiao text:
- 2-coloring of rank-3 hypergraphs reduces to total ordering (Opatnrý).
- Holyer: chromatic index of cubic graphs (3 vs. 4) is NP-complete.
- Equivalence cover number of splitgraphs is NP-complete (Blokhuis-Kloks).
- Treewidth of cobipartite graphs, claw-free graphs, bipartite graphs is NP-complete.

## Key Parameters

- The class of polynomial-time reductions, "≤_p", is a quasi-order on decision problems.
- P ≠ NP is the canonical conjecture; under it, NP-complete problems admit no polynomial-time algorithm.

## When To Use

- To classify computational hardness of new problems.
- To justify the use of approximation algorithms, parameterized algorithms, or heuristics.

## Risks & Pitfalls

- Search variants need not be in NP (they require constructing a certificate); the optimization variant may not be in NP. Often the decision variant suffices to determine the optimization complexity.
- A polynomial reduction from A to B does not imply B reduces to A.

## Related Concepts

- [[concepts/fixed-parameter-tractability]]
- [[concepts/hypergraph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
