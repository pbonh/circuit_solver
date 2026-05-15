---
title: "Shockley-Read-Hall Recombination"
type: concept
tags: [semiconductor, device-physics, recombination, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt"]
confidence: high
---

## Definition

Shockley-Read-Hall (SRH) recombination is the trap-assisted, indirect recombination of electrons and holes via bulk trap states located within the forbidden gap. It is the dominant recombination mechanism in indirect-gap semiconductors such as Si and Ge.

## How It Works

A trap level E_t with density N_t can capture an electron from the conduction band (rate ~ sigma_n v_th n N_t (1-F)) and subsequently a hole from the valence band, or vice versa. The net rate is U = (sigma_n sigma_p v_th N_t (pn - n_i^2)) / (sigma_n (n + n_1) + sigma_p (p + p_1)). Traps at midgap (E_t ~ E_i) maximize U; traps near band edges contribute less. Under low-level injection in an n-type semiconductor, the lifetime simplifies to tau_p = 1/(sigma_p v_th N_t).

## Key Parameters

- Trap density N_t and energy E_t.
- Capture cross sections sigma_n, sigma_p.
- Thermal velocity v_th.

## When To Use

- Modeling minority-carrier lifetime in Si and Ge.
- Designing fast-switching diodes by deliberate lifetime control (Au, Pt doping; high-energy irradiation).
- Diagnosing process-induced defects via lifetime measurement.

## Risks & Pitfalls

- Multi-level traps require more complex analysis than the single-level SRH model.
- Surface recombination (via interface states) is a 2-D analog and often more important than bulk SRH in modern devices.
- Lifetime is not a fixed material constant: it depends strongly on process and injection level.

## Related Concepts

- [[concepts/carrier-lifetime]]
- [[concepts/donor-acceptor-doping]]
- [[concepts/p-n-junction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
