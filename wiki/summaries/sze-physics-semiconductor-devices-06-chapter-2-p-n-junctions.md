---
title: "Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 2: p-n Junctions"
type: summary
tags: [semiconductor, device-physics, p-n-junction, diode, foundational, well-established, analog]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt"]
confidence: high
---

## Key Points

- The p-n junction is the foundational two-terminal semiconductor device; the chapter develops its electrostatics, static I-V, breakdown, transient behavior, noise, and terminal-function variants, and closes with heterojunctions.
- Depletion region: for an abrupt junction, the depletion approximation (n = p = 0 inside the depleted region) plus Poisson's equation gives a parabolic potential and triangular field profile; the built-in potential V_bi = (kT/q) ln(N_A N_D / n_i^2) for nondegenerate doping; depletion width W ~ sqrt(2 eps_s (V_bi - V - 2kT/q) / (q N)) for a one-sided junction.
- A two-tail correction term (-2kT/q) accounts for majority-carrier tails at the depletion-region boundaries.
- Depletion-layer capacitance C_D = eps_s / W; the 1/C_D^2 vs V plot is the standard tool for extracting V_bi and the substrate doping N.
- Linearly graded junctions and arbitrary doping profiles generalize the depletion analysis; the Debye length L_D = sqrt(eps_s kT / q^2 N) sets the spatial resolution of C-V doping profiling.
- Ideal Shockley I-V: J = J_s [exp(qV/kT) - 1] with J_s = q D_p p_no / L_p + q D_n n_po / L_n; minority-carrier diffusion across quasi-neutral regions controls forward current.
- Deviations from the ideal: (1) generation-recombination current in the depletion region (Sah-Noyce-Shockley) yields ideality factor n -> 2; (2) high-injection roll-off; (3) series resistance bends the high-current branch.
- Diffusion capacitance C_d = q^2 N L^2 / (kT) (per unit area) arises from minority-carrier storage and dominates forward-biased dynamic response.
- Junction breakdown mechanisms: thermal instability (temperature runaway), tunneling (Zener, for narrow, heavily doped junctions, with V_BR < ~6 Eg/q), and avalanche multiplication (impact ionization, for lighter doping); breakdown voltage increases with bandgap and decreases with doping.
- Transient behavior: reverse-recovery time t_rr is set by minority-carrier storage and lifetime; switching speed-vs-voltage trade-off motivates lifetime control with gold or platinum doping or irradiation.
- Noise: 1/f noise, generation-recombination noise, shot noise, thermal (Johnson) noise contribute according to the dominant transport mechanism and bias.
- Terminal-function variants: rectifier, Zener diode (voltage reference at controlled tunneling/avalanche breakdown), varistor (nonlinear protection), varactor (voltage-controlled capacitance for tuning and parametric amplifiers), fast-recovery diode, step-recovery (charge-storage) diode, p-i-n diode (high-voltage and microwave switch using a wide intrinsic region).
- Heterojunctions: anisotype (p-n with different bandgaps) and isotype (n-n or p-p with bandgap discontinuity); Anderson model uses electron affinity to predict band offsets; band-edge spikes/notches can dominate I-V; heterojunctions are essential building blocks for HBTs, lasers, photovoltaics, and HEMTs.

## Relevant Concepts

- [[concepts/p-n-junction]] — the main subject.
- [[concepts/depletion-region]] — central electrostatic structure.
- [[concepts/built-in-potential]] — equilibrium band-bending across the junction.
- [[concepts/junction-capacitance]] — depletion + diffusion contributions.
- [[concepts/shockley-diode-equation]] — ideal forward I-V.
- [[concepts/avalanche-breakdown]] — impact-ionization driven reverse breakdown.
- [[concepts/zener-breakdown]] — tunneling-driven reverse breakdown.
- [[concepts/reverse-recovery]] — minority-carrier storage transient.
- [[concepts/varactor-diode]] — voltage-controlled capacitance application.
- [[concepts/p-i-n-diode]] — wide-intrinsic high-voltage / microwave diode.
- [[concepts/shockley-read-hall-recombination]] — depletion-region recombination current.
- [[concepts/impact-ionization]] — avalanche mechanism.
- [[concepts/heterojunction]] — closing section of the chapter.
- [[concepts/poisson-equation]] — drives the electrostatics.
- [[concepts/drift-diffusion-equation]] — drives the current calculation.

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 2 — p-n Junctions
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
