---
title: PQ-Tree
type: claim
id: claim-pq-tree
tags:
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.85
---

## Definition

A PQ-tree (Booth-Luecker, 1976) is a rooted tree representing a set of permutations of a finite set V. Internal nodes are labeled P (children re-orderable arbitrarily) or Q (children's order is fixed up to reversal). Each leaf corresponds to one element of V.

## How It Works

The valid permutations of V represented by the tree are exactly the linear orderings of its leaves obtainable by:
- Permuting children of any P-node in any way.
- Reversing the order of children at any Q-node.

PQ-trees support the consecutive-ones property: given a (0,1)-matrix, find a row-permutation that makes the 1s in each column consecutive (used for interval graph recognition, planarity testing).

## Key Parameters

- Total operations on a PQ-tree run in O(n + m) amortized time.
- Number of distinct permutations represented can be exponential.

## When To Use

- Interval graph recognition.
- Planar graph recognition.
- Consecutive-ones detection in combinatorial matrices.

## Risks & Pitfalls

- Implementation is notoriously intricate; PC-trees are a related but cleaner variant.
- PQ-trees do not directly handle online or dynamic updates.

## Related Concepts

- [[concepts/interval-graph]]
- [[concepts/permutation-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
