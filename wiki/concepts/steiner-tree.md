---
title: Steiner Tree
type: claim
id: concepts/steiner-tree
tags:
- graph
- algorithm
- foundational
- well-established
- np-hard
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

Given a graph G and a set Ω ⊆ V of terminals, the Steiner tree problem asks for a connected subgraph S of G with smallest number of edges (or smallest total edge weight) that contains all terminals.

Equivalently, find a tree T in G with V(Ω) ⊆ V(T) that minimizes |E(T)|.

## How It Works

Steiner tree is NP-complete on general graphs. The Dreyfus-Wagner DP solves it in O(3^k · n + 2^k · n^2) on graphs with k terminals.

Chimani-Mutzel-Zey (2012): on graphs of treewidth ≤ k, Steiner tree is solvable in O(k · B_{2k+1} · n) using DP over a nice tree decomposition, where B_t is the t-th Bell number (number of set partitions). The state at each bag is a partition of the bag together with a "special" part for vertices not in the current subtree.

## Key Parameters

- |Ω| number of terminals.
- Treewidth k.
- Bell numbers B_t bound the state space at join nodes.

## When To Use

- Network design (Steiner trees over fiber lines).
- VLSI layout (connecting pins by minimum-length wires).
- Phylogenetic tree reconstruction.

## Risks & Pitfalls

- The problem is APX-hard; no PTAS unless P = NP.
- The Bell-number-based state at join nodes can grow super-exponentially in tw.

## Related Concepts

- [[concepts/tree-decomposition]]
- [[concepts/treewidth]]
- [[concepts/steiner-minimal-tree]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
