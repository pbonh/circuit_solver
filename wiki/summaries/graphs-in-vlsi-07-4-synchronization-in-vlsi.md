---
title: 'Graphs in VLSI — Chapter 4: Synchronization in VLSI'
type: source
id: source-graphs-in-vlsi-07-4-synchronization-in-vlsi
kind: derived-summary
tags:
- vlsi
- digital
- synchronization
- graph
- clock
- timing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt
---

## Key Points

- A synchronous VLSI circuit has four primary components: registers (flip-flops/latches), a clock generator (typically a PLL with a VCO), a clock distribution network, and combinational logic. Clock signals coordinate data flow but unequal arrival times produce clock skew s_if = t_i - t_f.
- The first integrated-circuit STA tool was PERT (US Navy, 1958, originally a project-management tool for the Polaris missile program), adopted for logic STA in 1965.
- Local timing constraints define a permissible range PR_if = [l_if, u_if] per datapath: l_if = -d_if + δ_h^f (hold/double-clocking lower bound) and u_if = T_CP - D_if - δ_s^f (setup/zero-clocking upper bound). Lower bound is independent of clock period; the upper bound shrinks as T_CP decreases.
- Global timing constraints arise from reconvergent (parallel) paths and feedback cycles. Reconvergent paths impose intersection-of-PR constraints; feedback uses skew antisymmetry s_ij = -s_ji. Cyclic data paths sum to zero skew along a cycle (Kirchhoff-voltage-law analog).
- A constraint graph captures setup edges E_u (weight T_CP - D_if - δ_s^f) and hold edges E_l (weight d_if - δ_h^f) plus zero-weight edges from a virtual node v_0. A feasible clock skew schedule corresponds to absence of negative cycles in this constraint graph; Bellman-Ford detects negative cycles, and the minimum clock period T_CP^min corresponds to a zero-weight cycle.
- Three primary objectives of clock skew scheduling are robustness, performance, and power. Robustness is improved by centering skew within each PR (quadratic programming with constraint Bs = 0 from cycle-basis matrix B). Performance ("cycle stealing") borrows idle time from fast paths and gives it to slow paths to reduce T_CP. Power-aware scheduling exploits idle time to downsize or use lower V_DD on non-critical paths.
- Delay insertion extends clock skew scheduling: adding delay to a data path can further reduce T_CP. Wave pipelining processes multiple data simultaneously within a datapath when T_CP < propagation delay; data skew (D_ij - d_ij) limits wave pipelining frequency.
- Cycle basis determination uses a spanning tree on the timing graph (ignoring edge direction): basis edges form the tree, chords each correspond to one independent cycle; the cycle connectivity matrix B encodes Bs = 0 in the QP.
- Clock tree synthesis has two stages: topological synthesis (abstract tree from arrival times and locations) and embedding (physical layout). Common topologies include H-tree (symmetric, zero skew but layout-restrictive), delay-matched trees, and mesh (low impedance, area-expensive).
- Topological synthesis algorithms include bottom-up (nearest-neighbor merging, balanced binary trees from 2^k registers) and top-down (recursive bipartitioning, method of means and medians).
- Embedding algorithms include Method of Means and Medians (recursive splitting by center of mass), Deferred Merge Embedding (DME) which builds merging segments / tilted rectangular regions on Manhattan grids producing zero skew, Elmore-delay-based extensions for buffered RC trees, Bounded Skew Tree (BST) using octilinear merging regions and a global skew bound s_max, and Useful Skew Tree (UST) that exploits per-datapath PR rather than a global bound. QuCTS (Ch. 11) generalizes UST with discrete-location constraints for SFQ circuits.

## Relevant Concepts

- [[concepts/clock-distribution-network]] — the backbone of any synchronous VLSI system.
- [[concepts/clock-skew-scheduling]] — central optimization problem of this chapter.
- [[concepts/clock-tree-synthesis]] — the two-step topological + embedding process.
- [[concepts/timing-graph]] — directed multigraph of registers and datapaths.
- [[concepts/constraint-graph]] — derived graph encoding difference constraints for clock arrival times.
- [[concepts/static-timing-analysis]] — graph-based delay verification.
- [[concepts/permissible-range]] — feasible interval [l_if, u_if] for clock skew per datapath.
- [[concepts/wave-pipelining]] — overlapping data within a single combinational path.
- [[concepts/deferred-merge-embedding]] — DME algorithm for zero-skew clock tree embedding.
- [[concepts/elmore-delay]] — first-moment delay approximation used in clock-tree synthesis.
- [[concepts/method-of-means-and-medians]] — center-of-mass recursive clock tree.
- [[concepts/h-tree]] — symmetric balanced clock tree topology.
- [[concepts/bellman-ford-algorithm]] — used to detect negative cycles in the constraint graph.
- [[concepts/laplacian-matrix]] — analogy with the cycle-skew sum-to-zero (KVL-like) property.
- [[concepts/spanning-tree]] — used to compute the cycle basis of a timing graph.
- [[concepts/directed-acyclic-graph]] — combinational-logic substructure of timing graphs.
- [[entities/qucts]] — SFQ clock tree synthesizer that builds on these methods.
- [[concepts/galvanically-asynchronous-locally-synchronous]] — GALS clocking paradigm referenced as alternative.
- [[concepts/phase-locked-loop]] — clock-generator entity used to produce the periodic clock.

## Source Metadata

- Source type: book chapter
- Book title: Graphs in VLSI
- Chapter: 4 — Synchronization in VLSI
- File path: `raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
