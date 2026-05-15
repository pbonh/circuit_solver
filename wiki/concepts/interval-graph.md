---
title: "Interval Graph"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

An interval graph is the intersection graph of a collection of intervals on the real line. Equivalent characterizations (Lekkerkerker-Boland):
- Chordal AND AT-free.
- The maximal cliques can be linearly ordered C_1 … C_t such that for any vertex v, the cliques containing v form an interval (consecutive clique arrangement).
- The clique tree is a path.

## How It Works

Interval graphs are recognized in linear time (Booth-Luecker via PQ-trees; lex-BFS; modular decomposition).

Many problems are polynomial on interval graphs:
- Coloring (chromatic number = max clique size).
- Independent set (greedy by interval endpoints).
- Bandwidth (Kleitman-Vohra O(n^2)).
- Domination (polynomial despite being NP-complete on chordal).

The simple elimination orders of interval graphs form the words of an antimatroid.

## Key Parameters

- |V(G)| = number of intervals.
- t = number of maximal cliques.
- Each minimal separator C_i ∩ C_{i+1}.

## When To Use

- Scheduling (each task is an interval).
- Genome alignment (overlapping fragments).
- Compiler register allocation in straight-line code.

## Risks & Pitfalls

- Interval graphs are not closed under minor or edge contraction.
- Test cases for "chordal but not interval": chord-less paths in chordal graphs with multiple branches.

## Related Concepts

- [[concepts/chordal-graph]]
- [[concepts/at-free-graph]]
- [[concepts/pq-tree]]
- [[concepts/clique-tree]]
- [[concepts/tree-degree]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
