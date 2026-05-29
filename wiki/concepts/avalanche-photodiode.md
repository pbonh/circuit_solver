---
title: Avalanche Photodiode (APD)
type: claim
id: concepts/avalanche-photodiode
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

An avalanche photodiode is a photodiode operated near reverse breakdown so that photogenerated carriers undergo impact-ionization multiplication, amplifying the photocurrent by a gain M = I_photo,out / I_primary. Often constructed with separate absorption and multiplication (SAM) regions to allow optimization of each.

## How It Works

Photons generate primary electron-hole pairs in a low-field absorption region; primary carriers drift into a thin high-field multiplication region where impact ionization causes a controlled avalanche. The excess noise factor F(M) = k M + (1 - k)(2 - 1/M) depends on the carrier ionization-rate ratio k = alpha_p / alpha_n -- so material choice (Si with small k for visible; InGaAs/InP SAM for 1.3-1.55 um) is crucial for low-noise operation.

## Key Parameters

- Avalanche gain M (typically 10-100; can exceed 1000 in Si).
- Excess noise factor F(M).
- Operating reverse bias and temperature.
- Wavelength response (set by absorption material).

## When To Use

- Long-haul fiber optic receivers (1.55 um InGaAs/InP).
- LIDAR receivers (Si APDs at 905 nm).
- Single-photon detection (Geiger-mode operation above breakdown).

## Risks & Pitfalls

- High reverse bias requires careful thermal and breakdown-uniformity engineering.
- Gain depends on temperature; voltage must track T.

## Related Concepts

- [[concepts/photodiode]]
- [[concepts/impact-ionization]]
- [[concepts/avalanche-breakdown]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-16-part-v-photonic-devices-and-sensors]]
- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
