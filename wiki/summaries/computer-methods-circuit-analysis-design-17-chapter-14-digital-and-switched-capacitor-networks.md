---
title: "Computer Methods for Circuit Analysis and Design — Chapter 14: Digital and Switched-Capacitor Networks"
type: summary
tags: [digital, switched-capacitor, ac, sensitivity, sparse-matrix, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/17-chapter-14-digital-and-switched-capacitor-networks.txt"]
confidence: high
---

## Key Points

- Most analog-network computational methods extend directly to digital networks (sampled-data systems) and switched-capacitor (SC) networks. The chapter unifies the treatment.
- Discrete signals: u(n) (unit step) and delta(n) (Dirac sequence) are the discrete-time analogs of u(t) and delta(t). A general discrete signal is x(n) = sum_k x(k) delta(n-k).
- A linear shift-invariant discrete system has impulse response h(n) and output y(n) = sum_k w(k) h(n-k) — the discrete convolution. Difference equation: sum_k a_k y(n-k) = sum_k b_k w(n-k), structurally similar to LMS integration formulas (Ch. 13).
- z-transform: H(z) = sum_n h(n) z^{-n}. Causal sequences have analytic H(z) for |z| > radius of convergence. Poles of H(z) inside the unit disk indicate stability.
- For sampled sinusoid w(nT) = e^{j omega nT}, the output is y(nT) = w(nT) H(e^{j omega T}) — frequency response is H evaluated on the unit circle.
- Digital network formulation (Section 14.3): each delay/multiplier element has a stamp into a system matrix analogous to MNA. Network functions in z are obtained by the same DFT-based interpolation as in Chapter 7, but on the unit circle in z. Appendix D codes digital-network analysis.
- Switched-capacitor (SC) networks (Section 14.4): MOS technology cannot easily produce resistors but produces capacitors and switches readily. An SC resistor is a capacitor switched between two nodes at high clock rate; its equivalent conductance is C * f_clock for f_signal << f_clock. SC networks enable integration of analog filters.
- SC formulation (Section 14.5): time is divided into "clock phases" (typically two: phase 1 and phase 2). During each phase, the network is a different LTI circuit (some switches closed, others open). Charge conservation at boundaries between phases yields the SC equations. Two-graph modified-nodal formulation (Chapter 4) is particularly suited.
- Minimal-size SC formulation: separate I-graph and V-graph per phase, with edge collapse/deletion based on switch states. The resulting matrix per phase is much smaller than a one-graph formulation.
- Spectral analysis of SC networks (Sections 14.6-14.8): an SC network is periodically time-varying (period T_clock). The output spectrum contains aliases at multiples of f_clock. The transfer function H(f) is computed by solving the SC equations at each frequency f.
- SC sensitivity (Section 14.9): the adjoint method of Chapter 6 extends to SC networks. One adjoint solve per output (per clock phase) gives sensitivities to all element values. Particularly important because SC IC tolerances are dominated by capacitor matching.
- Symbolic analysis (Section 14.11): the large-change-sensitivity F-matrix technique of Chapter 8 extends to SC networks, allowing symbolic transfer functions in element values.
- Sample-hold input modeling: the actual SC input is a sample-hold waveform (constant during each clock phase) rather than a true Dirac sequence. The chapter shows how to fold this into the formulation.

## Relevant Concepts

- [[concepts/discrete-time-signal]] — Sampled-data signal representation.
- [[concepts/z-transform]] — Discrete-time analog of the Laplace transform.
- [[concepts/digital-network-analysis]] — Difference-equation system analysis.
- [[concepts/switched-capacitor-network]] — SC filters and switched-capacitor analog signal processing.
- [[concepts/clock-phase-formulation]] — Phase-by-phase analysis of SC networks.
- [[concepts/sc-spectral-analysis]] — Frequency response of periodically-switched networks.
- [[concepts/two-graph-modified-nodal]] — Foundation for SC formulation.
- [[concepts/switch-model]] — Already covered.
- [[concepts/symbolic-analysis]]
- [[concepts/large-change-sensitivity]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 14 — Digital and Switched-Capacitor Networks
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/17-chapter-14-digital-and-switched-capacitor-networks.txt`
- Authors: Jiri Vlach, Kishore Singhal
