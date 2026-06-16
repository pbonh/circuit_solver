---
title: SAT and CDCL
type: concept
slug: sat-and-cdcl
created: 2026-06-16
updated: 2026-06-16
summary: Boolean Satisfiability and Conflict-Driven Clause Learning — the foundational NP-complete problem and its modern solver algorithm; basis for SMT, MaxSAT, and hardware formal verification.
tags: [sat, cdcl, dpll, formal-verification, parallel-computing, constraint-reasoning]
sources: [handbook-parallel-constraint-reasoning]
status: active
---

# SAT and CDCL

**Boolean Satisfiability (SAT)**: Given a propositional formula in CNF (conjunctive normal form — AND of ORs), determine if there exists a variable assignment satisfying all clauses. The prototypical NP-complete problem. Modern CDCL solvers handle instances with millions of variables and clauses.

## DPLL Algorithm

Davis-Putnam-Logemann-Loveland: recursive backtracking search.
1. Unit propagation (BCP): if a clause has one unassigned literal, force it
2. Pure literal elimination: assign literals that appear in only one polarity
3. Branch: choose a variable, try true then false
4. Backtrack on conflict

## CDCL (Conflict-Driven Clause Learning)

Extends DPLL with:
- **Conflict analysis**: on conflict, analyze the implication graph to identify the 1st-UIP (unique implication point); derive a conflict clause (learned clause) that prevents the same conflict from recurring
- **Non-chronological backtracking**: jump back to the decision level where the conflict clause becomes unit (not necessarily the previous level)
- **Clause learning**: add the derived conflict clause to the clause database for future propagation
- **VSIDS heuristic** (Variable State Independent Decaying Sum): bump activity of variables in recent conflicts; prioritize for branching

CDCL is the foundation of all industrial-strength SAT solvers (MiniSat, Glucose, CryptoMiniSat, Cadical, Kissat).

## Parallel SAT Strategies

**Divide-and-conquer**: split search space by fixing a subset of variables (guiding paths/cubes); distribute subproblems to workers. Work-stealing for load balance. Risk: hard subproblems concentrate on one worker.

**Portfolio**: run diversified CDCL solvers simultaneously (different heuristics, restarts, clause deletion policies). Best single solver has high variance; portfolio virtual best solver outperforms any individual.

**Clause-sharing portfolio**: portfolio solvers share high-quality learned clauses filtered by LBD (literal block distance) — shorter, more general clauses are more useful. The key mechanism distinguishing cooperative parallel SAT.

**Cube-and-conquer**: lookahead solver (MARCH-based) generates cubes (partial assignments); CDCL solvers solve cubes in parallel; solved the Boolean Pythagorean Triples problem (200TB proof, 2016).

## Circuit Verification Applications

- **Bounded Model Checking (BMC)**: unroll circuit k steps; encode reachability as SAT; check safety properties
- **Equivalence checking**: SAT between two circuit implementations (miter circuit → check if output can be 1)
- **ATPG** (Automatic Test Pattern Generation): SAT formulation of fault activation + propagation
- **Gate-level timing analysis**: constraint propagation related to CP/SAT

## Why it matters

- CDCL's learned clause mechanism makes it exponentially faster than DPLL on structured instances — industrial circuits are highly structured
- Parallel clause-sharing portfolios can achieve near-linear speedup on hardware verification (cited in handbook: orders of magnitude speedup for BMC)
- SAT is the substrate for [[smt-solving]] via DPLL(T) — adding theories extends applicability to arithmetic, bitvectors

## Related concepts and entities

- [[smt-solving]] - DPLL(T) extends CDCL with theory solvers
- [[constraint-reasoning]] - parent topic
- [[circuit-simulation]] - circuit verification uses SAT/CDCL
