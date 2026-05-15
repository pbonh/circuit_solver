---
title: "Numerical Damping"
type: concept
tags: [analog, transient, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Numerical (artificial) damping is the loss-like behavior introduced by an [[concepts/integration-method]] that is overly stable in the A-stable sense. Methods such as [[concepts/backward-euler]] and [[concepts/gear-bdf]] (Gear2) add an effective damping term to the discrete dynamics so that even a physically lossless circuit appears to lose energy over time.

## How It Works

The amplification factor of an integration method, applied to the test problem v̇ = λv, shows how the discrete operator handles each eigenmode. A-stable methods that lie strictly inside the unit circle for all stable continuous eigenmodes contract the discrete solution's amplitude more than the continuous solution's — that contraction shows up as damping in the simulated waveform. [[concepts/trapezoidal-rule]] sits exactly on the unit circle for purely imaginary λ, so it preserves oscillation amplitudes; BE and Gear2 lie strictly inside, so they damp.

## Key Parameters

- The choice of method (TR ≈ no damping, Gear2 ≈ mild, BE ≈ strong)
- The eigenvalue spectrum of the circuit (damping is most visible on lightly-loaded resonators)
- Step size h (damping is per-step, so larger h compounds the artifact)

## When To Use

The phenomenon is unavoidable for BE and Gear2; the practical handles are:
- Choose TR by default for analog circuits where oscillation fidelity matters.
- Use Gear2 instead of BE when some damping is needed (e.g., to suppress TR ringing on stiff circuits) without the heavy hand of BE.
- For an LC tank or other lossless resonator, TR is essentially mandatory.

## Risks & Pitfalls

- Beginning analog designers often see unexpected amplitude decay on LC tanks and oscillators and assume the circuit has lost gain — it is the integrator, not the circuit.
- Simulator defaults differ. Knowing which method is in effect (and how to switch) is part of debugging waveform anomalies.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/transient-analysis]]
- [[concepts/local-truncation-error]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
