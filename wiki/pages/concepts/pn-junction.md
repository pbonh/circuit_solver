---
title: p-n Junction
type: concept
slug: pn-junction
created: 2026-06-16
updated: 2026-06-16
summary: Fundamental semiconductor device formed by joining p-type and n-type regions; the building block of diodes, BJTs, MOSFETs, and all junction-based SPICE models.
tags: [semiconductor, pn-junction, diode, device-physics, spice]
sources: [physics-of-semiconductor-devices-3rd-ed]
status: active
---

# p-n Junction

A p-n junction is formed by placing p-type (hole-rich) and n-type (electron-rich) semiconductor regions in intimate contact. At equilibrium, diffusion and drift balance to establish a depletion region and built-in voltage V_bi. The junction is the fundamental building block of semiconductor electronics.

## Equilibrium and Depletion

Built-in voltage: V_bi = (kT/q) ln(N_A N_D / n_i²)
Depletion width: W = sqrt(2ε(V_bi - V_A)(N_A + N_D) / (q N_A N_D))
Junction capacitance: C_j = ε A / W (decreases with reverse bias)

## Current-Voltage Characteristics

**Ideal diode (Shockley)**: I = I_s [exp(V_A/V_th) - 1], where I_s is reverse saturation current dependent on minority-carrier diffusion lengths.

**Non-ideal corrections**: recombination current in the depletion region (ideality factor n=2 at low forward bias), high-level injection (n approaches 2 at high current), series resistance, and tunneling.

**Reverse bias leakage**: dominated by generation in the depletion region (I ∝ W·n_i / τ).

## Breakdown

- **Zener (tunneling) breakdown**: direct band-to-band tunneling; occurs at V < ~6V in heavily doped junctions; temperature coefficient negative
- **Avalanche breakdown**: impact ionization; temperature coefficient positive; characteristic of lightly doped junctions; ionization integral = 1 at breakdown

## Transient Behavior

- **Minority-carrier charge storage**: forward-biased junction stores minority carriers; limited by lifetime τ; turn-off time proportional to stored charge
- **Depletion-layer capacitance**: C_j(V) = C_j0 / (1 - V/V_bi)^m; m = 0.5 (abrupt) to 0.33 (graded); stored energy limits switching speed

## SPICE Model Connection

The SPICE diode model (Level 1) implements:
- DC: I_s, ideality n, series resistance R_s
- AC: junction capacitance C_j with grading coefficient m
- Transient: transit time τ_T for charge storage

From [[simulation-analog-mixed-signal-circuits]]: "charge-based models" that conserve charge properly (avoiding the old Meyer MOSCAP issue) are required for accurate switched-capacitor circuit simulation — rooted in proper junction capacitance treatment.

## Heterojunctions

p-n junctions between dissimilar semiconductors (GaAs/AlGaAs, Si/SiGe) offer bandgap engineering. Heterointerface conduction band offset confines carriers. Used in HBTs, MODFETs (2DEG), LEDs, and lasers.

## Related concepts and entities

- [[mosfet-physics]] - source/drain junctions; body diodes
- [[semiconductor-physics]] - parent topic
- [[spice-simulation]] - SPICE diode model implements junction physics
- [[circuit-simulation]] - junctions appear in every semiconductor circuit
