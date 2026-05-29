---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 3: Graph-Theoretic
  Formulation of Network Equations'
type: source
id: source-computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations
kind: derived-summary
tags:
- foundational
- analog
- graph
- sparse-matrix
- netlist
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/06-chapter-3-graph-theoretic-formulation-of-network-equations.txt
---

## Key Points

- Every two-terminal element is replaced by an oriented edge to produce the oriented graph of the network. Passive-element orientation is arbitrary; current-source orientation follows the arrow, and voltage-source orientation runs from + to -. After the substitution, all elements are treated uniformly.
- KVL: sum of voltage drops around any closed path is zero. KCL has a generalized form: for any cut separating the network into two parts, the sum of currents in the cut edges is zero.
- The incidence matrix A is n x b (n = ungrounded nodes, b = edges) with entries +/-1 indicating which edges leave/enter each node. KCL becomes A i = 0; KVL becomes v = A^T v_n. Rank of A for a connected network is n.
- Tree and cotree: a tree of an (n+1)-node connected graph is a connected subgraph containing all nodes and no closed paths; it has n edges called twigs. The remaining b-n edges form the cotree; they are chords. A basic cut intersects exactly one twig and as many chords as needed.
- The basic cutset matrix Q has rows corresponding to cuts (one per twig) and columns corresponding to edges. With recommended edge numbering (twigs first), Q = [1 | Q_c]. The basic loopset matrix B has rows = loops (one per chord) and columns = edges; with the recommended numbering, B = [B_t | 1].
- Orthogonality: B Q^T = 0 and Q B^T = 0. Consequently B_t = -Q_c^T, so either Q or B suffices. B = [-Q_c^T | 1] and Q = [1 | -B_t^T].
- Independent variables: chord currents are independent (i = B^T i_c, where i_c is the chord-current vector). Twig voltages are independent (v = Q^T v_t).
- Source placement: voltage sources must go into the tree (twig voltages are independent); current sources must go into the cotree (chord currents are independent). Recommended numbering: voltage sources first as twigs, then passive twigs; then passive chords, then current source chords.
- Topological nodal formulation: starting from A_a I = 0 and partitioning A_a = [A | A_s] (passive | source), with constitutive Y_b V_b = J_b, one derives Y V_n = J_s with Y = A Y_b A^T and J_s = -A_s J_p. This reproduces the nodal admittance formulation of Chapter 2.
- Topological loop formulation: similarly, starting from B_a V = 0 with partitioning, one gets Z I_c = E_s with Z = B Z_b B^T and E_s = -B_E E_p. This loop formulation works for nonplanar networks (unlike the mesh method of Chapter 2), but Z tends to be dense.
- State-variable formulation: chooses a normal tree containing all voltage sources, then as many capacitors as possible, then resistors, then completes with inductors. Independent state variables: tree-capacitor voltages and chord-inductor currents. The Q-matrix-based derivation yields sM X = A X + B W; M may be singular requiring further processing to reach the normal form sX = AX + BW. State variable formulation has fallen out of favor because of the elimination effort and dense matrices; algebraic-differential algorithms have replaced it.

## Relevant Concepts

- [[concepts/oriented-graph]] — Edges with orientations representing currents and voltages.
- [[concepts/incidence-matrix]] — n x b matrix relating edges to nodes; encodes KCL and KVL.
- [[concepts/kirchhoff-current-law]] — Generalized to cuts of the network.
- [[concepts/kirchhoff-voltage-law]] — Sum of voltage drops around any closed loop is zero.
- [[concepts/tree-cotree]] — Spanning tree partitioning; twigs and chords.
- [[concepts/cutset-matrix]] — Basic cuts (one per twig); Q matrix.
- [[concepts/loopset-matrix]] — Basic loops (one per chord); B matrix.
- [[concepts/orthogonality-relations]] — B Q^T = 0; structural duality between cuts and loops.
- [[concepts/topological-nodal-formulation]] — Y V_n = J_s with Y = A Y_b A^T.
- [[concepts/topological-loop-formulation]] — Z I_c = E_s with Z = B Z_b B^T.
- [[concepts/state-variable-formulation]] — Normal tree, chord-inductor and twig-capacitor states.
- [[concepts/normal-tree]] — Tree-selection rule for state-variable formulation.

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 3 — Graph-Theoretic Formulation of Network Equations
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/06-chapter-3-graph-theoretic-formulation-of-network-equations.txt`
- Authors: Jiri Vlach, Kishore Singhal
