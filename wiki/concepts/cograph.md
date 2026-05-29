---
title: Cograph
type: claim
id: concepts/cograph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

A cograph is a graph with no induced P_4 (path on 4 vertices). Equivalently:
- Every induced subgraph with at least two vertices has a twin.
- Recursively built by single vertices, disjoint union ⊕, and join ⊗.
- Has a cotree T (rooted, internal nodes labeled ⊕ or ⊗, leaves = vertices) such that adjacency = lowest common ancestor labeled ⊗.

## How It Works

Cographs are recognized in linear time via the cotree, which encodes the entire structure. The class is closed under complementation (because P_4 is self-complementary). Cographs are perfect (χ = ω on every induced subgraph) and distance-hereditary.

Many problems are linear-time on cographs given the cotree: max independent set, max clique, chromatic number, treewidth, tensor capacity. Cograph-width and k-cographs generalize the class.

## Key Parameters

- Cotree has O(n) nodes.
- χ = ω; α and ω computable via cotree recursion.

## When To Use

- As a tractable test bed for new graph algorithms.
- For problems where union/join decomposition naturally captures the structure (independence, domination).

## Risks & Pitfalls

- Not closed under edge contraction or minor.
- Joins of cographs need not be perfect for their tensor products (Hedetniemi's refutation).

## Related Concepts

- [[concepts/cotree]]
- [[concepts/twin]]
- [[concepts/perfect-graph]]
- [[concepts/distance-hereditary-graph]]
- [[concepts/k-cograph]]
- [[concepts/modular-decomposition]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
- [[summaries/guide-to-graph-algorithms-06-problem-formulations]]
- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
