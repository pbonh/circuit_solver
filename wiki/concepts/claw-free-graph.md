---
title: "Claw-Free Graph"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

A claw is the complete bipartite graph K_{1,3}: a center vertex adjacent to three pairwise-non-adjacent leaves. A graph is claw-free if it has no induced claw.

Linegraphs are always claw-free. Harary lists nine forbidden induced subgraphs characterizing linegraphs as a strict subset of claw-free graphs.

## How It Works

Recognition is polynomial (a graph is claw-free iff for every vertex v, N(v) induces a graph that contains no induced 3K_1). For sparse graphs, every vertex has ≤ 2√m neighbors; this enables faster algorithms.

Minty's algorithm (and the Faenza et al. improvement) computes α in claw-free graphs in O(n^3). Computing ω is NP-complete in claw-free graphs (reduces from triangle-free α).

## Key Parameters

- For sparse claw-free graphs, max degree ≤ O(√m).
- Treewidth, rankwidth: claw-free graphs have unbounded treewidth and rankwidth.

## When To Use

- Generalization of linegraphs for matching problems.
- Modeling resource-conflict scenarios (e.g. interval scheduling with bounded clique size).

## Risks & Pitfalls

- Claw-free graphs are not closed under edge-contraction (e.g. the bull contracts to a claw).
- ω is NP-complete; only α is polynomial.

## Related Concepts

- [[concepts/linegraph]]
- [[concepts/mintys-algorithm]]
- [[concepts/independent-set]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
