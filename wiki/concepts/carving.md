---
title: Carving
type: claim
id: claim-carving
tags:
- graph
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

A carving of a finite set V (with |V| ≥ 2) is a maximal family C of subsets of V such that:
- ∅, V ∉ C.
- No two members of C cross (i.e., for X, Y ∈ C at least one of X ∩ Y, X \ Y, Y \ X is empty — they form a laminar family).
- C is maximal under these conditions.

Equivalently, C corresponds to a ternary tree T with leaves identified with V; each internal edge induces a bipartition of V, and the carving is the collection of leaf-sets of one side of each bipartition.

## How It Works

Carvings underpin carving width (max edges crossing a carving cut) and rankwidth (max GF[2]-rank of a cut matrix). Bond carvings additionally require δ(X) to be a bond for each X ∈ C, where δ(X) is the set of edges with exactly one endpoint in X.

For 2-connected graphs, Theorem 4.44 shows a bond-carving of minimum p-width exists.

## Key Parameters

- Width = max_{X ∈ C} |δ(X)| (or weighted p(δ(X))).
- Ternary tree has at most |V| - 2 internal nodes.

## When To Use

- Branchwidth, carving width, and rankwidth computations.
- Planar-graph approximations (Seymour-Thomas treewidth approximation via carving width).

## Risks & Pitfalls

- "Cross-free" is sometimes called "laminar"; the two terms coincide for fixed ground set V.
- Restricting to ternary trees is essential; star-shaped trees would trivialize rankwidth.

## Related Concepts

- [[concepts/carving-width]]
- [[concepts/rankwidth]]
- [[concepts/treewidth]]
- [[concepts/antipodality]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
