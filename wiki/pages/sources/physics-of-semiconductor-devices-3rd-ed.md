---
title: "Physics of Semiconductor Devices, 3rd Edition"
type: source
slug: physics-of-semiconductor-devices-3rd-ed
created: 2026-06-16
updated: 2026-06-16
summary: Sze & Ng's comprehensive reference on semiconductor device physics — the foundational text covering p-n junctions, MOSFETs, bipolar transistors, and photonic devices.
source_file: Books/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng
tags: [semiconductor, device-physics, mosfet, pn-junction, vlsi, bipolar, photonic]
status: active
---

# Physics of Semiconductor Devices, 3rd Edition

- **Source file:** `sources/Books/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/`
- **Author / origin:** S. M. Sze (National Chiao Tung University) & Kwok K. Ng (Agere Systems/MVC)
- **Date:** 3rd ed., John Wiley & Sons, 2006. First ed. 1969; one of the most-cited works in engineering (>15,000 citations by 2006).

## Summary

The standard graduate reference for semiconductor device physics, spanning five parts: semiconductor physics, device building blocks, transistors, negative-resistance/power devices, and photonic devices/sensors. Over 50% revised from the 2nd edition (1981), adding contemporary topics: 3D MOSFETs, nonvolatile memory, MODFET, single-electron transistor, resonant-tunneling diode, IGBT, quantum cascade laser, and semiconductor sensors.

### Part I: Semiconductor Physics (Chapter 1)
Review of crystal structure, energy bands and bandgap, carrier concentration at thermal equilibrium (Fermi-Dirac statistics, intrinsic/extrinsic), carrier transport (drift, diffusion, Hall effect, high-field effects), phonon/optical/thermal properties, heterojunctions and nanostructures, and basic equations (Poisson, continuity, transport). Si and GaAs are the primary reference materials.

### Part II: Device Building Blocks (Chapters 2-4)

**p-n Junction (Ch. 2)**: Depletion approximation, built-in voltage, current-voltage characteristics (ideal diode equation + recombination/generation + tunneling + avalanche corrections), junction breakdown (Zener and avalanche), transient behavior (charge storage, minority-carrier lifetime), terminal functions, and heterojunctions. The p-n junction is the building block of essentially all semiconductor devices and underpins SPICE diode models.

**Metal-Semiconductor Contacts (Ch. 3)**: Schottky barrier formation (image-force lowering, Fermi level pinning), current transport (thermionic emission, thermionic-field emission, tunneling), measurement techniques, Schottky barrier devices, and ohmic contacts. Schottky diodes and ohmic contacts appear directly in SPICE device models.

**MIS/MOS Capacitor (Ch. 4)**: Ideal MIS capacitor theory (flat-band voltage, surface potential, depletion/inversion), Si MOS capacitor (oxide charges, interface traps, threshold voltage), and C-V characteristics. Foundation for MOSFET threshold voltage modeling.

### Part III: Transistors (Chapters 5-7)

**Bipolar Transistor (Ch. 5)**: Static characteristics (current gain, Ebers-Moll model), microwave characteristics (cutoff frequency f_T, maximum oscillation frequency f_max), HBT (heterojunction bipolar transistor). SPICE Gummel-Poon model is rooted in this physics.

**MOSFET (Ch. 6)**: Basic device characteristics (threshold voltage, I-V characteristics in linear/saturation, subthreshold), nonuniform doping and buried-channel, device scaling and short-channel effects (velocity saturation, DIBL, hot-carrier effects), MOSFET structures (SOI, FinFET/3D), circuit applications, nonvolatile memory (floating gate, SONOS), single-electron transistor. The MOSFET is the dominant device in VLSI and the primary target of SPICE BSIM models.

**JFETs, MESFETs, MODFETs (Ch. 7)**: Junction FET and metal-semiconductor FET for microwave/power; MODFET (HEMT) — modulation-doped heterostructure giving high electron mobility for low-noise microwave amplifiers.

### Part IV: Negative-Resistance and Power Devices (Chapters 8-11)

- **Tunnel Diode (Ch. 8)**: Heavily doped p-n junction; negative differential resistance via quantum-mechanical tunneling; resonant-tunneling diode (double barrier)
- **IMPATT Diode (Ch. 9)**: Avalanche transit-time device; highest CW solid-state power at millimeter-wave frequencies
- **Transferred-Electron Device (Ch. 10)**: Gunn diode; intervalley scattering in GaAs causes negative differential resistance; microwave oscillator
- **Thyristors and Power Devices (Ch. 11)**: SCR (p-n-p-n), GTO, MCT, IGBT; high-power switching (kA, kV range)

### Part V: Photonic Devices and Sensors (Chapters 12-14)

- **LEDs and Lasers (Ch. 12)**: Radiative recombination, LED efficiency, laser physics (threshold, gain, modes), heterostructure lasers, quantum cascade laser
- **Photodetectors and Solar Cells (Ch. 13)**: Photoconductors, PIN diodes, avalanche photodiodes, phototransistors, CCDs, MSM, QWIP, solar cells
- **Sensors (Ch. 14)**: Thermal (thermistors, pyroelectric), mechanical (piezoresistive, MEMS), magnetic (Hall, magnetoresistive), chemical (ChemFETs, biosensors)

## Key takeaways

- Device physics is the foundation of all SPICE device models — MOSFET I-V, threshold voltage, junction capacitances, and breakdown are derived from this physics
- Short-channel effects (DIBL, velocity saturation, hot carriers) drive the complexity of modern BSIM models; they originate from scaling physics in Ch. 6
- The MOS capacitor C-V analysis underlies threshold voltage models universally used in VLSI simulators
- Schottky and ohmic contact physics explains parasitic resistances in MOSFET simulations
- p-n junction theory (charge storage, minority-carrier lifetime) underpins SPICE transient diode models

## Pages updated from this source

- [[semiconductor-physics]] - topic page created
- [[mosfet-physics]] - concept created
- [[pn-junction]] - concept created
- [[overview]] - semiconductor device physics noted
