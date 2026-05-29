---
title: Graph Partitioning
type: claim
id: claim-graph-partitioning
tags:
- graph
- algorithm
- vlsi
- partitioning
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/00-preface.txt
confidence:
  base: 0.65
---

## Definition

Graph partitioning divides the vertices of a graph into a specified number of subsets (parts) so as to optimize an objective such as minimizing the total weight of edges cut while respecting balance constraints on the part sizes.

## How It Works

Given a (possibly weighted) graph G = (V, E) and a number k of parts, partitioning algorithms produce a mapping V -> {1,...,k} that minimizes edge cut or some related metric (ratio cut, normalized cut) subject to size or weight balance. Exact partitioning is NP-hard; practical algorithms include Kernighan-Lin, Fiduccia-Mattheyses, multilevel methods (hMETIS, METIS), and spectral approaches based on the Laplacian eigenvectors.

## Key Parameters

- Number of parts k and balance tolerance.
- Edge and vertex weights.
- Cut-size objective vs. communication-volume objective.
- Multilevel coarsening parameters and refinement iterations.

## When To Use

- VLSI physical design: dividing a netlist into modules during floorplanning and placement.
- Parallel circuit simulation: distributing a circuit graph across processors.
- Domain decomposition for iterative solvers on Laplacian systems.

## Risks & Pitfalls

- Heuristic-quality dependent; suboptimal cuts increase interconnect cost.
- Balance constraints can conflict with cut minimization.
- Repeated runs on the same input can give different results (stochastic refinement).

## Related Concepts

- [[concepts/graph-theory]]
- [[concepts/laplacian-matrix]]
- [[concepts/vlsi-design]]
- [[concepts/floorplanning]]

## Sources

- [[summaries/graphs-in-vlsi-00-preface]]
- [[summaries/graphs-in-vlsi-04-1-introduction]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
