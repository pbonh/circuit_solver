---
title: "Photoconductor"
type: concept
tags: [semiconductor, device-physics, photonic, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/18-chapter-13-photodetectors-and-solar-cells.txt"]
confidence: low
---

## Definition

A photoconductor is a bulk semiconductor (or thin film) with two ohmic contacts whose resistance decreases when illuminated. Photogenerated carriers add to the equilibrium conductivity until they recombine after a lifetime tau. Photoconductive gain = tau / t_tr (carrier lifetime divided by transit time) can exceed unity, so photoconductors can deliver more electrons per absorbed photon than a junction device.

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
