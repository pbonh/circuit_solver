---
title: "Separator"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/04-graphs.txt"]
confidence: high
---

## Definition

A separator in a graph G is a proper subset S ⊂ V such that G - S is disconnected. For two nonadjacent vertices a and b, an a|b-separator is a set S putting a and b in different components of G - S. A cutvertex is a separator of size one.

## How It Works

Separators encode where a graph "narrows." Removing a separator splits the graph into two or more components. Cutvertices and biconnected components organize the global structure into a block-cut tree.

## Key Parameters

- Connectivity κ(G): minimum size of any vertex separator (∞ for cliques).
- Biconnectivity: a graph is biconnected if every separator has size ≥ 2 (no cutvertex).

## When To Use

- As a precondition for dynamic programming over decompositions (chordal, treewidth, modular).
- For divide-and-conquer algorithms exploiting balanced separators.

## Risks & Pitfalls

- "Minimal" vs. "minimum" separators differ: minimum is smallest by cardinality globally; minimal is irreducible (no proper subset separates the same pair).
- For disconnected graphs the empty set is a (trivial) minimal separator.

## Related Concepts

- [[concepts/graph]]
- [[concepts/minimal-separator]]
- [[concepts/clique-separator]]
- [[concepts/connectedness]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
