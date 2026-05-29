---
title: Trapezoidal Rule
type: claim
id: concepts/trapezoidal-rule
tags:
- analog
- transient
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The Trapezoidal Rule (TR) is the second-order implicit [[concepts/integration-method]] most commonly used as the default in analog circuit simulation. It is the average of [[concepts/forward-euler]] and [[concepts/backward-euler]]: v̇_k = (2/h)(v_k − v_{k-1}) − v̇_{k-1}.

## How It Works

TR replaces d/dt by the discrete trapezoidal operator and substitutes into the circuit's DAE system. Like BE it is implicit, requiring a [[concepts/newton-raphson-method]] solve per timestep. Because it averages FE and BE, it inherits BE's stiff stability (in the marginal sense) and FE's lack of artificial damping (it preserves oscillation amplitudes on lossless systems).

## Key Parameters

- Step size h
- Order = 2 — LTE scales as O(h³) per step

## When To Use

- Default for general-purpose analog [[concepts/transient-analysis]]: second-order accurate, no artificial damping on LC tanks, acceptable behavior on most circuits.

## Risks & Pitfalls

- **Marginally stable on stiff circuits** — produces characteristic point-to-point ringing whose amplitude shrinks as h shrinks. Tightening `reltol` (which forces smaller h) typically eliminates the ringing.
- LTE estimate per step depends on the third derivative of the solution — circuits with sharp transitions need correspondingly small h to keep TR accurate.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/forward-euler]]
- [[concepts/backward-euler]]
- [[concepts/gear-bdf]]
- [[concepts/stiff-circuit]]
- [[concepts/numerical-damping]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
