---
title: Cauer-Parameter (Elliptic) Filter
type: claim
id: claim-cauer-parameter-filter
tags:
- analog
- ac
- well-established
- filter
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt
confidence:
  base: 0.65
---

## Definition

A Cauer-parameter (elliptic) filter is a class of filters that exhibit equiripple behavior in both pass-band and stop-band, achieving the steepest transition between the two for a given order. They are named after Wilhelm Cauer who first synthesized them. Vlach & Singhal use a ninth-order elliptic low-pass filter as the design example for Chapter 4.

## How It Works

The transfer function is parameterized by Jacobian elliptic functions; design tables and algorithms produce element values for a normalized prototype (typically R = 1 ohm, pass-band edge omega = 1 rad/s). The prototype is then frequency- and impedance-scaled to the target specification.

In the textbook example, the filter has:
- Pass-band: 0 to 3470 Hz with 0.03 dB ripple.
- Stop-band: starting at 3800 Hz with at least 50 dB attenuation.
- Order 9 (initial design tightened to 0.02 dB ripple for safety margin).

The passive LC prototype is converted to an active realization using FDNRs (replacing inductors).

## Key Parameters

- Filter order n.
- Pass-band ripple (dB).
- Stop-band attenuation (dB).
- Pass-band edge omega_p; stop-band edge omega_s.

## When To Use

- Anti-aliasing and reconstruction filters where sharp roll-off is required.
- Audio and communication channel filtering.
- Wherever steepest transition for a given order matters.

## Risks & Pitfalls

- Element sensitivities are higher than for Butterworth or Chebyshev filters of the same order.
- Stop-band notches require precise component matching.
- Active realizations have group-delay distortion sensitivities to OPAMP bandwidth.

## Related Concepts

- [[concepts/fdnr]]
- [[concepts/network-function]]
- [[concepts/operational-amplifier]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
