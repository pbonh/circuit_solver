---
title: "Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 1: Physics and Properties of Semiconductors (A Review)"
type: summary
tags: [semiconductor, device-physics, foundational, well-established, carrier-transport, band-structure]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt"]
confidence: high
---

## Key Points

- The chapter is a self-contained review of the semiconductor physics needed for the rest of the book, centered on Si and GaAs and organized into eight sections: crystal structure, energy bands, carrier concentration, transport phenomena, phonon/optical/thermal properties, heterojunctions and nanostructures, and basic device equations.
- Crystal structure: most semiconductors are diamond (Si, Ge) or zincblende (GaAs and other III-V) lattices, with rock-salt or wurtzite for compounds like PbS or CdS; Miller indices (hkl) label crystal planes; the reciprocal lattice and Brillouin zone (Wigner-Seitz cell) are introduced for E-k mapping.
- Energy bands: E(k) comes from Bloch-function solutions of the Schrodinger equation; the bandgap Eg (1.12 eV Si, 1.42 eV GaAs at 300 K) separates the conduction and valence bands; Si is indirect-gap and GaAs is direct-gap, which has major consequences for optical and high-speed devices; bandgap shrinks with temperature per Eg(T)=Eg(0)-aT^2/(T+beta).
- Effective mass m* is defined by band curvature (1/m*_ij = (1/hbar^2) d^2E/dk_i dk_j); separate heavy-hole and light-hole valence bands exist; group velocity is (1/hbar) dE/dk.
- Carrier concentration: equilibrium electron and hole densities follow from N(E) (density of states) integrated against the Fermi-Dirac distribution; effective density of states Nc, Nv collapse the result to n = Nc exp(-(Ec-EF)/kT) for nondegenerate semiconductors; mass-action law pn = ni^2 holds outside degeneracy.
- Dopants: donors (e.g., P in Si) and acceptors (e.g., B in Si) provide shallow hydrogenic levels (~25 meV for donors in Si, ~7 meV in GaAs, ~50 meV for acceptors); deep impurities (e.g., Au in Si) introduce mid-gap levels that act as efficient recombination centers; ionization fraction follows from charge neutrality.
- Transport: drift velocity vd = mu*E at low field; mobility is limited by acoustic phonon scattering (mu ~ T^-3/2 * m*^-5/2) and ionized impurity scattering (mu ~ T^3/2 / N_I); Matthiessen's rule combines mechanisms; at high field, vd saturates (~1e7 cm/s in Si); GaAs shows negative differential mobility (transferred-electron effect); resistivity is measured by four-point probe and Hall effect gives sign of carriers, concentration, and Hall mobility.
- Impact ionization: at high field, carriers create electron-hole pairs at a rate alpha(E); breakdown voltage rises with bandgap; ionization rates are empirically fit by exponentials in 1/E or 1/E^2.
- Recombination: direct band-to-band (radiative or Auger) dominates in direct-gap semiconductors (Rec ~ 1e-10 cm^3/s); indirect-gap (Si, Ge) recombines through bulk traps via Shockley-Read-Hall statistics, with mid-gap traps most effective; minority-carrier lifetime tau = 1/(sigma v_th N_t); gold in Si shortens lifetime usefully for fast switching.
- Diffusion: Fick's law gives J_diff = qD dn/dx; Einstein relation D = (kT/q)mu links diffusion to mobility; diffusion length L = sqrt(D tau).
- Thermionic emission (over barriers): J = A* T^2 exp(-q phi_b/kT), with A* the effective Richardson constant; tunneling probability is computed via WKB; both are fundamental conduction mechanisms in Schottky and tunnel diodes.
- Space-charge-limited current: Mott-Gurney law J = (9/8) eps_s mu V^2 / L^3 in the mobility regime; Child-Langmuir law in the ballistic regime; relevant in lightly doped or thin-film devices.
- Phonons: acoustic and optical branches; LO phonon energies 63 meV (Si), 35 meV (GaAs); phonons carry lattice thermal conduction (Si thermal conductivity 1.5 W/cm-K; diamond is highest known).
- Optical absorption: alpha ~ (h*nu - Eg)^gamma; allowed direct (gamma=1/2), forbidden direct (3/2), indirect (2 or 3 with phonon assistance); excitons add structure near band edge.
- Heterojunctions: classified Type-I (straddling), Type-II (staggered), Type-III (broken-gap); strained-layer epitaxy permits lattice-mismatched growth up to a critical thickness; band offsets AE_c, AE_v provide carrier confinement.
- Quantum confinement: quantum wells (2-D) have step-function density of states, wires (1-D) have 1/sqrt(E), dots (0-D) are delta-functions; subband energies E_i = (hbar pi i)^2/(2 m* L_z^2); minibands form in superlattices.
- Basic device equations: Poisson/Gauss law, drift-diffusion current density (Jn = q mu_n n E + q D_n dn/dx; similarly for holes), and continuity equations (dn/dt = G_n - U_n + (1/q) div J_n) form the foundation for all subsequent device chapters; worked examples include Haynes-Shockley experiment and surface-recombination-velocity boundary conditions.

## Relevant Concepts

- [[concepts/energy-band-structure]] — E-k relationship that underpins everything.
- [[concepts/bandgap]] — Eg as the key device parameter.
- [[concepts/effective-mass]] — characterizes carrier dynamics in bands.
- [[concepts/fermi-dirac-distribution]] — occupancy of electron states.
- [[concepts/carrier-concentration]] — equilibrium electron and hole densities.
- [[concepts/donor-acceptor-doping]] — controlled introduction of carriers.
- [[concepts/carrier-mobility]] — drift velocity per unit field.
- [[concepts/drift-diffusion-equation]] — current = drift + diffusion components.
- [[concepts/einstein-relation]] — D and mu are linked by kT/q.
- [[concepts/hall-effect]] — measurement of carrier type, density, and mobility.
- [[concepts/impact-ionization]] — generates carriers at high field; basis of avalanche breakdown.
- [[concepts/shockley-read-hall-recombination]] — trap-assisted recombination in indirect-gap materials.
- [[concepts/carrier-lifetime]] — minority-carrier lifetime tau.
- [[concepts/thermionic-emission]] — barrier-limited current with Richardson constant.
- [[concepts/quantum-mechanical-tunneling]] — WKB probability through finite barriers.
- [[concepts/space-charge-limited-current]] — Mott-Gurney / Child-Langmuir regimes.
- [[concepts/poisson-equation]] — electrostatics of charge / potential.
- [[concepts/continuity-equation]] — time-dependent carrier balance.
- [[concepts/heterojunction]] — Type-I/II/III band alignments and strained epitaxy.
- [[concepts/quantum-well]] — 2-D confined carrier system with subbands.
- [[entities/silicon]] — dominant semiconductor; indirect gap 1.12 eV.
- [[entities/gallium-arsenide]] — direct-gap III-V workhorse for optoelectronics and microwaves.

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 1 — Physics and Properties of Semiconductors: A Review
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
