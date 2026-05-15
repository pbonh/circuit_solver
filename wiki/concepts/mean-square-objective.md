---
title: "Mean-Square Objective Function"
type: concept
tags: [optimization, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/20-chapter-17-design-by-minimization.txt"]
confidence: medium
---

## Definition

A mean-square objective function E = (1/m) sum_i w_i (|phi(omega_i)| - A_i)^2 (or its complex-valued counterpart) measures the L_2 deviation of a network response from target values at m sample frequencies (or time points). It is the most common CAD optimization objective.

## How It Works

For each frequency omega_i, the network is analyzed (one TX = W solve). The squared error is accumulated, and the gradient with respect to each design variable h is obtained via one adjoint solve per frequency:

dE/dh = Re((1/m) sum_i w_i (|phi_i| - A_i)|phi_i|/phi_i * (X^a)_i^T (dT/dh) X_i).

Weights w_i allow emphasis on selected frequencies.

## Key Parameters

- Frequency sample set {omega_i}.
- Target amplitudes A_i.
- Weights w_i.
- Number of variable elements.

## When To Use

- Filter design (matching to a desired magnitude response).
- Equalizer design.
- Most analog optimization problems.

## Risks & Pitfalls

- Optimizing only at sample frequencies may produce overshoot between samples; dense sampling required.
- Mean-square does not bound the worst error; use minimax for hard specifications.
- Local minima depend on weight choice.

## Related Concepts

- [[concepts/objective-function]]
- [[concepts/adjoint-method]]
- [[concepts/minimax-optimization]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
