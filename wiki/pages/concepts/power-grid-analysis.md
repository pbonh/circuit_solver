---
title: Power Grid Analysis
type: concept
slug: power-grid-analysis
created: 2026-06-16
updated: 2026-06-16
summary: Analysis of on-chip/board power delivery networks modeled as resistive graphs; effective resistance predicts voltage drop; closed-form Laplacian methods enable fast design exploration.
tags: [vlsi, power-grid, effective-resistance, laplacian, voltage-drop, power-delivery]
sources: [graphs-in-vlsi]
status: active
---

# Power Grid Analysis

On-chip and board-level power delivery networks (PDNs) are resistive (and RC) graphs connecting voltage sources to switching logic loads. Power grid analysis computes the voltage drop (IR drop) at every node under worst-case load conditions, determines voltage regulators' effectiveness, and guides power grid sizing.

## Graph Model

The power grid is modeled as a weighted graph where:
- Nodes = circuit nodes (power and ground rails, decoupling capacitor nodes)
- Edges = metal wire segments with resistance (and capacitance for dynamic analysis)
- Sources = voltage regulators (V-sources in MNA)
- Loads = switching currents of logic cells (I-sources in MNA)

The resulting [[vlsi-graph-methods]] conductance matrix is the weighted Laplacian. Solving G·v = i gives the node voltages (voltage drops).

## Effective Resistance

Effective resistance R_eff(u,v) = (e_u - e_v)^T L^+ (e_u - e_v) predicts the worst-case voltage drop for a unit current injected at u and extracted at v. Key use: rank voltage regulator placement candidates by R_eff to the load cluster.

**Closed-form for regular meshes** (from [[graphs-in-vlsi]]): For an N×M resistive grid, the Green's function + infinity mirror technique yields exact closed-form effective resistance expressions — order-of-magnitude faster than full MNA simulation. Applicable to early-stage power grid exploration before full netlist is available.

## Analysis Methods

| Method | Complexity | Use |
|---|---|---|
| Direct LU (SPICE/MNA) | O(n^{1.5}) sparse | Small-medium grids |
| Multigrid (AMG) | O(n log n) | Large grids |
| Domain decomposition | O(n) parallel | Distributed power grid |
| Random walk (Monte Carlo Laplace) | O(n) per point | Selected hotspot nodes |
| H-matrix (hierarchical) | O(n log^2 n) | Long-range capacitive coupling |
| Closed-form (regular mesh) | O(1) | Early exploration |

## Voltage Regulator Placement

Given a set of load clusters, place distributed on-chip voltage regulators to minimize maximum voltage drop:
- Model: resistive grid + load current sources + regulator current sources (limited by max current)
- Algorithm (SPROUT-style): seed placement → growth → refinement using effective resistance to guide placement
- Optimization: MILP for exact placement under area and current constraints

## Connection to Circuit Simulation

Power grid analysis is a large sparse linear system solve — same Laplacian structure as [[spice-simulation]]'s MNA. The difference is scale (millions of nodes for full-chip power grid) vs. accuracy (DC analysis for IR drop, transient for simultaneous switching noise SSN).

## Related concepts and entities

- [[vlsi-graph-methods]] - Laplacian formulation of circuit graphs
- [[circuit-simulation]] - MNA solve is shared machinery
- [[differential-algebraic-equations]] - power grid as index-1 DAE
- [[graph-algorithms]] - algorithms for power grid structure
