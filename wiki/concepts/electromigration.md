---
title: Electromigration
type: claim
id: claim-electromigration
tags:
- vlsi
- reliability
- well-established
- semiconductor
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt
confidence:
  base: 0.65
---

> GraphsInVLSI Chapter 8 treats electromigration operationally as a design constraint on the on-chip power-delivery network: "objectives include dissipating less power while limiting the current density to reduce the likelihood of electromigration" and "even if the size of the regulator is unlimited, electromigration [525] limits the maximum current density that can be produced by a regulator. The current capacity of a regulator is therefore limited." The chapter extends its fast grid analysis to support an `Imax: S → R` function bounding each regulator's current capacity.

## Definition

Electromigration is the gradual movement of metal atoms in an interconnect caused by momentum transfer from conducting electrons. Over time, electromigration produces voids and hillocks in the metal, eventually leading to open or short circuits and chip failure.

## How It Works

Electron-wind force is proportional to current density J and time. Black's equation gives the mean time to failure: MTTF = A · J^{-n} · exp(E_a / kT) where n is empirically near 2 for many metals. Designers constrain peak current density on every wire and via through electromigration rules: wider wires, more vias, and current-limited regulators are required to meet reliability targets.

## Key Parameters

- Current density J (A/cm²).
- Operating temperature.
- Activation energy E_a (material-dependent).
- Time-to-failure target.

## When To Use

- Power and clock distribution design where currents are high.
- Reliability sign-off in physical verification.

## Risks & Pitfalls

- Hot spots cause accelerated failure.
- Margin must be reserved for transient/peak currents, not just average.

## Related Concepts

- [[concepts/power-distribution-network]]
- [[concepts/voltage-regulator-placement]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
