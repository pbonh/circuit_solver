---
title: "Z-Transform"
type: concept
tags: [digital, foundational, well-established, math]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/17-chapter-14-digital-and-switched-capacitor-networks.txt"]
confidence: high
---

## Definition

The z-transform is the discrete-time analog of the Laplace transform. For a sequence x(n), X(z) = sum_n x(n) z^{-n}. Causal sequences have an analytic X(z) outside a disk centered at the origin (region of convergence).

## How It Works

z-transform pairs:
- delta(n) ↔ 1.
- u(n) ↔ z/(z-1).
- a^n u(n) ↔ z/(z-a).

The frequency response of a discrete LSI system H(z) is obtained by evaluating on the unit circle: H(e^{j omega T}) = sum_n h(n) e^{-j omega n T}. Stability requires all poles of H(z) inside the unit disk.

Discrete network functions are rational in z. The same DFT-based interpolation as in Chapter 7 (for s-domain) extracts polynomial coefficients on the unit circle.

## Key Parameters

- z-plane (complex variable).
- Unit circle |z| = 1 (frequency response).
- Pole locations (stability indicator).

## When To Use

- Analysis of digital filters and digital control systems.
- Switched-capacitor network analysis (at the sample times).
- Bridge between continuous-time and sampled-data system analysis.

## Risks & Pitfalls

- Bilinear transform between s and z domains can warp frequency axis.
- Aliasing in the s ↔ z mapping.

## Related Concepts

- [[concepts/discrete-time-signal]]
- [[concepts/laplace-transform]]
- [[concepts/digital-network-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-17-chapter-14-digital-and-switched-capacitor-networks]]
