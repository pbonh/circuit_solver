---
title: Stiff Circuit
type: claim
id: claim-stiff-circuit
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
  base: 0.85
---

## Definition

A circuit is **stiff** when its dynamics include time constants much shorter than the timestep the user (or simulator) wants to take. The fastest dynamics may be quickly-decaying transients that no longer affect the visible response, but their presence forces explicit integrators to take very small steps to remain stable.

## How It Works

Linearizing the circuit around an operating point yields a Jacobian whose eigenvalues represent the system's time constants. A wide spread between the largest and smallest |λ| (the "stiffness ratio") is the defining property. Explicit methods like [[concepts/forward-euler]] are stable only when h is bounded by the inverse of the largest eigenvalue magnitude — so a stiff circuit forces tiny h even after the fast modes have died out.

## Key Parameters

- Stiffness ratio: max |λ_i| / min |λ_i|
- Maximum stable step for explicit methods: h_max ≈ 2 / max |λ_i|
- Choice of integration method (implicit stiffly-stable methods remove the stability ceiling on h)

## When To Use

The classification matters for two decisions:
1. **Pick a stiffly-stable integration method.** Use [[concepts/backward-euler]], [[concepts/trapezoidal-rule]], or [[concepts/gear-bdf]] — never Forward Euler on a real analog circuit.
2. **Watch for marginal-stability artifacts.** Even with stiffly-stable methods, TR produces point-to-point ringing on stiff circuits; BE and Gear2 produce [[concepts/numerical-damping]].

## Risks & Pitfalls

- Using Forward Euler / explicit methods on stiff circuits gives either divergence or absurdly small timesteps — this is why [[concepts/timing-simulation]] is restricted to partitions that are individually non-stiff.
- Even with implicit methods, ill-chosen tolerances can leave fast modes under-resolved and cause spurious behavior.
- Stiffness is a property of the linearization; nonlinearities can transiently change which eigenvalues are dominant.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/forward-euler]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/numerical-damping]]
- [[concepts/transient-analysis]]
- [[concepts/timing-simulation]]

## Sources

- [[summaries/hairer-ode-ii-00-front-matter]]
- [[summaries/hairer-ode-ii-01-preface]]
- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
