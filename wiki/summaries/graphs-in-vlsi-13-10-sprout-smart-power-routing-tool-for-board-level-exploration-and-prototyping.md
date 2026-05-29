---
title: 'Graphs in VLSI — Chapter 10: SPROUT — Smart Power Routing Tool for Board-Level
  Exploration and Prototyping'
type: source
id: source-graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping
kind: derived-summary
tags:
- vlsi
- power-integrity
- routing
- novel
- board
- tool
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt
---

## Key Points

- SPROUT is the first automated board-level power-network prototyping algorithm for PCBs. It takes a PCB layout description (BGA placement, design rules, blockages) and produces a routed power-network prototype suitable for impedance extraction, enabling rapid design-space exploration before committing to a final layout.
- The algorithm pipeline: (1) determine available routing space A_n for net n by subtracting buffers of other nets; (2) tile A_n and build an equivalent graph Γ_n whose nodes are tiles and whose edge weights are conductances proportional to inter-tile contact width; (3) seed the routing with shortest paths between terminal pairs (Dijkstra/A*); (4) iteratively grow and refine the subgraph with SmartGrow and SmartRefine; (5) reheat via dilation+erosion to escape local optima; (6) back-convert the subgraph to a polygon.
- The node-current metric J_p = Σ_pairs Σ_neighbors g_pj |V_i − V_j| drives both growth and refinement. It identifies "hot spots" carrying large current that benefit most from reinforcement, and "quiescent zones" whose nodes can be removed without significantly increasing impedance.
- SmartGrow adds nodes adjacent to high-current regions of the seed subgraph until the target area constraint A_max is reached. SmartRefine swaps low-current nodes for high-current additions to further reduce impedance without exceeding A_max.
- Subgraph reheating mimics simulated annealing: dilate the subgraph beyond A_max, then erode by removing nodes with smallest current. This escapes local minima.
- Multilayer routing is handled by decomposing the multi-layer problem into per-layer routing problems (Appendix C).
- Runtime is dominated by solving the Laplacian system inside the node-current metric: O((A_max / ΔA + k_r + k_e) (A_max / (Δx Δy))^q), q ∈ [1.5, 3]. Polygon clipping is O(vertices). For ≤ 10,000 vertices, clipping takes ≤ 50 seconds.
- Validation case 1 (two rails, 8-layer PCB): SPROUT inductance differs from manual layout by at most 12% (V_DD1 actually 12% lower); DC resistance within 3.1%.
- Validation case 2 (six rails, 10-layer PCB with congested BGA): All six rails routed in 11 minutes; SPROUT inductance 1-4% lower than manual; resistance within 11%.
- Case study 3 (area/impedance trade-off): For modem, CPU, DSP power rails, increasing allocated area reduces effective resistance with diminishing returns; inductance reduction depends on rail (modem/CPU benefit less due to decoupling capacitors). Beyond ~27.5 units, additional area does not reduce voltage droop further.

## Relevant Concepts

- [[entities/sprout]] — the tool described in this chapter.
- [[concepts/power-distribution-network]] — system being routed at board level.
- [[concepts/node-current-metric]] — the gradient-like quantity guiding SmartGrow / SmartRefine.
- [[concepts/smart-grow]] — area-constrained subgraph expansion procedure.
- [[concepts/smart-refine]] — node-swap local optimization for impedance reduction.
- [[concepts/subgraph-reheating]] — simulated-annealing-inspired escape from local minima.
- [[concepts/dijkstras-algorithm]] — used for seed shortest-path construction.
- [[concepts/a-star-algorithm]] — alternative seed-routing accelerator.
- [[concepts/polygon-clipping]] — used to compute available routing space.
- [[concepts/board-level-routing]] — PCB-specific routing context.
- [[concepts/interconnect-routing]] — broader routing concept SPROUT specializes.
- [[concepts/multilayer-routing]] — handled by decomposition (Appendix C).

## Source Metadata

- Source type: book chapter
- Book title: Graphs in VLSI
- Chapter: 10 — SPROUT — Smart Power Routing Tool for board-level exploration and prototyping
- File path: `raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
