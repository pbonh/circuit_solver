---
title: Symbolic Circuit Analysis
type: concept
slug: symbolic-circuit-analysis
created: 2026-06-16
updated: 2026-06-16
summary: Generating closed-form analytical expressions for circuit transfer functions, noise, and performance metrics in terms of symbolic component values — using BDD/DDD/GPDD graph-based methods.
tags: [symbolic-analysis, vlsi, bdd, ddd, analog-circuits, transfer-function, mna]
sources: [advanced-symbolic-analysis-vlsi]
status: active
---

# Symbolic Circuit Analysis

Symbolic analysis generates rational polynomial expressions H(s, p₁, p₂, ...) for circuit behavior (transfer functions, impedances, noise) where p_i are symbolic component values. Contrasted with numerical simulation (single operating point per run), symbolic analysis reveals the full parameter-dependence in closed form.

## Core Problem

Given a circuit netlist, compute the MNA (Modified Nodal Analysis) system matrix M(s, p); extract the determinant det(M) = sum of product terms (spanning trees). Each term is a monomial in the component values. The transfer function between any two nodes is a ratio of matrix cofactors.

**Complexity challenge**: for a circuit with n components, the determinant can have up to n! terms. Compact representation is essential. BDD-based methods exploit structure and sharing to keep the representation tractable.

## BDD-Based Methods

**BDD (Binary Decision Diagram)**: DAG where paths enumerate spanning trees; shared nodes represent common subsets. Operations are Boolean (AND/OR/XOR) corresponding to inclusion/exclusion in the determinant.

**DDD (Determinant Decision Diagrams)**: Multi-valued extension of BDD for algebraic terms. Each path = one product term. s-expansion captures frequency polynomial coefficients. DDD-based approximation: k-shortest-path finds dominant terms (highest-magnitude contributions).

**GPDD (Graph-Pair Decision Diagram)**: Combines two-graph theory with BDD. Cancellation-free — avoids subtractive cancellation that plagues direct determinant expansion for large circuits. More numerically stable than DDD.

## Advantages Over Numerical SPICE

| Capability | SPICE (Numerical) | Symbolic (DDD/GPDD) |
|---|---|---|
| Operating point | One per simulation run | Parameterized closed form |
| Sensitivity | Finite-difference | Exact analytic gradient |
| Process variation | Monte Carlo (many runs) | Direct parameter bounds |
| Design insight | Numerical waveforms | Poles/zeros, dominant terms |
| Optimization | Iterative simulation | Gradient-based on expressions |

## Applications in VLSI

- **Statistical yield analysis**: parameter-dependent transfer function → yield bounds without Monte Carlo
- **GPU Monte Carlo**: parallel DDD evaluation of thousands of process variation samples
- **Interconnect timing**: symbolic moments give Elmore delay and crosstalk bounds for RC trees
- **Analog synthesis**: optimize component values using symbolic expressions as objective functions
- **Sensitivity analysis**: exact partial derivatives from symbolic stamps

## Connection to Circuit Simulation

From [[advanced-symbolic-analysis-vlsi]]: "Symbolic analysis can serve as a good complement to numerical analysis." [[spice-simulation]] gives accurate numerical waveforms; symbolic analysis gives design insight and statistical analysis without repeated simulation. Both are needed in the full EDA flow.

## Related concepts and entities

- [[spice-simulation]] - the numerical counterpart
- [[circuit-simulation]] - parent topic
- [[graph-algorithms]] - BDD/DDD are graph data structures with graph operations
- [[differential-algebraic-equations]] - MNA circuit equations that symbolic analysis solves symbolically
