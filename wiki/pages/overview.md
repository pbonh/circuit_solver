---
title: Overview
type: overview
slug: overview
created: 2026-06-16
updated: 2026-06-16
summary: Knowledge base for the Circuit Solver Delta project — circuit simulation, VLSI algorithms, graph methods, numerical solvers, semiconductor physics, distributed systems, and data tooling.
tags: []
status: active
---

# Overview

This wiki supports the Circuit Solver Delta project. It covers circuit simulation methods, VLSI graph algorithms, numerical ODE/DAE solvers, combinatorial and constraint reasoning, scalable distributed systems, semiconductor physics, Rust systems programming, and Python data analysis tooling.

## Thesis

Circuit simulation at scale requires mastery of four orthogonal problem spaces: (1) accurate numerical solution of nonlinear DAEs (Newton-Raphson, BDF integration, Radau/Rosenbrock, convergence), (2) efficient graph-based circuit representation and analysis (VLSI graph algorithms, effective resistance, Laplacian, symbolic analysis, clock/power trees), (3) formal verification and combinatorial optimization (SAT/SMT, constraint programming, parallel search), and (4) scalable system infrastructure (big graph analytics, distributed databases, Kafka/Flink stream processing). Behavioral HDLs (Verilog-AMS) bridge transistor-level accuracy and system-level productivity. Rust provides the performance + safety substrate for implementing new simulation engines.

## Key entities

- [[ken-kundert]] - Spectre architect; foundational SPICE/analog simulation methodology

## Key concepts — Circuit Simulation

- [[circuit-simulation]] - the central topic tying this wiki together
- [[spice-simulation]] - DC, AC, transient, Fourier analysis modes
- [[newton-raphson]] - nonlinear solver; convergence criteria; KCL vs. ΔI check
- [[integration-methods]] - FE, BE, TR, G2; stability, LTE, stiff circuits
- [[homotopy-methods]] - source stepping, Gmin stepping, pseudo-transient for convergence recovery
- [[verilog-ams]] - analog/mixed-signal HDL for behavioral and mixed-level simulation
- [[differential-algebraic-equations]] - MNA circuit equations as index-1 DAE
- [[bdf-methods]] - BDF1-6 multistep stiff solvers (Gear2 in SPICE)
- [[stiff-ode-methods]] - Radau IIA, SDIRK, Rosenbrock for stiff ODEs and DAEs
- [[runge-kutta-methods]] - implicit RK family; RADAU5, Lobatto, Rosenbrock
- [[symbolic-circuit-analysis]] - BDD/DDD/GPDD closed-form transfer functions

## Key concepts — VLSI Graph Methods

- [[vlsi-graph-methods]] - circuit Laplacian = MNA conductance matrix; graph-based EDA
- [[power-grid-analysis]] - effective resistance, voltage regulators, IR drop
- [[treewidth-and-graph-structure]] - treewidth enables FPT algorithms on VLSI netlists
- [[graph-algorithms]] - traversal, MST, clique, parameterized algorithms
- [[big-graph-systems]] - Pregel, PowerGraph, GraphChi for billion-edge graphs
- [[pregel-model]] - BSP vertex-centric computation model

## Key concepts — Device Physics

- [[semiconductor-physics]] - energy bands, carrier transport, device building blocks
- [[mosfet-physics]] - threshold voltage, I-V, scaling, short-channel effects, BSIM models
- [[pn-junction]] - fundamental diode building block; SPICE diode model physics

## Key concepts — Formal Methods and Optimization

- [[constraint-reasoning]] - SAT, SMT, CP, MILP, model checking for VLSI verification
- [[sat-and-cdcl]] - CDCL algorithm; parallel clause-sharing portfolios; hardware BMC
- [[smt-solving]] - DPLL(T); bitvector theories for hardware verification

## Key concepts — Systems and Tooling

- [[rust-systems-programming]] - memory-safe zero-cost concurrency for simulation engines
- [[python-data-science]] - pandas/NumPy/matplotlib/seaborn/Plotly for simulation post-processing
- [[devs-simulation]] - DEVS formalism for discrete-event behavioral simulation

## Key topics

- [[circuit-simulation]] (topic) — the full circuit simulation discipline
- [[graph-algorithms]] (topic) — algorithms for VLSI graph problems
- [[semiconductor-physics]] (topic) — device physics for EDA
- [[constraint-reasoning]] (topic) — formal methods
- [[data-analysis-tooling]] (topic) — Python ecosystem for EDA result analysis
- [[scalable-distributed-systems]] (topic) — infrastructure for large-scale simulation

## Open questions

- How to implement a next-generation Rust circuit simulator using RADAU5 instead of BDF2?
- Can GPU-accelerated symbolic Monte Carlo (from [[advanced-symbolic-analysis-vlsi]]) replace traditional SPICE MC at scale?
- How does distributed MNA solve (domain decomposition from [[graphs-in-vlsi]]) compare to Pregel-style iterative solvers for resistive networks?
- What is the right data pipeline: Kafka → Flink → Parquet for real-time simulation yield tracking?
- Can SAT/SMT formal verification and SPICE simulation be co-scheduled to achieve full mixed-signal correctness?
