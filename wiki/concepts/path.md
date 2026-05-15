---
title: "Path"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/04-graphs.txt"]
confidence: high
---

## Definition

A path P in a graph is a nonempty ordered sequence [x_1, …, x_t] of distinct vertices such that consecutive pairs are adjacent. The endpoints are x_1 and x_t; the length is ℓ(P) = t - 1 (the number of edges in P). The notation P_n denotes the path graph on n vertices. The distance d(x, y) between two vertices is the minimum length over all x ~ y paths.

## How It Works

Paths are the building blocks for connectedness, distance metrics, BFS-trees, and many decomposition theorems. Chords of a path P are graph edges that connect non-consecutive vertices of V(P). Algorithmic primitives like shortest path, augmenting path (in matching), and alternating path are layered on top.

## Key Parameters

- Length ℓ(P) = |E(P)| = |V(P)| - 1.
- For a graph, the diameter is max_{x,y} d(x, y).

## When To Use

- Defining distance, diameter, eccentricity, radius.
- As a substructure in matching (augmenting path) and flow (path in the residual graph).
- As an obstruction (P4-free cographs, asteroidal-triple-free graphs avoid certain paths).

## Risks & Pitfalls

- "Walk" allows vertex/edge repetition; "trail" allows vertex repetition but not edge; "path" allows neither.
- The text uses "chain" to mean a walk-like structure in matching arguments.

## Related Concepts

- [[concepts/graph]]
- [[concepts/cycle]]
- [[concepts/connectedness]]
- [[concepts/matching]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
