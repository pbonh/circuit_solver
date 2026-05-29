---
title: 'Advanced Symbolic Analysis for VLSI Systems — Chapter 9: Symbolic Nodal Analysis
  Using Nullors'
type: source
id: summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors
kind: publication
tags:
- analog
- symbolic
- nullor
- mosfet
- sensitivity
- noise
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors.txt
---

## Key Points

- Modeling active devices by nullor (and mirror-pathological) equivalents converts the formulation from MNA to pure NA `Y_n v_n = i_n`, reducing matrix rank significantly compared to traditional MNA.
- Nullator/norator connection rules for NA formulation: grounded nullator -> drop column; floating nullator -> merge columns; grounded norator -> drop row; floating norator -> merge rows. Result: matrix dimension equals (number of nodes) - (number of nullators) - (number of grounds).
- MOSFET small-signal nullor model: a nullator at the gate forces `i_DS = gm * v_GS` through the norator at drain-source — i.e. the MOSFET is essentially a VCCS in nullor terms.
- All four controlled sources (E, F, G, H) have nullor equivalents derivable from the VCCS-with-nullor pattern; current conveyors (CCI/CCII/CCIII) and CFOA (AD844) are built by combining nullor and pathological mirror elements; compacted models reduce element count.
- ROW/COL set algorithm for NA formulation: ROW = nodes ordered by applying norator (current) property; COL = nodes ordered by applying nullator (voltage) property; admittance tables A (grounded) and B (floating) populate `Y` via index lookup.
- Worked example: non-inverting CMOS low-voltage amplifier reduces to a 3x3 system whose solution gives `v_out/v_in = gm1 * A_i / (g_o2 + g_o4)`, where `A_i = gm4/gm3` is the current-mirror gain.
- For cascode current mirrors, output resistance `r_out = gm4 / (g_o4 g_o2)` falls out symbolically from a 4x4 system.
- The Miller-amplifier example combines NA with nullor+pathological-mirror modeling to derive symbolic transfer functions matching HSPICE results.
- Symbolic NA enables sensitivity analysis: differentiating the closed-form transfer function with respect to a device parameter (e.g., a transconductance, a width) yields a ranking of dominant contributors.
- Symbolic noise analysis of CMOS amplifiers uses the same nullor framework: each MOSFET contributes a thermal and 1/f noise current source; superposition gives the total output noise PSD. NLEV 0, 1, 2 noise models are demonstrated to agree with HSPICE numerically on common-source, differential-pair, and three-stage uncompensated amplifier examples.
- The reduced NA matrix is then fed to DDD, GPDD, or GBST symbolic engines for compact term generation.

## Relevant Concepts

- [[concepts/nullor]] — central modeling element of the chapter.
- [[concepts/pathological-element]] — VM/CM compaction of active blocks.
- [[concepts/nodal-admittance-matrix]] — pure NA formulation enabled by nullor modeling.
- [[concepts/modified-nodal-analysis]] — baseline that nullor-NA replaces.
- [[concepts/dependent-source]] — E/F/G/H all expressible via nullor.
- [[concepts/mosfet-small-signal-model]] — nullor representation.
- [[concepts/symbolic-sensitivity-analysis]] — direct application of symbolic NA.
- [[concepts/symbolic-noise-analysis]] — direct application of symbolic NA.
- [[concepts/determinant-decision-diagram]] — downstream solver for the reduced NA.
- [[concepts/graph-pair-decision-diagram]] — alternative downstream solver.
- [[entities/hspice]] — numerical reference for noise/AC validation.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 9 — Symbolic Nodal Analysis of Analog Circuits Using Nullors
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors.txt`
- Author: Esteban Tlelo-Cuautle
