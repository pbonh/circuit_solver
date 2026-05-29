---
title: 'Graphs in VLSI — Chapter 11: QuCTS — single flux Quantum Clock Tree Synthesis'
type: source
id: summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis
kind: publication
tags:
- vlsi
- superconductive
- clock
- synchronization
- novel
- sfq
- tool
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt
---

## Key Points

- Rapid Single Flux Quantum (RSFQ) is a superconductive digital technology with orders-of-magnitude higher frequency and lower power than CMOS, but requires cryogenic operation (≈ 4 K) and uses quantized SFQ pulses rather than DC levels. Most logic gates (AND, OR) are sequential in RSFQ, and most gates have fanout 1, requiring splitters for fanout > 1.
- QuCTS is the first clock tree synthesis tool for RSFQ that exploits useful clock skew. It operates in four stages: (1) build the timing graph; (2) determine minimum clock period; (3) generate clock skew schedule via quadratic programming; (4) synthesize binary clock tree via clustering and route with delay equilibration.
- Timing graph construction adds a dummy I/O node connecting all input and output edges; this forces zero clock skew between circuit inputs and outputs and lets the floating signal nets fit into a graph.
- Minimum clock period combines two constraints: (i) per-cycle T_i = (1/n) Σ (D_max + δ_s) for cycle p_ii of n nodes; (ii) per-edge T_min = max_{(i,j)} (D_max - d_min + δ_s + δ_h). Reconvergent paths can in principle benefit from delay insertion, but enumerating all simple paths is impractical, so QuCTS uses the per-edge bound.
- Clock skew scheduling QP: minimize ||s - s*||^2 subject to s_min ≤ s ≤ s_max and B s = 0 where B is the circuit connectivity matrix encoding the cycle basis. Solved in O(|V|^3). s* is the center of each permissible range. Once solved, arrival times τ at each register are propagated from an arbitrary reference using τ_p = s_px + τ_x.
- Binary clock tree generation uses hierarchical clustering (K-Means or BIRCH) over points (x, y, w·τ); the weight w controls how strongly clock arrival time dominates over spatial proximity. The resulting tree has N − 1 splitters for N sinks (each splitter has fanout 2).
- Delay equilibration places splitters and JTL delay elements between sibling gate pairs (processed in reverse BFS order from leaves). Coarse routing: a proxy graph contains the two gates plus candidate cell locations within a corridor along the line between the gates, fully connected by Manhattan-distance edges. The k-shortest-path algorithm finds candidate proxy paths.
- Proxy-path analysis: splitter is placed at gate cell g_k along the path; the mismatch ε(g_k) = τ_A - τ_B - W_A,k + W_B,k - S_A,k + S_B,k is minimized by varying splitter position and delay-element selections from a discrete set D. The number of combinations is reduced from O(n choose) to a manageable count by restricting k ≤ 2 (splitter close to the later-arriving gate).
- Fine routing on a Hanan grid produces an exact PTL layout; the novel "aura snaking" wire-snaking technique increases wire length by 2d at each iteration, with a final iteration choosing d* = v|t_A - t_B|/2 to precisely match required arrival time.
- Case study (AMD2901, 1049 clocked gates, 225 mm² die): clock skew schedule generated in <1 minute for a 154 ps clock period; clock network layout generated in 52.5 minutes with 2290 placed cells (1049 splitters, 1241 delay elements), 1027 mm total wirelength, 5.134 mm² area, max arrival-time error 1.6 ps. Larger benchmarks (ITC'99 B18 with 45,710 clocked gates) complete in 2309 minutes.

## Relevant Concepts

- [[entities/qucts]] — the tool described in this chapter.
- [[concepts/single-flux-quantum]] — underlying superconductive technology.
- [[concepts/rsfq]] — rapid single flux quantum logic family.
- [[concepts/clock-skew-scheduling]] — first stage of QuCTS.
- [[concepts/clock-tree-synthesis]] — broader context.
- [[concepts/timing-graph]] — initial representation of the sequential circuit.
- [[concepts/permissible-range]] — feasible skew interval per data path.
- [[concepts/josephson-junction]] — primitive device in RSFQ.
- [[concepts/passive-transmission-line]] — RSFQ interconnect requiring matched impedance.
- [[concepts/josephson-transmission-line]] — active RSFQ interconnect / delay element.
- [[concepts/proxy-graph]] — splitter/delay placement candidate graph.
- [[concepts/aura-snaking]] — novel wire-snaking technique introduced here.
- [[concepts/hanan-grid]] — used for fine routing.
- [[concepts/k-shortest-path-algorithm]] — enumerates candidate proxy paths.
- [[concepts/deferred-merge-embedding]] — referenced CMOS baseline; QuCTS generalizes with useful skew.

## Source Metadata

- Source type: book chapter
- Book title: Graphs in VLSI
- Chapter: 11 — QuCTS — single flux Quantum Clock Tree Synthesis
- File path: `raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
