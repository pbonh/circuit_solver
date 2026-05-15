---
title: "Outerplanar Graph"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A planar graph G is outerplanar if it can be embedded in the plane with all vertices on the same face (the exterior). The class is closed under taking minors; its obstruction set is {K_4, K_{2,3}}.

A maximal outerplanar graph (MOP) is one to which no edge can be added without losing outerplanarity.

## How It Works

Outerplanar graphs have treewidth ≤ 2 (Lemma 4.242). Every MOP is Hamiltonian and consists of triangles sharing edges in a tree-like pattern (it is a 2-tree with a specific embedding).

Recursive characterization: a MOP is either an edge or obtained from a smaller MOP by attaching a new vertex adjacent to the endpoints of a non-minimal-separator edge.

## Key Parameters

- |V(G)|.
- tw ≤ 2.
- All vertices on one face.

## When To Use

- Modeling planar arrangements with all components on the boundary.
- Test bed for treewidth-bounded algorithms.

## Risks & Pitfalls

- Outerplanar ≠ planar; the constraint is global (all vertices on one face).
- A planar graph can have unbounded treewidth (e.g. grid).

## Related Concepts

- [[concepts/k-outerplanar-graph]]
- [[concepts/treewidth]]
- [[concepts/minor]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
