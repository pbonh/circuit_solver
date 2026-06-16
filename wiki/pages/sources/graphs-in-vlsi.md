---
title: "Graphs in VLSI"
type: source
slug: graphs-in-vlsi
created: 2026-06-16
updated: 2026-06-16
summary: Bairamkulov & Friedman's monograph on graph-theoretic methods in VLSI — circuit analysis (Laplacian/MNA), effective resistance of power grids, timing analysis, clock tree synthesis, voltage regulator placement, and power routing.
source_file: Books/GraphsInVLSI
tags: [vlsi, graph-algorithms, circuit-simulation, effective-resistance, clock-tree, power-grid, mna, laplacian]
status: active
---

# Graphs in VLSI

- **Source file:** `sources/Books/GraphsInVLSI/`
- **Author / origin:** R. Bairamkulov & E. G. Friedman; Springer Nature, 2023
- **Date:** 2023

## Summary

A research monograph applying graph theory to VLSI design challenges — from circuit formulation through power delivery and clock synthesis. Bridges abstract graph theory and practical EDA algorithms.

### Part 1: Graph Fundamentals (Ch. 2)
Graph categories: hypergraphs (multi-pin nets), multigraphs (parallel wires), weighted graphs (wire resistance/capacitance), directed graphs (signal flow, current). Exploration: BFS, DFS, topological sort. Bipartite graphs (2-coloring, partitioning), DAGs (scheduling, timing). Common problems: pathfinding (Dijkstra, Bellman-Ford, Floyd-Warshall), spanning trees (Prim, Kruskal — used in clock/power trees), graph coloring (register allocation), topological sort (critical path analysis).

### Part 2: Graphs in VLSI Circuits (Ch. 3)
**RTL layer**: Register allocation via interference graphs (graph coloring assigns variables to registers); task scheduling as DAG critical path; synchronization via constraint graphs.

**Gate layer**: OBDD (ordered BDD) for logic synthesis and equivalence checking; AIG (and-inverter graph) — the compact two-input AND + inverter universal basis used by modern synthesis tools (ABC, OpenROAD).

**Circuit layer**: MNA produces the Laplacian matrix of the circuit graph. For a resistive circuit: G (conductance matrix) is the weighted Laplacian — G = D - A where D is degree matrix and A is adjacency matrix (weighted by conductances). Voltage node potentials are the solution to G·v = i_s.

**Physical layer**: Partitioning (min-cut — Fiduccia-Mattheyses, spectral methods); floorplanning (sequence pairs, B*-trees, constraint-graph optimization); placement (force-directed, SimAnneal); routing (maze routing = BFS, A*; global routing = integer programming on routing graph).

### Part 3: Synchronization in VLSI (Ch. 4)
**Graph-based timing analysis**: Timing graph G = (V, E) where V = flip-flops/gates, E = paths with delay weights. Static timing analysis (STA) = longest-path problem. Setup/hold constraints expressed as timing constraint systems (TCS). Slack computation via backward/forward passes. Clock period determined by the longest data path.

**Clock skew scheduling**: Useful clock skew — intentional skew to violate hold or reduce setup violations; optimal skew scheduling via LP over the timing constraint system.

**Clock tree synthesis (CTS)**: Tree topology constraints (bounded skew), embedding (zero-skew tree via deferred merge embedding and method of means & medians), Elmore delay model for distributed RC tree delay. Bounded-skew and useful-skew tree constructions.

### Part 4: Circuit Analysis (Ch. 5)
**MNA (Modified Nodal Analysis)**: G·v = i_s for resistive circuits; the Laplacian structure enables spectral analysis and graph-theoretic algorithms. Conductance matrix = weighted Laplacian.

**Iterative numerical methods**: Domain decomposition (Schwarz alternating method for partitioned circuit grids); H-matrix (hierarchical matrix approximation for dense interactions at long range — applicable to large power grids with capacitive coupling); multigrid methods (coarse-grid correction for fast linear system solve — algebraic multigrid for unstructured circuit graphs).

**Non-MNA techniques**: Scattering parameters (S-parameters for RF circuits via graph-theoretic signal flow); random walks for computing Green's functions (harmonic measure, Monte Carlo Laplace solver); lattice graph methods for structured grids.

### Part 5: Effective Resistance of Power Grids (Ch. 6-7)
**Effective resistance** between two nodes i, j in a resistive network: R_eff(i,j) = (e_i - e_j)^T L^+ (e_i - e_j) where L^+ is the Laplacian pseudoinverse. Measures the equivalent resistance seen between two terminals — directly predicts voltage drop in power grids.

**Truncated infinite mesh (Ch. 6)**: Closed-form expression for effective resistance in a truncated infinite resistive mesh using Green's function (Fourier analysis on the lattice + image charge technique for boundary conditions). Faster and more accurate than brute-force simulation for regular grid power delivery networks.

**Finite grids (Ch. 7)**: Infinity mirror technique for finite mesh effective resistance — a sequence of image charges mirrors the boundary conditions. Exact closed-form expressions for N×M resistive grids. Applications: mesh reduction, touch screen resistive noise, substrate noise coupling. Appendix A: Green's function derivation for truncated grid.

### Part 6: Power Delivery Optimization (Ch. 8-10)
**Voltage regulator placement (Ch. 8)**: Power grid modeled as resistive graph; fast grid analysis via eigenvalue decomposition. Load clustering reduces problem size; MILP for optimal voltage regulator placement under current, impedance, and placement constraints.

**Power delivery exploration (Ch. 9)**: Co-optimization of electrical (voltage drop, power loss) and non-electrical (area, power integrity margins) metrics. Circuit simulation procedure integrated into optimization loop.

**SPROUT tool (Ch. 10)**: Board-level power routing tool. Algorithm: seed subgraph → growth → refinement → subgraph reheating. Equivalent graph model of available routing space; runtime analysis; 2-rail and 6-rail case studies; area/impedance tradeoff.

### Part 7: SFQ Clock Tree Synthesis (Ch. 11)
**QuCTS**: Clock tree synthesis for single-flux-quantum (SFQ) superconducting logic. Timing graph for SFQ path constraints; minimum clock period optimization; delay equilibration via coarse routing (Steiner tree) + fine routing (Josephson junction delay trimming). Distinct from CMOS CTS due to single-direction pulse propagation.

## Key takeaways

- The MNA conductance matrix is the weighted graph Laplacian; circuit analysis = linear algebra on graphs
- Effective resistance (Laplacian pseudoinverse) is the key metric for power grid analysis — closed-form expressions for regular meshes enable fast design exploration
- Clock tree synthesis is a spanning tree problem with delay and skew constraints; useful skew scheduling is LP over timing constraint graphs
- AIG (and-inverter graph) is the standard logic representation in modern synthesis tools
- Voltage regulator placement is MILP over a resistive grid model
- Random walk Monte Carlo solves Laplace equations on irregular grids — an alternative to direct MNA for large power grids
- Domain decomposition and multigrid methods scale MNA to million-node grids without full matrix inversion

## Pages updated from this source

- [[vlsi-graph-methods]] - concept created (Laplacian, effective resistance, timing graph)
- [[power-grid-analysis]] - concept created
- [[graph-algorithms]] - extended with VLSI-specific applications
- [[circuit-simulation]] - Laplacian/MNA connection reinforced
- [[differential-algebraic-equations]] - MNA as circuit Laplacian
