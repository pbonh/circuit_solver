---
title: Junction Capacitance
type: claim
id: claim-junction-capacitance
tags:
- semiconductor
- device-physics
- p-n-junction
- ac
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt
confidence:
  base: 0.85
---

## Definition

The junction capacitance of a p-n junction has two contributions: a depletion (or space-charge) capacitance C_D = eps_s / W set by the width of the depletion region, and a diffusion capacitance C_d set by stored minority-carrier charge in the quasi-neutral regions under forward bias.

## How It Works

- C_D = eps_s / W, where W shrinks under forward bias and widens under reverse bias; thus C_D decreases with reverse bias, the basis of the varactor diode.
- C_d arises because forward bias injects minority carriers whose charge must be supplied/removed when V changes. C_d ~ q I tau / (kT) for a long-base diode and dominates at moderate to large forward bias.

## Key Parameters

- Doping levels (set W).
- Bias V (sets W and minority-carrier injection level).
- Junction grading (abrupt, linearly graded, hyperabrupt).
- Minority-carrier lifetime tau (sets C_d).

## When To Use

- Designing varactors and varicaps for voltage-controlled oscillators.
- Modeling small-signal high-frequency response of any p-n junction device.
- Extracting doping profile from C-V measurements (1/C^2 vs V slope and intercept).

## Risks & Pitfalls

- C_d severely limits switching speed at high forward bias.
- Junction grading dramatically changes C(V) law: hyperabrupt for wide-range tuning.

## Related Concepts

- [[concepts/depletion-region]]
- [[concepts/varactor-diode]]
- [[concepts/p-n-junction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
