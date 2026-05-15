---
title: "Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 12: LEDs and Lasers"
type: summary
tags: [semiconductor, device-physics, photonic, p-n-junction, heterojunction, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/17-chapter-12-leds-and-lasers.txt"]
confidence: high
---

## Key Points

- The chapter develops the physics of semiconductor light emitters: spontaneous emission (LEDs) and stimulated emission (lasers). Coverage spans radiative recombination, LED structures and efficiencies, laser physics (population inversion, gain, waveguiding), operating characteristics (threshold current, spectra, modulation, far-field, degradation), and specialty lasers (quantum-well, quantum-wire, quantum-dot, VCSEL, quantum-cascade).
- Radiative transitions: spontaneous emission rate Rsp from filled conduction-band electrons recombining with valence-band holes; emission peak occurs at h*nu = Eg + kT/2 and the linewidth is ~1.8 kT broad. Photon emission strongly favored in direct-bandgap materials (InGaAlP for red-amber, GaAs for IR, InGaN for blue/green).
- Methods of excitation: minority-carrier injection at a p-n or heterojunction is dominant for LEDs and lasers; pumping by optical absorption or electron beam used in some specialty lasers.
- LED device structures: planar diffused, mesa, surface-emitting, edge-emitting, transparent-substrate; surface roughening or shaped chips improve extraction efficiency.
- LED materials: GaAs/GaAlAs (700-870 nm), AlGaInP (yellow-red), GaN/InGaN (UV to blue/green), SiC (early blue). White LEDs by phosphor down-conversion of blue or UV.
- LED efficiencies: internal quantum efficiency (radiative / total recombination), extraction efficiency (escape fraction through surfaces), wall-plug (external) efficiency (light power out / electrical power in), luminous efficiency (in lumens per electrical watt).
- LED frequency response: modulation bandwidth limited by minority-carrier lifetime; typically 100-500 MHz; high-speed LEDs for short-distance optical interconnects achieve > 1 GHz.
- Laser physics: stimulated emission requires population inversion -- more carriers in the upper level than the lower. In a semiconductor, this requires quasi-Fermi-level separation EFn - EFp > h*nu (Bernard-Duraffourg condition).
- Optical resonator: Fabry-Perot cavity (cleaved facets), DBR/DFB grating, or vertical microcavity. Round-trip phase and gain conditions select the lasing modes. Threshold gain g_th = alpha_i + (1/L) ln(1/R).
- Waveguiding: heterojunction laser structures (double heterostructure) confine both carriers and photons in the same narrow active layer.
- Laser materials and structures: Fabry-Perot edge emitters in GaAs/AlGaAs, GaInAsP/InP (1.3 and 1.55 um for fiber communication), VCSELs (vertical-cavity surface-emitting lasers with high-reflectivity DBRs), DFB lasers for single-mode emission.
- Threshold current density: typically 0.5-5 kA/cm^2 for separate-confinement quantum-well lasers; higher for bulk lasers. Quantum-well, quantum-wire, and quantum-dot active regions progressively lower J_th via reduced density of states.
- Spectra, efficiency, far-field, turn-on delay, modulation response, wavelength tuning, and degradation mechanisms are also covered.
- Specialty lasers: quantum-cascade lasers use intersubband transitions in a designer multi-quantum-well stack to emit mid-IR / THz light without recombining across the bandgap; VCSELs enable low-cost arrays for short-range data links.

## Relevant Concepts

- [[concepts/light-emitting-diode]]
- [[concepts/semiconductor-laser]]
- [[concepts/radiative-recombination]]
- [[concepts/p-n-junction]]
- [[concepts/heterojunction]]
- [[concepts/quantum-well]]
- [[concepts/quantum-cascade-laser]]
- [[concepts/vcsel]]
- [[concepts/population-inversion]]
- [[entities/gallium-arsenide]]
- [[entities/indium-phosphide]]

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 12 — LEDs and Lasers
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/17-chapter-12-leds-and-lasers.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
