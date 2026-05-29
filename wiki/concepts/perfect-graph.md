---
title: Perfect Graph
type: claim
id: concepts/perfect-graph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A graph G is perfect if χ(H) = ω(H) for every induced subgraph H of G. The Strong Perfect Graph Theorem (Chudnovsky-Cornuéjols-Liu-Seymour-Vušković 2008) characterizes perfect graphs as those without odd holes (induced cycles of odd length ≥ 5) or odd antiholes.

Perfect Graph Theorem (Lovász 1972): G is perfect iff Ḡ is perfect.

## How It Works

For perfect graphs, χ, ω, and α are polynomial-time computable via the Lovász theta function (Grötschel-Lovász-Schrijver 1988). Equivalent characterization: G is perfect iff every induced subgraph H satisfies α(H) · ω(H) ≥ |V(H)| (does not hold for odd cycles).

Many classes are perfect: bipartite, linegraphs of bipartite, chordal, distance-hereditary, comparability, and their complements.

## Key Parameters

- χ = ω on all induced subgraphs.
- α and ω computable in polynomial time.

## When To Use

- As a tractable subclass for NP-complete coloring problems.
- Verifying class membership via the Strong Perfect Graph Theorem.

## Risks & Pitfalls

- Recognition is polynomial-time (Chudnovsky et al.) but practically slow.
- AT-free graphs are NOT all perfect (C_5 is AT-free but not perfect).

## Related Concepts

- [[concepts/chromatic-number]]
- [[concepts/clique]]
- [[concepts/chordal-graph]]
- [[concepts/distance-hereditary-graph]]
- [[concepts/cograph]]
- [[concepts/bipartite-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
