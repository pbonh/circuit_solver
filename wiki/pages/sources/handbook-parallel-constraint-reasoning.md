---
title: "Handbook of Parallel Constraint Reasoning"
type: source
slug: handbook-parallel-constraint-reasoning
created: 2026-06-16
updated: 2026-06-16
summary: Comprehensive reference on parallel algorithms for SAT, MaxSAT, QBF, SMT, constraint programming, MILP, theorem proving, ASP, BDD, and model checking — spanning single-chip to 80,000-core deployments.
source_file: Books/HandbookOfParallelConstraintReasoning
tags: [constraint-reasoning, sat, smt, parallel-computing, formal-verification, optimization, cdcl]
status: active
---

# Handbook of Parallel Constraint Reasoning

- **Source file:** `sources/Books/HandbookOfParallelConstraintReasoning/`
- **Author / origin:** Edited by Youssef Hamadi & Lakhdar Saïs; 54 co-authors; Springer, 2018
- **Date:** 2018

## Summary

A 17-chapter handbook surveying parallel approaches across the full spectrum of constraint reasoning formalisms. Covers single-chip multi-core and GPU through 80,000-core cloud deployments. Common mechanisms thread through all chapters: divide-and-conquer with work stealing, portfolio algorithms (competing solvers), knowledge sharing (clause/lemma sharing, bound propagation), and synchronization for determinism.

### Part I: Theory and Algorithms

**Parallel SAT (Ch. 1)**: Core algorithms: DPLL (recursive backtracking), CDCL (conflict-driven clause learning with non-chronological backtracking + VSIDS heuristic). Parallelization strategies:
- *Divide-and-conquer*: split search space by variable assignments (guiding path, cubes); work-stealing for dynamic load balance
- *Portfolio*: run diversified solvers in parallel; clause-sharing portfolios exchange learned clauses filtered by quality (LBD score, size)
- *Cube-and-conquer* (Ch. 2): lookahead solver creates cubes (partial assignments defining subproblems); CDCL solvers solve cubes in parallel; interleaving for dynamic load balance; solved Boolean Pythagorean Triples problem (200TB proof)

**Parallel MaxSAT (Ch. 3)**: Optimization variant — maximize satisfied clauses. Linear search (iterate cost bounds) and unsatisfiability-based (core-guided) algorithms. Parallel portfolio and search-space splitting; clause sharing restricted to hard clauses; deterministic synchronization mechanisms.

**Parallel QBF (Ch. 4)**: Quantified Boolean Formulas — PSPACE-complete. CDCL-like algorithms with universal vs. existential quantifier handling; certificate generation; search-space splitting challenges from alternating quantifiers.

**Parallel SMT (Ch. 5)**: Satisfiability Modulo Theories — SAT extended with arithmetic, bitvectors, arrays, uninterpreted functions. DPLL(T) architecture: SAT core + theory solvers (T-solver). Parallel portfolios with lemma sharing; centralized lemma databases; search-space partitioning with interpolants. Critical for hardware/software verification.

**Parallel Theorem Proving (Ch. 6)**: First-order logic; resolution and superposition calculi. Clause-diffusion paradigm: distribute clauses across workers; each worker generates inferences from local + received clauses; contraction (subsumption, simplification) must be coordinated. Multi-search strategies.

**Parallel ASP (Ch. 7)**: Answer Set Programming — stable model semantics; used for knowledge representation and planning. Parallel grounding (ASP programs must be grounded before solving); parallel CDCL-based ASP search (clasp-based); GPU-based Datalog; MapReduce for large-scale Datalog.

**Parallel MILP (Ch. 8)**: Mixed Integer Linear Programming — LP relaxation + branch-and-bound. Key dimensions: scalability (distributed vs. shared memory), knowledge sharing (cutting planes, dual bounds, feasible solutions), load balancing (work stealing from branch-and-bound tree), determinism. Frameworks: ParaSCIP, FiberSCIP (up to 80,000 cores).

**Parallel Constraint Programming (Ch. 9)**: CP = filtering (propagation to reduce variable domains) + backtrack search. Parallel search tree: static partitioning (easy, load-imbalanced) vs. dynamic partitioning (work stealing). Embarrassingly parallel when good decomposition exists. Portfolio of complementary propagation strategies.

**Parallel Stochastic Local Search (SLS, Ch. 10)**: Population-based local search (simulated annealing, WalkSAT, GSAT). Naturally parallel: independent restarts or cooperative search with solution sharing.

**Parallel Breadth-First Search**: A* (heuristic best-first search on state space); parallel model checking for linear temporal logic (LTL) via parallel BFS on Kripke structures; parallel BDD operations (applies, compose, existential quantification); parallel model-based diagnosis.

### Part II: Tools and Applications

**ML for Portfolio Composition**: Automated selection and weighting of parallel portfolios using algorithm selection + scheduling techniques (SATzilla, ISAC, ParHydra). Feature extraction from problem instances; train/predict best solver configuration.

**Application: Hardware Verification**: Parallel SAT accelerates BMC (bounded model checking) and inductive model checking by orders of magnitude — critical for industrial digital design verification. Direct connection to circuit simulation domain.

**Application: Optical Network Design**: Parallel SLS for wavelength assignment in optical networks — combinatorial optimization under wavelength-continuity constraints.

## Key takeaways

- Clause/lemma sharing in portfolio SAT/SMT is the key mechanism distinguishing "embarrassingly parallel" from genuinely cooperative parallel search
- CDCL's non-chronological backtracking + learned clause reuse underlies the best modern SAT, MaxSAT, SMT, and ASP solvers
- SMT = the right tool for formal verification of mixed arithmetic/logic properties in hardware designs
- MILP can scale to 80,000 cores via distributed branch-and-bound with cutting-plane sharing
- Hardware verification (BMC, model checking) is a direct application of parallel SAT/SMT to circuits — bridging to [[circuit-simulation]]
- BDD-based model checking provides exact verification of finite-state circuits; parallel BDD operations are a research frontier

## Pages updated from this source

- [[constraint-reasoning]] - topic created
- [[sat-and-cdcl]] - concept created
- [[smt-solving]] - concept created
- [[overview]] - formal methods connection noted
