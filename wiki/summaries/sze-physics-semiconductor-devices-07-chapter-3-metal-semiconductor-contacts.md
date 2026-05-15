---
title: "Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 3: Metal-Semiconductor Contacts"
type: summary
tags: [semiconductor, device-physics, schottky-barrier, ohmic-contact, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/07-chapter-3-metal-semiconductor-contacts.txt"]
confidence: high
---

## Key Points

- Historical context: rectifying point-contact diodes (Braun 1874) preceded the Schottky-Mott-Bethe theoretical framework (1938-1942) that established thermionic emission over a barrier as the dominant conduction mechanism in metal-semiconductor (M-S) rectifying contacts.
- In the ideal Schottky model, the barrier height for electrons is q phi_Bn0 = q(phi_m - chi), where phi_m is the metal work function and chi the semiconductor electron affinity; for holes q phi_Bp = Eg - q phi_Bn.
- Real barriers deviate from this ideal because of surface states / interface states that pin the Fermi level near the charge-neutrality level of the semiconductor surface (typical pinning at ~Eg/3 from valence band in many III-V materials); the pinning makes phi_B nearly independent of metal choice for some semiconductors.
- Image-force lowering: under bias, the conduction-band edge near the metal is depressed by an amount D phi = sqrt(q E_m / (4 pi eps_s)), reducing the effective barrier; this barrier lowering becomes appreciable at high reverse bias and is responsible for the soft reverse characteristic of Schottky diodes.
- Barrier-height engineering: chemical interlayers, ion implantation, surface treatments, and alloying allow tailoring phi_B over a useful range.
- Current transport mechanisms classified:
  - Thermionic emission (Bethe, dominant in moderately doped semiconductors): J = A* T^2 exp(-q phi_B/kT) [exp(qV/(nkT)) - 1], with A* the effective Richardson constant.
  - Diffusion theory (Schottky, dominant in low-mobility / wide-depletion-region semiconductors): drift-diffusion across the depletion region limits current.
  - Thermionic-emission-diffusion (Crowell-Sze): unified model combining both regimes.
  - Tunneling (field emission, thermionic-field emission): dominant in heavily doped semiconductors and ohmic contacts.
  - Minority-carrier injection: usually small for Schottky diodes (majority-carrier device), giving fast switching.
- The MIS tunnel diode and the relationship to Schottky diodes is discussed; thin interfacial oxides can substantially alter barrier height and ideality factor.
- Barrier-height measurement: forward I-V (Richardson plot of ln(I_s/T^2) vs 1/T), activation energy, C-V intercept method, and photoelectric (Fowler) threshold method are presented; cross-correlation between methods detects nonidealities.
- Device structures: Schottky-barrier diodes, point-contact diodes, mixer/detector diodes, Schottky-clamped logic, MESFETs, Schottky-barrier solar cells, and Schottky photodetectors all derive from M-S rectifying junctions.
- Ohmic contacts: low-resistance, linear I-V contacts are required at every device terminal. They are formed by heavily doping the semiconductor under the metal so that the barrier becomes thin enough to conduct by tunneling rather than thermionic emission. Specific contact resistivity rho_c (units Ohm-cm^2) is the figure of merit, decreasing exponentially with the inverse of sqrt(N_D). Silicide formation (TiSi2, NiSi, PtSi) and refractory-metal contacts are key process technologies.

## Relevant Concepts

- [[concepts/schottky-barrier]] — central subject.
- [[concepts/ohmic-contact]] — heavily doped tunneling contact.
- [[concepts/thermionic-emission]] — dominant Schottky transport mechanism.
- [[concepts/image-force-lowering]] — barrier reduction under bias.
- [[concepts/fermi-level-pinning]] — surface-state effect.
- [[concepts/quantum-mechanical-tunneling]] — basis of ohmic contact and field emission.
- [[concepts/junction-capacitance]] — C-V analysis of Schottky barriers.
- [[concepts/p-n-junction]] — comparison and contrast.
- [[entities/silicide]] — common contact metallurgy.

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 3 — Metal-Semiconductor Contacts
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/07-chapter-3-metal-semiconductor-contacts.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
