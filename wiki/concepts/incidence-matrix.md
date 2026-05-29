---
title: Incidence Matrix
type: claim
id: concepts/incidence-matrix
tags:
- graph
- sparse-matrix
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The incidence matrix B(G) of a graph G = (V, E) is a |V|×|E| matrix whose entry B[i][j] records whether vertex v_i is incident to edge e_j. For undirected graphs B[i][j] = 1 if v_i is an endpoint of e_j; for directed graphs B[i][j] = +1 if e_j originates at v_i, -1 if e_j terminates at v_i, 0 otherwise.

## How It Works

In matrix-based graph systems, B (and Bᵀ) supports edge-oriented queries. Multiplying an indicator vector x_S (1 for vertices in subset S) by B yields a vector whose entries report how many endpoints of each edge fall inside S — entries of value 2 (for undirected graphs) identify the edges of the induced subgraph. GBASE uses Bᵀ for queries whose result type is a set of edges, while A and Aᵀ handle vertex-set results. The incidence matrix is dense in columns (each column has only two non-zeros) and is normally stored sparsely.

## Key Parameters

- Directed vs. undirected representation.
- Storage format and block layout.
- Whether entries are weighted (e.g., resistance for circuit graphs).

## When To Use

- Edge-set queries in matrix-based graph systems (GBASE).
- Constructing induced subgraphs algebraically.
- Network-flow and electrical-network formulations where Kirchhoff laws map to Bᵀ·v.

## Risks & Pitfalls

- |E| can be much larger than |V|, making B taller than A is wide.
- Some operations (e.g., k-hop reachability) prefer A; choosing the wrong matrix wastes work.

## Related Concepts

- [[concepts/adjacency-matrix]]
- [[concepts/algebraic-graph-theory]]
- [[concepts/matrix-based-graph-analytics]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
