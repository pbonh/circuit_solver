---
title: Single-Electron Transistor (SET)
type: claim
id: claim-single-electron-transistor
tags:
- semiconductor
- device-physics
- quantum
- advanced
- emerging
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt
confidence:
  base: 0.65
---

## Definition

A single-electron transistor (SET), first observed experimentally in 1987 (Sze Sect. 6.8), is a three-terminal device in which "a central single-electron island that has to be extremely small ... is connected between the source and drain via capacitors through which tunneling occurs to conduct current. The third terminal is the insulated gate" (Fig. 53a of Sze). Coulomb blockade — the energy cost `q²/(2C_Σ)` to add one electron to the island, where `C_Σ = C_S + C_D + C_G` (Eq. 127) — prevents current flow unless the gate voltage tunes the island to an allowed charge state.

## How It Works

When the charging energy `E_C = q²/(2 C_Σ)` exceeds kT and the tunnel resistance exceeds `h/q² ~ 26 kΩ`, the island carries an integer number of excess electrons. Conduction occurs only at gate voltages where two adjacent charge states are degenerate. The conductance G(V_G) is therefore a periodic Coulomb-oscillation pattern; in the I_D-V_D plane the forbidden regions form the "Coulomb-blockade diamonds" pictured in Sze Sect. 6.8 (p. 365 figure caption).

## Key Parameters

- Total island capacitance C_Sigma (smaller is better, gives larger E_C).
- Tunnel-barrier resistance.
- Operating temperature (kT must be < E_C; nm-scale islands needed for room-T).

## When To Use

- Charge-sensitive electrometers.
- Quantum-computation candidates (charge qubits).
- Research devices probing mesoscopic physics.

## Risks & Pitfalls

- Background-charge fluctuations destabilize the operating point.
- Manufacturing reproducibility of nm-scale islands is very difficult.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/quantum-mechanical-tunneling]]
- [[concepts/mis-capacitor]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
