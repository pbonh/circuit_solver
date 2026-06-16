---
title: "Combinatorial Search: From Algorithms to Systems"
type: source
slug: combinatorial-search
created: 2026-06-16
updated: 2026-06-16
summary: Survey of combinatorial search algorithms and systems for satisfiability and constraint problems — parallel tree search, parallel local search, learning, autonomous search, and continuous optimization.
source_file: Books/CombinatorialSearch
tags: [combinatorial-search, sat, constraint-solving, local-search, parallel-computing, autonomous-search]
status: active
---

# Combinatorial Search: From Algorithms to Systems

- **Source file:** `sources/Books/CombinatorialSearch/`
- **Author / origin:** [Springer]
- **Date:** ~2013-2014

## Summary

A research monograph on combinatorial search methods for satisfiability and constraint satisfaction, with emphasis on parallel and distributed approaches. Closely related to [[handbook-parallel-constraint-reasoning]] but more focused on SAT and CSP.

### Chapter Summaries

**Distributed Constraint Networks (Ch. 2)**: Boosting distributed CSP solving. Asynchronous backtracking (ABT), consistency techniques in distributed settings, message complexity.

**Parallel Tree Search for SAT (Ch. 3)**: DPLL/CDCL parallelization. Search-space splitting (guiding paths, cube-and-conquer). Work stealing from the search tree. Clause sharing between workers. Comparison of synchronization vs. communication costs.

**Parallel Local Search for SAT (Ch. 4)**: Multiple independent restarts vs. cooperative population-based search. Information sharing between local search instances. Tabu search, simulated annealing, GSAT/WalkSAT for SAT. Restart strategies and phase transition.

**Learning Variable Dependencies (Ch. 5)**: Exploiting structure in SAT/CSP instances. Learning variable correlations from failed attempts. Probing and backbone detection (variables that are forced to a particular value in all solutions). Variable ordering heuristics based on structural learning.

**Continuous Search (Ch. 6)**: Continuous relaxations of combinatorial problems; gradient-based methods; ant colony optimization; genetic algorithms — for optimization variants of CSP and SAT.

**Autonomous Search (Ch. 7)**: Self-tuning solvers that automatically adapt parameters. Algorithm selection (portfolio solvers), parameter tuning (irace, SMAC), online algorithm control via reinforcement learning. Combines ML with combinatorial search.

### Connection to Circuit Simulation and VLSI

- SAT/CSP-based automatic test pattern generation (ATPG) for digital circuits
- Constraint-based placement and routing (CP formulations)
- Autonomous search → adapt simulation parameters automatically (timestep, tolerance) for better convergence
- Local search → metaheuristic circuit optimization (analog sizing, routing)
- Variable dependency learning → identify which circuit parameters most affect yield (sensitivity analysis)

## Key takeaways

- Clause sharing in parallel SAT is the key mechanism for cooperative search — learned clauses are the "knowledge" transferred between workers
- Autonomous search (algorithm selection + parameter tuning) can be applied to circuit simulator parameter selection
- Continuous search methods (ACO, GA) bridge combinatorial and continuous optimization — applicable to analog circuit sizing
- Phase transition in SAT (near k-SAT threshold) corresponds to hard circuit verification instances

## Pages updated from this source

- [[sat-and-cdcl]] - extended with parallel tree search and local search
- [[constraint-reasoning]] - extended with autonomous search and learning
