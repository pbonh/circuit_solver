---
title: "Advanced Symbolic Analysis for VLSI Systems — Chapter 3: Binary Decision Diagram for Symbolic Analysis"
type: summary
tags: [foundational, bdd, symbolic, graph, data-structure]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt"]
confidence: high
---

## Key Points

- A BDD encodes Shannon expansion `f = x_i f|_{x_i=1} + bar(x_i) f|_{x_i=0}` recursively until cofactors collapse to terminals 0/1; each non-terminal is the triple `(var, solid, dashed)`.
- History: Lee (1959) introduced "Binary Decision Programs"; Akers (1978) formalized BDD; Bryant (1986) added variable ordering plus canonicity, defining ROBDD (Reduced Ordered BDD); Brace et al. (1990) defined modern BDD packages and the ITE operator.
- Canonicity requires: (i) fixed variable order, (ii) removal of superfluous (don't-care) nodes whose two cofactors are equal, (iii) sharing of equal cofactors. A minimal ROBDD is unique and serves as a fingerprint of the function.
- BDD size depends critically on variable order; finding an optimal order is NP-complete.
- All logic operations can be implemented via the single ITE(F,G,H) = F G + bar(F) H operator (Brace et al.); ITE Shannon-expands recursively with terminal-case shortcuts.
- BDD extends naturally beyond Boolean algebra: Minato's Zero-suppressed BDD (ZBDD) represents subset systems; analogous multilinear-arithmetic BDDs represent SOP polynomials.
- For determinants, the Laplace expansion `det(A) = a_{i,j} (-1)^{i+j} Minor(A, a_{i,j}) + Rem(A, a_{i,j})` is treated as a binary decision on each non-zero entry — this defines DDD (Shi and Tan, 2000), the first BDD-based symbolic engine.
- For spanning-tree enumeration, Minty's 1965 In/Out edge decomposition is reformulated as edge-Collapse / edge-Removal so that intermediate subgraphs can be compared and shared; this is the foundation of GPDD (graph-pair decision diagram, ca. 2007).
- Two distinct sharing measures: triple-based (bottom-up, used by logic BDD packages) and object-based (top-down, comparing intermediate minors or subgraphs via a hash table — preferred when objects have an efficient canonical identifier).
- Zero-suppression: when a solid arrow points to terminal "0", the multiplied term vanishes — the node can be removed and incoming arrows redirected. This is essential for algebraic BDDs because solid arrows represent multiplications.
- Spanning-tree object-based sharing may not be canonical because two topologically different subgraphs can produce the same algebraic product (after node renumbering during edge collapse). A bottom-up Reduce pass compacts the result.
- Benefits of BDD for symbolic analysis: implicit enumeration suppresses combinatorial blowup; sharing speeds up post-processing (sensitivity, Monte Carlo, evaluation); approximations and sensitivity analysis become local BDD edits.

## Relevant Concepts

- [[concepts/binary-decision-diagram]] — chapter's central topic and theoretical foundations.
- [[concepts/shannon-expansion]] — base recursion for Boolean and algebraic BDDs.
- [[concepts/robdd]] — Bryant's reduced ordered BDD; canonical form.
- [[concepts/zero-suppressed-bdd]] — Minato's ZBDD; key for algebraic SOP representations.
- [[concepts/ite-operator]] — Brace et al.'s universal Boolean operator for BDD implementation.
- [[concepts/variable-ordering]] — NP-complete decision affecting BDD size; central to all BDD practice.
- [[concepts/determinant-decision-diagram]] — first BDD application to MNA determinant expansion.
- [[concepts/spanning-tree-enumeration]] — Minty's algorithm reformulated with edge collapse for sharing.
- [[concepts/graph-pair-decision-diagram]] — GPDD constructed by BDD-managed spanning-tree enumeration.
- [[concepts/cramers-rule]] — the linear-system foundation that determinant-based methods rely on.
- [[entities/bryant-bdd-paper]] — Bryant 1986 paper establishing canonicity.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 3 — Binary Decision Diagram for Symbolic Analysis
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt`
- Authors: Guoyong Shi (per Chap. 1 acknowledgments)
