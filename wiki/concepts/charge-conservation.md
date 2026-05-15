---
title: "Charge Conservation"
type: concept
tags: [analog, transient, device-model, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Charge conservation, in circuit simulation, is the property that the total charge on a node (or in a device terminal) is computed consistently from a state quantity — a charge function — rather than as the time integral of a capacitance multiplied by a voltage difference that may itself be voltage-dependent. Models written in terms of explicit charge functions conserve charge; models written in terms of capacitances `C(v)` without a corresponding charge expression do not.

## How It Works

A **charge-based model** declares q = Q(v) for each terminal and the current is i = dq/dt. The integration method updates q directly, so on a closed loop the simulator's discrete update can be shown to satisfy ∮ q = constant (within KCL tolerance). A **capacitance-based model** computes i = C(v) dv/dt for each terminal; integrating C(v) along a closed loop in v-space can yield path-dependent integrals, so the implied node charges drift over a complete cycle and KCL is not strictly maintained even at the model level.

Even with charge-based models, the simulator's per-step KCL is only an approximation governed by `reltol` and `abstol`. To make the global behavior match true charge conservation, the user must tighten these tolerances on circuits where charge conservation matters.

## Key Parameters

- Model formulation: charge-based (newer MOS, BSIM family) vs. capacitance-based (old Meyer MOSCAP in MOS1/2/3)
- `reltol`, `abstol` — control the KCL violation per step
- Particularly important for switched-capacitor circuits, sample-and-holds, charge pumps, dynamic logic

## When To Use

When simulating:
- Switched-capacitor filters (charge transfer between caps is the signal)
- Sample-and-holds and ADC front ends
- Dynamic logic, DRAM cells, charge pumps
- PLL loop filters where leakage must be modeled accurately

## Risks & Pitfalls

- Using legacy capacitance-based MOS models (MOS1, MOS2, MOS3 Meyer caps) for charge-sensitive circuits gives systematic drift that looks like real charge loss but is purely numerical.
- Even with modern charge-based models, default tolerances may produce visible KCL violation on long, sensitive simulations — tighten reltol and abstol.

## Related Concepts

- [[concepts/local-truncation-error]]
- [[concepts/transient-analysis]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/integration-method]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
