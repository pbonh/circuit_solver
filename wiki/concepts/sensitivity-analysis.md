---
title: Sensitivity Analysis
type: claim
id: claim-sensitivity-analysis
tags:
- sensitivity
- analog
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt
confidence:
  base: 0.85
---

## Definition

Sensitivity analysis quantifies how a network response (a voltage, current, transfer function value, pole location, etc.) changes in response to small variations in network parameters (element values, model parameters). In CAD it serves two complementary roles: tolerance/parasitic analysis and gradient supply for optimization.

## How It Works

Several formulations are used in Vlach and Singhal's text:
- **Direct (incremental) method**: differentiate the network equations with respect to the parameter and solve the resulting linear system.
- **Adjoint method** (Tellegen-based): one extra solve of an adjoint network yields sensitivities to all parameters of a chosen output.
- **Symbolic method**: derive the network function symbolically and differentiate analytically.
- **Large-change sensitivity**: a related family that captures non-infinitesimal parameter perturbations.

Sensitivities can be calculated for amplitude and phase of frequency-domain response, for poles and zeros, and for time-domain quantities (Chapter 16).

## Key Parameters

- Choice of output (scalar vs. vector, frequency vs. time).
- Choice of method (direct vs. adjoint vs. symbolic).
- Order of derivative (first, second, higher) — relevant in symbolic differentiation.
- Linear vs. nonlinear network (nonlinear case requires sensitivity of the operating point and of the linearization).

## When To Use

- Tolerance budgeting: identifying which components require tight manufacturing control.
- Parasitic-influence studies, very difficult on the bench but easy in CAD.
- Optimization: providing gradients of the objective function to algorithms such as SQP.

## Risks & Pitfalls

- First-order sensitivities are only locally valid; large parameter changes need large-change sensitivity techniques.
- Numerical sensitivity via finite differences is noisy and parameter-step-sensitive; analytical or adjoint methods are preferred.
- Sensitivities of poles can be ill-defined for repeated poles.

## Related Concepts

- [[concepts/adjoint-method]]
- [[concepts/symbolic-analysis]]
- [[concepts/large-change-sensitivity]]
- [[concepts/optimization-theory]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
- [[summaries/computer-methods-circuit-analysis-design-11-chapter-8-large-change-sensitivity-and-related-topics]]
- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
