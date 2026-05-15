---
title: "Advanced Symbolic Analysis for VLSI Systems — Chapter 2: Symbolic Analysis Techniques in a Nutshell"
type: summary
tags: [foundational, symbolic, analog, vlsi, history, overview]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/05-2-symbolic-analysis-techniques-in-a-nutshell.txt"]
confidence: high
---

## Key Points

- Symbolic analysis solves `A x = b` in symbolic form, with Cramer's rule reducing the task to symbolic computation of `det(A)` and its cofactors; categorized as fully symbolic, partial/mixed, or algebraic (only `s` as symbol).
- Six historical formulation schemes (1950s–60s): nodal, state-variable, hybrid, tableau, signal-flow, port methods; SPICE adopted nodal/MNA and dominated since the early 1970s.
- Five traditional categories of symbolic methods: tree enumeration, signal-flow graph (SFG), parameter extraction, interpolation, matrix-determinant methods.
- Modern strategies fall into two camps for the circuit-size problem: hierarchical decompositions and approximations (SBG: before generation, SDG: during, SAG: after).
- Notable historical symbolic analyzers: ASAP, ISAAC, SCAPP, SYNAP, RAINIER; modern graph-based: SCAD3.
- DDD (Determinant Decision Diagram) is introduced as the first BDD-based approach to symbolic MNA; later improved with logic-operation construction and hierarchical extensions; Y-Delta transformation enables symbolic MOR.
- GPDD (Graph-Pair Decision Diagram) is the second BDD-based family, reformulating the classical two-graph spanning-tree-pair enumeration; key advantage is cancellation-freedom, and its working symbols are device parameters directly (more useful for synthesis/sizing than DDD's matrix-entry symbols).
- Nullor-based NA reduces matrix dimension by collapsing nullator/norator pairs; for an active RC filter example, MNA gives order 15 while NA-with-nullors gives order 6.
- Symbolic analysis has been applied to noise, distortion (weak nonlinearity), sensitivity, fault diagnosis, design centering, reliability, circuit synthesis, layout-level optimization, and interconnect compact modeling.
- Model order reduction (Krylov/PRIMA/SPRIM/AWE, balanced truncation/TBR/PMTBR, SBPOR/SOGA, varPMTBR) is presented as a special case of symbolic analysis when `s` (and sometimes process variables) is treated as the symbol; variational MOR is an active research direction.
- Mathematical preliminaries: full vs. sparse matrices, determinant expansions by permutations and by Laplace cofactor expansion (along an entry, row, or column), Cramer's rule.

## Relevant Concepts

- [[concepts/symbolic-analysis]] — chapter is a historical and methodological overview of the field.
- [[concepts/modified-nodal-analysis]] — dominant matrix formulation underlying DDD-based symbolic work.
- [[concepts/determinant-decision-diagram]] — first BDD-based symbolic engine, introduced here.
- [[concepts/graph-pair-decision-diagram]] — second BDD-based engine; cancellation-free.
- [[concepts/two-graph-method]] — classical spanning-tree-pair enumeration GPDD generalizes.
- [[concepts/nullor]] — pathological element used to compress MNA to NA.
- [[concepts/cramers-rule]] — basis for expressing unknowns as ratios of determinants.
- [[concepts/symbolic-approximation]] — SBG/SDG/SAG taxonomy.
- [[concepts/model-order-reduction]] — Krylov/PRIMA/balanced-truncation methods discussed.
- [[concepts/krylov-subspace-mor]] — moment-matching family of MOR.
- [[concepts/balanced-truncation]] — Gramian-based MOR family.
- [[concepts/variational-mor]] — MOR under process variation; varPMTBR, multi-dim moment matching.
- [[concepts/binary-decision-diagram]] — substrate of DDD and GPDD.
- [[entities/scad3]] — modern graph-based symbolic analyzer.
- [[entities/isaac]] — classical symbolic analyzer.
- [[entities/spice]] — the numerical baseline tool.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 2 — Symbolic Analysis Techniques in a Nutshell
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/05-2-symbolic-analysis-techniques-in-a-nutshell.txt`
- Authors: Guoyong Shi, Sheldon X.-D. Tan, Esteban Tlelo-Cuautle (jointly authored)
