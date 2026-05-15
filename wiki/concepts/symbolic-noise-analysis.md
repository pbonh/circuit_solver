---
title: "Symbolic Noise Analysis"
type: concept
tags: [analog, noise, symbolic, mosfet, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors.txt"]
confidence: medium
---

## Definition

Symbolic noise analysis computes the closed-form expression for the output (or input-referred) noise power spectral density of a small-signal linear circuit as a function of the device-level noise sources (thermal, shot, 1/f) and the circuit parameters.

## How It Works

Each device's small-signal model is augmented with an equivalent noise current/voltage source whose PSD is a known function of device parameters (e.g., `4 k T gm` for thermal, `K_F / (Cox L^2 f)` for 1/f). For each noise source, the transfer function to the output node is computed symbolically (via DDD/GPDD on the augmented MNA/NA). Superposition (sum of squared magnitudes weighted by source PSDs) gives the total output noise.

## Key Parameters

- Noise model level (NLEV 0, 1, 2 in HSPICE).
- Set of noise sources included (thermal only vs. thermal + 1/f).
- Bandwidth and frequency points evaluated.

## When To Use

- Low-noise amplifier design.
- Sensor front-end analysis.
- Verification of analytical noise budgets against HSPICE.

## Risks & Pitfalls

- 1/f noise model parameters vary by foundry/process; symbolic results must be evaluated with the right `K_F` and `A_F`.
- Correlated noise sources (e.g., cyclostationary) are outside basic symbolic noise analysis.

## Related Concepts

- [[concepts/symbolic-analysis]]
- [[concepts/mosfet-small-signal-model]]
- [[concepts/nullor]]
- [[concepts/symbolic-sensitivity-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
