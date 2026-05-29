---
title: Phase-Locked Loop (PLL)
type: claim
id: claim-phase-locked-loop
tags:
- vlsi
- analog
- mixed-signal
- clock
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt
confidence:
  base: 0.65
---

## Definition

GraphsInVLSI Sect. 4 names the PLL as the standard on-chip clock-generation circuit (ref [333]): "Clock generator circuit, such as a phase locked loop (PLL) ... utilizing a voltage controlled oscillator (VCO) to produce a clock signal."

## How It Works

Per GraphsInVLSI Fig. 4.2: "(a) A low frequency oscillator generates a reference periodic signal [334]. This signal exhibits low variations in response to environmental conditions, such as the temperature. (b) A high frequency voltage controlled oscillator (VCO), such as a relaxation oscillator [335] or Pierce oscillator [336], generates a high frequency signal. The output of the VCO exhibits high sensitivity to parameter variations. (c) The frequency of the VCO output is downscaled by a frequency divider. (d) The phase detector compares the phase of signal c to the phase of the reference oscillator. (e) The change in average phase difference is converted into the input voltage of the VCO, thereby maintaining a constant high frequency at the PLL output." Once locked, the output frequency equals N × reference frequency, where N is the divider ratio.

## Key Parameters

- Reference frequency.
- VCO tuning range and gain.
- Loop bandwidth.
- Jitter and lock time.

## When To Use

- On-chip clock generation in essentially every modern SoC.
- Frequency synthesis in RF and communication transceivers.
- Clock and data recovery in serial links.

## Risks & Pitfalls

- Supply and substrate noise couple into the VCO, increasing jitter.
- Loop stability requires careful filter design.
- Lock acquisition and re-lock time during dynamic frequency changes.

## Related Concepts

- [[concepts/clock-distribution-network]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
