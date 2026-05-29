---
title: Frequency-Dependent Negative Resistance (FDNR)
type: claim
id: concepts/fdnr
tags:
- analog
- ac
- device-model
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A frequency-dependent negative resistance (FDNR) is an active two-terminal subnetwork whose driving-point impedance is Z(s) = D / s^2. It is realized using OPAMPs and capacitors (e.g., the generalized impedance converter of Fig. 4.1.5 with two of its conductances replaced by capacitances) and is used in active filter synthesis as a replacement for inductors.

## How It Works

Starting from a passive RLC ladder filter, an impedance transformation Z(s) → Z(s)/s is applied. The transformation:
- Inductors (sL) become resistors (L).
- Resistors (R) become capacitors (1/(sR)).
- Capacitors (1/sC) become FDNRs (1/(s^2 C)).

The resulting active filter has no inductors and is suitable for integration. Vlach & Singhal's Chapter 4 example designs a ninth-order Cauer-parameter low-pass filter (pass-band 0-3470 Hz, stop-band starting at 3800 Hz, minimum 50 dB attenuation) using FDNRs.

## Key Parameters

- D (the coefficient of 1/s^2 in the impedance).
- OPAMP gain-bandwidth product (limits FDNR accuracy at high frequencies).
- Capacitor matching (controls D's tolerance).

## When To Use

- Active filter design where inductors are impractical (IC fabrication).
- Audio-frequency low-pass and band-pass filters.
- Educational illustration of impedance-transformation synthesis.

## Risks & Pitfalls

- Real OPAMPs have finite bandwidth — FDNR behavior deteriorates near and beyond the GBW.
- Component matching directly affects filter performance.
- Power supply noise can degrade active-filter SNR.

## Related Concepts

- [[concepts/operational-amplifier]]
- [[concepts/cauer-parameter-filter]]
- [[concepts/convertor]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
