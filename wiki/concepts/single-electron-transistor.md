---
title: "Single-Electron Transistor (SET)"
type: concept
tags: [semiconductor, device-physics, quantum, advanced, emerging]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: low
---

## Definition

A single-electron transistor is a three-terminal device in which a small conducting island is weakly coupled to source and drain by tunnel junctions and capacitively coupled to a gate. Coulomb blockade -- the energy cost q^2/(2 C_Sigma) to add one electron -- prevents current flow unless the gate voltage tunes the island to an allowed charge state, producing periodic conductance oscillations with V_G.

## How It Works

When the charging energy E_C = q^2 / (2 C_Sigma) exceeds kT and the tunnel resistance exceeds h/q^2 ~ 26 kOhm, the island carries an integer number of excess electrons. Conduction occurs only at gate voltages where two adjacent charge states are degenerate. The conductance G(V_G) is therefore a periodic Coulomb-oscillation pattern.

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
