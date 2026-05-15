---
title: "Phase-Locked Loop (PLL)"
type: concept
tags: [vlsi, analog, mixed-signal, clock, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt"]
confidence: low
---

## Definition

A phase-locked loop (PLL) is a feedback control circuit that generates an output clock whose phase (and hence frequency) tracks that of a reference input. PLLs are the standard on-chip clock generator in VLSI systems.

## How It Works

A PLL contains a phase detector, low-pass loop filter, voltage-controlled oscillator (VCO), and (optionally) a frequency divider in the feedback path. The phase detector compares the divided output to a reference; the filtered phase error tunes the VCO frequency. Once locked, the output frequency equals N × reference frequency, where N is the divider ratio.

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
