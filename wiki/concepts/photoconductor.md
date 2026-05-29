---
title: Photoconductor
type: claim
id: concepts/photoconductor
tags:
- semiconductor
- device-physics
- photonic
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/18-chapter-13-photodetectors-and-solar-cells.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Per Sze & Ng (Sect. 13.2): "A photoconductor consists simply of a slab of semiconductor, in bulk or thin-film form, with ohmic contacts affixed to the opposite ends (Fig. 2). When incident light falls on the surface of the photoconductor, carriers are generated either by band-to-band transitions (*intrinsic*) or by transitions involving forbidden-gap energy levels (*extrinsic*), resulting in an increase in conductivity." Conductivity `σ = q(μ_n n + μ_p p)`; under illumination the dominant contribution is the increase in carrier number. The wavelength cutoff is set by ΔE — the bandgap Eg for intrinsic photoconductors, an impurity-to-band energy for extrinsic. The book's Table 1 gives the gain range as 1-10⁶ and the response-time range as 10⁻⁸-10⁻³ s.

## How It Works

Light absorption creates electron-hole pairs that drift under an applied bias. The change in conductivity D sigma = q (mu_n D n + mu_p D p) gives a photocurrent that persists as long as the carriers remain free. Long-lifetime materials (CdS, PbS) give high responsivity but slow response; short-lifetime materials trade gain for bandwidth.

## Key Parameters

- Photoconductive gain G = tau / t_tr.
- Bandwidth (1/(2 pi tau)).
- Responsivity and dark current.
- Wavelength response (set by alpha(lambda)).

## When To Use

- IR detection in PbS, PbSe, HgCdTe.
- Photocopy drums (high-gain xerography).
- Visible-light photocells in consumer products.

## Risks & Pitfalls

- Lifetime-bandwidth product is the fundamental trade-off.
- Sensitivity to surface traps and ambient.

## Related Concepts

- [[concepts/photodiode]]
- [[concepts/carrier-lifetime]]
- [[concepts/carrier-mobility]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
