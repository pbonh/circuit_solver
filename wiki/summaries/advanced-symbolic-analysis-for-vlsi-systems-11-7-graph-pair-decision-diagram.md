---
title: 'Advanced Symbolic Analysis for VLSI Systems — Chapter 7: Graph-Pair Decision
  Diagram'
type: source
id: source-advanced-symbolic-analysis-for-vlsi-systems-11-7-graph-pair-decision-diagram
kind: derived-summary
tags:
- gpdd
- bdd
- graph
- symbolic
- analog
- advanced
- cancellation-free
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/11-7-graph-pair-decision-diagram.txt
---

## Key Points

- Graph-Pair Decision Diagram (GPDD) is the BDD-based implicit-enumeration implementation of the generalized two-graph method developed in Chap. 6.
- Initial graph-pair rules: VC edges go in V-graph only, CS edges in I-graph only; VS, CC, and passive Y/Z edges go in both; nullor edges are precollapsed; input/output is modeled as a VCVS `Vs = X * Vc`.
- Admissible tree-pair rules: nullor edges always present, CC and VS edges always present (as common or paired), VCCS edges optional paired, Y edges optional common.
- Edge-pair operations table: each symbol type (Y, E, F, G, H) maps INCLUDE/EXCLUDE decisions to a pair of (Short, Open) edge operations on I-graph and V-graph. For E/F/H with mandatory CC/VS edges, the unused role's edge is opened.
- BDD construction via modified Minty algorithm: edge collapse for inclusion, edge removal for exclusion, terminated at the 1-terminal when both graphs reduce to a single node and at the 0-terminal when either becomes disconnected.
- Sharing via hashing reduced graph-pairs (canonical comparison of edges + node labels after renumbering); the reduced pair is the hash object.
- Sign determination is a separate recursive algorithm derived from the incidence matrices of the spanning tree-pair; documented details deferred to the cited paper [201].
- Performance results: GPDD without hierarchical decomposition solved muA725 (26 BJTs) representing 3.42 x 10^18 product terms in ~6 seconds and ~100 MB. The total term count is invariant under symbol order; only the GPDD size varies.
- Cancellation-free advantage demonstrated on a two-stage RC ladder: DDD produces `adg - aef - bcg` which expands to terms that algebraically cancel down to `G1 G2 G3 + G1 G2 G4 + G1 G3 G4`; GPDD generates the latter directly, with no cancellable pairs and thus no roundoff accumulation.
- GPDD's symbols are the primitive small-signal circuit parameters (not composite MNA entries), which makes sensitivity analysis (e.g., `∂H/∂G_3`) a simple BDD edit rather than a chain-rule expansion with re-cancellations.
- Implementation knobs: symbol order, hash design (key choice), construction direction (DFS vs. BFS vs. parallel). Hierarchical extensions (Chap. 8) are needed for very large analog circuits.

## Relevant Concepts

- [[concepts/graph-pair-decision-diagram]] — central topic; constructive definition.
- [[concepts/two-graph-method]] — theoretical foundation (Chap. 6).
- [[concepts/binary-decision-diagram]] — substrate.
- [[concepts/spanning-tree-enumeration]] — combinatorial primitive.
- [[concepts/symbolic-cancellation]] — chapter explicitly demonstrates GPDD's cancellation-freedom advantage over DDD.
- [[concepts/dependent-source]] — E/F/G/H edge-operation rules tabulated.
- [[concepts/nullor]] — precollapsed before reduction.
- [[concepts/determinant-decision-diagram]] — compared against in performance and cancellation discussions.
- [[concepts/symbolic-sensitivity-analysis]] — GPDD-friendly post-processing application.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 7 — Graph-Pair Decision Diagram
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/11-7-graph-pair-decision-diagram.txt`
- Author: Guoyong Shi
