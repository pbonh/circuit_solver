---
title: Circle Graph
type: claim
id: claim-circle-graph
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

A circle graph is the intersection graph of a set of chords of a circle in the Euclidean plane. Two chords are adjacent iff they cross.

## How It Works

Circle graphs admit O(n^3) treewidth computation via dynamic programming on plane triangulations of the polygon formed by the chord endpoints. A scanline (chord of the polygon with no endpoint shared with G) separates chords into "left," "right," and "crossing" — useful for minimal separator enumeration.

Davies-McCarty: vertices of a circle graph can be partitioned into 7ω parts each inducing a permutation graph. This implies polynomial χ-bounding for circle graphs.

Geelen-Kwon-McCarthy-Wollan: any class of graphs without H-vertex-minor (for fixed H a circle graph) has bounded rankwidth.

## Key Parameters

- Number of chord crossings.
- |V(G)| = number of chords.

## When To Use

- Coordinate placement on layered networks.
- Compiler register allocation in tree-structured programs.
- Test bed for vertex-minor and rankwidth research.

## Risks & Pitfalls

- Circle graph recognition runs in "almost linear" time but is intricate.
- Independence number is computable in polynomial time but coloring is NP-hard.

## Related Concepts

- [[concepts/treewidth]]
- [[concepts/permutation-graph]]
- [[concepts/rankwidth]]
- [[concepts/chi-boundedness]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
