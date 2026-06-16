---
title: VLSI Graph Methods
type: concept
slug: vlsi-graph-methods
created: 2026-06-16
updated: 2026-06-16
summary: Graph-theoretic representations and algorithms used across VLSI design — circuit Laplacian (MNA), timing graphs, effective resistance, and physical design graphs — connecting abstract graph theory to circuit EDA.
tags: [vlsi, graph-algorithms, laplacian, mna, timing-analysis, clock-tree, eda]
sources: [graphs-in-vlsi]
status: active
---

# VLSI Graph Methods

VLSI circuits are inherently graph-structured at every abstraction level. Graph algorithms drive logic synthesis, timing analysis, physical design, and circuit simulation. The circuit graph Laplacian directly expresses the MNA conductance matrix, linking electrical analysis to spectral graph theory.

## VLSI Abstraction Levels and Their Graphs

| Level | Graph Representation | Key Algorithms |
|---|---|---|
| RTL | DAG (scheduling), interference graph (register alloc) | Topological sort, graph coloring |
| Gate | AIG (and-inverter graph), BDD | Technology mapping, equivalence checking |
| Circuit | Conductance matrix = weighted Laplacian | MNA solve, effective resistance |
| Physical | Partition graph, placement force graph, routing graph | Min-cut, force-directed, maze routing |

## Circuit Laplacian and MNA

The MNA conductance matrix G for a resistive circuit is the weighted graph Laplacian:
  G_{ij} = -g_{ij}  (off-diagonal: negative conductance of branch i-j)
  G_{ii} = sum_j g_{ij}  (diagonal: sum of conductances at node i)

This is exactly the weighted Laplacian L = D - A where D is the degree matrix and A is the adjacency matrix. Linear circuit analysis = solving L·v = i_s, a sparse linear system. All spectral graph theory results apply: the zero eigenvalue = connected components, eigenvalue gap = algebraic connectivity (Fiedler value), eigenvectors = natural modes.

## Effective Resistance

The effective resistance between nodes u and v:
  R_eff(u,v) = (e_u - e_v)^T L^+ (e_u - e_v)

where L^+ is the Laplacian pseudoinverse. This is the equivalent resistance seen looking into the circuit between u and v, and directly predicts voltage drop in power grids.

For regular infinite resistive meshes (power grid approximation), closed-form expressions exist via Green's functions and the method of images — dramatically faster than full MNA simulation for power grid exploration.

## Timing Graph and STA

Timing graph G = (V, E) where V = flip-flops and gates, E = paths with delay weights. Static timing analysis (STA) = longest path computation (Bellman-Ford on the timing graph). Setup constraint = path delay < clock period. Hold constraint = path delay > hold time. Slack = headroom to constraint violation.

Clock skew scheduling = LP over timing constraint system variables. Zero-skew tree synthesis = spanning tree with balanced path lengths. Useful skew = intentional imbalance to relax setup timing.

## Physical Design Graph Algorithms

- **Partitioning**: Fiduccia-Mattheyses (FM) = iterative min-cut on hypergraph; spectral = eigenvalue-based balanced bisection
- **Placement**: Force-directed = spring model on graph distances; SimAnneal on adjacency-weighted objective
- **Routing**: Global routing on routing graph (Steiner tree, integer LP); detailed routing = maze routing (BFS/A*) per net

## Connection to [[circuit-simulation]]

MNA = sparse Laplacian linear system solve at each NR iteration. [[spice-simulation]]'s inner loop is exactly a graph Laplacian solve. [[power-grid-analysis]] uses the Laplacian pseudoinverse for voltage drop.

## Related concepts and entities

- [[graph-algorithms]] - algorithmic foundations
- [[power-grid-analysis]] - effective resistance applied to power delivery
- [[circuit-simulation]] - MNA = circuit Laplacian solve
- [[differential-algebraic-equations]] - MNA is an index-1 DAE with Laplacian structure
- [[symbolic-circuit-analysis]] - symbolic Laplacian via DDD/GPDD
- [[treewidth-and-graph-structure]] - low treewidth enables efficient VLSI algorithms
