---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 4: General Formulation
  Methods'
type: source
id: summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods
kind: publication
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
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt
---

## Key Points

- The tableau formulation (Hachtel-Brayton-Gustavson, 1971) collects KCL (A I = 0), KVL (V_b - A^T V_n = 0) and branch constitutive equations (Y_b V_b + Z_b I_b = W_b) into a single large matrix equation T X = W. All branch voltages, branch currents, and node voltages are unknowns. Capacitors are entered in admittance form, inductors in impedance form to keep s in the numerator.
- Tableau matrix density is low (~9-15% for example circuits), and the matrices are large but extremely sparse. Coding sparse solvers for tableau structure is complicated but yields a flexible formulation handling all ideal elements (op-amps, nullor, ideal sources, transformers, etc.).
- Block elimination on the tableau: substituting V_b = A^T V_n into the CE gives a (b+n) x (b+n) system. If every element has an admittance description and only current sources are present, further elimination yields A Y_b A^T V_n = -A J_b — the classical nodal admittance formulation YV = J of Chapter 3.
- Many useful ideal elements (voltage source, VVT, CVT, CCT, transformer, OPAMP, nullator/norator, current-controlled nonlinear, inductor in impedance form) have no admittance description; these require modified nodal analysis (MNA).
- Modified nodal analysis (Ho-Ruehli-Brennan, 1975): partition elements into those with admittance description (block 1) and those without (block 2). After substitution, the system has node voltages and additional branch currents as unknowns. The system matrix has the (Y_n A_2^T; Y_2 A_2^T Z_2) block structure stamping into rows and columns of additional unknowns.
- MNA by inspection: maintain G (frequency-independent), C (frequency-dependent), and W vectors. For each ideal element, append an extra row (its CE) and an extra column (its branch current) to the system. Fig. 4.4.1 collects the inspection stamps for all standard ideal elements.
- A perfect switch can be incorporated by inserting a switch-coefficient F into the resistor stamp; F = 0 for open, F = 1 for short. The system matrix is generated once and switching states are toggled by changing F without re-formulation.
- Active-network nodal analysis (Section 4.5): manual technique with rules — substitute known voltages from voltage sources, convert resistors to conductances, write KCL only at nodes not connected to voltage sources. For ideal OPAMPs, the two input terminals are at equal potential (virtual short) and no KCL is written at the output node. This yields 2x2 and 3x3 systems for the examples versus 6x6 and 7x7 from MNA.
- Two-graph (V-graph and I-graph) formulation eliminates redundant variables. Rules for drawing the I-graph: collapse the edge if its current is uninteresting; delete if its current is zero. Rules for the V-graph: delete if its voltage is uninteresting; collapse if its voltage is zero. KCL uses A_i I_b = 0; KVL uses V_b = A_v^T V_n. Y_b and Z_b need not be square.
- Two-graph tableau formulation: similar to single-graph tableau but with separate incidence matrices for the two graphs, eliminating variables known to be zero (e.g., VVT input current, CVT input voltage). Tableau size shrinks from 25x25 to 14x14 in the example.
- Two-graph MNA: the most compact systematic formulation. Stamp formula: an admittance y with I-graph edge from j_i to j_i' and V-graph edge from j_v to j_v' contributes +y at (j_i, j_v) and (j_i', j_v'), -y at (j_i, j_v') and (j_i', j_v). Eliminates virtually all redundancies. Fig. 4.8.2 collects the two-graph stamps.
- Graph representation in software: a netlist table of (element type, from-node, to-node) is enough to derive both graphs. Two-graph derivation involves node collapse (e.g., the voltage source collapses its two end nodes in the I-graph) and edge deletion (e.g., VVT input current is deleted from the I-graph). Renumbering is implicit when nodes are collapsed.
- Comparison of formulation sizes on a second-order active filter (Fig. 4.1.4) and a generalized impedance converter (Fig. 4.1.5):
  - One-graph tableau: 18x18 and 25x25 (densities ~12% and ~9%).
  - Two-graph tableau: 15x15 and 14x14.
  - Modified nodal: 6x6 and 7x7.
  - Two-graph MNA: 4x4 and 3x3.
  - By-hand (active-network nodal): 2x2 and 3x3.
- The methods of Sections 4.4 (MNA by inspection) and 4.8 (two-graph MNA) are coded in the Appendix D analysis program; the two-graph formulation is particularly advantageous for switched-capacitor networks (Chapter 14).
- Chapter closes with a ninth-order Cauer-parameter low-pass filter design example using FDNRs (frequency-dependent negative resistance, an active impedance-transformed circuit replacing inductors), to be optimized in Chapter 17.

## Relevant Concepts

- [[concepts/tableau-formulation]] — Full algebraic-differential system with KCL, KVL, and CE.
- [[concepts/modified-nodal-analysis]] — Compact formulation handling all ideal elements.
- [[concepts/two-graph-formulation]] — Separate I-graph and V-graph eliminate redundant variables.
- [[concepts/two-graph-modified-nodal]] — Most compact systematic formulation.
- [[concepts/branch-stamping]] — Element-by-element stamping into G, C, W matrices.
- [[concepts/active-network-analysis]] — Manual rules for active circuits with op-amps and voltage sources.
- [[concepts/incidence-matrix]] — Encodes KCL and KVL in topological formulations.
- [[concepts/operational-amplifier]] — Ideal op-amp constraints (virtual short, zero input currents) drive matrix-size reductions.
- [[concepts/fdnr]] — Frequency-dependent negative resistance, used in the active-filter example.
- [[concepts/sparse-tableau-approach]] — Hachtel-Brayton-Gustavson methodology.
- [[concepts/switch-model]] — F=0/F=1 representation enabling switch toggling without re-formulation.
- [[concepts/cauer-parameter-filter]] — Elliptic filter design used in the chapter's example.

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 4 — General Formulation Methods
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt`
- Authors: Jiri Vlach, Kishore Singhal
