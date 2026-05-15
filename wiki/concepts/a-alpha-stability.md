---
title: "A(alpha)-Stability"
type: concept
tags: [ode, numerical-integration, stiff, stability, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

Widlund's (1967) relaxation of [[concepts/a-stability]]: a method is A(α)-stable if its [[concepts/stability-region]] contains the open sector S_α = {z : |arg(-z)| < α, z ≠ 0} for some 0 < α ≤ π/2. The case α = π/2 recovers full A-stability. The relaxation makes room for high-order BDF formulas that A-stability forbids by the [[concepts/dahlquist-barrier]].

## How It Works

The geometry is a wedge in the left half plane opening symmetrically about the negative real axis. For [[concepts/gear-bdf]] formulas of orders k = 1..6 the maximal angles are α ≈ 90°, 90°, 86°, 73°, 52°, 18° respectively; BDF7 has no positive α and is unusable. Grigorieff–Schroll (1978) showed A(α)-stable k-step LMS methods of order k exist for every α < π/2 and every k, so order is no longer barred — only the sector shrinks. Eigenvalues of the Jacobian must lie inside S_α scaled by h for stability; eigenvalues near the imaginary axis (oscillatory modes) escape the sector and de-stabilize the integration.

## Key Parameters

- Sector half-angle α (degrees or radians).
- Method order p and the (k, α) trade-off for BDF and multistep classes.
- Eigenvalue distribution of the Jacobian relative to the negative real axis.

## When To Use

- Stiff systems whose Jacobian spectrum stays clustered near the negative real axis (chemical kinetics, parabolic [[concepts/method-of-lines]] discretisations).
- Whenever higher-order accuracy is needed than the order-2 ceiling that A-stability imposes for LMS methods.

## Risks & Pitfalls

- Eigenvalues with large imaginary parts (oscillatory / lightly damped modes) fall outside the sector and cause spurious growth — A(α)-stability is not enough for problems with nearly-imaginary Jacobian eigenvalues.
- The angle α shrinks rapidly for high BDF orders; BDF6 (α ≈ 18°) is fragile in practice.
- A(α)-stability says nothing about damping at infinity; pair with [[concepts/l-stability]] reasoning for very stiff transients.

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/ao-stability]]
- [[concepts/l-stability]]
- [[concepts/dahlquist-barrier]]
- [[concepts/gear-bdf]]
- [[concepts/stability-region]]
- [[concepts/stiff-stability]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
