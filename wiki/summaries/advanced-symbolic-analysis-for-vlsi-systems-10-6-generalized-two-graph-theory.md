---
title: 'Advanced Symbolic Analysis for VLSI Systems — Chapter 6: Generalized Two-Graph
  Theory'
type: source
id: summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory
kind: publication
tags:
- graph
- symbolic
- analog
- foundational
- advanced
- two-graph
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/10-6-generalized-two-graph-theory.txt
---

## Key Points

- The classical two-graph method (Mayeda-Seshu, 1959) computes the symbolic determinant as a signed sum over common spanning-tree pairs of two graphs (V-graph and I-graph) but was originally limited to RCL-gm networks.
- Extensions to all four dependent sources (VCVS/E, CCCS/F, VCCS/G, CCVS/H) were independently re-derived by Giomi et al., Yang/Tan, and others ca. 2000-2007; this chapter presents a unified, rigorous, BDD-friendly formulation.
- Conversion strategy: every E, F, H element is converted to a VCCS with unity-resistor stamping; matrix-stamp row/column manipulation reveals the corresponding edge constraint, formalized as a two-graph rule.
- Two-graph rules per element type:
  - VCVS (E): VS edge in both graphs; must be included in every I-graph spanning tree. Pairings: VC-VS (weight E) or VS-VS (weight 1).
  - CCCS (F): CC edge in both graphs; must be included in every V-graph spanning tree. Pairings: CC-CS (weight F) or CC-CC (weight 1).
  - CCVS (H): CC edge in V-graph + VS edge in I-graph, both required. Pairings: CC-VS (weight H) or CC-CC/VS-VS pair (weight 1).
  - VCCS (G): the original rule — symmetric edge in both graphs, weight `g`.
- Nullor: NL edge in V-graph only (precollapsed), NR edge in I-graph only (precollapsed); pair contributes unity weight and must appear in every spanning two-tree.
- Pathological mirrors (VM, CM) are handled via bidirectional edges in the two-graph; merging across a VM/CM produces oppositely-signed node-set indices that flip admittance signs in NAM stamping (the "Parallel-G Connection Rule").
- Compact two-graph representation: precollapse singular edges (NL, NR, VM, CM, CC, VS) before NAM stamping or spanning-tree-pair enumeration. The result is a smaller NAM (or smaller graph for tree enumeration), saving symbolic analysis time.
- Two complete worked examples: an ICCII+/ICCII- voltage-mode filter and a DXCCII current-mode filter; both produce identical transfer functions via NAM-stamping and via tree-pair enumeration, validating the unified rule set.
- The chapter is a theoretical foundation; the next chapter (GPDD) covers the BDD-style implementation.

## Relevant Concepts

- [[concepts/two-graph-method]] — central topic; generalized to all linear element types.
- [[concepts/graph-pair-decision-diagram]] — Chap. 7 implementation built on this theory.
- [[concepts/nullor]] — handled by precollapse in V/I graphs respectively.
- [[concepts/pathological-element]] — VM, CM, and bidirectional-edge two-graph treatment.
- [[concepts/dependent-source]] — VCVS, CCCS, VCCS, CCVS treated uniformly.
- [[concepts/modified-nodal-analysis]] — base formulation for the conversion rules.
- [[concepts/nodal-admittance-matrix]] — compact NAM derivation from precollapsed two-graph.
- [[concepts/spanning-tree-enumeration]] — algorithmic primitive for two-graph term generation.
- [[concepts/symbolic-cancellation]] — two-graph is cancellation-free, in contrast to MNA-based DDD.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 6 — Generalized Two-Graph Theory
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/10-6-generalized-two-graph-theory.txt`
- Author: Guoyong Shi
