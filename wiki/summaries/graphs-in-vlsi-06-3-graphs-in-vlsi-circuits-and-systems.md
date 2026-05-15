---
title: "Graphs in VLSI — Chapter 3: Graphs in VLSI Circuits and Systems"
type: summary
tags: [graph, vlsi, digital, foundational, well-established, eda, partitioning, routing, placement, floorplanning]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt"]
confidence: high
---

## Key Points

- VLSI design is organized as a hierarchy of abstraction layers — register transfer (RTL), gate, circuit, and physical — connected by transformations from behavioral description to physical layout. Layered abstraction provides focus, simplification, and generalization.
- At the RTL, register allocation is cast as graph coloring on an interference graph whose nodes are program variables and edges encode overlapping live ranges. Chaitin's coloring algorithm (1981) was the first applied to register allocation; modern algorithms hybridize coloring with linear scan and achieve O(log n)-time coloring.
- Task scheduling produces a topological ordering of a DAG of tasks. Parallel scheduling on heterogeneous processors is NP-hard; classic algorithms include HEFT (Heterogeneous-Earliest-Finish-Time) and CPOP (Critical-Path-on-a-Processor), both O(|E||V|).
- Synchronization at RTL is performed via clock skew scheduling on a timing graph. Positive skew (clock counter to data flow) reduces effective period and is immune to race conditions; negative skew (clock aligned with data flow) increases effective period but risks race/double-clocking. A permissible-range constraint [l_if, u_if] on each datapath is enforced via linear or quadratic programming.
- At the gate layer, logic circuits map to directed graphs where fan-in/fan-out correspond to indegree/outdegree. Boolean functions are represented as truth tables (canonical but exponential), Ordered Binary Decision Diagrams (OBDDs, canonical when reduced — ROBDDs — with strong dependence on variable ordering), and And-Inverter Graphs (AIGs, scalable but non-canonical; FRAIG is semi-canonical via structural hashing). Equivalence checking uses the miter technique and SAT solving.
- At the circuit layer, Kirchhoff (1845) introduced graph-based circuit analysis. The incidence matrix Y, directed reduced incidence matrix Yd^g, adjacency matrix A_w, degree matrix D, and weighted Laplacian L = D − A encode the circuit. KCL is Yd^g J + Q = 0, KVL is W(Yd^g)^T = V_g, and node voltages relative to ground satisfy V_g = L_g^{-1} Q. The Laplacian L is singular due to ground; grounding removes one row/column to make L_g invertible. Effective resistance R_ij = v_i − v_j when injecting unit current at i and drawing at j.
- Physical-layer design consists of partitioning, floorplanning, placement, and routing. Partitioning is cast as minimum k-cut (and hypergraph min-cut for multi-pin nets). The Kernighan-Lin (KL) algorithm (1970s) introduced node-swap heuristics with gain G_ab = D_a + D_b − 2c_ab; Fiduccia-Mattheyses (FM, 1982) generalized to hypergraphs, unequal partitions, and single-node moves. Modern methods include METIS multilevel clustering, genetic optimization, ant colony, and particle swarm optimization.
- Floorplanning is a rectangular-packing problem. Quality metrics are area efficiency η_A(F) and total wirelength via Half-Perimeter WireLength (HPWL). Floorplans are represented by horizontal/vertical constraint graphs, O-trees (DFS-traversal-based), and B*-trees (binary trees with right/top child relationships).
- Placement determines exact block positions to minimize wirelength, congestion, timing criticality, and power. Rectilinear spanning trees and Steiner trees on Hanan grids estimate net length. Congestion maps and A* traversal guide placement adjustments.
- Routing produced the first wire-routing algorithm — Lee's maze router (1961), a BFS variant. A* drastically reduces traversed nodes. Modern routing is hierarchical: global routing on a channel/switchbox connectivity graph, then detailed routing typically on two layers with channel-routing constraint graphs (vertical and horizontal constraint graphs analogous to register-allocation interference graphs).

## Relevant Concepts

- [[concepts/vlsi-design]] — the engineering hierarchy this chapter walks through.
- [[concepts/abstraction-layer]] — fundamental design-management technique used to manage VLSI complexity.
- [[concepts/register-transfer-level]] — abstraction at which clock skew scheduling and register allocation occur.
- [[concepts/register-allocation]] — graph-coloring formulation on interference graph.
- [[concepts/interference-graph]] — encodes live-range conflicts among variables.
- [[concepts/graph-coloring]] — NP-hard core of register allocation.
- [[concepts/task-scheduling]] — topological-ordering problem on task DAGs with parallel scheduling.
- [[concepts/clock-skew-scheduling]] — timing-graph optimization for synchronous systems.
- [[concepts/timing-graph]] — directed graph of clocked elements and datapaths.
- [[concepts/ordered-binary-decision-diagram]] — canonical (when reduced) Boolean function representation.
- [[concepts/and-inverter-graph]] — scalable but non-canonical Boolean representation.
- [[concepts/boolean-satisfiability]] — verification problem reducible to graph search.
- [[concepts/modified-nodal-analysis]] — circuit analysis built on Laplacian matrix.
- [[concepts/laplacian-matrix]] — central matrix in circuit-graph analysis; L = D − A.
- [[concepts/incidence-matrix]] — encodes node-edge incidence for KCL/KVL.
- [[concepts/effective-resistance]] — R_ij from Laplacian-based node voltages.
- [[concepts/graph-partitioning]] — physical-layer decomposition step.
- [[concepts/kernighan-lin-algorithm]] — classical min-cut bisection heuristic.
- [[concepts/fiduccia-mattheyses-algorithm]] — hypergraph generalization of KL.
- [[concepts/floorplanning]] — rectangular packing using constraint graphs and tree representations.
- [[concepts/placement]] — block-positioning optimization.
- [[concepts/interconnect-routing]] — wire synthesis stage.
- [[concepts/maze-routing]] — Lee's 1961 BFS-based grid routing.
- [[concepts/steiner-minimal-tree]] — net-length estimation in placement and routing.
- [[concepts/hanan-grid]] — search-space restriction for RSMT.
- [[concepts/a-star-algorithm]] — heuristic-guided routing.
- [[concepts/directed-acyclic-graph]] — used for task graphs and combinational logic.

## Source Metadata

- Source type: book chapter
- Book title: Graphs in VLSI
- Chapter: 3 — Graphs in VLSI circuits and systems
- File path: `raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
