---
title: Twin
type: claim
id: claim-twin
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
  base: 0.85
---

## Definition

Two vertices x and y are twins if every other vertex is adjacent to both or to neither. A true twin satisfies N[x] = N[y] (both x ∼ y and same closed neighborhoods); a false twin satisfies N(x) = N(y) (x not adjacent to y, same open neighborhoods).

## How It Works

A twin is a 2-element module. Cographs are exactly the graphs in which every induced subgraph with ≥ 2 vertices has a twin. Distance-hereditary graphs admit elimination via pendant vertices and twins. The k-cograph hierarchy generalizes the twin structure to "labeled twins" governed by symmetric Boolean k×k matrices.

Detecting twins takes linear time using equivalence classes of neighborhoods.

## Key Parameters

- Twin equivalence classes partition V into modules.
- Representative graph R(G) collapses twin classes to single vertices.

## When To Use

- Preprocessing step to reduce a graph by identifying equivalent vertices.
- Distance-hereditary recognition / k-cograph recognition.
- Domino-graph linear-time recognition uses the representative graph (it must be a linegraph of a triangle-free graph with specific pendant properties).

## Risks & Pitfalls

- "Anti-twin" is different: pair (x, y) such that every third vertex is adjacent to exactly one of them. Switch-equivalence of cographs uses anti-twins.

## Related Concepts

- [[concepts/cograph]]
- [[concepts/module]]
- [[concepts/distance-hereditary-graph]]
- [[concepts/k-cograph]]
- [[concepts/domino-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
