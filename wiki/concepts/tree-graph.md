---
title: Tree (Graph)
type: claim
id: claim-tree-graph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt
confidence:
  base: 0.85
---

## Definition

A tree is a connected, acyclic, undirected simple graph. A tree with n nodes has exactly n−1 edges; any two nodes are connected by a unique path. A forest is a graph whose connected components are trees.

## How It Works

A rooted tree designates one node as root; edges are oriented away from the root, giving each non-root node a unique parent (indegree 1). Terminology includes parent, child, sibling, leaf (no children), internal node, ancestor, descendant, level (distance from root), and height (max level). An m-ary tree has maximum outdegree m; a tree is balanced if all leaves are at level h or h−1, full if every internal node has 0 or m children, and complete if all internal levels are full and leaves are left-aligned.

## Key Parameters

- Number of nodes n, edges n−1.
- Height h, branching factor m.
- Root (if rooted).

## When To Use

- Hierarchical data: file systems, parse trees, decision trees.
- Clock and power tree structures in VLSI.
- Spanning trees as substructures of more general graphs.

## Risks & Pitfalls

- Balancing constraints affect performance of search/insert operations.
- Acyclicity must be preserved when adding edges (adding any edge creates exactly one cycle).

## Related Concepts

- [[concepts/spanning-tree]]
- [[concepts/minimum-spanning-tree]]
- [[concepts/steiner-minimal-tree]]
- [[concepts/clock-tree-synthesis]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
