---
title: "Robertson-Seymour Graph Minors Series"
type: entity
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Overview

The Robertson-Seymour Graph Minors series is a sequence of more than 20 research papers by Neil Robertson and Paul Seymour (1983-2004) that culminated in the proof of the Graph Minor Theorem: the class of all finite graphs is well-quasi-ordered by the minor relation.

Key papers cited in Kloks-Xiao Chapter 4:
- Graph Minors X: Obstructions to tree-decomposition (JCT-B 52, 1991) — provides the bias / tilt lemma used in planar carving width.
- Graph Minors XI: Circuits on a surface (JCT-B 60, 1994) — slopes-to-antipodality argument.

## Characteristics

- Cumulative proof of the Wagner conjecture (Graph Minor Theorem).
- Introduces treewidth, branchwidth, tree-decomposition, tangle, bramble, well-quasi-order via minors, and the structure theorem for H-minor-free graphs.
- Establishes cubic-time minor detection for fixed H.

## Common Strategies

- Tangle / bramble / tree-decomposition duality.
- Excluded-minor structure theorems (each H-minor-free graph decomposes into "almost-embedded" pieces and apex vertices).
- Layered network duality (bond carvings, antipodality, planarity).

## Related Entities

- [[entities/graph-algorithm-research]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
