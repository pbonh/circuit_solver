---
title: SMT Solving
type: concept
slug: smt-solving
created: 2026-06-16
updated: 2026-06-16
summary: Satisfiability Modulo Theories — extends SAT with arithmetic, bitvectors, and arrays; the primary engine for hardware formal verification and software model checking.
tags: [smt, sat, formal-verification, hardware-verification, constraint-reasoning]
sources: [handbook-parallel-constraint-reasoning]
status: active
---

# SMT Solving

Satisfiability Modulo Theories (SMT) extends Boolean SAT with decision procedures for background theories — linear arithmetic, bitvector arithmetic, arrays, uninterpreted functions, strings. SMT formulas can express properties about circuits, programs, and protocols that SAT alone cannot.

## DPLL(T) Architecture

Modern SMT solvers (Z3, CVC5, Yices2, Bitwuzla) use the DPLL(T) framework:
1. **SAT core** (CDCL): handles Boolean structure; makes Boolean decisions and propagates
2. **Theory solver (T-solver)**: checks consistency of theory literals assigned by SAT core; returns conflicts (theory lemmas = clauses ruling out the inconsistency) or satisfying assignment
3. **Theory combination**: Nelson-Oppen combination for multiple disjoint theories

## Key Theories for Hardware

| Theory | Application |
|---|---|
| Bitvector (QF_BV) | Fixed-width integer arithmetic in hardware; register operations |
| Linear arithmetic (QF_LIA/LRA) | Timing constraints, resource bounds |
| Arrays | Memory, register files |
| Uninterpreted functions (QF_UF) | Abstraction of uninterpreted operations |

## Parallel SMT

- **Portfolio**: multiple SMT solvers with different heuristics; lemma sharing (theory lemmas are high-quality learned facts)
- **Centralized lemma database**: shared repository of theory lemmas; reduces redundant theory solver calls across workers
- **Search-space partitioning**: split on Boolean decisions as in parallel SAT; theory solver state must be managed per partition
- **Interpolants**: use Craig interpolants for property-directed search (IC3/PDR style model checking)

## Applications

- **Bounded model checking (BMC)**: encode circuit semantics in bitvectors; check reachability of error states within k steps
- **Equivalence checking**: bitvector SMT between RTL descriptions
- **Timing analysis**: linear arithmetic over path delays
- **Software verification**: KLEE, SAGE, S2E — SMT for program analysis

## Connection to Circuit Simulation

From [[handbook-parallel-constraint-reasoning]]: parallel SAT/SMT speeds up hardware verification by orders of magnitude. This complements [[spice-simulation]] — SAT/SMT handles discrete/formal correctness, SPICE handles continuous physical behavior. Both are needed for full mixed-signal design verification.

## Related concepts and entities

- [[sat-and-cdcl]] - the Boolean core; DPLL(T) extends CDCL
- [[constraint-reasoning]] - parent topic
- [[circuit-simulation]] - SMT complements SPICE for formal verification
