---
title: "Short-Channel Effects"
type: concept
tags: [semiconductor, device-physics, mosfet, scaling, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: medium
---

## Definition

Short-channel effects are the collection of departures from long-channel MOSFET behavior that emerge as the channel length L is reduced toward the source and drain depletion widths. The major effects are Vt rolloff (charge sharing), drain-induced barrier lowering (DIBL), velocity saturation, hot-carrier injection, channel-length modulation, and punch-through.

## How It Works

- Charge sharing: a fraction of the channel depletion charge is supplied by the source/drain depletion regions, reducing Vt at short L.
- DIBL: high V_DS extends the drain depletion region into the channel, lowering the channel barrier and reducing Vt (and raising subthreshold leakage).
- Channel-length modulation: pinch-off point moves toward source with increasing V_DS, reducing effective L and increasing I_Dsat (lambda parameter).
- Velocity saturation: high E along the channel forces v_d to saturate at v_s, breaking the (V_GS - Vt)^2 law into a more linear dependence.
- Punch-through: source and drain depletion regions touch, allowing direct current flow independent of V_GS.

## Key Parameters

- Channel length L vs source/drain junction depth and gate-oxide thickness.
- Substrate doping (heavier doping mitigates SCE but adds variability).
- Drain bias V_DS.

## When To Use

- Setting design rules at advanced technology nodes.
- Choosing halo / pocket implant doses to suppress Vt rolloff.

## Risks & Pitfalls

- SCE mitigation by heavier channel doping raises body effect and reduces mobility.
- LDD/extension engineering balances hot-carrier reliability and series resistance.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/threshold-voltage]]
- [[concepts/subthreshold-conduction]]
- [[concepts/hot-carrier-effects]]
- [[concepts/dennard-scaling]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
