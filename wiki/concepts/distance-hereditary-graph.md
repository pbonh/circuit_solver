---
title: Distance-Hereditary Graph
type: claim
id: claim-distance-hereditary-graph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/06-problem-formulations.txt
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

A graph G is distance-hereditary if every connected induced subgraph H of G preserves distances: d_H(x, y) = d_G(x, y) for all x, y ∈ V(H). Equivalently, for any two nonadjacent vertices x, y all chordless x ~ y paths have the same length.

Forbidden-induced characterization: no house, hole (induced cycle of length ≥ 5), domino, or gem.

## How It Works

Distance-hereditary graphs are exactly the graphs of rankwidth ≤ 1 (Theorem 4.66 in the text). They are closed under taking induced subgraphs and under adding pendant vertices or twins. Every distance-hereditary graph has a "one-vertex elimination" by pendants and twins.

Linear-time recognition is possible via this elimination order. Many MS1 problems are linear-time on distance-hereditary graphs (special case of Courcelle).

## Key Parameters

- Diameter ≤ n / 2 + constant.
- Bounded rankwidth (= 1).

## When To Use

- Models where local neighborhoods determine global distances.
- Generalization of trees and cographs to a wider class.

## Risks & Pitfalls

- Distance-hereditary graphs are not chordal (they include odd cycles via "hole" exclusion only for length ≥ 5).
- "Distance-hereditary" properties hold for induced subgraphs, not arbitrary subgraphs.

## Related Concepts

- [[concepts/rankwidth]]
- [[concepts/cograph]]
- [[concepts/tree]]
- [[concepts/perfect-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-06-problem-formulations]]
- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
