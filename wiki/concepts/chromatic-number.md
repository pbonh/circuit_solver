---
title: "Chromatic Number"
type: concept
tags: [graph, foundational, well-established, np-hard]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/04-graphs.txt"]
confidence: high
---

## Definition

A proper coloring of a graph G is a map V → C such that adjacent vertices receive different colors. The chromatic number χ(G) is the smallest |C| such that a proper coloring exists.

By definition χ(G) ≤ 2 iff G is bipartite. For any G, χ(G) ≥ ω(G).

## How It Works

Computing χ is NP-complete in general (even for planar graphs with χ = 3; the χ ≤ 4 case is the Four-Color Theorem). For perfect graphs, χ(G) = ω(G) on every induced subgraph and χ is computable in polynomial time via the Lovász theta function.

χ-bounded classes are classes for which χ ≤ f(ω) for some function f. Examples: cographs, chordal graphs, distance-hereditary graphs, AT-free graphs, k-cographs, and graphs of bounded rankwidth.

## Key Parameters

- χ(G).
- "Defect" coloring relaxes properness: each monochromatic component has max degree ≤ d.
- "Clustered" coloring: each monochromatic component has size ≤ c.

## When To Use

- Register allocation (interference graph).
- Frequency assignment, scheduling, map coloring.

## Risks & Pitfalls

- Approximation hardness: no polynomial algorithm achieves n^(1-ε) approximation unless P = NP.
- χ-boundedness is not preserved under all graph products (Hedetniemi's conjecture was refuted by Shitov 2019).

## Related Concepts

- [[concepts/graph]]
- [[concepts/clique]]
- [[concepts/bipartite-graph]]
- [[concepts/perfect-graph]]
- [[concepts/chi-boundedness]]
- [[concepts/clustered-coloring]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
