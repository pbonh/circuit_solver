---
title: "Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 6: MOSFETs"
type: summary
tags: [semiconductor, device-physics, mosfet, digital, analog, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: high
---

## Key Points

- The MOSFET (MOS field-effect transistor) is the dominant transistor of the integrated-circuit era. The chapter covers basic characteristics, scaling, structures, circuit applications, and nonvolatile memory and ends with the single-electron transistor.
- Field-effect-transistor family tree: MOSFET, JFET, MESFET, MODFET; n-channel and p-channel, enhancement and depletion modes; bulk, partially-depleted SOI, fully-depleted SOI, double-gate, FinFET, and nanowire variants are introduced.
- Basic I-V model: gradual-channel approximation gives I_D = (mu_eff W / L) [(V_GS - Vt) V_DS - V_DS^2 / 2] in the linear regime, saturating at I_Dsat = (mu_eff W / (2L)) (V_GS - Vt)^2 once V_DS >= V_GS - Vt (channel pinch-off). The transconductance g_m = dI_D/dV_GS = mu_eff C_ox (W/L)(V_GS - Vt) in saturation.
- Threshold voltage Vt is set by the MIS capacitor analysis of Chapter 4 with the additional contributions of body effect and short-channel effects.
- Subthreshold conduction: below Vt the channel is in weak inversion and I_D varies exponentially with V_GS, with the subthreshold slope S = (kT/q) ln(10) (1 + C_D/C_ox) ~ 60 mV/decade at room temperature in the ideal limit. Reducing C_D / C_ox by thinner oxide or thinner body brings S closer to the kT-limited 60 mV/dec ideal.
- Mobility: surface mobility mu_eff is lower than bulk because of Coulomb scattering by interface charges, phonon scattering, and surface-roughness scattering; the universal mobility curve plots mu_eff vs the effective field, with mu ~ E_eff^-1/3 in the phonon-limited regime.
- Temperature dependence: mu_eff falls with T; Vt drops by ~1 mV/K; intrinsic carrier density limits the upper operating temperature.
- Nonuniform doping (high-low and low-high profiles) and buried-channel (PMOSFET with p+ poly-Si gate historically) devices modify Vt-V_BS characteristic and short-channel behavior.
- Device scaling (Dennard scaling): scaling all dimensions and voltages by the same factor 1/k preserves field magnitudes and proportional performance. Constant-field scaling has been replaced by mixed-scaling, then by gate-engineered and material-engineered scaling at modern nodes.
- Short-channel effects: charge-sharing from source/drain reduces Vt at short L; channel-length modulation (lambda parameter) gives finite output conductance in saturation; drain-induced barrier lowering (DIBL) further reduces Vt at high V_DS; impact-ionization-driven substrate current and hot-carrier injection into gate oxide degrade Vt and damage interface; punch-through occurs when the source/drain depletion regions touch.
- MOSFET structures: light/heavy channel doping, halo implants, retrograde wells; gate-stack engineering (heavily doped poly-Si, metal gates, high-k oxide-replacement with HfO2 and TiN/TaN gates); LDD (lightly doped drain) and extension regions for hot-carrier mitigation; SOI (silicon-on-insulator) for reduced parasitic capacitance, isolation, soft-error immunity; thin-film transistors (TFT) for displays; 3-D structures (FinFET, gate-all-around) for sub-30 nm nodes; power MOSFETs (vertical DMOS, trench MOSFET, LDMOS).
- Circuit applications: equivalent-circuit elements (g_m, g_ds, C_gs, C_gd) and microwave performance metrics (f_T, f_max); basic blocks: inverter, NAND/NOR logic, transmission gate, op-amp input pair.
- Nonvolatile memory: floating-gate devices (EPROM, EEPROM, flash) store charge on an isolated polysilicon gate, programmed by F-N tunneling or channel hot-electron injection and erased by F-N tunneling; charge-trapping devices (SONOS, MONOS) use a nitride trap layer rather than a floating gate.
- Single-electron transistor (SET): a small island weakly coupled by tunneling junctions exhibits Coulomb blockade when its charging energy q^2 / 2C > kT, allowing controlled transfer of single electrons; operation requires small islands and low temperature or both very small (nm-scale) and room-T operation.

## Relevant Concepts

- [[concepts/mosfet]] — chapter subject.
- [[concepts/mis-capacitor]] — gate stack.
- [[concepts/threshold-voltage]] — gate-voltage threshold.
- [[concepts/inversion-layer]] — channel charge.
- [[concepts/subthreshold-conduction]] — exponential below Vt.
- [[concepts/short-channel-effects]] — DIBL, charge sharing, punch-through.
- [[concepts/hot-carrier-effects]] — impact-ionization-driven degradation.
- [[concepts/dennard-scaling]] — historical scaling rules.
- [[concepts/finfet]] — modern 3-D MOSFET.
- [[concepts/silicon-on-insulator]] — SOI substrate technology.
- [[concepts/floating-gate-memory]] — flash, EEPROM.
- [[concepts/single-electron-transistor]] — quantum-island device.
- [[concepts/cmos-logic]] — circuit application.
- [[concepts/poly-silicon-gate]] — historical gate material.

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 6 — MOSFETs
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
