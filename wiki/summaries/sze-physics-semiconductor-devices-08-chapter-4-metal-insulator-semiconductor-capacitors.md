---
title: 'Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 4: Metal-Insulator-Semiconductor
  Capacitors'
type: source
id: source-sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors
kind: derived-summary
tags:
- semiconductor
- device-physics
- mosfet
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/08-chapter-4-metal-insulator-semiconductor-capacitors.txt
---

## Key Points

- The MIS capacitor is the workhorse vehicle for studying the semiconductor surface and serves directly as the gate stack of every MOSFET; analysis of its ideal C-V behavior underlies MOSFET threshold-voltage theory.
- The semiconductor surface passes through four regimes as a function of gate voltage V_G: accumulation (majority carriers pile up at the surface), flatband (V_G = V_FB), depletion (the surface is depleted of majority carriers), and inversion (a minority-carrier inversion layer forms once the band bending exceeds 2 phi_B).
- Surface space-charge analysis: solving Poisson's equation gives the surface potential psi_s, depletion width W = sqrt(2 eps_s psi_s / (q N_a)), and total surface charge Q_s(psi_s) which includes contributions from accumulated, depleted, and inverted carrier populations.
- Threshold voltage Vt = V_FB + 2 phi_B + sqrt(2 eps_s q N_a (2 phi_B)) / C_ox is the gate voltage at which strong inversion occurs and the MOSFET channel forms.
- Ideal C-V curves: in accumulation, C = C_ox; in depletion, C drops as the depletion capacitance C_D adds in series; in inversion, low-frequency C returns to C_ox (minority carriers can follow), high-frequency C saturates at the minimum depletion-width value.
- Deep depletion (rapid voltage ramp) prevents minority-carrier generation from keeping up, useful for pulsed MOS C-V characterization.
- Interface traps (D_it, units cm^-2 eV^-1) at the Si-SiO2 interface broaden and distort the C-V curve and degrade MOSFET subthreshold slope and mobility. Measurement methods: high-low-frequency C-V, conductance technique, and charge pumping yield D_it(E).
- Oxide charges classified: fixed oxide charge Q_f (near the interface, positive in Si-SiO2), mobile ions Q_m (Na+, K+ historically important reliability hazards), oxide-trapped charge Q_ot (radiation-induced), and interface-trapped charge Q_it; together they shift V_FB.
- Work-function difference phi_MS between gate and substrate is part of V_FB; doped poly-Si gates can have phi_MS tailored to either n+ or p+ doping for nMOS / pMOS.
- Carrier transport through thin oxides: Fowler-Nordheim tunneling at high field, direct tunneling at thin oxides; both set the gate-leakage and breakdown behavior of MOS capacitors.
- Nonequilibrium and avalanche regimes near breakdown produce hot-carrier injection into the oxide.
- Accumulation- and inversion-layer thickness: quantum-mechanical confinement of carriers in the narrow surface well raises the centroid above the geometric interface (~1-3 nm), adding an effective oxide thickness penalty that is significant for ultrathin gate dielectrics.
- Dielectric breakdown: time-dependent dielectric breakdown (TDDB) governed by stress-induced leakage current, percolation models, and the Weibull statistics of oxide failure; sets scaling limits for SiO2 (~1 nm equivalent oxide thickness) and motivates high-k replacements.

## Relevant Concepts

- [[concepts/mis-capacitor]] — the chapter subject.
- [[concepts/mosfet]] — directly built on MIS theory.
- [[concepts/threshold-voltage]] — derived from MIS analysis.
- [[concepts/inversion-layer]] — minority-carrier sheet that forms at strong inversion.
- [[concepts/flatband-voltage]] — reference point in C-V analysis.
- [[concepts/interface-traps]] — defects at Si-SiO2 interface.
- [[concepts/oxide-charge]] — fixed, mobile, trapped charges in the dielectric.
- [[concepts/fowler-nordheim-tunneling]] — high-field oxide conduction.
- [[concepts/dielectric-breakdown]] — TDDB and oxide reliability.
- [[concepts/poisson-equation]] — solves the electrostatics.
- [[entities/silicon-dioxide]] — the canonical MIS dielectric.

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 4 — Metal-Insulator-Semiconductor Capacitors
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/08-chapter-4-metal-insulator-semiconductor-capacitors.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
