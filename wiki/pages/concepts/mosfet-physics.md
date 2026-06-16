---
title: MOSFET Physics
type: concept
slug: mosfet-physics
created: 2026-06-16
updated: 2026-06-16
summary: Physical operating principles of the MOSFET — threshold voltage, I-V characteristics, scaling, and short-channel effects — forming the basis of all VLSI circuit simulation models.
tags: [mosfet, vlsi, device-physics, spice, scaling, bsim]
sources: [physics-of-semiconductor-devices-3rd-ed]
status: active
---

# MOSFET Physics

The Metal-Oxide-Semiconductor Field-Effect Transistor (MOSFET) is the dominant device in VLSI. In a MOSFET, the gate capacitively modulates the surface potential of a semiconductor channel between source and drain. The gate voltage controls whether a conducting inversion layer (channel) forms.

## Threshold Voltage

V_T = V_FB + 2φ_F + Q_dep / C_ox

where V_FB is flat-band voltage (includes workfunction difference and oxide charge), φ_F is Fermi potential, Q_dep is depletion charge, C_ox is oxide capacitance. Short-channel effects reduce V_T (DIBL — drain-induced barrier lowering).

## I-V Characteristics

**Linear region** (V_DS < V_GS - V_T):
  I_D = μ_n C_ox (W/L) [(V_GS - V_T) V_DS - V_DS²/2]

**Saturation** (V_DS ≥ V_GS - V_T):
  I_D,sat = μ_n C_ox (W/2L) (V_GS - V_T)²

**Subthreshold**: I_D ∝ exp(V_GS / n V_th) — drain current below V_T; subthreshold swing S = n·(kT/q)·ln10; ideal minimum 60 mV/decade at room temperature.

## Short-Channel Effects

As L shrinks (device scaling), long-channel approximations break down:
- **Velocity saturation**: carrier velocity saturates at v_sat ≈ 10^7 cm/s; I_D becomes linear in (V_GS - V_T) not quadratic
- **DIBL**: drain field lowers source barrier; V_T decreases with V_DS; degrades off-current
- **Hot-carrier effects**: energetic carriers create interface traps and oxide damage; reliability concern
- **Channel-length modulation**: finite output resistance in saturation (λ parameter in SPICE LEVEL 1/2/3)
- **Polysilicon depletion, quantum effects**: at thin oxides, inversion layer centroid shifts; effective oxide thickness increases

## Device Structures

- **SOI MOSFET**: silicon-on-insulator reduces parasitic junction capacitance and body effects
- **FinFET / 3D MOSFET**: double/triple gate for better electrostatic control; standard below 14nm
- **Nonvolatile memory**: floating-gate (Flash) and SONOS store charge in gate stack to shift V_T

## SPICE Model Connection

SPICE LEVEL 1-3 models are analytical approximations of the above physics. BSIM3v3/BSIM4/BSIM-CMG are semi-empirical models with 100+ parameters fitted to measurements. The SPICE model equations for V_T, I_D, and capacitances are discretized forms of semiconductor equations.

From [[spice-simulation]]: the MOS capacitor charge-based models in modern simulators (vs. old Meyer capacitance model) correctly conserve charge — this directly references the MIS physics in Chapter 4 of this source.

## Why it matters

- MOSFET I-V and capacitance models drive the accuracy of all VLSI circuit simulations
- Short-channel effects are why BSIM models have grown so complex — each effect adds parameters and branches
- Threshold voltage variation from doping fluctuations is a primary cause of SRAM mismatch — modeled as V_T mismatch in Monte Carlo simulation

## Related concepts and entities

- [[pn-junction]] - source/drain junctions are p-n junctions
- [[spice-simulation]] - uses MOSFET models for circuit simulation
- [[semiconductor-physics]] - parent topic
- [[verilog-ams]] - behavioral MOSFET models can be written in Verilog-AMS
- [[circuit-simulation]] - MOSFET is the primary device in VLSI circuits
