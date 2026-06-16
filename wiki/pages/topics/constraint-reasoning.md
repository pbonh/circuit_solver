---
title: Constraint Reasoning
type: topic
slug: constraint-reasoning
created: 2026-06-16
updated: 2026-06-16
summary: Formal methods for combinatorial problem solving — SAT, SMT, constraint programming, MILP, model checking — relevant to circuit verification, VLSI placement/routing, and design optimization.
tags: [sat, smt, constraint-programming, milp, formal-verification, cdcl]
sources: [handbook-parallel-constraint-reasoning, combinatorial-search]
status: active
---

# Constraint Reasoning

The discipline of solving combinatorial problems via formal logical and mathematical constraints. Spans propositional logic (SAT, MaxSAT), first-order theories (SMT), constraint programming (CP), mixed-integer programming (MILP), and model checking (BDD, LTL). All benefit from parallelism through portfolio algorithms and divide-and-conquer.

## Overview

- **SAT**: Boolean satisfiability — the core NP-complete problem; modern CDCL solvers can handle millions of variables
- **MaxSAT**: optimization variant; maximize satisfied clauses — useful for cost-bounded circuit satisfiability
- **SMT**: SAT + theory solvers (linear arithmetic, bitvectors, arrays) — the right tool for hardware verification
- **CP**: filtering/propagation + search — expressive constraints; useful for placement, scheduling
- **MILP**: continuous LP relaxation + branch-and-bound; strong for circuit-level optimization with linear objectives
- **Model checking**: LTL/CTL over finite-state systems; BDD-based (symbolic) or SAT-based (BMC)

## Connection to Circuit Simulation / VLSI

- **Formal verification**: SAT/SMT-based bounded model checking (BMC) verifies digital circuit correctness; parallel SAT makes this practical for large circuits
- **VLSI placement/routing**: CP and MILP formulations for legalization, timing-driven placement
- **Circuit optimization**: MaxSAT for satisfying soft timing constraints; MILP for wire sizing
- **BDD**: compact representation of Boolean functions — basis of symbolic simulation

## Entities and concepts in this topic

- [[sat-and-cdcl]] - core SAT algorithm and parallel extensions
- [[smt-solving]] - SAT + theories; hardware/software verification
- [[graph-algorithms]] - many VLSI problems reduce to graph optimization, which in turn may reduce to SAT/CP
- [[handbook-parallel-constraint-reasoning]] - 17-chapter parallel constraint reasoning handbook
- [[combinatorial-search]] - algorithms to systems: parallel tree/local search for SAT and CSP

## Open threads

- Integration of SMT solvers with circuit simulation for mixed analog-digital formal analysis
- SAT-based equivalence checking between behavioral and gate-level circuit descriptions
- ML-guided portfolio selection for VLSI-specific constraint instances
