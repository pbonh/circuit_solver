---
title: "Computer Methods for Circuit Analysis and Design — Appendix E: Sparse Matrix Solver"
type: summary
tags: [sparse-matrix, software, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/25-appendix-e-sparse-matrix-solver.txt"]
confidence: high
---

## Key Points

- FORTRAN sparse-matrix solver for structurally-symmetric matrices that do not require numerical pivoting. Designed specifically for circuit-analysis problems.
- Three interface routines provide the API:
  - INTF1(IJA, NZA, N, W, IW, WMAX, JUMAX, IER): symbolic factorization — called once per matrix structure. Determines the row/column reordering and the structure of the upper-triangle factor.
  - INTF2(A, W, DUL, IER): numeric factorization — called each time the matrix values change.
  - INTF3(DUL, B, W, X, R): solve — called each time the right-hand side changes.
- Algorithms: minimum local fill-in ordering (Section 2.10 of the book), symbolic factorization based on the Yale sparse code (Eisenstat et al.), numeric factorization, and forward/back substitution.
- Test problem: a 10x10 = 100-node 2D resistive grid. Results on IBM 4341:
  - Total CPU time: 0.4 sec.
  - 460 nonzeros in admittance matrix; 1146 in factors (3x fill-in growth — a particularly severe test).
  - 400 ops in factorization, 698 in solution.
  - Comparison: a dense solver would need 333k and 10k ops respectively.
- Time breakdown: 50% ordering, 20% symbolic factor, 20% numeric factor, 4% solution. Ordering can be sped up by ~2x in production codes.
- Workspace requirement: integer vector W of size > 8N + 2 NZA + 1.
- For very large networks (>1000 nodes), the authors recommend a minimum-degree preprocessor with clique-based logic in the ordering routine (production-grade alternative).

## Relevant Concepts

- [[concepts/sparse-matrix-package]] — Already covered.
- [[concepts/sparse-matrix-methods]]
- [[concepts/minimum-fill-in]]
- [[concepts/symbolic-factorization]]
- [[concepts/reordering]]

## Source Metadata

- Source type: book appendix (software listing and documentation)
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: Appendix E — Sparse Matrix Solver
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/25-appendix-e-sparse-matrix-solver.txt`
- Authors: Jiri Vlach, Kishore Singhal
