---
title: Chordal Graph
type: claim
id: claim-chordal-graph
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
  base: 0.85
---

## Definition

A graph is chordal (also called triangulated) if it has no induced cycle of length ≥ 4 — equivalently, every cycle of length ≥ 4 has a chord. Equivalent characterizations:
- All minimal separators are cliques.
- The graph has a perfect elimination order (every successively removed vertex is simplicial in the remaining graph).
- The graph is the intersection graph of a family of subtrees of some tree.
- It has a clique tree.

## How It Works

Chordal graphs have at most n maximal cliques and can be recognized in linear time via lexicographic BFS. They are perfect, and many NP-complete problems (e.g. coloring, max clique, max independent set) are polynomial on chordal graphs.

Treewidth equals max-clique-size - 1: tw(G) = ω(G) - 1 for chordal G.

## Key Parameters

- Number of maximal cliques ≤ n.
- tw(G) = ω(G) - 1.
- b(G) = ω(G) (bramble number for chordal graphs, Lemma 4.15).

## When To Use

- As the target of triangulations (minimal triangulations underlie treewidth computation).
- As the structural model for many DP-friendly algorithms.

## Risks & Pitfalls

- Domination is NP-complete on chordal graphs despite the polynomial-time independent-set algorithm.
- "Chordal" requires no induced cycle of length ≥ 4; an induced C_3 (triangle) is allowed.

## Related Concepts

- [[concepts/triangulation]]
- [[concepts/clique-tree]]
- [[concepts/simplicial-vertex]]
- [[concepts/treewidth]]
- [[concepts/perfect-graph]]
- [[concepts/interval-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-06-problem-formulations]]
- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
