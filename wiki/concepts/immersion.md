---
title: Immersion
type: claim
id: claim-immersion
tags:
- graph
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

A graph H immerses in G if there is a map η : H → G such that:
1. η(V(H)) → V(G) is injective.
2. For every edge {x, y} ∈ E(H), η(xy) is a path in G from η(x) to η(y).
3. For distinct edges e ≠ f, η(e) and η(f) are edge-disjoint.

Robertson-Seymour: graphs are well-quasi-ordered by weak immersions (weakly, allowing the additional condition below to fail).

For digraphs the same definition applies with directed paths.

## How It Works

Equivalently, H immerses in G iff H is an induced subgraph of some G' obtained from G by edge-lifts (replacing edges {x, a} and {x, b} sharing endpoint x by edge {a, b}).

Subcubic graphs (max degree ≤ 3) are well-quasi-ordered by strong immersion (Exercise 4.98); in subcubic graphs, immersion equals minor equals topological minor.

## Key Parameters

- The function η.
- Edge-disjointness is the key property distinguishing immersion from minor.

## When To Use

- Modeling subgraph relations preserving edge structure but allowing path expansion.
- Tournament-class structure theorems.

## Risks & Pitfalls

- Immersion is strictly weaker than topological minor (which forbids internal vertices on paths from being other η(v)).
- Strong immersion adds non-incidence condition (see strong-immersion concept).

## Related Concepts

- [[concepts/strong-immersion]]
- [[concepts/topological-minor]]
- [[concepts/minor]]
- [[concepts/well-quasi-order]]
- [[concepts/tournament]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
