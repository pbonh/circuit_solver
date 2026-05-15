---
title: "Piecewise-Linear Analysis (Katzenelson)"
type: concept
tags: [dc, analog, well-established, device-model]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/15-chapter-12-dc-solution-of-networks.txt"]
confidence: medium
---

## Definition

Piecewise-linear (PWL) analysis replaces each nonlinear device curve i = f(v) by a piecewise-linear approximation with breakpoints at tabulated values. The DC solution is then computed by sequence-tracking which linear region each device is in, taking steps along the curve, and switching regions when breakpoints are crossed. The Katzenelson algorithm [1] is the classical PWL DC solver.

## How It Works

Each device has a finite set of linear segments. At any time, the network is a piecewise-linear system. The algorithm:
1. Start with all devices in some initial segment.
2. Compute the linear DC solution for the current segment configuration.
3. If any device has crossed a breakpoint, switch segments accordingly.
4. Repeat until all devices are in segments consistent with the solution.

Avoids Newton-Raphson convergence issues and overflow problems with exponentials.

## Key Parameters

- Number of breakpoints per device.
- Tolerance for crossing detection.
- Initial segment assignment.

## When To Use

- Devices specified by tabulated data (no analytic form).
- DC analysis where Newton-Raphson struggles with steep nonlinearities.
- Educational illustration of an alternative to Newton-Raphson.

## Risks & Pitfalls

- Many breakpoints inflate analysis cost.
- Discontinuities in derivatives at breakpoints can confuse subsequent analyses (e.g., AC, transient).
- Multiple DC solutions still require special continuation strategies.

## Related Concepts

- [[concepts/dc-analysis]]
- [[concepts/newton-raphson-method]]
- [[concepts/spline-approximation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks]]
