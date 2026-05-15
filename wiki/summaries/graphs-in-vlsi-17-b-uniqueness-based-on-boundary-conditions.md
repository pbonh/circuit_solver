---
title: "Graphs in VLSI — Appendix B: Uniqueness Based on Boundary Conditions"
type: summary
tags: [vlsi, mathematics, well-established, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/17-b-uniqueness-based-on-boundary-conditions.txt"]
confidence: high
---

## Key Points

- Establishes the uniqueness theorem for the resistive-lattice problem: given Dirichlet boundary conditions φ(x, y) = φ_b(x, y) on a set S_v plus a grounded reference node, and injected currents I(x, y), the resulting node potentials are uniquely determined.
- The proof is by contradiction: assuming two distinct solutions φ_1 and φ_2 satisfy the same KCL and boundary conditions, their difference φ_3 = φ_1 − φ_2 satisfies the no-current Laplace-like equation 4φ_3(x, y) − Σ neighbors = 0. Since φ_3 = 0 at the grounded node and the difference satisfies the harmonic equation with zero boundary, φ_3 ≡ 0, giving φ_1 = φ_2.
- Justifies the image method used in Chapters 6-7: replacing the truncation with image currents that produce the correct boundary condition gives the unique potential distribution of the truncated grid.

## Relevant Concepts

- [[concepts/method-of-images]] — relies on uniqueness for correctness.
- [[concepts/infinity-mirror-technique]] — relies on uniqueness for correctness.
- [[concepts/lattice-graph]] — the substrate of the theorem.
- [[concepts/modified-nodal-analysis]] — KCL formulation underpinning the proof.

## Source Metadata

- Source type: book appendix
- Book title: Graphs in VLSI
- Chapter: Appendix B — Uniqueness based on boundary conditions
- File path: `raw/GraphsInVLSI/_txt/17-b-uniqueness-based-on-boundary-conditions.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
