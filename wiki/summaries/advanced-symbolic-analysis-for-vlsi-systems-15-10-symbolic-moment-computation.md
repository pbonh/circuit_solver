---
title: "Advanced Symbolic Analysis for VLSI Systems — Chapter 10: Symbolic Moment Computation"
type: summary
tags: [interconnect, symbolic, moment, bdd, advanced, statistical, vlsi]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/15-10-symbolic-moment-computation.txt"]
confidence: high
---

## Key Points

- Submicron and nanometer interconnects can no longer be ignored as parasitics; their R, C, and L give rise to enormous linear RLC networks that drive the need for reduced-order modeling.
- Process variation in interconnect dimensions makes timing and crosstalk stochastic; symbolic moment computation gives an analytical handle for repeated re-evaluation under parameter samples.
- For an RLC network in state-space form `E dx/dt + A x = B u`, `y = F x`, the transfer function is `H(s) = F (E s + A)^{-1} B`, and its Taylor expansion in `s` gives moments `m_k = F (A^{-1} E)^k A^{-1} B`. State moments `mu_k = (A^{-1} E)^k A^{-1} B` are computed recursively.
- Direct symbolic matrix inversion is intractable; the chapter develops a Symbolic Moment Calculator (SMC) built as a BDD that computes all orders of moments by reusing a 0th-order structure.
- For a tree-structured RLC network, the recursive moment formula `m_{i,k} = sum_{R_alpha in P_i} R_alpha * mC_{alpha,k} - sum_{L_alpha in P_i} L_alpha * mC_{alpha,k-1}` is decomposed into R-moments and L-moments at each node. The recursion follows the tree topology.
- Capacitor moments `mC_{alpha,k} = sum_{j in T_alpha} C_j * m_{j,k-1}` are precomputed bottom-up; the C-tree mirrors the circuit tree structure (this part is not a BDD, just a tree).
- Each R or L node corresponds to a BDD-triple (multiply on solid arrow, add on dashed arrow). Connecting them up encodes the recursive moment computation in a compact graph.
- Resistive links/loops cannot be handled by tree recursion. Kron's branch tearing decomposes the network by selecting a tearing branch and replacing its current with an injected source, producing a forest of tree subcircuits whose moments can be computed independently and reassembled.
- BDD-based bookkeeping over tearing decisions enables subnetwork sharing — many tearings produce common sub-trees that are computed once.
- Mesh networks driven by multiple independent sources are handled with the same tearing-plus-BDD approach. Multi-source / multi-port networks are exactly where traditional symbolic transfer functions struggle.
- Moment sensitivity (derivative of moment with respect to parameter) is also formulable in the SMC framework, useful for statistical timing analysis under process variation.
- Construction complexity is cubic polynomial in the network size, vs. exponential for exact symbolic representations; this gives the SMC method practical scalability for large clock trees / power grids.

## Relevant Concepts

- [[concepts/symbolic-moment-computation]] — chapter's central method.
- [[concepts/binary-decision-diagram]] — substrate for the Symbolic Moment Calculator.
- [[concepts/branch-tearing]] — Kron's technique for handling mesh / resistive-loop topologies.
- [[concepts/elmore-delay]] — first-order moment interpretation.
- [[concepts/model-order-reduction]] — moment-matching ROM context.
- [[concepts/krylov-subspace-mor]] — AWE/PRIMA family that this approach complements.
- [[concepts/process-variation]] — driving application.
- [[concepts/statistical-timing-analysis]] — primary use of the computed moments.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 10 — Symbolic Moment Computation
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/15-10-symbolic-moment-computation.txt`
- Author: Guoyong Shi
