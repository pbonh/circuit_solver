---
title: Treewidth
type: claim
id: concepts/treewidth
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
- raw/GuideToGraphAlgorithms/_txt/06-problem-formulations.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

The treewidth tw(G) is defined as min{ω(H) - 1 : H is a chordal embedding (triangulation) of G}. Equivalently:
- tw(G) is the minimum width of a tree-decomposition of G.
- tw(G) + 1 = bramble number b(G) (Seymour-Thomas 1993).

A chordal embedding H has V(H) = V(G), E(G) ⊆ E(H), and H is chordal.

## How It Works

Treewidth measures how "tree-like" a graph is. Trees have tw = 1, cycles have tw = 2, K_n has tw = n - 1.

- Bodlaender (1996) gives a linear-time algorithm for tw(G) ≤ k (for fixed k).
- Courcelle's theorem: MS2 problems are linear-time on graphs of bounded treewidth.
- Computing tw is NP-complete in general, even for bipartite, cobipartite, and claw-free graphs.

Dynamic programming over a nice tree-decomposition (start / introduce / forget / join nodes) yields efficient algorithms for many problems, e.g. Steiner tree in O(k · B_{2k+1} · n).

## Key Parameters

- tw(G) ≥ 1 (with tw = 1 for trees, tw = 0 by some conventions).
- O(n^3) for many MS1 problems on bounded-rankwidth via Courcelle.
- O(M(n)) for circle graphs via plane triangulations.

## When To Use

- Designing FPT algorithms for NP-hard problems on graphs.
- Decomposing structured graphs (chordal, k-outerplanar, bounded-tree-degree).

## Risks & Pitfalls

- "Treewidth" and "pathwidth" differ: pathwidth allows only paths as decomposition trees.
- Approximation: treewidth of planar graphs has an O(n^4) factor-3/2 approximation via carving width.

## Related Concepts

- [[concepts/tree-decomposition]]
- [[concepts/chordal-graph]]
- [[concepts/triangulation]]
- [[concepts/clique-tree]]
- [[concepts/bramble]]
- [[concepts/carving-width]]
- [[concepts/courcelle-theorem]]

## Sources

- [[summaries/guide-to-graph-algorithms-01-preface]]
- [[summaries/guide-to-graph-algorithms-02-about-the-authors]]
- [[summaries/guide-to-graph-algorithms-06-problem-formulations]]
- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
