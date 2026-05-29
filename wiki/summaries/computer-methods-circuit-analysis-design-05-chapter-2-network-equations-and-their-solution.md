---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 2: Network Equations
  and Their Solution'
type: source
id: source-computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution
kind: derived-summary
tags:
- foundational
- analog
- dc
- ac
- sparse-matrix
- netlist
- graph
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt
---

## Key Points

- Combines KCL and KVL with element constitutive equations to form linear network equations for linear time-invariant R, L, C circuits in the frequency domain (set s = j2 pi f after assigning unit value to the source).
- Nodal admittance formulation YV = J is preferred for computer applications. The indefinite nodal admittance matrix has row and column sums equal to zero (linearly dependent); grounding any node deletes its row and column, yielding the definite nodal admittance matrix.
- Inspection rules for Y: diagonal entry = sum of admittances at the node; off-diagonal = -(sum of admittances between the two nodes); RHS entry = currents from independent sources entering the node. Symbolic stamp y * (e_j - e_k)(e_j - e_k)^T allows element-by-element assembly during a single scan of the netlist.
- VCTs (voltage-to-current transducers) stamp into Y as g*(e_k - e_k')(e_j - e_j')^T, generally producing structurally symmetric but numerically asymmetric matrices. Hybrid-π models for FETs and BJTs are stamped directly into Y.
- Mesh impedance formulation Z I = E is the dual: works only for planar networks and is rarely used in CAD due to complicated planarity-testing and mesh-finding algorithms. Uses KVL; diagonal = sum of impedances around the mesh.
- Gaussian elimination (~n³/3 operations) reduces A x = b to upper triangular. Cramer's rule and explicit matrix inversion are inappropriate for production code.
- LU decomposition (triangular factorization) is preferred: A = LU, with forward substitution Lz = b (≈n²/2 ops) and back substitution Ux = z (≈n²/2 ops). The Crout and row-decomposition variants of the same algorithm have a tighter innermost loop than the Gaussian variant (LUG, CROUT, LUROW FORTRAN routines provided as Fig. 2.5.1).
- Advantages of LU over Gaussian elimination: (1) easy multiple-RHS solves; (2) transpose system A^T x = e (needed for sensitivities, Ch. 6); (3) determinant = product of L diagonals; (4) in-place overwriting of A.
- For symmetric matrices (passive circuits without controlled sources), factorization can be done as U^T D U, nearly halving cost and storage.
- Pivoting: partial pivoting (largest |a_ij| in remaining column) protects accuracy; full pivoting searches whole submatrix and also permutes variables. In sparse matrices, pivot selection (reordering) is dominated by sparsity preservation rather than numerical considerations.
- Sparsity exploitation: only nonzero entries are stored and operated on. Nodal matrix has at most n + 2b nonzeros where n = nodes, b = branches. Pivot selection minimizes fill-ins, with operation count growing approximately linearly with n instead of n³/3.
- Reordering strategies, ranked best-to-worst by reordering quality: minimum local fill-in (best, expensive), minimum degree (close, much cheaper), fewest-nonzeros static ordering, no reordering. Phase-splitter example: dense ~1550 ops → no reorder 377 → fewest-nonzero 209 → minimum degree 147 → minimum fill 141.
- Sparse forward/back substitution: proceed by columns in forward sub, by rows in back sub; only nonzero z_i and required x_i need be touched. A symbolic substitution phase pre-determines the pattern.
- Sparse implementation structure: interface, ordering, symbolic factorization, symbolic solution, numeric factorization, numeric solution. Interpretive codes or compiled machine code amortize indexing overhead across many solves.
- Sparse data structures: (B, JB, IB) row-pointer + column-index + value scheme. For structurally symmetric matrices a single index array suffices (diagonal | upper rows | lower columns).
- Graph model of a sparse symmetric matrix: vertices = variables, edges = above-diagonal nonzeros. Elimination of a vertex creates a clique among its neighbors (fill-ins). Minimum-degree and minimum-fill-in algorithms operate on this graph. Berry's algorithm (1971) implements minimum local fill-in; Tinney-Walker (1967), Wing-Huang (1975, with fill-info updating used in Appendix E).
- Linked-list storage during reordering: each adjacency list is a chain of (vertex, next-pointer) cells, drawn from a free list; supports cheap insertions for fill-ins and deletions for eliminated vertices. Circular lists simplify chain breaking on deletion.

## Relevant Concepts

- [[concepts/kirchhoff-current-law]] — KCL: sum of currents leaving a node is zero; basis of nodal analysis.
- [[concepts/kirchhoff-voltage-law]] — KVL: sum of voltage drops around a loop is zero; basis of mesh analysis.
- [[concepts/nodal-analysis]] — YV = J with element stamping rules.
- [[concepts/nodal-admittance-matrix]] — Definite and indefinite forms; row/column sums; grounding removes a row and column.
- [[concepts/mesh-analysis]] — Dual of nodal; restricted to planar networks.
- [[concepts/hybrid-pi-model]] — Small-signal BJT and FET equivalent stamped directly into Y.
- [[concepts/gaussian-elimination]] — n³/3 dense factor-and-eliminate algorithm.
- [[concepts/lu-decomposition]] — A = LU, the workhorse of network equation solving.
- [[concepts/crout-algorithm]] — A specific LU variant with tighter inner loop; used in CROUT subroutine.
- [[concepts/forward-back-substitution]] — n²/2 + n²/2 cost; only relevant nonzeros touched in sparse case.
- [[concepts/pivoting]] — Partial and full strategies for numerical accuracy.
- [[concepts/sparse-matrix-methods]] — Operation count grows linearly with size in well-ordered circuit matrices.
- [[concepts/reordering]] — Permutation of rows/columns to minimize fill-in.
- [[concepts/minimum-degree-ordering]] — Pivot the vertex of smallest degree at each step.
- [[concepts/minimum-fill-in]] — Pivot the vertex that creates fewest fill-ins.
- [[concepts/fill-in]] — Zero entries that become nonzero during elimination.
- [[concepts/symbolic-factorization]] — Pre-compute the nonzero pattern of L, U for repeated solves with same structure.
- [[concepts/elimination-graph]] — Graph model used to track fill-ins during ordering.
- [[concepts/cramers-rule]] — Determinant-based formula; computationally inefficient.

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 2 — Network Equations and Their Solution
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt`
- Authors: Jiri Vlach, Kishore Singhal
