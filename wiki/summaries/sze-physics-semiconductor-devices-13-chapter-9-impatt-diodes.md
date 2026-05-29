---
title: 'Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 9: IMPATT Diodes'
type: source
id: source-sze-physics-semiconductor-devices-13-chapter-9-impatt-diodes
kind: derived-summary
tags:
- semiconductor
- device-physics
- rf
- mm-wave
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/13-chapter-9-impatt-diodes.txt
---

## Key Points

- IMPATT (impact-ionization avalanche transit-time) diodes produce the highest continuous-wave (CW) microwave power of any solid-state device at millimeter-wave frequencies (above 30 GHz). They combine impact-ionization avalanche multiplication at one end of the device with carrier drift through a transit region to produce the 180-deg phase shift between voltage and current that yields a negative resistance at microwave frequencies.
- Static characteristics: device structure (typically p+-n-i-n+ or Read profile) with sharp avalanche region near the heavily doped junction and a wider drift region. Breakdown voltage is set by impact ionization; design balances breakdown and transit-time considerations.
- Avalanche region: carrier pairs are generated periodically by the AC field on top of the DC breakdown field; injection lags voltage peak by ~90 deg due to inductive nature of the avalanche.
- Drift region: injected carriers drift at saturation velocity v_s across the depletion region, providing a further ~90 deg phase shift.
- Combined ~180-deg phase shift gives negative resistance at the transit-time frequency f = v_s / (2 L).
- Small-signal analysis (Read, Misawa) and large-signal (Scharfetter, Gummel) analyses predict efficiency and the power-frequency product.
- Power-frequency limits: electronic (set by the v_s E_c product and the breakdown integral, leading to P x f^2 ~ const) and thermal (set by heat removal and junction temperature).
- Noise behavior: IMPATTs are noisy because of the random nature of avalanche multiplication; figure-of-merit noise measure typically 30-40 dB.
- Device-design examples: Si and GaAs IMPATTs; double-drift structures (carriers of both signs contribute) and hi-lo / lo-hi-lo profiles to reduce noise.
- BARITT (barrier-injection transit-time) diode: uses minority-carrier injection over a barrier rather than avalanche; lower noise but lower efficiency than IMPATT.
- TUNNETT (tunnel-injection transit-time) diode: uses field-emission tunneling for injection; potentially capable of mm-wave operation with reduced noise.

## Relevant Concepts

- [[concepts/impatt-diode]]
- [[concepts/impact-ionization]]
- [[concepts/avalanche-breakdown]]
- [[concepts/negative-differential-resistance]]
- [[concepts/p-n-junction]]
- [[entities/silicon]]
- [[entities/gallium-arsenide]]

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 9 — IMPATT Diodes
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/13-chapter-9-impatt-diodes.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
