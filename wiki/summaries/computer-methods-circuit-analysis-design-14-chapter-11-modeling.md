---
title: "Computer Methods for Circuit Analysis and Design — Chapter 11: Modeling"
type: summary
tags: [foundational, device-model, analog, well-established, mosfet, bjt]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/14-chapter-11-modeling.txt"]
confidence: high
---

## Key Points

- Modeling represents the electrical behavior of semiconductor devices and functional blocks by equations, circuits, or tables. Two levels: device-level models (R, L, C, controlled sources for individual transistors/diodes) and macromodels (terminal-behavior representations of functional blocks like op-amps and gates).
- The chapter takes a strictly mathematical approach, not semiconductor physics. CAD specialists typically receive model equations from device specialists and implement them.
- Diode model (Section 11.1): I = I_s [exp(qV/kT) - 1]. At room temperature V_T = kT/q ≈ 25 mV. Operating point (V_0, I_0); dynamic conductance g(V_0) = (I_s/V_T) exp(V_0/V_T). High-frequency model adds a parallel capacitor; more detailed model splits into bulk resistance R_b, depletion capacitance C_j (voltage-dependent, with tangent-line continuation), and diffusion capacitance C_D.
- Linearized diode model: I + delta I = I_eq + Y_eq delta V, expressing the device as a small-signal conductance plus equivalent source — fits directly into the MNA framework.
- FET models (Section 11.2): general 3-terminal device with I_G = 0 and I_D = f(V_GS, V_DS). Two FET types covered:
  - JFET/MESFET (junction or metal-semiconductor barrier): pinch-off voltage V_0 and built-in potential V_a. Linear region: I_D = beta (V_DS [V_GS - V_DS/2 + V_a]/V_0 ...). Saturation region: I_D = saturation expression. Capacitance C_GS depends on bias.
  - MOSFET/IGFET (metal-oxide or insulated-gate): includes substrate as fourth terminal. Includes bulk-junction diodes I_BD' and I_BS'. I_D = beta [(V_GS - V_t) V_DS - V_DS^2/2] linear; I_D = (beta/2)(V_GS - V_t)^2 saturation.
- BJT models (Section 11.3): Ebers-Moll, given in the Motivation chapter. Hybrid-π small-signal model (Fig. 11.3.9) with R_b'e, R_b'c, R_ce, C_b'e, C_b'c, C_ce, R_bb', and g_m V_b'e current source. Typical parameter ranges given in the chapter for production transistors.
- Macromodeling (Section 11.4) — the OPAMP example: macromodels capture terminal behavior with far fewer components than the 20-30 transistors of a real op-amp, and don't require internal-structure information. Three features modeled in detail:
  - Gain characteristic: Bode-plot poles modeled by cascaded sections of [transconductance g_mk into R_k parallel C_k], coupled by VCTs. Each section contributes one pole at -1/(R_k C_k).
  - Finite output swing: nonlinear resistor at the output with low conductance for |V_o| < V_omax and high conductance outside.
  - Slew rate: nonlinear VCT limits maximum charging current I_m into the output capacitor; dV/dt is bounded by I_m / C.
- Cubic splines (Section 11.5) are introduced as a tool for approximating tabulated nonlinear device equations: the model is replaced with a piecewise-cubic interpolation, providing cheap evaluation of f and f' (and optionally f'') for use in Newton-Raphson iteration. Avoids the labor and risk of deriving analytic derivatives of complex device equations.

## Relevant Concepts

- [[concepts/device-modeling]] — Mathematical representation of semiconductor devices for CAD.
- [[concepts/diode-model]] — Shockley equation, junction capacitances, bulk resistance.
- [[concepts/fet-model]] — JFET/MESFET and MOSFET equations.
- [[concepts/bjt-model]] — Ebers-Moll and hybrid-π forms.
- [[concepts/hybrid-pi-model]] — Already covered.
- [[concepts/ebers-moll-model]] — Already covered.
- [[concepts/macromodeling]] — Already covered.
- [[concepts/operational-amplifier-macromodel]] — Gain, finite swing, slew rate.
- [[concepts/spline-approximation]] — Already covered; this chapter introduces cubic splines for nonlinear device approximation.
- [[concepts/slew-rate]] — Maximum rate of output voltage change.

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 11 — Modeling
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/14-chapter-11-modeling.txt`
- Authors: Jiri Vlach, Kishore Singhal
