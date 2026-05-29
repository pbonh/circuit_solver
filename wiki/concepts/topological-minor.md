---
title: Topological Minor
type: claim
id: claim-topological-minor
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

H is a topological minor of G if some subgraph of G is isomorphic to a subdivision of H. Equivalently, there is a homeomorphic embedding η : H → G that maps vertices to distinct vertices and edges to internally-disjoint paths in G.

## How It Works

Subdivision: replace each edge of H by a path of arbitrary length. Topological minor sits between minor (weaker) and subgraph (stronger):
- H is a subgraph of G ⇒ H is a topological minor of G.
- H is a topological minor of G ⇒ H is a minor of G.

For subcubic graphs, the three relations coincide. Topological minor containment of fixed H in G is FPT in |H| (Grohe-Marx-Wollan-Kawarabashi, cubic algorithm).

Graphs are NOT well-quasi-ordered by topological minor: the sequence of subdivided "duplicated paths" with two extra pendants (Liu-Thomas Definition 4.164) is an infinite antichain. The obstruction is the "Robertson chain."

## Key Parameters

- Length of subdivisions on each edge of H.
- Subcubic graphs vs. unbounded degree.

## When To Use

- Modeling subgraph containment with flexibility on path lengths.
- Structural decomposition of graphs avoiding certain topological subgraphs.

## Risks & Pitfalls

- Not wqo in general; need additional structural conditions for wqo results.
- Distinct from "topological subgraph" in some texts.

## Related Concepts

- [[concepts/minor]]
- [[concepts/immersion]]
- [[concepts/well-quasi-order]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
