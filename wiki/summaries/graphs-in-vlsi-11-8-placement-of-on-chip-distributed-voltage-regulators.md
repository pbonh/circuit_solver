---
title: "Graphs in VLSI — Chapter 8: Placement of On-Chip Distributed Voltage Regulators"
type: summary
tags: [vlsi, power-integrity, optimization, placement, novel, regulator]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt"]
confidence: high
---

## Key Points

- Conventional VLSI power delivery uses a single off-chip PMIC/VRM whose large physical distance to the on-chip load creates parasitic-induced voltage drops and slow load regulation. Heterogeneous power delivery supplements this with on-chip point-of-load (POL) regulators that are physically closer to the load, dramatically improving power quality.
- Three classes of on-chip regulators: switching-mode power supply (SMPS, high efficiency, large area), switched-capacitor (SC, medium area, low regulation quality), and linear / low-dropout (LDO, small area, low efficiency, fast regulation). Modern flows favor LDOs near the load.
- For optimization purposes, a regulator is modeled as a constant voltage source. Despite poor absolute accuracy, this model is high-fidelity (the optimum location coincides with the SPICE-accurate optimum), which is what global optimization actually requires.
- Power network is modeled as a uniform two-layer resistive mesh derived from the dominant wire pitch and resistance per direction in each benchmark (e.g., ibmpg4 → 284 × 571 grid, k = 2.15). The Infinity Mirror Technique (Ch. 7) provides constant-time analysis of this mesh.
- Fast grid analysis: given loads L = {(x_p, I_p)} and voltage regulators S = {(x_q, V_q)}, the voltage at any node u is V^g(u) = Σ_p I_p v^g(u, x_p) + Σ_q I_q v^g(u, x_q) where v^g(u, x) = Σ over IMT image sources of Φ_k(u − x). The unknown regulator currents are determined by solving an (m × m) linear system constructed from target voltages and KCL (sum of regulator currents equals sum of load currents).
- Limited regulator current: LDOs have a maximum supply current (area- and electromigration-limited). If the algorithm computes i(s_q) > I_max(s_q), the regulator is transferred to the load set with fixed current, and the system is re-solved iteratively.
- Load clustering: the smoothness of power-grid voltages allows up to two orders of magnitude reduction in the number of loads (via spatial clustering) with negligible effect on minimum-voltage estimates. This drastically reduces per-evaluation runtime during optimization.
- Optimization formulation: minimize v_drop(S) = -min(v^g(L))|_S subject to (a) regulator positions restricted to whitespace A and (b) i(S) ≤ i_max. Since convexity is unknown, a global optimizer is required; the case studies use Discrete Particle Swarm Optimization (DPSO).
- Three case studies on IBM ibmpg1-6 benchmarks: (1) unrestricted placement with up to 50 regulators — additional regulators consistently lower voltage drop with diminishing returns; (2) restricted placement around congested whitespace blockages — ~10% larger voltage drop than unrestricted; (3) restricted regulator current capacity (1.2× evenly-distributed budget) — ~24% larger voltage drop, regulators more uniformly spread. Optimization runs complete in under 10 minutes for grids up to ibmpg6's 3630 × 3644.

## Relevant Concepts

- [[concepts/voltage-regulator-placement]] — the optimization problem this chapter solves.
- [[concepts/on-chip-voltage-regulator]] — point-of-load LDO/SMPS/SC regulators.
- [[concepts/heterogeneous-power-delivery]] — off-chip + on-chip combined regulation strategy.
- [[concepts/infinity-mirror-technique]] — analytic engine providing constant-time grid analysis.
- [[concepts/power-distribution-network]] — modeled as a uniform two-layer resistive mesh.
- [[concepts/particle-swarm-optimization]] — DPSO chosen for the global-optimization step.
- [[concepts/ir-drop-analysis]] — the metric being minimized.
- [[concepts/load-clustering]] — preprocessing to reduce load count without losing accuracy.
- [[concepts/electromigration]] — physical limit on regulator current.
- [[entities/ldo-regulator]] — primary on-chip regulator type in the case studies.

## Source Metadata

- Source type: book chapter
- Book title: Graphs in VLSI
- Chapter: 8 — Placement of on-chip distributed voltage regulators
- File path: `raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
