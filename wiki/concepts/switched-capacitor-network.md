---
title: Switched-Capacitor (SC) Network
type: claim
id: claim-switched-capacitor-network
tags:
- switched-capacitor
- analog
- mixed-signal
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/17-chapter-14-digital-and-switched-capacitor-networks.txt
confidence:
  base: 0.85
---

## Definition

A switched-capacitor (SC) network simulates resistors using capacitors switched at a high clock rate. For f_signal << f_clock, a capacitor C alternately connected between two nodes by switches acts as an effective conductance G_eq = C * f_clock. SC networks make analog signal processing fully integrable in MOS technology.

## How It Works

The network has two (or more) periodic clock phases. During each phase, certain switches are closed and others open, producing a different LTI network. At the boundary between phases, charge is redistributed instantaneously.

Frequency response is determined by:
- The signal frequency f_signal (continuous).
- The clock frequency f_clock (defines the SC "resistor" values).
- The capacitor ratios (set the filter shape).

SC networks are periodically time-varying (period T_clock); their output spectrum contains aliases at f_signal +/- n f_clock. For f_signal << f_clock/2, the SC network behaves like an equivalent continuous-time network.

## Key Parameters

- Clock frequency f_clock.
- Capacitor values and ratios.
- Number of clock phases (typically 2).
- Signal bandwidth (must be << f_clock / 2 to avoid aliasing).

## When To Use

- Integrated analog filters (anti-aliasing, smoothing, audio).
- Sample-and-hold, comparators, ADCs.
- Replacement of resistors with capacitors in MOS-only processes.

## Risks & Pitfalls

- Aliasing requires anti-aliasing pre-filters.
- Clock feedthrough and charge injection from switches introduce noise.
- Capacitor matching (typically 0.1-1%) sets filter precision.

## Related Concepts

- [[concepts/two-graph-modified-nodal]]
- [[concepts/switch-model]]
- [[concepts/clock-phase-formulation]]
- [[concepts/sc-spectral-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-17-chapter-14-digital-and-switched-capacitor-networks]]
