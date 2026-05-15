---
title: "Advanced Symbolic Analysis for VLSI Systems — Chapter 8: Hierarchical Analysis Methods"
type: summary
tags: [hierarchical, ddd, gpdd, symbolic, analog, advanced, scalability]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/12-8-hierarchical-analysis-methods.txt"]
confidence: high
---

## Key Points

- Even with BDD-based encoding, flat DDD/GPDD analysis remains exponential in circuit size (with a much lower base than non-BDD methods). Hierarchical decomposition is required to scale to larger analog circuits.
- Existing hierarchical methods divide into two camps: circuit-topology-based (spanning-tree on a graph or graph pair) and matrix-based (MNA partitioning, Gaussian/Schur decomposition).
- Historical sequence: Coates flow-graph (Starzyk-Konczykowska 1986), Mason SFG (Hassoun-McCarville 1993), RMNA / Gaussian elimination (Hassoun-Lin 1995), direct symbolic Gaussian elimination (Pierzchala-Rodanski 2001), regularity-based (Doboli-Vemuri 2001), approximate hierarchical via dominant-tree weighting (Guerra et al. 2002), DDD/Schur-based (Tan et al.).
- Most pre-BDD hierarchical methods generate Sequence-of-Expressions (SOE) with divisions, suffering from (i) numerical instability from small divisors, (ii) growing expression length under post-processing (especially sensitivity), and (iii) difficulty generating s-expanded polynomials.
- SOEs can be represented as a BDD (multiply on solid arrows, add on dashed arrows) for sequential ladder circuits. For non-ladder cases, only SOP-form (DDD/GPDD) avoids divisions.
- For a 20-section RC ladder, the SOP expansion gives 1.66e8 terms; a GPDD with a good order encodes them in 120 vertices.
- Schur decomposition with DDD: partition the MNA into `[[A11, A12], [A21, A22]]`, reduce internal block A11, and form the multi-port admittance stamp `Y2 = A21 A11^{-1} A12`. Each entry of `Y2` is a symbolic expression (built via DDD), and the next-level stamp is built from these entries. This is equivalent in form to the "symbolic stamp" approach.
- Symbolic stamp framework: a multi-port subcircuit is characterized once by a port-admittance matrix whose entries are DDD or GPDD subgraphs; the stamp is shared across hierarchy levels — analogous to SPICE element stamps but with symbolic content.
- This chapter develops a new MIMO (multi-input multi-output) graph reduction rule for GPDD so multi-port modules can be processed as graph elements rather than scalar admittances.
- Two hierarchical strategies introduced:
  - GPDD+DDD — generate symbolic stamps via GPDD, assemble into an MNA, then run DDD on the assembled matrix.
  - HierGPDD — pure GPDD-on-graph-partitioning, no MNA in the loop; all levels analyzed via GPDD's graph-pair reduction.
- Both strategies preserve cancellation-freedom (when GPDD is used at every level) and scale considerably better than flat methods on large benchmark analog circuits.

## Relevant Concepts

- [[concepts/hierarchical-symbolic-analysis]] — chapter's central topic.
- [[concepts/symbolic-stamp]] — multi-port symbolic admittance block; shared across hierarchy levels.
- [[concepts/schur-decomposition]] — matrix-partitioning primitive used by DDD-based hierarchy.
- [[concepts/sequence-of-expressions]] — traditional non-BDD hierarchical output form.
- [[concepts/gaussian-elimination]] — symbolic LU; source of SOE generation.
- [[concepts/determinant-decision-diagram]]
- [[concepts/graph-pair-decision-diagram]]
- [[concepts/two-graph-method]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/symbolic-cancellation]] — chapter explains how Schur-based DDD hierarchy reintroduces cancellation while GPDD-based hierarchy does not.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 8 — Hierarchical Analysis Methods
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/12-8-hierarchical-analysis-methods.txt`
- Author: Guoyong Shi
