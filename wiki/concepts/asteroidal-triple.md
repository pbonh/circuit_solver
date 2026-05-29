---
title: Asteroidal Triple
type: claim
id: concepts/asteroidal-triple
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

An asteroidal triple in a graph G is a set of three vertices {a, b, c} such that for each one, the other two are connected by a path that avoids the closed neighborhood of the third. Equivalently, the set {a, b, c} is "asteroidal" if removing N[v] from G keeps the other two in the same component, for v ∈ {a, b, c}.

More generally, an asteroidal set A satisfies the same property for all vertices in A (Definition 4.186).

## How It Works

Lekkerkerker-Boland: a graph is an interval graph iff it is chordal and has no asteroidal triple.

Gallai gave a list of minimal forbidden subgraphs with asteroidal triple. Classic examples:
- Subdivided claw (each edge of K_{1,3} subdivided once).
- Independent set of 3 vertices in C_6.
- Three leaves of a 3-sun.

AT-free graphs (graphs without asteroidal triples) include interval graphs, permutation graphs, and cocomparability graphs.

## Key Parameters

- The triple {a, b, c}.
- Asteroidal number: largest asteroidal set.

## When To Use

- Recognizing interval / permutation graphs.
- Demonstrating structural restrictions on graph classes.

## Risks & Pitfalls

- AT-free graphs are not perfect (C_5 is AT-free but not perfect).
- Computing the asteroidal number is NP-complete in general.

## Related Concepts

- [[concepts/at-free-graph]]
- [[concepts/interval-graph]]
- [[concepts/permutation-graph]]
- [[concepts/chordal-graph]]
- [[concepts/dominating-pair]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
