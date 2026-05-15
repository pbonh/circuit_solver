---
title: "Switch Model (F-Coefficient)"
type: concept
tags: [analog, transient, device-model, well-established, switched-capacitor]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt"]
confidence: medium
---

## Definition

A perfect switch can be incorporated into the MNA framework by inserting a switch coefficient F into a resistor-like stamp. Setting F = 0 produces an open circuit; setting F = 1 produces a short circuit. The system matrix is generated once, and toggling the switch state requires only updating the F value, not re-formulating the equations.

## How It Works

The unified stamp (Eq. 4.4.1 of Vlach & Singhal) introduces an extra column for the switch current and an extra row for the constitutive equation. With F = 0 the constitutive equation becomes I = 0 (open circuit, no current); with F = 1 it becomes V_j - V_j' = 0 (short circuit, equal voltages).

This is the foundation of switched-capacitor network analysis (Chapter 14 of the book), where the network topology changes periodically as switches toggle on and off.

## Key Parameters

- F (0 = open, 1 = short).
- For non-ideal switches, F can take intermediate values or a frequency-dependent admittance.

## When To Use

- Switched-capacitor filters and sample-and-hold circuits.
- Switching power converters.
- Simulators that need to vary topology without rebuilding the matrix.

## Risks & Pitfalls

- An open switch leaves a floating node — initial conditions on adjacent capacitors must be carefully managed.
- Ideal switches produce stiff transients; real switches have finite on-resistance and finite off-conductance.
- Charge injection and clock feedthrough are not captured by the ideal model.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/branch-stamping]]
- [[concepts/switched-capacitor-network]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
- [[summaries/computer-methods-circuit-analysis-design-17-chapter-14-digital-and-switched-capacitor-networks]]
